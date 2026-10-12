use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use oxurack::{
    CoreError, LabelKind, LabelTarget, LabelValue, MidiOutModule, Oxurack, SimulationDriver, Tag,
    TagError,
};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_lfo::LfoModule;
use oxurack_mod_turingmachine::TuringMachineModule;

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/patches/turing-machine-lfo.ron"
);
static NEXT_FILE: AtomicUsize = AtomicUsize::new(0);

fn loaded_rack() -> Oxurack {
    let mut rack = installed_rack();
    rack.load_patch(Path::new(FIXTURE)).unwrap();
    rack
}

fn installed_rack() -> Oxurack {
    let mut rack = Oxurack::new();
    rack.try_install::<ClockModule>().unwrap();
    rack.try_install::<LfoModule>().unwrap();
    rack.try_install::<TuringMachineModule>().unwrap();
    rack.try_install::<MidiOutModule>().unwrap();
    rack.set_driver_mode(SimulationDriver::Manual);
    rack
}

fn temp_file(name: &str) -> PathBuf {
    let id = NEXT_FILE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "oxurack-labels-{}-{name}-{id}.ron",
        std::process::id()
    ))
}

fn source_cable() -> LabelTarget {
    LabelTarget::Cable {
        source: ("clock_main".into(), "pulse".into()),
        target: ("tm_main".into(), "clock".into()),
        occurrence: 0,
    }
}

