use std::path::PathBuf;

use oxurack::{
    CoreCommand, CoreError, KnobRef, MidiOutModule, Oxurack, ParameterValue, ScopeSelector,
    SimulationDriver,
};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_lfo::LfoModule;
use oxurack_mod_turingmachine::TuringMachineModule;

struct OtherLfo;

impl oxurack::Module for OtherLfo {
    const KIND: &'static str = "lfo";
    const DISPLAY_NAME: &'static str = "Other LFO";
    fn port_schema() -> &'static [oxurack::PortSchema] {
        &[]
    }
    fn parameter_schema() -> &'static [oxurack::ParameterSchema] {
        &[]
    }
    fn spawn(
        _world: &mut bevy_ecs::world::World,
        _instance_name: &str,
        _parameters: &std::collections::HashMap<String, ParameterValue>,
    ) -> Result<bevy_ecs::prelude::Entity, CoreError> {
        unreachable!()
    }
}

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/patches/turing-machine-lfo.ron")
}

fn loaded() -> Oxurack {
    let mut rack = Oxurack::new();
    rack.try_install::<ClockModule>().unwrap();
    rack.try_install::<LfoModule>().unwrap();
    rack.try_install::<TuringMachineModule>().unwrap();
    rack.try_install::<MidiOutModule>().unwrap();
    rack.load_patch(&fixture()).unwrap();
    rack.set_driver_mode(SimulationDriver::Manual);
    rack
}

#[test]
fn handles_are_type_checked_instance_bound_and_retired_independently() {
    let mut rack = loaded();
    let lfo = rack.module::<LfoModule>("lfo_main").unwrap();
    let tm = rack.module::<TuringMachineModule>("tm_main").unwrap();
    assert!(matches!(
        rack.module::<ClockModule>("lfo_main"),
        Err(CoreError::ModuleTypeMismatch { .. })
    ));
    assert!(matches!(
        rack.module::<OtherLfo>("lfo_main"),
        Err(CoreError::ModuleTypeMismatch { .. })
    ));
    assert!(matches!(
        rack.module::<LfoModule>("clock_main"),
        Err(CoreError::ModuleTypeMismatch { .. })
    ));
    let bound = lfo.knob("frequency_hz");
    rack.set_knob(&bound, ParameterValue::Float(0.25)).unwrap();
    assert_eq!(rack.knob(&bound).unwrap(), ParameterValue::Float(0.25));
    let other_rack = loaded();
    assert!(matches!(
        other_rack.knob(&bound),
        Err(CoreError::StaleHandle { .. })
    ));
    rack.configure_scope(&[ScopeSelector::new("lfo_main", "bipolar_out")], 32)
        .unwrap();
    #[cfg(feature = "graph")]
    let graph_modules = rack.graph().modules().len();
    rack.enqueue_command(CoreCommand::SetBpm(133.0)).unwrap();
    rack.remove_module("lfo_main").unwrap();
    #[cfg(feature = "graph")]
    assert_eq!(rack.graph().modules().len(), graph_modules - 1);
    assert!(rack.scope().is_none());
    assert!(matches!(
        rack.knob(&bound),
        Err(CoreError::StaleHandle { .. })
    ));
    assert!(matches!(
        lfo.parameter(&rack, "frequency_hz"),
        Err(CoreError::StaleHandle { .. })
    ));
    assert!(tm.parameter(&rack, "write_probability").is_ok());
    let queued = rack.process_commands();
    assert_eq!(queued.len(), 1);
    assert!(matches!(
        queued[0].result,
        oxurack::CommandResult::Completed(Ok(()))
    ));
    assert_eq!(rack.bpm().unwrap(), 133.0);
    assert!(matches!(
        rack.remove_module("lfo_main"),
        Err(CoreError::UnknownModule(_))
    ));
}

#[test]
fn reference_parsing_rejects_ambiguous_dots_and_tuple_names_preserve_them() {
    assert!(KnobRef::from_dotted(".x").is_err());
    assert!(KnobRef::from_dotted("x.").is_err());
    assert!(KnobRef::from_dotted("x.y.z").is_err());
    let exact = KnobRef::new("module.with.dot", "parameter.with.dot");
    assert_eq!(exact.module(), "module.with.dot");
    assert_eq!(exact.parameter(), "parameter.with.dot");
}

