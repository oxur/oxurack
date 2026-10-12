use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use oxurack::{CoreError, MidiOutModule, Oxurack, ParameterValue, Patch, SimulationDriver};
use oxurack_core::{deserialize_patch, save_patch_to_file};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_lfo::LfoModule;
use oxurack_mod_turingmachine::TuringMachineModule;

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/patches/turing-machine-lfo.ron"
);
static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

fn temp_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "oxurack-persistence-{}-{}-{label}.ron",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
    ))
}

fn rack(path: &Path) -> Oxurack {
    let mut rack = Oxurack::new();
    rack.try_install::<ClockModule>().unwrap();
    rack.try_install::<LfoModule>().unwrap();
    rack.try_install::<TuringMachineModule>().unwrap();
    rack.try_install::<MidiOutModule>().unwrap();
    rack.load_patch(path).unwrap();
    rack.set_driver_mode(SimulationDriver::Manual);
    rack
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

fn fixture_patch() -> Patch {
    deserialize_patch(&std::fs::read_to_string(FIXTURE).unwrap()).unwrap()
}

#[test]
fn live_configuration_saves_and_replays_in_fresh_racks() {
    let source = temp_path("source");
    let saved = temp_path("saved");
    save_patch_to_file(&fixture_patch(), &source).unwrap();

    let mut edited = rack(&source);
    edited.start();
    for _ in 0..200 {
        edited.advance_samples(1).unwrap();
        for _event in edited.drain_midi() {}
    }
    assert_eq!(
        edited.parameter("lfo_main", "frequency_hz").unwrap(),
        ParameterValue::Float(0.5)
    );
    edited
        .set_parameter("lfo_main", "frequency_hz", ParameterValue::Float(1.0))
        .unwrap();
    edited
        .set_parameter("tm_main", "write_probability", ParameterValue::Float(0.25))
        .unwrap();
    assert_eq!(edited.sample_time().completed_samples(), 200);
    edited.save_patch(&saved).unwrap();
    assert_eq!(edited.sample_time().completed_samples(), 200);
    for _ in 0..100 {
        edited.advance_samples(1).unwrap();
        for _event in edited.drain_midi() {}
    }
    edited.stop();

    let mut saved_first = rack(&saved);
    let mut saved_second = rack(&saved);
    let mut reference_patch = fixture_patch();
    reference_patch
        .modules
        .iter_mut()
        .find(|module| module.instance_name == "lfo_main")
        .unwrap()
        .parameters
        .insert("frequency_hz".into(), ParameterValue::Float(1.0));
    reference_patch
        .modules
        .iter_mut()
        .find(|module| module.instance_name == "tm_main")
        .unwrap()
        .parameters
        .insert("write_probability".into(), ParameterValue::Float(0.25));
    let reference_path = temp_path("reference");
    save_patch_to_file(&reference_patch, &reference_path).unwrap();
    let mut reference = rack(&reference_path);

    let saved_trace = trace(&mut saved_first, 10_000);
    assert!(!saved_trace.is_empty());
    assert_eq!(saved_trace, trace(&mut saved_second, 10_000));
    assert_eq!(saved_trace, trace(&mut reference, 10_000));
    let mut original = rack(&source);
    assert_ne!(saved_trace, trace(&mut original, 10_000));

    let saved_patch = deserialize_patch(&std::fs::read_to_string(&saved).unwrap()).unwrap();
    assert_eq!(saved_patch.master_seed, 42);
    assert_eq!(saved_patch.version, "1.0");
    assert_eq!(saved_patch.bpm, 120.0);
    assert_eq!(
        saved_patch
            .modules
            .iter()
            .find(|module| module.instance_name == "lfo_main")
            .unwrap()
            .parameters
            .get("frequency_hz"),
        Some(&ParameterValue::Float(1.0))
    );
    assert_eq!(
        saved_patch
            .modules
            .iter()
            .find(|module| module.instance_name == "tm_main")
            .unwrap()
            .parameters
            .get("write_probability"),
        Some(&ParameterValue::Float(0.25))
    );

    for path in [source, saved, reference_path] {
        let _ = std::fs::remove_file(path);
    }
}

#[test]
fn live_parameter_api_returns_existing_error_classes() {
    let path = temp_path("errors");
    save_patch_to_file(&fixture_patch(), &path).unwrap();
    let mut unloaded = Oxurack::new();
    assert!(matches!(
        unloaded.parameter("lfo_main", "frequency_hz"),
        Err(CoreError::NoPatchLoaded)
    ));
    assert!(matches!(
        unloaded.snapshot_patch(),
        Err(CoreError::NoPatchLoaded)
    ));

    let mut rack = rack(&path);
    assert!(matches!(
        rack.parameter("absent", "frequency_hz"),
        Err(CoreError::UnknownModule(name)) if name == "absent"
    ));
    assert!(matches!(
        rack.parameter("lfo_main", "absent"),
        Err(CoreError::UnknownParameter { module, param }) if module == "lfo_main" && param == "absent"
    ));
    assert!(matches!(
        rack.set_parameter("lfo_main", "frequency_hz", ParameterValue::Int(3)),
        Err(CoreError::InvalidParameterValue { module, param, .. }) if module == "lfo_main" && param == "frequency_hz"
    ));
    assert_eq!(
        rack.parameter("lfo_main", "frequency_hz").unwrap(),
        ParameterValue::Float(0.5)
    );
    let _ = std::fs::remove_file(path);
}
