use oxurack::{
    CableTransform, CoreError, LabelKind, LabelTarget, LabelValue, MidiDelivery, MidiOutModule,
    Module, Oxurack, ParameterValue, Patch, ScopeRecord, ScopeSelector, SimulationDriver, Value,
};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_lfo::LfoModule;
use oxurack_mod_turingmachine::TuringMachineModule;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static LATE_HOOK_CALLS: AtomicUsize = AtomicUsize::new(0);

#[derive(bevy_ecs::prelude::Resource)]
struct CandidateMarker;

struct LateFailure;

#[derive(bevy_ecs::prelude::Resource, Default)]
struct HookOrder(Vec<u8>);

#[derive(bevy_ecs::prelude::Resource, Default)]
struct HookTicks(usize);

struct FirstHook;
struct SecondHook;
struct ConflictingHook;

fn count_hook_tick(mut count: bevy_ecs::prelude::ResMut<HookTicks>) {
    count.0 += 1;
}

macro_rules! hook_module {
    ($type:ty, $kind:literal, $value:literal) => {
        impl Module for $type {
            const KIND: &'static str = $kind;
            const DISPLAY_NAME: &'static str = $kind;

            fn port_schema() -> &'static [oxurack::PortSchema] {
                &[]
            }
            fn parameter_schema() -> &'static [oxurack::ParameterSchema] {
                &[]
            }
            fn spawn(
                world: &mut bevy_ecs::world::World,
                _instance_name: &str,
                _parameters: &HashMap<String, ParameterValue>,
            ) -> Result<bevy_ecs::entity::Entity, CoreError> {
                Ok(world.spawn_empty().id())
            }
            fn register_systems(
                app: &mut bevy_app::App,
                schedule: bevy_ecs::schedule::InternedScheduleLabel,
            ) {
                app.world_mut().init_resource::<HookOrder>();
                app.world_mut().resource_mut::<HookOrder>().0.push($value);
                app.world_mut().init_resource::<HookTicks>();
                app.add_systems(schedule, count_hook_tick);
            }
        }
    };
}

hook_module!(FirstHook, "first_hook", 1);
hook_module!(SecondHook, "second_hook", 2);
hook_module!(ConflictingHook, "first_hook", 9);

impl oxurack::Module for LateFailure {
    const KIND: &'static str = "late_failure";
    const DISPLAY_NAME: &'static str = "Late Failure";

    fn port_schema() -> &'static [oxurack::PortSchema] {
        &[]
    }

    fn parameter_schema() -> &'static [oxurack::ParameterSchema] {
        &[]
    }

    fn register_systems(
        _app: &mut bevy_app::App,
        _schedule: bevy_ecs::schedule::InternedScheduleLabel,
    ) {
        LATE_HOOK_CALLS.fetch_add(1, Ordering::SeqCst);
    }

    fn spawn(
        world: &mut bevy_ecs::world::World,
        instance_name: &str,
        _parameters: &HashMap<String, ParameterValue>,
    ) -> Result<bevy_ecs::entity::Entity, CoreError> {
        world.insert_resource(CandidateMarker);
        let entity = world.spawn_empty().id();
        world.write_message(oxurack_core::ModuleAdded {
            entity,
            instance_name: instance_name.to_owned(),
            kind: Self::KIND.into(),
        });
        Err(CoreError::NotImplemented("intentional late spawn failure"))
    }
}

const MIDI_PATCH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/patches/turing-machine-midi.ron"
);
const LFO_PATCH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/patches/turing-machine-lfo.ron"
);

fn rack() -> Oxurack {
    let mut rack = Oxurack::new();
    rack.try_install::<ClockModule>().unwrap();
    rack.try_install::<LfoModule>().unwrap();
    rack.try_install::<TuringMachineModule>().unwrap();
    rack.try_install::<MidiOutModule>().unwrap();
    rack.set_driver_mode(SimulationDriver::Manual);
    rack
}