#[test]
fn command_queue_is_fifo_bounded_and_reset_cancels_only_pending_slots() {
    let mut rack = loaded();
    let first = rack
        .enqueue_command(CoreCommand::SetParameter {
            module: "lfo_main".into(),
            param: "frequency_hz".into(),
            value: ParameterValue::Float(0.5),
        })
        .unwrap();
    let panic = rack.enqueue_command(CoreCommand::Panic).unwrap();
    let tail = rack.enqueue_command(CoreCommand::SetBpm(133.0)).unwrap();
    let final_tail = rack
        .enqueue_command(CoreCommand::SavePatch(
            std::env::temp_dir().join(format!("oxurack-cancelled-save-{}.ron", std::process::id())),
        ))
        .unwrap();
    let results = rack.process_commands();
    assert_eq!(results.len(), 4);
    assert_eq!(results[0].id, first);
    assert!(matches!(
        results[0].result,
        oxurack::CommandResult::Completed(Ok(()))
    ));
    assert_eq!(results[1].id, panic);
    assert!(matches!(
        results[1].result,
        oxurack::CommandResult::Completed(Ok(()))
    ));
    assert_eq!(results[2].id, tail);
    assert!(matches!(
        results[2].result,
        oxurack::CommandResult::CancelledByReset
    ));
    assert_eq!(results[3].id, final_tail);
    assert!(matches!(
        results[3].result,
        oxurack::CommandResult::CancelledByReset
    ));
    assert_eq!(rack.bpm().unwrap(), 120.0);
    assert!(rack.process_commands().is_empty());

    for _ in 0..64 {
        rack.enqueue_command(CoreCommand::SetBpm(120.0)).unwrap();
    }
    assert!(matches!(
        rack.enqueue_command(CoreCommand::Panic),
        Err(CoreError::CommandQueueFull { capacity: 64 })
    ));
    assert_eq!(rack.process_commands().len(), 64);
}

#[test]
fn direct_reset_cancellation_occupies_its_queue_slot_until_processed() {
    let mut rack = loaded();
    rack.enqueue_command(CoreCommand::SetBpm(132.0)).unwrap();
    rack.reset().unwrap();
    for _ in 0..63 {
        rack.enqueue_command(CoreCommand::SetBpm(120.0)).unwrap();
    }
    assert!(matches!(
        rack.enqueue_command(CoreCommand::Panic),
        Err(CoreError::CommandQueueFull { capacity: 64 })
    ));
    let results = rack.process_commands();
    assert_eq!(results.len(), 64);
    assert!(matches!(
        results[0].result,
        oxurack::CommandResult::CancelledByReset
    ));
}

#[test]
fn command_bpm_is_patch_metadata_and_invalid_values_do_not_mutate() {
    let mut rack = loaded();
    let clock = rack.module::<ClockModule>("clock_main").unwrap();
    rack.execute_command(&CoreCommand::SetBpm(133.0)).unwrap();
    assert_eq!(rack.bpm().unwrap(), 133.0);
    assert_eq!(
        clock.parameter(&rack, "bpm").unwrap(),
        ParameterValue::Float(120.0)
    );
    for invalid in [0.0, -1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(rack.execute_command(&CoreCommand::SetBpm(invalid)).is_err());
        assert_eq!(rack.bpm().unwrap(), 133.0);
    }
    let patch = rack.snapshot_patch().unwrap();
    assert_eq!(patch.bpm, 133.0);
}

