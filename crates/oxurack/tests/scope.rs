use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use bevy_app::{App, FixedUpdate};
use bevy_ecs::prelude::{Entity, Query, ResMut, World};
use bevy_ecs::schedule::{InternedScheduleLabel, IntoScheduleConfigs};
use oxurack::{
    MidiDelivery, MidiOutModule, ModuleConfig, Oxurack, ParameterValue, Patch, PortDirection,
    ScopeError, ScopeRecord, ScopeSelector, SimulationDriver, SimulationRunning, TickPhase, Value,
    ValueKind,
};
use oxurack_core::{
    CurrentValue, MergePolicy, OxurackModule, Port, PortSchema, deserialize_patch,
    save_patch_to_file, spawn_module_entity, spawn_port_on_module,
};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_lfo::LfoModule;
use oxurack_mod_turingmachine::TuringMachineModule;

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/patches/turing-machine-lfo.ron"
);
static NEXT: AtomicU64 = AtomicU64::new(0);

struct RawTestModule;

impl OxurackModule for RawTestModule {
    const KIND: &'static str = "scope_raw_test";
    const DISPLAY_NAME: &'static str = "Scope Raw Test";

    fn port_schema() -> &'static [PortSchema] {
        static PORTS: [PortSchema; 2] = [
            PortSchema {
                name: "value",
                direction: PortDirection::Output,
                value_kind: ValueKind::Raw,
                merge_policy: MergePolicy::Reject,
                description: "Test-only raw signal.",
            },
            PortSchema {
                name: "midi",
                direction: PortDirection::Output,
                value_kind: ValueKind::Midi,
                merge_policy: MergePolicy::Reject,
                description: "Test-only unsupported MIDI signal.",
            },
        ];
        &PORTS
    }

    fn parameter_schema() -> &'static [oxurack::ParameterSchema] {
        &[]
    }

    fn spawn(
        world: &mut World,
        instance_name: &str,
        _parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, oxurack::CoreError> {
        let module = spawn_module_entity(world, Self::KIND, instance_name);
        spawn_port_on_module(
            world,
            module,
            "value",
            PortDirection::Output,
            ValueKind::Raw,
            MergePolicy::Reject,
        );
        spawn_port_on_module(
            world,
            module,
            "midi",
            PortDirection::Output,
            ValueKind::Midi,
            MergePolicy::Reject,
        );
        Ok(module)
    }

    fn register_systems(app: &mut App, schedule: InternedScheduleLabel) {
        app.add_systems(schedule, publish_raw.in_set(TickPhase::Produce));
    }
}

fn publish_raw(mut ports: Query<(&Port, &mut CurrentValue)>) {
    for (port, mut value) in &mut ports {
        if port.name.as_ref() == "value" {
            value.0 = Value::Raw(17);
        }
    }
}

fn patch() -> Patch {
    let mut patch = deserialize_patch(&std::fs::read_to_string(FIXTURE).unwrap()).unwrap();
    for module in &mut patch.modules {
        match module.instance_name.as_str() {
            "clock_main" => {
                module
                    .parameters
                    .insert("bpm".into(), ParameterValue::Float(1000.0));
                module
                    .parameters
                    .insert("ppqn".into(), ParameterValue::Int(30));
                module
                    .parameters
                    .insert("pulse_width_ms".into(), ParameterValue::Int(1));
            }
            "lfo_main" => {
                module
                    .parameters
                    .insert("waveform".into(), ParameterValue::String("saw_up".into()));
                module
                    .parameters
                    .insert("frequency_hz".into(), ParameterValue::Float(62.5));
                module
                    .parameters
                    .insert("phase_offset".into(), ParameterValue::Float(0.0));
                module
                    .parameters
                    .insert("pulse_width".into(), ParameterValue::Float(0.5));
            }
            _ => {}
        }
    }
    patch.modules.push(ModuleConfig {
        kind: RawTestModule::KIND.into(),
        instance_name: "raw_main".into(),
        ..ModuleConfig::default()
    });
    patch
}

fn save_temp(patch: &Patch) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "oxurack-scope-{}-{}.ron",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    save_patch_to_file(patch, &path).unwrap();
    path
}