#[test]
fn lifecycle_admission_precedence_and_candidate_replacement() {
    let mut rack = rack();
    rack.load_patch(std::path::Path::new(MIDI_PATCH)).unwrap();
    rack.start();
    assert!(matches!(
        rack.load_patch(std::path::Path::new("/missing")),
        Err(CoreError::LifecycleRequiresStopped)
    ));
    rack.stop();
    rack.load_patch(std::path::Path::new(MIDI_PATCH)).unwrap();
    assert_eq!(rack.sample_time().completed_samples(), 0);
    rack.load_patch(std::path::Path::new(MIDI_PATCH)).unwrap();
    assert_eq!(rack.sample_time().completed_samples(), 0);
    assert!(rack.snapshot_patch().is_ok());
    rack.unload_patch().unwrap();
    assert!(matches!(
        rack.snapshot_patch(),
        Err(CoreError::NoPatchLoaded)
    ));
    assert_eq!(rack.sample_time().completed_samples(), 0);
    rack.unload_patch().unwrap();
    rack.load_patch(std::path::Path::new(LFO_PATCH)).unwrap();
    assert!(rack.snapshot_patch().is_ok());
}

#[test]
fn reset_appends_channel_silence_and_restart_uses_live_snapshot() {
    let mut rack = rack();
    rack.load_patch(std::path::Path::new(MIDI_PATCH)).unwrap();
    rack.configure_scope(&[ScopeSelector::new("tm_main", "note")], 64)
        .unwrap();
    rack.start();
    for _ in 0..300 {
        rack.advance_samples(1).unwrap();
        for _event in rack.drain_midi() {}
    }
    let before = rack.sample_time();
    let scope_before: Vec<_> = rack.scope().unwrap().records().copied().collect();
    rack.reset().unwrap();
    assert_eq!(
        rack.scope().unwrap().records().copied().collect::<Vec<_>>(),
        scope_before
    );
    let cleanup: Vec<_> = rack.drain_midi().collect();
    assert!(
        cleanup
            .iter()
            .all(|event| event.sample == before.completed_samples())
    );
    assert!(
        cleanup
            .iter()
            .any(|event| event.message.to_bytes() == [0xB0, 123, 0])
    );
    assert!(
        cleanup
            .iter()
            .any(|event| event.message.to_bytes() == [0xB0, 120, 0])
    );
    assert_eq!(rack.sample_time(), before);
    rack.reset().unwrap();
    assert!(rack.drain_midi().next().is_none());
    assert_eq!(
        rack.advance_samples(1).unwrap(),
        1,
        "reset preserved running state"
    );
    for _event in rack.drain_midi() {}
    rack.stop();
    let restart_input = std::env::temp_dir().join(format!(
        "oxurack-live-snapshot-input-{}.ron",
        std::process::id()
    ));
    std::fs::copy(MIDI_PATCH, &restart_input).unwrap();
    rack.load_patch(&restart_input).unwrap();
    std::fs::remove_file(&restart_input).unwrap();
    rack.set_parameter(
        "tm_main",
        "write_probability",
        oxurack::ParameterValue::Float(0.25),
    )
    .unwrap();
    rack.restart().unwrap();
    assert_eq!(rack.sample_time().completed_samples(), 0);
    assert_eq!(rack.driver_mode(), SimulationDriver::Manual);
    assert!(rack.scope().is_none());
    assert_eq!(
        rack.parameter("tm_main", "write_probability").unwrap(),
        oxurack::ParameterValue::Float(0.25)
    );
}

#[test]
fn stopped_pending_output_blocks_replacement_without_clearing_events() {
    let mut rack = rack();
    rack.load_patch(std::path::Path::new(MIDI_PATCH)).unwrap();
    rack.start();
    for _ in 0..300 {
        rack.advance_samples(1).unwrap();
        for _event in rack.drain_midi() {}
    }
    rack.reset().unwrap();
    rack.stop();
    let before = rack.sample_time();
    let result = rack.load_patch(std::path::Path::new("/missing"));
    assert!(matches!(result, Err(CoreError::PendingMidi { count }) if count > 0));
    assert_eq!(rack.sample_time(), before);
    let pending: Vec<_> = rack.drain_midi().collect();
    assert!(!pending.is_empty());
    assert_eq!(rack.snapshot_patch().unwrap().master_seed, 42);
}

