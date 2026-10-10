use std::sync::atomic::{AtomicU64, Ordering};

use oxurack::{Oxurack, Patch, SimulationDriver, TickError};
use oxurack_core::{deserialize_patch, save_patch_to_file};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_turingmachine::{Scale, StepOutputs, TuringMachine, TuringMachineModule};

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/patches/turing-machine-midi.ron"
);
static TEMP_PATCH_COUNTER: AtomicU64 = AtomicU64::new(0);

fn patch() -> Patch {
    let mut patch = deserialize_patch(&std::fs::read_to_string(FIXTURE).unwrap()).unwrap();
    patch.modules[0]
        .parameters
        .insert("bpm".into(), oxurack::ParameterValue::Float(1000.0));
    patch.modules[0]
        .parameters
        .insert("ppqn".into(), oxurack::ParameterValue::Int(30));
    patch
}

fn with_patch(patch: &Patch) -> (Oxurack, std::path::PathBuf) {
    let path = std::env::temp_dir().join(format!(
        "oxurack-midi-output-{}-{}.ron",
        std::process::id(),
        TEMP_PATCH_COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    save_patch_to_file(patch, &path).unwrap();
    let mut rack = Oxurack::new();
    rack.try_install::<ClockModule>().unwrap();
    rack.try_install::<TuringMachineModule>().unwrap();
    rack.try_install::<oxurack::MidiOutModule>().unwrap();
    rack.load_patch(&path).unwrap();
    rack.set_driver_mode(SimulationDriver::Manual).start();
    (rack, path)
}

fn expected_engine() -> TuringMachine {
    let mut engine =
        TuringMachine::with_seed(oxurack::ModuleSpawnContext::new(42, "tm_main").instance_seed());
    engine.set_write(0.75);
    engine.set_length(8);
    engine.set_scale(Scale::new(vec![0, 2, 4, 7, 9], "Major Pentatonic"));
    engine.set_note_range(48..=72);
    engine
}

fn expected_trace(samples: u64) -> Vec<(u64, [u8; 3])> {
    let mut engine = expected_engine();
    let mut pending: Option<StepOutputs> = None;
    let mut active_note = None;
    let mut expected = Vec::new();
    for sample in 0..samples {
        if sample % 2 == 1 {
            let output = pending.take().expect("Clock edge at previous even sample");
            if let Some(old) = active_note.take() {
                expected.push((sample, [0x80, old, 0]));
            }
            if let Some(note) = output.note
                && output.gate
            {
                let velocity = output.velocity.unwrap_or(64);
                if velocity > 0 {
                    expected.push((sample, [0x90, note, velocity]));
                    active_note = Some(note);
                }
            }
        }
        if sample % 2 == 0 {
            pending = Some(engine.tick());
        }
    }
    expected
}

fn installed_seeded_trace_matches_standalone_and_literal_transition() {
    let patch = patch();
    let (mut rack, path) = with_patch(&patch);
    let mut actual = Vec::new();
    for sample in 0..24 {
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        actual.extend(
            rack.drain_midi()
                .map(|event| (event.sample, event.message.to_bytes())),
        );
        assert_eq!(rack.sample_time().completed_samples(), sample + 1);
    }
    assert_eq!(actual, expected_trace(24));
    assert!(!actual.is_empty());
    let _ = std::fs::remove_file(path);
}

#[test]
fn installed_seeded_trace_matches_standalone_and_literal_transition_test() {
    installed_seeded_trace_matches_standalone_and_literal_transition();
}

#[test]
fn unread_output_denies_the_next_sample_and_reports_completed_progress() {
    let patch = patch();
    let (mut rack, path) = with_patch(&patch);
    let result = rack.advance_samples(24);
    match result {
        Err(oxurack::CoreError::Tick(TickError::OutputBackpressure {
            pending_messages,
            completed_samples,
        })) => {
            assert!(pending_messages > 0);
            assert_eq!(completed_samples, rack.sample_time().completed_samples());
            assert!(completed_samples >= 2);
        }
        other => panic!("expected unread-output backpressure, got {other:?}"),
    }
    let queued: Vec<_> = rack
        .drain_midi()
        .map(|event| (event.sample, event.message.to_bytes()))
        .collect();
    assert!(!queued.is_empty());
    assert!(
        rack.midi_output_error().is_some(),
        "drain does not clear terminal fault"
    );
    assert_eq!(
        rack.advance_samples(1).unwrap(),
        0,
        "drain does not restart a terminally faulted rack"
    );
    let _ = std::fs::remove_file(path);
}