#[test]
fn command_labels_and_cables_use_real_live_apis() {
    let mut rack = loaded();
    rack.execute_command(&CoreCommand::SetLabel {
        target: oxurack::LabelTarget::Module("tm_main".into()),
        kind: oxurack::LabelKind::DisplayName,
        value: oxurack::LabelValue::Text("Command TM".into()),
    })
    .unwrap();
    assert_eq!(
        rack.label(
            &oxurack::LabelTarget::Module("tm_main".into()),
            oxurack::LabelKind::DisplayName
        )
        .unwrap(),
        Some(oxurack::LabelValue::Text("Command TM".into()))
    );
    #[cfg(feature = "graph")]
    let before = rack.graph().cables().len();
    rack.execute_command(&CoreCommand::AddCable {
        source: ("lfo_main".into(), "bipolar_out".into()),
        target: ("tm_main".into(), "write_modulation".into()),
        transform: None,
    })
    .unwrap();
    #[cfg(feature = "graph")]
    assert_eq!(rack.graph().cables().len(), before + 1);
    rack.execute_command(&CoreCommand::RemoveCable {
        source: ("lfo_main".into(), "bipolar_out".into()),
        target: ("tm_main".into(), "write_modulation".into()),
    })
    .unwrap();
    #[cfg(feature = "graph")]
    assert_eq!(rack.graph().cables().len(), before);
}

#[test]
fn failed_replacement_keeps_existing_handle_and_successful_restart_stales_it() {
    let mut rack = loaded();
    let handle = rack.module::<LfoModule>("lfo_main").unwrap();
    let missing = std::env::temp_dir().join(format!("oxurack-missing-{}.ron", std::process::id()));
    assert!(rack.load_patch(&missing).is_err());
    assert!(handle.parameter(&rack, "frequency_hz").is_ok());
    rack.restart().unwrap();
    assert!(matches!(
        handle.parameter(&rack, "frequency_hz"),
        Err(CoreError::StaleHandle { .. })
    ));
}

#[test]
fn successful_unload_stales_handles_even_for_the_same_name_after_reload() {
    let mut rack = loaded();
    let handle = rack.module::<LfoModule>("lfo_main").unwrap();
    rack.unload_patch().unwrap();
    assert!(matches!(
        handle.parameter(&rack, "frequency_hz"),
        Err(CoreError::StaleHandle { .. })
    ));
    rack.load_patch(&fixture()).unwrap();
    assert!(rack.module::<LfoModule>("lfo_main").is_ok());
    assert!(matches!(
        handle.parameter(&rack, "frequency_hz"),
        Err(CoreError::StaleHandle { .. })
    ));
}

#[test]
fn bound_ports_do_not_turn_into_unbound_references_after_removal() {
    let mut rack = loaded();
    let module = rack.module::<LfoModule>("lfo_main").unwrap();
    let bound_port = module.port("bipolar_out");
    let unbound_port = oxurack::PortRef::new("lfo_main", "bipolar_out");
    rack.remove_module("lfo_main").unwrap();
    let target = oxurack::PortRef::new("tm_main", "write_modulation");
    assert!(matches!(
        rack.patch_ports(&bound_port, &target, None),
        Err(CoreError::StaleHandle { .. })
    ));
    assert!(rack.patch_ports(&unbound_port, &target, None).is_err());
}

#[test]
fn midi_sink_removal_queues_its_final_cleanup_without_rewinding_time() {
    let mut rack = loaded();
    rack.start();
    for _ in 0..300 {
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        drop(rack.drain_midi());
    }
    rack.stop();
    assert_eq!(rack.sample_time().completed_samples(), 300);
    rack.remove_module("midi_main").unwrap();
    let cleanup: Vec<_> = rack.drain_midi().collect();
    let messages: Vec<_> = cleanup
        .iter()
        .map(|event| (event.sample, event.message.to_bytes()))
        .collect();
    assert_eq!(
        messages,
        vec![
            (300, [0x80, 62, 0]),
            (300, [0xB0, 123, 0]),
            (300, [0xB0, 120, 0]),
        ]
    );
    assert_eq!(rack.sample_time().completed_samples(), 300);
    assert!(rack.module::<MidiOutModule>("midi_main").is_err());
}

#[test]
fn removing_every_module_leaves_a_loaded_empty_patch_header() {
    let mut rack = loaded();
    for name in ["lfo_main", "tm_main", "clock_main", "midi_main"] {
        rack.remove_module(name).unwrap();
    }
    let patch = rack.snapshot_patch().unwrap();
    assert!(patch.modules.is_empty());
    assert!(patch.cables.is_empty());
    assert_eq!(patch.bpm, 120.0);
    assert_eq!(rack.sample_time().completed_samples(), 0);
    assert!(matches!(
        rack.remove_module("midi_main"),
        Err(CoreError::UnknownModule(_))
    ));
}