#[test]
fn stopped_pending_cleanup_precedes_load_restart_and_unload_and_preserves_old_owner() {
    for operation in ["load", "restart", "unload"] {
        let mut rack = rack();
        rack.load_patch(std::path::Path::new(MIDI_PATCH)).unwrap();
        rack.configure_scope(&[ScopeSelector::new("tm_main", "note")], 512)
            .unwrap();
        rack.start();
        for running_result in [
            rack.load_patch(std::path::Path::new("/missing-running-first")),
            rack.restart(),
            rack.unload_patch(),
        ] {
            assert!(matches!(
                running_result,
                Err(CoreError::LifecycleRequiresStopped)
            ));
            assert_eq!(rack.sample_time().completed_samples(), 0);
        }
        for _ in 0..300 {
            rack.advance_samples(1).unwrap();
            for _event in rack.drain_midi() {}
        }
        rack.reset().unwrap();
        rack.stop();
        let snapshot = rack.snapshot_patch().unwrap();
        let sample = rack.sample_time();
        let scope: Vec<_> = rack.scope().unwrap().records().copied().collect();
        let pending = match operation {
            "load" => rack.load_patch(std::path::Path::new("/missing-after-pending")),
            "restart" => rack.restart(),
            "unload" => rack.unload_patch(),
            _ => unreachable!(),
        };
        let expected_count = match pending {
            Err(CoreError::PendingMidi { count }) => count,
            result => {
                panic!("{operation} must reject unread cleanup before other work: {result:?}")
            }
        };
        assert!(expected_count > 0);
        assert_eq!(rack.snapshot_patch().unwrap(), snapshot);
        assert_eq!(rack.sample_time(), sample);
        assert_eq!(
            rack.scope().unwrap().records().copied().collect::<Vec<_>>(),
            scope
        );
        assert_eq!(rack.drain_midi().count(), expected_count);
        match operation {
            "load" => rack.load_patch(std::path::Path::new(MIDI_PATCH)).unwrap(),
            "restart" => rack.restart().unwrap(),
            "unload" => rack.unload_patch().unwrap(),
            _ => unreachable!(),
        }
        assert_eq!(rack.sample_time().completed_samples(), 0);
        assert!(rack.scope().is_none());
    }
}

#[test]
fn candidate_sink_reservations_are_isolated_from_the_live_owner() {
    let mut rack = rack();
    rack.load_patch(std::path::Path::new(MIDI_PATCH)).unwrap();
    let source = std::fs::read_to_string(MIDI_PATCH).unwrap();
    let mut patch: Patch = oxurack_core::deserialize_patch(&source).unwrap();
    let mut second_sink = patch
        .modules
        .iter()
        .find(|module| module.kind == "midi_out")
        .unwrap()
        .clone();
    second_sink.instance_name = "midi_second".to_owned();
    patch.modules.push(second_sink);
    patch
        .modules
        .iter_mut()
        .find(|module| module.kind == "midi_out" && module.instance_name == "midi_main")
        .unwrap()
        .parameters
        .insert("channel".into(), ParameterValue::Int(1));
    let path = std::env::temp_dir().join(format!(
        "oxurack-lifecycle-candidate-{}.ron",
        std::process::id()
    ));
    oxurack_core::save_patch_to_file(&patch, &path).unwrap();
    rack.load_patch(&path).unwrap();
    let live = rack.snapshot_patch().unwrap();
    assert_eq!(
        live.modules
            .iter()
            .filter(|module| module.kind == "midi_out")
            .count(),
        2
    );

    patch
        .modules
        .iter_mut()
        .find(|module| module.instance_name == "midi_second")
        .unwrap()
        .parameters
        .insert("channel".into(), ParameterValue::Int(1));
    oxurack_core::save_patch_to_file(&patch, &path).unwrap();
    assert!(rack.load_patch(&path).is_err());
    assert_eq!(rack.snapshot_patch().unwrap(), live);

    let mut single = oxurack_core::deserialize_patch(&source).unwrap();
    single
        .modules
        .iter_mut()
        .find(|module| module.kind == "midi_out")
        .unwrap()
        .parameters
        .insert("channel".into(), ParameterValue::Int(1));
    oxurack_core::save_patch_to_file(&single, &path).unwrap();
    rack.load_patch(&path).unwrap();
    assert_eq!(
        rack.snapshot_patch()
            .unwrap()
            .modules
            .iter()
            .filter(|module| module.kind == "midi_out")
            .count(),
        1
    );
    let _ = std::fs::remove_file(path);
}