fn rack_with_patch(patch: &Patch) -> (Oxurack, std::path::PathBuf) {
    let path = save_temp(patch);
    let mut rack = Oxurack::new();
    rack.try_install::<ClockModule>().unwrap();
    rack.try_install::<LfoModule>().unwrap();
    rack.try_install::<TuringMachineModule>().unwrap();
    rack.try_install::<MidiOutModule>().unwrap();
    rack.try_install::<RawTestModule>().unwrap();
    rack.load_patch(&path).unwrap();
    rack.set_driver_mode(SimulationDriver::Manual);
    (rack, path)
}

fn selectors() -> Vec<ScopeSelector> {
    [
        ("clock_main", "pulse"),
        ("lfo_main", "bipolar_out"),
        ("lfo_main", "out"),
        ("tm_main", "write_modulation"),
        ("raw_main", "value"),
    ]
    .into_iter()
    .map(|(module, port)| ScopeSelector::new(module, port))
    .collect()
}

fn run_samples(rack: &mut Oxurack, count: usize) {
    rack.start();
    for _ in 0..count {
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        let _: Vec<_> = rack.drain_midi().collect();
    }
    rack.stop();
}

fn midi_trace(rack: &mut Oxurack, count: usize) -> Vec<(u64, [u8; 3])> {
    rack.start();
    let mut trace = Vec::new();
    for _ in 0..count {
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        let events: Vec<_> = rack.drain_midi().collect();
        for event in &events {
            rack.record_scope_midi(*event, MidiDelivery::Generated);
        }
        trace.extend(
            events
                .into_iter()
                .map(|event| (event.sample, event.message.to_bytes())),
        );
    }
    rack.stop();
    trace
}

fn stop_during_consume(mut running: ResMut<SimulationRunning>) {
    running.0 = false;
}

#[test]
fn observes_selected_loaded_ports_at_the_latched_sample_index() {
    let (mut rack, path) = rack_with_patch(&patch());
    rack.configure_scope(&selectors(), 64).unwrap();
    run_samples(&mut rack, 4);

    let values: Vec<_> = rack
        .scope()
        .unwrap()
        .records()
        .map(|record| match record {
            ScopeRecord::Signal {
                sample,
                selection,
                value,
            } => (*sample, *selection, *value),
            other => panic!("unexpected record: {other:?}"),
        })
        .collect();
    assert_eq!(values.len(), 20);
    for sample in 0..4 {
        let row = &values[sample as usize * 5..sample as usize * 5 + 5];
        assert_eq!(
            row.iter()
                .map(|(_, selection, _)| *selection)
                .collect::<Vec<_>>(),
            [0, 1, 2, 3, 4]
        );
        assert!(row.iter().all(|(index, _, _)| *index == sample));
        let (pulse, bipolar, unipolar, modulation, raw) =
            (row[0].2, row[1].2, row[2].2, row[3].2, row[4].2);
        assert_eq!(pulse, Value::Gate(sample % 2 == 0));
        assert_eq!(bipolar, Value::Bipolar(-1.0 + sample as f32 * 0.125));
        assert_eq!(unipolar, Value::Float(sample as f32 * 0.0625));
        assert_eq!(modulation, Value::Bipolar(-0.5 + sample as f32 * 0.0625));
        assert_eq!(raw, Value::Raw(17));
    }
    let _ = std::fs::remove_file(path);
}

#[test]
fn consume_stop_keeps_the_already_latched_sample_observation() {
    let (mut rack, path) = rack_with_patch(&patch());
    rack.configure_scope(&[ScopeSelector::new("lfo_main", "bipolar_out")], 8)
        .unwrap();
    let mut app = rack.into_app();
    app.add_systems(FixedUpdate, stop_during_consume.in_set(TickPhase::Consume));
    app.world_mut().resource_mut::<SimulationRunning>().0 = true;
    app.world_mut().run_schedule(FixedUpdate);
    app.world_mut().run_schedule(FixedUpdate);
    let capture = app.world().resource::<oxurack::ScopeCapture>();
    assert_eq!(capture.records().count(), 1);
    assert_eq!(
        capture.records().next(),
        Some(&ScopeRecord::Signal {
            sample: 0,
            selection: 0,
            value: Value::Bipolar(-1.0)
        })
    );
    assert_eq!(
        app.world()
            .resource::<oxurack::SampleTime>()
            .completed_samples(),
        1
    );
    let _ = std::fs::remove_file(path);
}