#[test]
fn removing_unselected_module_preserves_survivor_scope_records() {
    let mut rack = loaded();
    let selectors = [ScopeSelector::new("tm_main", "write_probability")];
    rack.configure_scope(&selectors, 32).unwrap();
    rack.start();
    rack.advance_samples(1).unwrap();
    drop(rack.drain_midi());
    rack.stop();
    let before: Vec<_> = rack.scope().unwrap().records().copied().collect();
    let overwritten = rack.scope().unwrap().overwritten();
    rack.remove_module("lfo_main").unwrap();
    let scope = rack.scope().unwrap();
    assert_eq!(scope.selectors(), &selectors);
    assert_eq!(scope.records().copied().collect::<Vec<_>>(), before);
    assert_eq!(scope.overwritten(), overwritten);
}

#[test]
fn removal_admission_errors_preserve_the_live_owner() {
    let mut rack = loaded();
    let before = rack.snapshot_patch().unwrap();
    rack.start();
    assert!(matches!(
        rack.remove_module("lfo_main"),
        Err(CoreError::LifecycleRequiresStopped)
    ));
    for sample in 0..252 {
        rack.advance_samples(1).unwrap();
        if sample != 251 {
            drop(rack.drain_midi());
        }
    }
    rack.stop();
    let sample = rack.sample_time();
    assert!(matches!(
        rack.remove_module("lfo_main"),
        Err(CoreError::PendingMidi { .. })
    ));
    assert_eq!(rack.sample_time(), sample);
    assert_eq!(rack.snapshot_patch().unwrap(), before);
    drop(rack.drain_midi());
    rack.remove_module("lfo_main").unwrap();
}

#[test]
fn unconnected_lfo_removal_matches_uninterrupted_survivor_control() {
    let mut subject = loaded();
    let mut control = loaded();
    let source = ("lfo_main", "bipolar_out");
    let target = ("tm_main", "write_modulation");
    subject.unpatch(source, target, 0).unwrap();
    control.unpatch(source, target, 0).unwrap();
    let selectors = [
        ScopeSelector::new("clock_main", "pulse"),
        ScopeSelector::new("tm_main", "write_probability"),
    ];
    subject.configure_scope(&selectors, 4096).unwrap();
    control.configure_scope(&selectors, 4096).unwrap();
    subject.start();
    control.start();
    for _ in 0..300 {
        assert_eq!(subject.advance_samples(1).unwrap(), 1);
        assert_eq!(control.advance_samples(1).unwrap(), 1);
        let subject_events: Vec<_> = subject.drain_midi().collect();
        let control_events: Vec<_> = control.drain_midi().collect();
        assert_eq!(subject_events, control_events);
        for event in subject_events {
            subject.record_scope_midi(event, oxurack::MidiDelivery::Generated);
        }
        for event in control_events {
            control.record_scope_midi(event, oxurack::MidiDelivery::Generated);
        }
    }
    subject.stop();
    control.stop();
    subject.remove_module("lfo_main").unwrap();
    subject.start();
    control.start();
    for _ in 0..200 {
        assert_eq!(subject.advance_samples(1).unwrap(), 1);
        assert_eq!(control.advance_samples(1).unwrap(), 1);
        let subject_events: Vec<_> = subject.drain_midi().collect();
        let control_events: Vec<_> = control.drain_midi().collect();
        assert_eq!(subject_events, control_events);
        for event in subject_events {
            subject.record_scope_midi(event, oxurack::MidiDelivery::Generated);
        }
        for event in control_events {
            control.record_scope_midi(event, oxurack::MidiDelivery::Generated);
        }
    }
    subject.stop();
    control.stop();
    assert_eq!(subject.sample_time(), control.sample_time());
    assert_eq!(
        subject
            .scope()
            .unwrap()
            .records()
            .copied()
            .collect::<Vec<_>>(),
        control
            .scope()
            .unwrap()
            .records()
            .copied()
            .collect::<Vec<_>>()
    );
    assert_eq!(
        subject.scope().unwrap().overwritten(),
        control.scope().unwrap().overwritten()
    );
}