#[test]
fn late_spawner_failure_isolated_and_old_engine_continues() {
    LATE_HOOK_CALLS.store(0, Ordering::SeqCst);
    let mut subject = rack();
    let mut control = rack();
    subject.try_install::<LateFailure>().unwrap();
    subject.load_patch(std::path::Path::new(LFO_PATCH)).unwrap();
    control.load_patch(std::path::Path::new(LFO_PATCH)).unwrap();
    for current in [&mut subject, &mut control] {
        current
            .configure_scope(&[ScopeSelector::new("lfo_main", "bipolar_out")], 256)
            .unwrap();
        current.start();
    }
    for _ in 0..200 {
        for current in [&mut subject, &mut control] {
            current.advance_samples(1).unwrap();
            for _event in current.drain_midi() {}
        }
    }
    subject.stop();
    control.stop();
    let snapshot = subject.snapshot_patch().unwrap();
    let sample = subject.sample_time();
    let records = subject.scope().unwrap().records().count();
    let mut candidate: Patch =
        oxurack_core::deserialize_patch(&std::fs::read_to_string(LFO_PATCH).unwrap()).unwrap();
    let mut failed_sink = oxurack::ModuleConfig {
        kind: MidiOutModule::KIND.to_owned(),
        instance_name: "candidate_midi".to_owned(),
        ..Default::default()
    };
    failed_sink
        .parameters
        .insert("channel".into(), ParameterValue::Int(1));
    candidate.modules.push(failed_sink);
    candidate.modules.push(oxurack::ModuleConfig {
        kind: LateFailure::KIND.to_owned(),
        instance_name: "late_failure_instance".to_owned(),
        ..Default::default()
    });
    let path =
        std::env::temp_dir().join(format!("oxurack-late-failure-{}.ron", std::process::id()));
    oxurack_core::save_patch_to_file(&candidate, &path).unwrap();
    assert!(matches!(
        subject.load_patch(&path),
        Err(CoreError::NotImplemented("intentional late spawn failure"))
    ));
    assert_eq!(LATE_HOOK_CALLS.load(Ordering::SeqCst), 3);
    assert_eq!(subject.snapshot_patch().unwrap(), snapshot);
    assert_eq!(subject.sample_time(), sample);
    assert_eq!(subject.scope().unwrap().records().count(), records);
    let _ = std::fs::remove_file(path);

    let a = trace(&mut subject, 1000);
    let b = trace(&mut control, 1000);
    assert_eq!(a, b);

    let mut recovered: Patch =
        oxurack_core::deserialize_patch(&std::fs::read_to_string(LFO_PATCH).unwrap()).unwrap();
    let mut recovered_sink = oxurack::ModuleConfig {
        kind: MidiOutModule::KIND.to_owned(),
        instance_name: "candidate_midi".to_owned(),
        ..Default::default()
    };
    recovered_sink
        .parameters
        .insert("channel".into(), ParameterValue::Int(1));
    recovered.modules.push(recovered_sink);
    let recovered_path =
        std::env::temp_dir().join(format!("oxurack-late-recovery-{}.ron", std::process::id()));
    oxurack_core::save_patch_to_file(&recovered, &recovered_path).unwrap();
    subject.load_patch(&recovered_path).unwrap();
    assert_eq!(LATE_HOOK_CALLS.load(Ordering::SeqCst), 4);
    let recovered_snapshot = subject.snapshot_patch().unwrap();
    assert_eq!(recovered_snapshot.modules.len(), 5);
    assert_eq!(
        recovered_snapshot
            .modules
            .iter()
            .find(|module| module.instance_name == "candidate_midi")
            .unwrap()
            .parameters
            .get("channel"),
        Some(&ParameterValue::Int(1))
    );
    let _ = std::fs::remove_file(recovered_path);
    let subject_app = subject.into_app();
    let _control_app = control.into_app();
    assert!(
        subject_app
            .world()
            .get_resource::<CandidateMarker>()
            .is_none()
    );
}