#[test]
fn public_label_reads_writes_and_clear_semantics() {
    let mut rack = loaded_rack();
    let rack_target = LabelTarget::Rack;
    let module = LabelTarget::Module("tm_main".into());
    let cable = source_cable();

    assert_eq!(
        rack.label(&rack_target, LabelKind::DisplayName).unwrap(),
        Some(LabelValue::Text(String::new()))
    );
    assert_eq!(rack.label(&module, LabelKind::DisplayName).unwrap(), None);
    assert_eq!(rack.label(&cable, LabelKind::Description).unwrap(), None);

    let tags = [
        Tag::new("zeta").unwrap(),
        Tag::new("alpha").unwrap(),
        Tag::new("zeta").unwrap(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(
        tags.iter().map(Tag::as_str).collect::<Vec<_>>(),
        ["alpha", "zeta"]
    );
    rack.set_label(
        &rack_target,
        LabelKind::DisplayName,
        LabelValue::Text("Turing workbench".into()),
    )
    .unwrap();
    rack.set_label(
        &rack_target,
        LabelKind::Description,
        LabelValue::Text("Live metadata".into()),
    )
    .unwrap();
    rack.set_label(
        &rack_target,
        LabelKind::Tags,
        LabelValue::Tags(tags.clone()),
    )
    .unwrap();
    rack.set_label(
        &rack_target,
        LabelKind::Notes,
        LabelValue::Notes(vec![
            "second".into(),
            "first".into(),
            "second".into(),
            "".into(),
        ]),
    )
    .unwrap();
    rack.set_label(
        &module,
        LabelKind::DisplayName,
        LabelValue::Text("Melody generator".into()),
    )
    .unwrap();
    rack.set_label(
        &module,
        LabelKind::Description,
        LabelValue::Text("Raw module description".into()),
    )
    .unwrap();
    rack.set_label(&module, LabelKind::Tags, LabelValue::Tags(tags.clone()))
        .unwrap();
    rack.set_label(
        &cable,
        LabelKind::DisplayName,
        LabelValue::Text("Master clock".into()),
    )
    .unwrap();
    rack.set_label(
        &cable,
        LabelKind::Description,
        LabelValue::Text("Clock pulse to Turing Machine".into()),
    )
    .unwrap();
    rack.set_label(&cable, LabelKind::Tags, LabelValue::Tags(tags.clone()))
        .unwrap();

    assert_eq!(
        rack.label(&rack_target, LabelKind::Tags).unwrap(),
        Some(LabelValue::Tags(tags.clone()))
    );
    assert_eq!(
        rack.label(&rack_target, LabelKind::Notes).unwrap(),
        Some(LabelValue::Notes(vec![
            "second".into(),
            "first".into(),
            "second".into(),
            "".into()
        ]))
    );
    assert_eq!(
        rack.label(&module, LabelKind::Tags).unwrap(),
        Some(LabelValue::Tags(tags.clone()))
    );
    assert_eq!(
        rack.label(&cable, LabelKind::DisplayName).unwrap(),
        Some(LabelValue::Text("Master clock".into()))
    );

    rack.set_label(
        &module,
        LabelKind::DisplayName,
        LabelValue::Text(String::new()),
    )
    .unwrap();
    assert_eq!(rack.label(&module, LabelKind::DisplayName).unwrap(), None);
    rack.set_label(&module, LabelKind::Tags, LabelValue::Tags(BTreeSet::new()))
        .unwrap();
    assert_eq!(rack.label(&module, LabelKind::Tags).unwrap(), None);
    rack.set_label(&rack_target, LabelKind::Notes, LabelValue::Notes(vec![]))
        .unwrap();
    assert_eq!(
        rack.label(&rack_target, LabelKind::Notes).unwrap(),
        Some(LabelValue::Notes(vec![]))
    );
    assert!(matches!(
        rack.set_label(&module, LabelKind::Notes, LabelValue::Notes(vec![])),
        Err(CoreError::InvalidLabelValue {
            kind: LabelKind::Notes,
            ..
        })
    ));
    assert!(matches!(
        rack.set_label(&module, LabelKind::Tags, LabelValue::Text("wrong".into())),
        Err(CoreError::InvalidLabelValue {
            kind: LabelKind::Tags,
            ..
        })
    ));
    assert_eq!(Tag::new(""), Err(TagError::Empty));
    assert_eq!(
        Tag::new(" lead"),
        Err(TagError::LeadingOrTrailingWhitespace)
    );
    assert_eq!(
        Tag::new("lead "),
        Err(TagError::LeadingOrTrailingWhitespace)
    );
    assert_eq!(
        Tag::new("internal space").unwrap().as_str(),
        "internal space"
    );
    assert!(matches!(
        rack.label(&module, LabelKind::Notes),
        Err(CoreError::InvalidLabelValue {
            kind: LabelKind::Notes,
            ..
        })
    ));

    #[cfg(feature = "graph")]
    {
        let module_info = rack
            .graph()
            .modules()
            .into_iter()
            .find(|module| module.instance_name == "tm_main")
            .unwrap();
        assert_eq!(module_info.display_name, "Tm Main");
        assert_eq!(
            module_info.description,
            Some("Raw module description".into())
        );
        let cable_info = rack.graph().cables().into_iter().next().unwrap();
        assert_eq!(cable_info.display_name, Some("Master clock".into()));
    }
}

#[test]
fn failed_value_validation_does_not_change_raw_state() {
    let mut rack = loaded_rack();
    let module = LabelTarget::Module("tm_main".into());
    rack.set_label(
        &module,
        LabelKind::DisplayName,
        LabelValue::Text("Lead".into()),
    )
    .unwrap();
    assert!(matches!(
        rack.set_label(
            &module,
            LabelKind::DisplayName,
            LabelValue::Tags(BTreeSet::new())
        ),
        Err(CoreError::InvalidLabelValue {
            kind: LabelKind::DisplayName,
            ..
        })
    ));
    assert_eq!(
        rack.label(&module, LabelKind::DisplayName).unwrap(),
        Some(LabelValue::Text("Lead".into()))
    );
}

#[test]
fn labels_are_snapshotted_saved_and_reloaded_as_raw_metadata() {
    let mut rack = loaded_rack();
    let rack_target = LabelTarget::Rack;
    let module = LabelTarget::Module("tm_main".into());
    let cable = source_cable();
    let tags = [
        Tag::new("generative").unwrap(),
        Tag::new("workbench").unwrap(),
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    rack.set_label(
        &rack_target,
        LabelKind::DisplayName,
        LabelValue::Text("Turing workbench".into()),
    )
    .unwrap();
    rack.set_label(
        &rack_target,
        LabelKind::Description,
        LabelValue::Text("Rack description".into()),
    )
    .unwrap();
    rack.set_label(
        &rack_target,
        LabelKind::Tags,
        LabelValue::Tags(tags.clone()),
    )
    .unwrap();
    rack.set_label(
        &rack_target,
        LabelKind::Notes,
        LabelValue::Notes(vec!["one".into(), "two".into()]),
    )
    .unwrap();
    rack.set_label(
        &module,
        LabelKind::DisplayName,
        LabelValue::Text("Melody generator".into()),
    )
    .unwrap();
    rack.set_label(
        &module,
        LabelKind::Description,
        LabelValue::Text("Module description".into()),
    )
    .unwrap();
    rack.set_label(&module, LabelKind::Tags, LabelValue::Tags(tags.clone()))
        .unwrap();
    rack.set_label(
        &cable,
        LabelKind::DisplayName,
        LabelValue::Text("Clock line".into()),
    )
    .unwrap();
    rack.set_label(
        &cable,
        LabelKind::Description,
        LabelValue::Text("Pulse route".into()),
    )
    .unwrap();
    rack.set_label(&cable, LabelKind::Tags, LabelValue::Tags(tags.clone()))
        .unwrap();

    let before = rack.snapshot_patch().unwrap();
    assert_eq!(before.version, "1.0");
    assert_eq!(before.master_seed, 42);
    assert_eq!(before.bpm, 120.0);
    let file = temp_file("roundtrip");
    rack.save_patch(&file).unwrap();
    let mut reloaded = installed_rack();
    reloaded.load_patch(&file).unwrap();
    let after = reloaded.snapshot_patch().unwrap();
    assert_eq!(after, before);
    assert_eq!(
        reloaded.label(&rack_target, LabelKind::Notes).unwrap(),
        Some(LabelValue::Notes(vec!["one".into(), "two".into()]))
    );
    assert_eq!(
        reloaded.label(&module, LabelKind::Tags).unwrap(),
        Some(LabelValue::Tags(tags.clone()))
    );
    assert_eq!(
        reloaded.label(&cable, LabelKind::Description).unwrap(),
        Some(LabelValue::Text("Pulse route".into()))
    );
    std::fs::remove_file(file).unwrap();
}

#[test]
fn no_patch_and_unknown_active_target_errors_are_typed() {
    let mut rack = Oxurack::new();
    let target = LabelTarget::Module("missing".into());
    assert!(matches!(
        rack.label(&target, LabelKind::DisplayName),
        Err(CoreError::NoPatchLoaded)
    ));
    assert!(matches!(
        rack.set_label(
            &LabelTarget::Rack,
            LabelKind::Tags,
            LabelValue::Tags(BTreeSet::new())
        ),
        Err(CoreError::NoPatchLoaded)
    ));
    rack.try_install::<ClockModule>().unwrap();
    rack.try_install::<LfoModule>().unwrap();
    rack.try_install::<TuringMachineModule>().unwrap();
    rack.try_install::<MidiOutModule>().unwrap();
    rack.load_patch(Path::new(FIXTURE)).unwrap();
    assert!(matches!(
        rack.label(&target, LabelKind::DisplayName),
        Err(CoreError::UnknownModule(name)) if name == "missing"
    ));
    let missing_port = LabelTarget::Cable {
        source: ("clock_main".into(), "not_a_port".into()),
        target: ("tm_main".into(), "clock".into()),
        occurrence: 0,
    };
    assert!(matches!(
        rack.label(&missing_port, LabelKind::DisplayName),
        Err(CoreError::Patch(_))
    ));

    let empty_file = temp_file("empty-patch");
    std::fs::write(
        &empty_file,
        "(version: \"1.0\", master_seed: 0, bpm: 120.0, modules: [], cables: [])",
    )
    .unwrap();
    let mut empty_rack = Oxurack::new();
    empty_rack.load_patch(&empty_file).unwrap();
    empty_rack
        .set_label(
            &LabelTarget::Rack,
            LabelKind::DisplayName,
            LabelValue::Text("Empty patch".into()),
        )
        .unwrap();
    assert_eq!(
        empty_rack
            .label(&LabelTarget::Rack, LabelKind::DisplayName)
            .unwrap(),
        Some(LabelValue::Text("Empty patch".into()))
    );
    std::fs::remove_file(empty_file).unwrap();
}