#[test]
fn capacity_clear_resume_and_mixed_midi_fifo_are_bounded() {
    let (mut rack, path) = rack_with_patch(&patch());
    rack.configure_scope(&[ScopeSelector::new("lfo_main", "bipolar_out")], 3)
        .unwrap();
    run_samples(&mut rack, 5);
    let capture = rack.scope().unwrap();
    assert_eq!(capture.overwritten(), 2);
    assert_eq!(
        capture.records().copied().collect::<Vec<_>>(),
        [
            ScopeRecord::Signal {
                sample: 2,
                selection: 0,
                value: Value::Bipolar(-0.75)
            },
            ScopeRecord::Signal {
                sample: 3,
                selection: 0,
                value: Value::Bipolar(-0.625)
            },
            ScopeRecord::Signal {
                sample: 4,
                selection: 0,
                value: Value::Bipolar(-0.5)
            },
        ]
    );
    assert_eq!(rack.advance_samples(1).unwrap(), 0);
    assert_eq!(rack.scope().unwrap().records().count(), 3);
    rack.clear_scope();
    assert_eq!(rack.scope().unwrap().overwritten(), 0);
    rack.start();
    assert_eq!(rack.advance_samples(1).unwrap(), 1);
    let _: Vec<_> = rack.drain_midi().collect();
    rack.stop();
    assert_eq!(
        rack.scope().unwrap().records().next(),
        Some(&ScopeRecord::Signal {
            sample: 5,
            selection: 0,
            value: Value::Bipolar(-0.375)
        })
    );

    let (mut mixed, mixed_path) = rack_with_patch(&patch());
    mixed
        .configure_scope(&[ScopeSelector::new("lfo_main", "bipolar_out")], 3)
        .unwrap();
    mixed.start();
    mixed.advance_samples(1).unwrap();
    let first: Vec<_> = mixed.drain_midi().collect();
    let mut event = first.first().copied().unwrap_or_else(|| {
        let (mut template, temp) = rack_with_patch(&patch());
        template.start();
        let event = (0..24)
            .find_map(|_| {
                template.advance_samples(1).unwrap();
                let events: Vec<_> = template.drain_midi().collect();
                events.into_iter().next()
            })
            .unwrap();
        template.stop();
        let _ = std::fs::remove_file(temp);
        event
    });
    event.sample = 0;
    mixed.record_scope_midi(event, MidiDelivery::Generated);
    mixed.record_scope_midi(event, MidiDelivery::Sent);
    mixed.advance_samples(1).unwrap();
    let _: Vec<_> = mixed.drain_midi().collect();
    mixed.stop();
    assert_eq!(mixed.scope().unwrap().overwritten(), 1);
    assert!(matches!(
        mixed.scope().unwrap().records().next(),
        Some(ScopeRecord::Midi {
            delivery: MidiDelivery::Generated,
            ..
        })
    ));
    assert!(matches!(
        mixed.scope().unwrap().records().nth(1),
        Some(ScopeRecord::Midi {
            delivery: MidiDelivery::Sent,
            ..
        })
    ));
    assert!(matches!(
        mixed.scope().unwrap().records().nth(2),
        Some(ScopeRecord::Signal { sample: 1, .. })
    ));
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(mixed_path);
}