#[test]
fn install_recipes_keep_order_reinstall_and_collision_contract() {
    let mut rack = rack();
    rack.try_install::<FirstHook>().unwrap();
    rack.try_install::<SecondHook>().unwrap();
    rack.try_install::<FirstHook>().unwrap();
    assert!(matches!(
        rack.try_install::<ConflictingHook>(),
        Err(CoreError::ModuleKindConflict { .. })
    ));
    rack.load_patch(std::path::Path::new(LFO_PATCH)).unwrap();
    rack.set_driver_mode(SimulationDriver::Manual).start();
    rack.advance_samples(1).unwrap();
    for _event in rack.drain_midi() {}
    rack.stop();
    let app = rack.into_app();
    assert_eq!(app.world().resource::<HookOrder>().0, vec![1, 2]);
    assert_eq!(app.world().resource::<HookTicks>().0, 2);
}

#[test]
fn repeated_real_patch_replacement_matches_fresh_structure_and_graph() {
    type PortIndexProjection = Vec<(String, String)>;
    type ModuleIndexProjection = Vec<(String, String, PortIndexProjection)>;

    #[derive(Debug, PartialEq)]
    struct StructureProjection {
        modules: Vec<(String, String, u64)>,
        ports: Vec<(String, String, String)>,
        cables: Vec<(
            String,
            String,
            String,
            String,
            Option<CableTransform>,
            usize,
        )>,
        handles: Vec<(String, String)>,
        index_from: ModuleIndexProjection,
        index_to: ModuleIndexProjection,
        #[cfg(feature = "graph")]
        graph_modules: Vec<(String, String)>,
        #[cfg(feature = "graph")]
        graph_cables: Vec<(String, String, String, String)>,
        #[cfg(feature = "graph")]
        graph_membership: (Vec<bool>, Vec<bool>),
    }

    fn structure(rack: Oxurack) -> StructureProjection {
        let mut app = rack.into_app();
        let world = app.world_mut();
        let mut modules: Vec<_> = {
            let mut query = world.query::<(bevy_ecs::entity::Entity, &oxurack_core::Module)>();
            query
                .iter(world)
                .map(|(entity, module)| {
                    (
                        module.instance_name.clone(),
                        module.kind.to_string(),
                        entity.to_bits(),
                    )
                })
                .collect()
        };
        let port_entities: Vec<_> = {
            let mut query = world.query::<(
                bevy_ecs::entity::Entity,
                &oxurack_core::Port,
                &bevy_ecs::hierarchy::ChildOf,
            )>();
            query
                .iter(world)
                .map(|(entity, port, parent)| {
                    let module = world.get::<oxurack_core::Module>(parent.parent()).unwrap();
                    (
                        entity,
                        module.instance_name.clone(),
                        port.name.as_ref().to_owned(),
                        format!("{:?}:{:?}", port.direction, port.value_kind),
                    )
                })
                .collect()
        };
        let mut ports: Vec<_> = port_entities
            .iter()
            .map(|(_, module, port, kind)| (module.clone(), port.clone(), kind.clone()))
            .collect();
        let cable_entities: Vec<_> = {
            let mut query = world.query::<(&oxurack_core::Cable, &oxurack_core::CableOrder)>();
            query
                .iter(world)
                .map(|(cable, order)| {
                    let source = world.get::<oxurack_core::Port>(cable.source_port).unwrap();
                    let source_module = world
                        .get::<oxurack_core::Module>(
                            world
                                .get::<bevy_ecs::hierarchy::ChildOf>(cable.source_port)
                                .unwrap()
                                .parent(),
                        )
                        .unwrap();
                    let target = world.get::<oxurack_core::Port>(cable.target_port).unwrap();
                    let target_module = world
                        .get::<oxurack_core::Module>(
                            world
                                .get::<bevy_ecs::hierarchy::ChildOf>(cable.target_port)
                                .unwrap()
                                .parent(),
                        )
                        .unwrap();
                    (
                        source_module.instance_name.clone(),
                        source.name.as_ref().to_owned(),
                        target_module.instance_name.clone(),
                        target.name.as_ref().to_owned(),
                        cable.transform,
                        order.0,
                    )
                })
                .collect()
        };
        let mut cables = cable_entities;
        let mut handles: Vec<_> = world
            .resource::<oxurack_core::PatchHandle>()
            .modules
            .iter()
            .map(|(name, entity)| {
                (
                    name.clone(),
                    world
                        .get::<oxurack_core::Module>(*entity)
                        .unwrap()
                        .kind
                        .to_string(),
                )
            })
            .collect();
        let index = world.resource::<oxurack_core::CableIndex>();
        let mut index_from: Vec<_> = port_entities
            .iter()
            .map(|(entity, module, port, _)| {
                let endpoints = index
                    .cables_from(*entity)
                    .iter()
                    .map(|cable_entity| {
                        let cable = world.get::<oxurack_core::Cable>(*cable_entity).unwrap();
                        let target = world.get::<oxurack_core::Port>(cable.target_port).unwrap();
                        let target_module = world
                            .get::<oxurack_core::Module>(
                                world
                                    .get::<bevy_ecs::hierarchy::ChildOf>(cable.target_port)
                                    .unwrap()
                                    .parent(),
                            )
                            .unwrap();
                        (
                            target_module.instance_name.clone(),
                            target.name.as_ref().to_owned(),
                        )
                    })
                    .collect();
                (module.clone(), port.clone(), endpoints)
            })
            .collect();
        let mut index_to: Vec<_> = port_entities
            .iter()
            .map(|(entity, module, port, _)| {
                let endpoints = index
                    .cables_targeting(*entity)
                    .iter()
                    .map(|cable_entity| {
                        let cable = world.get::<oxurack_core::Cable>(*cable_entity).unwrap();
                        let source = world.get::<oxurack_core::Port>(cable.source_port).unwrap();
                        let source_module = world
                            .get::<oxurack_core::Module>(
                                world
                                    .get::<bevy_ecs::hierarchy::ChildOf>(cable.source_port)
                                    .unwrap()
                                    .parent(),
                            )
                            .unwrap();
                        (
                            source_module.instance_name.clone(),
                            source.name.as_ref().to_owned(),
                        )
                    })
                    .collect();
                (module.clone(), port.clone(), endpoints)
            })
            .collect();
        #[cfg(feature = "graph")]
        let cable_entity_ids: Vec<_> = {
            let mut query = world.query::<(bevy_ecs::entity::Entity, &oxurack_core::Cable)>();
            query.iter(world).map(|(entity, _)| entity).collect()
        };
        #[cfg(feature = "graph")]
        let (graph_modules, graph_cables, graph_membership) = {
            let view = oxurack_graph::GraphView::new(world);
            let graph_modules = view
                .modules()
                .into_iter()
                .map(|module| (module.instance_name, module.kind))
                .collect();
            let graph_cables = view
                .cables()
                .into_iter()
                .map(|cable| {
                    (
                        cable.source_module_instance,
                        cable.source_port_name,
                        cable.target_module_instance,
                        cable.target_port_name,
                    )
                })
                .collect();
            let graph_membership = (
                world
                    .resource::<oxurack_core::PatchHandle>()
                    .modules
                    .values()
                    .map(|entity| view.contains_module(*entity))
                    .collect(),
                cable_entity_ids
                    .iter()
                    .map(|entity| view.contains_cable(*entity))
                    .collect(),
            );
            for (instance_name, entity) in &world.resource::<oxurack_core::PatchHandle>().modules {
                let port_set = view.ports_of(*entity);
                let actual_inputs: Vec<_> = port_set
                    .inputs
                    .iter()
                    .map(|port| (port.name.clone(), port.value_kind.clone()))
                    .collect();
                let actual_outputs: Vec<_> = port_set
                    .outputs
                    .iter()
                    .map(|port| (port.name.clone(), port.value_kind.clone()))
                    .collect();
                let mut expected_inputs: Vec<_> = port_entities
                    .iter()
                    .filter(|(_, module, _, kind)| {
                        module == instance_name && kind.starts_with("Input:")
                    })
                    .map(|(_, _, port, kind)| {
                        (port.clone(), kind.trim_start_matches("Input:").to_owned())
                    })
                    .collect();
                let mut expected_outputs: Vec<_> = port_entities
                    .iter()
                    .filter(|(_, module, _, kind)| {
                        module == instance_name && kind.starts_with("Output:")
                    })
                    .map(|(_, _, port, kind)| {
                        (port.clone(), kind.trim_start_matches("Output:").to_owned())
                    })
                    .collect();
                expected_inputs.sort();
                expected_outputs.sort();
                assert_eq!(
                    actual_inputs, expected_inputs,
                    "GraphView input ports for {instance_name}"
                );
                assert_eq!(
                    actual_outputs, expected_outputs,
                    "GraphView output ports for {instance_name}"
                );
            }
            (graph_modules, graph_cables, graph_membership)
        };
        modules.sort();
        ports.sort();
        cables.sort_by_key(|cable| {
            (
                cable.0.clone(),
                cable.1.clone(),
                cable.2.clone(),
                cable.3.clone(),
                cable.5,
            )
        });
        handles.sort();
        index_from.sort_by_key(|entry| (entry.0.clone(), entry.1.clone()));
        index_to.sort_by_key(|entry| (entry.0.clone(), entry.1.clone()));
        StructureProjection {
            modules,
            ports,
            cables,
            handles,
            index_from,
            index_to,
            #[cfg(feature = "graph")]
            graph_modules,
            #[cfg(feature = "graph")]
            graph_cables,
            #[cfg(feature = "graph")]
            graph_membership,
        }
    }

    let mut replacement = rack();
    replacement
        .load_patch(std::path::Path::new(MIDI_PATCH))
        .unwrap();
    replacement.start();
    for _ in 0..300 {
        replacement.advance_samples(1).unwrap();
        for _event in replacement.drain_midi() {}
    }
    replacement.stop();
    replacement
        .load_patch(std::path::Path::new(MIDI_PATCH))
        .unwrap();
    for _event in replacement.drain_midi() {}
    replacement
        .load_patch(std::path::Path::new(MIDI_PATCH))
        .unwrap();
    for _event in replacement.drain_midi() {}
    let mut fresh = rack();
    fresh.load_patch(std::path::Path::new(MIDI_PATCH)).unwrap();
    assert_eq!(structure(replacement), structure(fresh));
}

