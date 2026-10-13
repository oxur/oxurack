use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use oxurack::{
    CableTransform, CoreError, LabelKind, LabelTarget, LabelValue, MidiOutModule, Oxurack,
    SimulationDriver,
};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_lfo::LfoModule;
use oxurack_mod_turingmachine::TuringMachineModule;

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/patches/turing-machine-lfo.ron"
);
static NEXT: AtomicUsize = AtomicUsize::new(0);

fn rack() -> Oxurack {
    rack_from(Path::new(FIXTURE))
}

fn rack_from(path: &Path) -> Oxurack {
    let mut rack = Oxurack::new();
    rack.try_install::<ClockModule>().unwrap();
    rack.try_install::<LfoModule>().unwrap();
    rack.try_install::<TuringMachineModule>().unwrap();
    rack.try_install::<MidiOutModule>().unwrap();
    rack.load_patch(path).unwrap();
    rack.set_driver_mode(SimulationDriver::Manual);
    rack
}

fn trace(path: &Path, samples: u64) -> Vec<(u64, [u8; 3])> {
    let mut rack = rack_from(path);
    rack.start();
    let mut events = Vec::new();
    for _ in 0..samples {
        rack.advance_samples(1).unwrap();
        events.extend(
            rack.drain_midi()
                .map(|event| (event.sample, event.message.to_bytes())),
        );
    }
    rack.stop();
    events
}

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "oxurack-cables-{}-{}-{name}.ron",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ))
}

fn modulation() -> ((&'static str, &'static str), (&'static str, &'static str)) {
    (("lfo_main", "bipolar_out"), ("tm_main", "write_modulation"))
}

#[test]
fn live_patch_unpatch_and_labels_follow_exact_occurrences() {
    let mut rack = rack();
    let (source, target) = modulation();
    let constant = CableTransform::Affine {
        factor: 0.0,
        offset: 0.25,
    };

    #[cfg(feature = "graph")]
    assert_eq!(rack.graph().cables().len(), 9);
    rack.unpatch(source, target, 0).unwrap();
    #[cfg(feature = "graph")]
    assert_eq!(rack.graph().cables().len(), 8);
    assert_eq!(rack.patch(source, target, Some(constant)).unwrap(), 0);
    #[cfg(feature = "graph")]
    assert_eq!(rack.graph().cables().len(), 9);
    let added = rack.patch(source, target, Some(constant)).unwrap();
    assert_eq!(added, 1);
    let label = LabelTarget::Cable {
        source: (source.0.into(), source.1.into()),
        target: (target.0.into(), target.1.into()),
        occurrence: added,
    };
    rack.set_label(
        &label,
        LabelKind::Description,
        LabelValue::Text("constant lane".into()),
    )
    .unwrap();
    assert_eq!(rack.sample_time().completed_samples(), 0);
    #[cfg(feature = "graph")]
    assert_eq!(rack.graph().cables().len(), 10);

    rack.unpatch(source, target, 0).unwrap();
    #[cfg(feature = "graph")]
    assert_eq!(rack.graph().cables().len(), 9);
    let shifted = LabelTarget::Cable {
        source: (source.0.into(), source.1.into()),
        target: (target.0.into(), target.1.into()),
        occurrence: 0,
    };
    assert_eq!(
        rack.label(&shifted, LabelKind::Description).unwrap(),
        Some(LabelValue::Text("constant lane".into()))
    );
    let snapshot = rack.snapshot_patch().unwrap();
    let surviving = snapshot
        .cables
        .iter()
        .find(|cable| cable.source == (source.0.into(), source.1.into()))
        .unwrap();
    assert_eq!(
        surviving.transform,
        Some(CableTransform::Affine {
            factor: 0.0,
            offset: 0.25
        })
    );
    assert_eq!(surviving.description, "constant lane");
    assert_eq!(rack.patch(source, target, Some(constant)).unwrap(), 1);
    assert_eq!(rack.sample_time().completed_samples(), 0);
}

#[test]
fn invalid_edits_are_typed_and_do_not_advance_the_rack() {
    let mut empty = Oxurack::new();
    assert!(matches!(
        empty.patch(("a", "out"), ("b", "in"), None),
        Err(CoreError::NoPatchLoaded)
    ));
    let mut rack = rack();
    let before = rack.sample_time();
    let (source, target) = modulation();
    assert!(matches!(
        rack.patch(
            source,
            target,
            Some(CableTransform::Clamp { min: 1.0, max: 0.0 })
        ),
        Err(CoreError::InvalidCableTransform { .. })
    ));
    assert!(matches!(
        rack.patch(("outside", "out"), target, None),
        Err(CoreError::UnknownModule(name)) if name == "outside"
    ));
    assert!(matches!(
        rack.patch(("lfo_main", "missing"), target, None),
        Err(CoreError::Patch(
            oxurack_core::PatchError::UnknownPort { .. }
        ))
    ));
    assert!(matches!(
        rack.patch(("tm_main", "clock"), target, None),
        Err(CoreError::InvalidCableDirection { .. })
    ));
    assert!(matches!(
        rack.patch(
            source,
            ("lfo_main", "bipolar_out"),
            Some(CableTransform::Unipolar)
        ),
        Err(CoreError::InvalidCableDirection { .. })
    ));
    assert_eq!(rack.sample_time(), before);
    assert!(matches!(
        rack.unpatch(source, target, usize::MAX),
        Err(CoreError::UnknownCable {
            occurrence: usize::MAX,
            ..
        })
    ));
    assert_eq!(rack.sample_time(), before);
    #[cfg(feature = "graph")]
    assert_eq!(rack.graph().cables().len(), 9);
}