#[test]
fn invalid_reconfiguration_preserves_capture_and_patch_lifecycle() {
    let mut fresh = Oxurack::new();
    assert!(matches!(
        fresh.configure_scope(&[], 1),
        Err(ScopeError::NoPatch)
    ));
    let (mut rack, path) = rack_with_patch(&patch());
    let valid = [ScopeSelector::new("lfo_main", "bipolar_out")];
    rack.configure_scope(&valid, 8).unwrap();
    rack.configure_scope(&valid, 9).unwrap();
    assert_eq!(rack.scope().unwrap().capacity(), 9);
    assert_eq!(rack.scope().unwrap().records().count(), 0);
    run_samples(&mut rack, 1);
    let before = rack.scope().unwrap().records().copied().collect::<Vec<_>>();
    for (bad, capacity) in [
        (vec![ScopeSelector::new("missing", "x")], 8),
        (vec![ScopeSelector::new("lfo_main", "missing")], 8),
        (vec![valid[0].clone(), valid[0].clone()], 8),
        (vec![ScopeSelector::new("raw_main", "midi")], 8),
        (vec![], 0),
        (vec![], usize::MAX),
    ] {
        assert!(rack.configure_scope(&bad, capacity).is_err());
        assert_eq!(
            rack.scope().unwrap().records().copied().collect::<Vec<_>>(),
            before
        );
    }
    rack.start();
    assert!(matches!(
        rack.configure_scope(&valid, 8),
        Err(ScopeError::Running)
    ));
    rack.stop();
    let missing_patch = std::env::temp_dir().join("oxurack-scope-no-such-patch.ron");
    assert!(rack.load_patch(&missing_patch).is_err());
    assert!(rack.scope().is_some());
    let before_disable = rack.sample_time();
    rack.disable_scope();
    rack.disable_scope();
    assert!(rack.scope().is_none());
    assert_eq!(rack.sample_time(), before_disable);
    rack.configure_scope(&valid, 8).unwrap();
    let mut replacement = patch();
    replacement
        .modules
        .iter_mut()
        .find(|module| module.instance_name == "midi_main")
        .unwrap()
        .parameters
        .insert("channel".into(), ParameterValue::Int(1));
    let replacement_path = save_temp(&replacement);
    rack.load_patch(&replacement_path).unwrap();
    assert!(rack.scope().is_none());
    let _ = std::fs::remove_file(path);
    let _ = std::fs::remove_file(replacement_path);
}

#[test]
fn scope_csv_schema_has_typed_signal_and_generated_midi_rows() {
    let (mut rack, path) = rack_with_patch(&patch());
    rack.configure_scope(&[ScopeSelector::new("lfo_main", "out")], 16)
        .unwrap();
    run_samples(&mut rack, 4);
    let mut csv = Vec::new();
    rack.scope().unwrap().write_csv(&mut csv).unwrap();
    let csv = String::from_utf8(csv).unwrap();
    assert!(csv.starts_with("sample,record,module,port,value_kind,value,status,data1,data2\n"));
    assert!(csv.contains("0,signal,lfo_main,out,float,0,,,\n"));
    assert_eq!(ScopeSelector::new("a", "b").module, "a");
    let _ = std::fs::remove_file(path);
}

#[test]
fn midi_only_capture_preserves_loaded_lfo_trace_and_never_marks_sent_offline() {
    let mut lfo_patch = deserialize_patch(&std::fs::read_to_string(FIXTURE).unwrap()).unwrap();
    lfo_patch.modules.push(ModuleConfig {
        kind: RawTestModule::KIND.into(),
        instance_name: "raw_main".into(),
        ..ModuleConfig::default()
    });
    let (mut baseline, baseline_path) = rack_with_patch(&lfo_patch);
    let baseline_trace = midi_trace(&mut baseline, 10_000);
    assert!(!baseline_trace.is_empty());

    let (mut captured, captured_path) = rack_with_patch(&lfo_patch);
    assert!(matches!(
        captured.configure_scope(&[ScopeSelector::new("raw_main", "midi")], 10_000),
        Err(ScopeError::UnsupportedKind {
            kind: ValueKind::Midi,
            ..
        })
    ));
    captured.configure_scope(&[], 10_000).unwrap();
    let captured_trace = midi_trace(&mut captured, 10_000);
    assert_eq!(captured_trace, baseline_trace);
    let generated: Vec<_> = captured
        .scope()
        .unwrap()
        .records()
        .map(|record| match record {
            ScopeRecord::Midi {
                event,
                delivery: MidiDelivery::Generated,
            } => (event.sample, event.message.to_bytes()),
            ScopeRecord::Midi {
                delivery: MidiDelivery::Sent,
                ..
            } => panic!("offline runner cannot mark MIDI as sent"),
            ScopeRecord::Signal { .. } => panic!("MIDI-only selection has no signals"),
            _ => panic!("unexpected future Scope record variant"),
        })
        .collect();
    assert_eq!(generated, baseline_trace);
    let _ = std::fs::remove_file(baseline_path);
    let _ = std::fs::remove_file(captured_path);
}