#[test]
fn edited_saved_lifetime_replay_matches_fresh_trace_and_scope() {
    let saved = std::env::temp_dir().join(format!(
        "oxurack-lifecycle-edited-{}.ron",
        std::process::id()
    ));
    let mut replay = rack();
    replay.load_patch(std::path::Path::new(LFO_PATCH)).unwrap();
    replay.start();
    for _ in 0..200 {
        replay.advance_samples(1).unwrap();
        for _event in replay.drain_midi() {}
    }
    replay
        .set_parameter("lfo_main", "frequency_hz", ParameterValue::Float(1.0))
        .unwrap();
    replay
        .set_parameter("tm_main", "write_probability", ParameterValue::Float(0.25))
        .unwrap();
    let source = ("lfo_main", "bipolar_out");
    let target = ("tm_main", "write_modulation");
    replay.unpatch(source, target, 0).unwrap();
    let occurrence = replay
        .patch(
            source,
            target,
            Some(CableTransform::Affine {
                factor: 0.0,
                offset: 0.25,
            }),
        )
        .unwrap();
    replay
        .set_label(
            &LabelTarget::Cable {
                source: (source.0.into(), source.1.into()),
                target: (target.0.into(), target.1.into()),
                occurrence,
            },
            LabelKind::Description,
            LabelValue::Text("Constant +0.25 write modulation".into()),
        )
        .unwrap();
    replay.save_patch(&saved).unwrap();
    let snapshot = replay.snapshot_patch().unwrap();
    assert_eq!(
        snapshot
            .modules
            .iter()
            .find(|module| module.instance_name == "tm_main")
            .unwrap()
            .parameters
            .get("write_probability"),
        Some(&ParameterValue::Float(0.25))
    );
    let edited_cable = snapshot
        .cables
        .iter()
        .find(|cable| cable.source == (source.0.into(), source.1.into()))
        .unwrap();
    assert_eq!(
        edited_cable.transform,
        Some(CableTransform::Affine {
            factor: 0.0,
            offset: 0.25
        })
    );
    assert_eq!(edited_cable.description, "Constant +0.25 write modulation");
    for _ in 0..100 {
        replay.advance_samples(1).unwrap();
        for _event in replay.drain_midi() {}
    }
    replay.stop();
    replay.unload_patch().unwrap();
    for _event in replay.drain_midi() {}
    replay.load_patch(&saved).unwrap();

    let selectors = [
        ScopeSelector::new("lfo_main", "bipolar_out"),
        ScopeSelector::new("tm_main", "write_probability"),
    ];
    replay.configure_scope(&selectors, 21_000).unwrap();
    let (trace, signals, copied_midi, overwritten) = capture_replay(&mut replay);
    assert_eq!(signals.len(), 20_000);
    assert_eq!(overwritten, 0);
    assert_eq!(
        signals.iter().take(4).copied().collect::<Vec<_>>(),
        vec![
            (0, 0, Value::Bipolar(0.0)),
            (0, 1, Value::Float(0.0)),
            (1, 0, signals[2].2),
            (1, 1, Value::Float(0.5)),
        ]
    );
    for sample in 0..10_000 {
        assert_eq!(signals[sample as usize * 2].0, sample);
        assert_eq!(signals[sample as usize * 2].1, 0);
        assert_eq!(
            signals[sample as usize * 2 + 1],
            (sample, 1, Value::Float(if sample == 0 { 0.0 } else { 0.5 }),)
        );
    }
    assert_eq!(copied_midi.len(), trace.len());
    assert_eq!(
        copied_midi,
        trace
            .iter()
            .map(|(sample, bytes)| (*sample, *bytes, MidiDelivery::Generated))
            .collect::<Vec<_>>()
    );

    let mut fresh = rack();
    fresh.load_patch(&saved).unwrap();
    fresh.configure_scope(&selectors, 21_000).unwrap();
    let (fresh_trace, fresh_signals, fresh_midi, fresh_overwritten) = capture_replay(&mut fresh);
    assert_eq!(trace, fresh_trace);
    assert_eq!(signals, fresh_signals);
    assert_eq!(fresh_overwritten, 0);
    assert_eq!(trace.len(), 72);
    assert_eq!(copied_midi, fresh_midi);
    let _ = std::fs::remove_file(saved);
}