#[test]
fn edited_patch_saves_literal_configuration_and_replays_fresh() {
    let source_file = temp_path("source");
    let saved_file = temp_path("saved");
    std::fs::copy(FIXTURE, &source_file).unwrap();

    let mut edited = rack_from(&source_file);
    edited.start();
    for _ in 0..200 {
        edited.advance_samples(1).unwrap();
        for _event in edited.drain_midi() {}
    }
    let endpoints = modulation();
    edited.unpatch(endpoints.0, endpoints.1, 0).unwrap();
    let occurrence = edited
        .patch(
            endpoints.0,
            endpoints.1,
            Some(CableTransform::Affine {
                factor: 0.0,
                offset: 0.25,
            }),
        )
        .unwrap();
    edited
        .set_label(
            &LabelTarget::Cable {
                source: (endpoints.0.0.into(), endpoints.0.1.into()),
                target: (endpoints.1.0.into(), endpoints.1.1.into()),
                occurrence,
            },
            LabelKind::Description,
            LabelValue::Text("Constant +0.25 write modulation".into()),
        )
        .unwrap();
    edited.save_patch(&saved_file).unwrap();
    assert_eq!(edited.sample_time().completed_samples(), 200);
    edited.stop();

    let mut fresh = Oxurack::new();
    fresh.try_install::<ClockModule>().unwrap();
    fresh.try_install::<LfoModule>().unwrap();
    fresh.try_install::<TuringMachineModule>().unwrap();
    fresh.try_install::<MidiOutModule>().unwrap();
    fresh.load_patch(&saved_file).unwrap();
    fresh.set_driver_mode(SimulationDriver::Manual);
    fresh
        .configure_scope(
            &[oxurack::ScopeSelector::new("tm_main", "write_modulation")],
            100_000,
        )
        .unwrap();
    fresh.start();
    fresh.advance_samples(1).unwrap();
    let modulation = fresh.scope().unwrap().records().next().copied().unwrap();
    assert!(matches!(
        modulation,
        oxurack::ScopeRecord::Signal {
            sample: 0,
            selection: 0,
            value: oxurack::Value::Bipolar(0.25)
        }
    ));
    let _ = fresh.drain_midi().count();
    fresh.stop();
    let saved = fresh.snapshot_patch().unwrap();
    let cable = saved
        .cables
        .iter()
        .find(|cable| cable.source == ("lfo_main".into(), "bipolar_out".into()))
        .unwrap();
    assert_eq!(
        cable.transform,
        Some(CableTransform::Affine {
            factor: 0.0,
            offset: 0.25
        })
    );
    assert_eq!(cable.description, "Constant +0.25 write modulation");
    assert_eq!(saved.modules.len(), 4);
    assert_eq!(saved.cables.len(), 9);
    assert_eq!(saved.version, "1.0");
    assert_eq!(saved.master_seed, 42);
    assert_eq!(saved.bpm, 120.0);
    let module_parameters = saved
        .modules
        .iter()
        .map(|module| (module.instance_name.as_str(), &module.parameters))
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(
        module_parameters["tm_main"]["write_probability"],
        oxurack::ParameterValue::Float(0.5)
    );
    assert_eq!(
        module_parameters["lfo_main"]["frequency_hz"],
        oxurack::ParameterValue::Float(0.5)
    );
    let cable_endpoints = saved
        .cables
        .iter()
        .map(|cable| {
            format!(
                "{}.{} -> {}.{}",
                cable.source.0, cable.source.1, cable.target.0, cable.target.1
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        cable_endpoints,
        [
            "tm_main.gate -> midi_main.gate",
            "tm_main.noise_cc -> midi_main.noise_cc",
            "tm_main.note -> midi_main.note",
            "tm_main.note_valid -> midi_main.note_valid",
            "tm_main.step -> midi_main.step",
            "tm_main.velocity -> midi_main.velocity",
            "tm_main.velocity_valid -> midi_main.velocity_valid",
            "clock_main.pulse -> tm_main.clock",
            "lfo_main.bipolar_out -> tm_main.write_modulation",
        ]
    );

    let mut reference =
        oxurack_core::deserialize_patch(&std::fs::read_to_string(FIXTURE).unwrap()).unwrap();
    let reference_cable = reference
        .cables
        .iter_mut()
        .find(|cable| cable.source == ("lfo_main".into(), "bipolar_out".into()))
        .unwrap();
    reference_cable.transform = Some(CableTransform::Affine {
        factor: 0.0,
        offset: 0.25,
    });
    reference_cable.description = "Constant +0.25 write modulation".into();
    let reference_file = temp_path("reference");
    oxurack_core::save_patch_to_file(&reference, &reference_file).unwrap();
    let saved_trace = trace(&saved_file, 10_000);
    let reference_trace = trace(&reference_file, 10_000);
    let original_trace = trace(Path::new(FIXTURE), 10_000);
    assert_eq!(saved_trace, reference_trace);
    assert_ne!(saved_trace, original_trace);

    std::fs::remove_file(source_file).unwrap();
    std::fs::remove_file(saved_file).unwrap();
    std::fs::remove_file(reference_file).unwrap();
}