type MidiTrace = Vec<(u64, [u8; 3])>;
type SignalTrace = Vec<(u64, usize, Value)>;
type ScopeMidiTrace = Vec<(u64, [u8; 3], MidiDelivery)>;
type ReplayCapture = (MidiTrace, SignalTrace, ScopeMidiTrace, u64);

fn capture_replay(rack: &mut Oxurack) -> ReplayCapture {
    rack.start();
    let mut trace = Vec::new();
    let mut drained = Vec::with_capacity(128);
    for _ in 0..10_000 {
        rack.advance_samples(1).unwrap();
        drained.clear();
        drained.extend(rack.drain_midi());
        for event in &drained {
            trace.push((event.sample, event.message.to_bytes()));
            rack.record_scope_midi(*event, MidiDelivery::Generated);
        }
    }
    rack.stop();
    let capture = rack.scope().unwrap();
    let signals = capture
        .records()
        .filter_map(|record| match record {
            ScopeRecord::Signal {
                sample,
                selection,
                value,
            } => Some((*sample, *selection, *value)),
            ScopeRecord::Midi { .. } => None,
            _ => None,
        })
        .collect();
    let copied_midi = capture
        .records()
        .filter_map(|record| match record {
            ScopeRecord::Midi { event, delivery } => {
                Some((event.sample, event.message.to_bytes(), *delivery))
            }
            ScopeRecord::Signal { .. } => None,
            _ => None,
        })
        .collect();
    (trace, signals, copied_midi, capture.overwritten())
}

fn trace(rack: &mut Oxurack, count: u64) -> Vec<(u64, [u8; 3])> {
    rack.start();
    let mut events = Vec::new();
    for _ in 0..count {
        rack.advance_samples(1).unwrap();
        events.extend(
            rack.drain_midi()
                .map(|event| (event.sample, event.message.to_bytes())),
        );
    }
    rack.stop();
    events
}
