use std::num::NonZeroUsize;
use std::path::Path;

use oxurack::{MidiOutModule, ObservationChange, ObservationStart, Oxurack, PatchOrigin, TagMatch};
use oxurack_mod_clock::ClockModule;
use oxurack_mod_lfo::LfoModule;
use oxurack_mod_turingmachine::TuringMachineModule;

fn fixture() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/session-observation.ron")
}

fn rack() -> Oxurack {
    let mut rack = Oxurack::new();
    rack.try_install::<ClockModule>().unwrap();
    rack.try_install::<TuringMachineModule>().unwrap();
    rack.try_install::<LfoModule>().unwrap();
    rack.try_install::<MidiOutModule>().unwrap();
    rack
}

struct EmptyModule;
impl oxurack::Module for EmptyModule {
    const KIND: &'static str = "observation_empty";
    const DISPLAY_NAME: &'static str = "Observation Empty";
    fn port_schema() -> &'static [oxurack::PortSchema] {
        &[]
    }
    fn parameter_schema() -> &'static [oxurack::ParameterSchema] {
        &[]
    }
    fn spawn(
        world: &mut bevy_ecs::world::World,
        _name: &str,
        _parameters: &std::collections::HashMap<String, oxurack::ParameterValue>,
    ) -> Result<bevy_ecs::entity::Entity, oxurack::CoreError> {
        Ok(world.spawn_empty().id())
    }
}

struct LateFailureModule;
impl oxurack::Module for LateFailureModule {
    const KIND: &'static str = "observation_late_failure";
    const DISPLAY_NAME: &'static str = "Observation Late Failure";
    fn port_schema() -> &'static [oxurack::PortSchema] {
        &[]
    }
    fn parameter_schema() -> &'static [oxurack::ParameterSchema] {
        &[]
    }
    fn spawn(
        _world: &mut bevy_ecs::world::World,
        _name: &str,
        _parameters: &std::collections::HashMap<String, oxurack::ParameterValue>,
    ) -> Result<bevy_ecs::entity::Entity, oxurack::CoreError> {
        Err(oxurack::CoreError::NotImplemented(
            "observation late candidate failure",
        ))
    }
}

#[test]
fn actual_load_producers_publish_one_ordered_owned_batch() {
    let path = fixture();
    let mut rack = rack();
    rack.load_patch(&path).unwrap();
    assert!(rack.has_patch());
    assert_eq!(
        rack.patch_info().unwrap().unwrap().origin,
        PatchOrigin::File(path)
    );

    let mut cursor = rack.observe(ObservationStart::OldestRetained).unwrap();
    let first = rack
        .read_observations(&mut cursor, NonZeroUsize::new(7).unwrap())
        .unwrap();
    assert_eq!(first.records.len(), 7);
    assert!(first.gap.is_none());
    assert!(!first.records.last().unwrap().last_in_batch);
    let second = rack
        .read_observations(&mut cursor, NonZeroUsize::new(14).unwrap())
        .unwrap();
    assert_eq!(second.records.len(), 14);
    assert!(second.gap.is_none());
    assert!(
        !second.records[..13]
            .iter()
            .any(|record| record.last_in_batch)
    );
    assert!(second.records.last().unwrap().last_in_batch);
    let records: Vec<_> = first.records.into_iter().chain(second.records).collect();
    assert!(
        records.iter().all(|record| record.batch.get() == 1
            && record.lifetime.get() == 1
            && record.sample == 0)
    );
    assert_eq!(records.first().unwrap().sequence.get(), 1);
    assert_eq!(records.last().unwrap().sequence.get(), 21);
    assert!(matches!(
        records.last().unwrap().change,
        ObservationChange::Core(oxurack_core::OwnedChange::PatchLoaded {
            module_count: 2,
            cable_count: 1,
            label_count: 13,
            ..
        })
    ));
    let chronology: Vec<String> = records
        .iter()
        .map(|record| match &record.change {
            ObservationChange::Core(oxurack_core::OwnedChange::ModuleAdded { module, .. }) => {
                format!("module:{module}")
            }
            ObservationChange::Core(oxurack_core::OwnedChange::LabelChanged {
                target: oxurack_core::LogicalTarget::Module(module),
                kind,
                ..
            }) => format!("label:{module}:{kind:?}"),
            ObservationChange::Core(oxurack_core::OwnedChange::LabelChanged {
                target: oxurack_core::LogicalTarget::Cable(_),
                kind,
                ..
            }) => format!("label:cable:{kind:?}"),
            ObservationChange::Core(oxurack_core::OwnedChange::LabelChanged {
                target: oxurack_core::LogicalTarget::Rack,
                kind,
                ..
            }) => format!("label:rack:{kind:?}"),
            ObservationChange::Core(oxurack_core::OwnedChange::ParameterChanged {
                module,
                param,
                ..
            }) => format!("parameter:{module}:{param}"),
            ObservationChange::Core(oxurack_core::OwnedChange::CableConnected {
                cable, ..
            }) => {
                format!("cable:{}:{}", cable.id.get(), cable.occurrence)
            }
            ObservationChange::Core(oxurack_core::OwnedChange::PatchLoaded { .. }) => {
                "loaded".to_owned()
            }
            other => panic!("unexpected load change: {other:?}"),
        })
        .collect();
    assert_eq!(
        chronology,
        [
            "module:z_clock",
            "label:z_clock:DisplayName",
            "label:z_clock:Description",
            "label:z_clock:Tags",
            "parameter:z_clock:bpm",
            "parameter:z_clock:ppqn",
            "module:a_lfo",
            "label:a_lfo:DisplayName",
            "label:a_lfo:Description",
            "label:a_lfo:Tags",
            "parameter:a_lfo:frequency_hz",
            "parameter:a_lfo:waveform",
            "cable:1:0",
            "label:cable:DisplayName",
            "label:cable:Description",
            "label:cable:Tags",
            "label:rack:DisplayName",
            "label:rack:Description",
            "label:rack:Tags",
            "label:rack:Notes",
            "loaded",
        ]
    );

    let labels: Vec<_> = records
        .iter()
        .filter_map(|record| match &record.change {
            ObservationChange::Core(oxurack_core::OwnedChange::LabelChanged {
                target: oxurack_core::LogicalTarget::Rack,
                kind,
                ..
            }) => Some(format!("{kind:?}")),
            _ => None,
        })
        .collect();
    assert_eq!(labels, ["DisplayName", "Description", "Tags", "Notes"]);
    assert_eq!(rack.cables().unwrap().len(), 1);
    assert_eq!(
        rack.find_tags(TagMatch::Any, &["workbench"]).unwrap().len(),
        4
    );
}

#[test]
fn cursors_are_independent_and_wrong_rack_does_not_advance_them() {
    let mut first = rack();
    first.load_patch(&fixture()).unwrap();
    let mut cursor = first.observe(ObservationStart::OldestRetained).unwrap();
    let clone = cursor.clone();
    let other = rack();
    assert!(matches!(
        other.read_observations(&mut cursor, NonZeroUsize::new(1).unwrap()),
        Err(oxurack::ObservationError::WrongFacade)
    ));
    let one = first
        .read_observations(&mut cursor, NonZeroUsize::new(1).unwrap())
        .unwrap();
    let clone_one = first
        .read_observations(&mut clone.clone(), NonZeroUsize::new(1).unwrap())
        .unwrap();
    assert_eq!(one.records[0].sequence, clone_one.records[0].sequence);
    assert_eq!(one.records[0].sequence.get(), 1);
}

#[test]
fn slow_cursor_reports_exact_eviction_gap_after_live_metadata_writes() {
    let mut rack = rack();
    rack.load_patch(&fixture()).unwrap();
    let mut slow = rack.observe(ObservationStart::Now).unwrap();
    let mut fast = rack.observe(ObservationStart::Now).unwrap();
    for index in 0..1025 {
        rack.set_bpm(100.0 + (index % 20) as f32).unwrap();
        let page = rack
            .read_observations(&mut fast, NonZeroUsize::new(1).unwrap())
            .unwrap();
        assert_eq!(page.records.len(), 1);
    }
    let page = rack
        .read_observations(&mut slow, NonZeroUsize::new(1).unwrap())
        .unwrap();
    let gap = page.gap.unwrap();
    assert_eq!((gap.from.get(), gap.to.get()), (22, 23));
    assert_eq!(page.records[0].sequence.get(), 23);
}

#[test]
fn unload_and_restart_publish_boundaries_and_restart_keeps_file_origin() {
    let path = fixture();
    let mut rack = rack();
    rack.load_patch(&path).unwrap();
    rack.restart().unwrap();
    let info = rack.patch_info().unwrap().unwrap();
    assert_eq!(info.origin, PatchOrigin::File(path.clone()));
    assert_eq!(info.lifetime.get(), 2);
    rack.unload_patch().unwrap();
    assert!(!rack.has_patch());
    rack.unload_patch().unwrap();
    let mut cursor = rack.observe(ObservationStart::OldestRetained).unwrap();
    let page = rack
        .read_observations(&mut cursor, NonZeroUsize::new(64).unwrap())
        .unwrap();
    let unloads: Vec<_> = page
        .records
        .iter()
        .filter_map(|record| match &record.change {
            ObservationChange::PatchUnloaded {
                old,
                successor,
                reason,
            } => Some((old.lifetime.get(), successor.get(), *reason)),
            _ => None,
        })
        .collect();
    assert_eq!(
        unloads,
        [
            (1, 2, oxurack::UnloadReason::Restart),
            (2, 3, oxurack::UnloadReason::ExplicitUnload)
        ]
    );
}

#[test]
fn failed_file_candidate_preserves_current_observation_owner_and_tag_queries_are_exact() {
    let path = fixture();
    let mut rack = rack();
    rack.load_patch(&path).unwrap();
    let before = rack.patch_info().unwrap().unwrap();
    let mut cursor = rack.observe(ObservationStart::Now).unwrap();
    assert!(
        rack.load_patch(Path::new("/definitely/missing/session-observation.ron"))
            .is_err()
    );
    assert_eq!(rack.patch_info().unwrap().unwrap(), before);
    assert!(
        rack.read_observations(&mut cursor, NonZeroUsize::new(8).unwrap())
            .unwrap()
            .records
            .is_empty()
    );

    let all = rack
        .find_tags(TagMatch::All, &["workbench", "workbench"])
        .unwrap();
    assert_eq!(all.len(), 4);
    assert!(all.iter().any(|result| {
        result.target == oxurack_core::LogicalTarget::Module("a_lfo".to_owned())
    }));
    assert!(
        rack.find_tags(TagMatch::Any, &["Workbench"])
            .unwrap()
            .is_empty()
    );
    assert!(matches!(
        rack.find_tags(TagMatch::Any, &[" bad"]),
        Err(oxurack::TagQueryError::InvalidTag { .. })
    ));
}

#[test]
fn late_private_candidate_failure_preserves_old_owner_and_cursor() {
    let path = fixture();
    let mut rack = rack();
    rack.try_install::<EmptyModule>().unwrap();
    rack.try_install::<LateFailureModule>().unwrap();
    rack.load_patch(&path).unwrap();
    let before = rack.patch_info().unwrap().unwrap();
    let sample = rack.sample_time().completed_samples();
    let mut cursor = rack.observe(ObservationStart::Now).unwrap();
    let failed_path = std::env::temp_dir().join(format!(
        "oxurack-observation-late-failure-{}.ron",
        std::process::id()
    ));
    std::fs::write(
        &failed_path,
        "(version: \"1.0\", master_seed: 1, bpm: 120.0, modules: [(kind: \"observation_empty\", instance_name: \"spawned_first\", parameters: {}, display_name: \"\", description: \"\", tags: []), (kind: \"observation_late_failure\", instance_name: \"fail_second\", parameters: {}, display_name: \"\", description: \"\", tags: [])], cables: [], rack: None)",
    )
    .unwrap();

    assert!(rack.load_patch(&failed_path).is_err());

    assert_eq!(rack.patch_info().unwrap().unwrap(), before);
    assert_eq!(rack.sample_time().completed_samples(), sample);
    assert!(
        rack.read_observations(&mut cursor, NonZeroUsize::new(8).unwrap())
            .unwrap()
            .records
            .is_empty()
    );
    std::fs::remove_file(failed_path).unwrap();
}

#[test]
fn direct_and_command_parameter_aliases_publish_once_with_their_source() {
    let mut rack = rack();
    rack.load_patch(&fixture()).unwrap();
    let mut cursor = rack.observe(ObservationStart::Now).unwrap();
    rack.set_parameter("z_clock", "bpm", oxurack::ParameterValue::Float(91.0))
        .unwrap();
    rack.execute_command(&oxurack::CoreCommand::SetParameter {
        module: "z_clock".to_owned(),
        param: "bpm".to_owned(),
        value: oxurack::ParameterValue::Float(122.0),
    })
    .unwrap();
    let page = rack
        .read_observations(&mut cursor, NonZeroUsize::new(8).unwrap())
        .unwrap();
    assert_eq!(page.records.len(), 2);
    let sources: Vec<_> = page
        .records
        .iter()
        .map(|record| match &record.change {
            ObservationChange::Core(oxurack_core::OwnedChange::ParameterChanged {
                source, ..
            }) => source,
            other => panic!("unexpected change: {other:?}"),
        })
        .collect();
    assert_eq!(
        sources,
        [
            &oxurack::ChangeSource::DirectApi,
            &oxurack::ChangeSource::Command
        ]
    );
}

#[test]
fn direct_and_command_label_cable_bpm_and_reset_paths_preserve_provenance() {
    let mut rack = rack();
    rack.load_patch(&fixture()).unwrap();
    let mut cursor = rack.observe(ObservationStart::Now).unwrap();

    rack.set_label(
        &oxurack::LabelTarget::Rack,
        oxurack::LabelKind::Description,
        oxurack::LabelValue::Text("direct".into()),
    )
    .unwrap();
    rack.execute_command(&oxurack::CoreCommand::SetLabel {
        target: oxurack::LabelTarget::Rack,
        kind: oxurack::LabelKind::Description,
        value: oxurack::LabelValue::Text("command".into()),
    })
    .unwrap();
    rack.unpatch(("z_clock", "pulse"), ("a_lfo", "sync_in"), 0)
        .unwrap();
    rack.execute_command(&oxurack::CoreCommand::AddCable {
        source: ("z_clock".into(), "pulse".into()),
        target: ("a_lfo".into(), "sync_in".into()),
        transform: None,
    })
    .unwrap();
    rack.set_bpm(120.0).unwrap();
    rack.execute_command(&oxurack::CoreCommand::SetBpm(121.0))
        .unwrap();
    rack.reset().unwrap();
    rack.execute_command(&oxurack::CoreCommand::Panic).unwrap();

    let page = rack
        .read_observations(&mut cursor, NonZeroUsize::new(16).unwrap())
        .unwrap();
    assert_eq!(page.records.len(), 8);
    let sources: Vec<_> = page
        .records
        .iter()
        .map(|record| match &record.change {
            ObservationChange::Core(oxurack_core::OwnedChange::LabelChanged { source, .. })
            | ObservationChange::Core(oxurack_core::OwnedChange::CableDisconnected {
                source,
                ..
            })
            | ObservationChange::Core(oxurack_core::OwnedChange::CableConnected {
                source, ..
            }) => source.clone(),
            ObservationChange::GlobalBpmChanged { source, .. }
            | ObservationChange::Reset { source } => source.clone(),
            other => panic!("unexpected observation: {other:?}"),
        })
        .collect();
    assert_eq!(
        sources,
        [
            oxurack::ChangeSource::DirectApi,
            oxurack::ChangeSource::Command,
            oxurack::ChangeSource::DirectApi,
            oxurack::ChangeSource::Command,
            oxurack::ChangeSource::DirectApi,
            oxurack::ChangeSource::Command,
            oxurack::ChangeSource::DirectApi,
            oxurack::ChangeSource::Command,
        ]
    );
}

#[test]
fn parallel_cable_identity_survives_occurrence_reindexing() {
    let mut rack = rack();
    let parallel_fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/patches/turing-machine-lfo.ron");
    rack.load_patch(&parallel_fixture).unwrap();
    let occurrence = rack
        .patch(
            ("lfo_main", "bipolar_out"),
            ("tm_main", "write_modulation"),
            None,
        )
        .unwrap();
    assert_eq!(occurrence, 1);
    let before = rack.cables().unwrap();
    assert_eq!(before.len(), 10);
    let id_two = before.iter().find(|info| info.cable.id.get() == 2).unwrap();
    let id_ten = before
        .iter()
        .find(|info| info.cable.id.get() == 10)
        .unwrap();
    assert_eq!(id_two.cable.occurrence, 0);
    assert_eq!(id_ten.cable.occurrence, 1);
    rack.unpatch(
        ("lfo_main", "bipolar_out"),
        ("tm_main", "write_modulation"),
        0,
    )
    .unwrap();
    let after = rack.cables().unwrap();
    let surviving = after.iter().find(|info| info.cable.id.get() == 10).unwrap();
    assert_eq!(surviving.cable.occurrence, 0);
    assert!(!after.iter().any(|info| info.cable.id.get() == 2));
}

#[test]
fn oversized_candidate_keeps_the_exact_staging_offset_gap() {
    let mut rack = Oxurack::new();
    rack.try_install::<EmptyModule>().unwrap();
    let path = std::env::temp_dir().join(format!(
        "oxurack-observation-large-{}.ron",
        std::process::id()
    ));
    let mut source = String::from("(version: \"1.0\", master_seed: 1, bpm: 120.0, modules: [");
    for index in 0..1100 {
        if index > 0 {
            source.push(',');
        }
        source.push_str(&format!("(kind: \"observation_empty\", instance_name: \"m{index}\", parameters: {{}}, display_name: \"\", description: \"\", tags: [])"));
    }
    source.push_str("], cables: [], rack: None)");
    std::fs::write(&path, source).unwrap();
    let mut cursor = rack.observe(ObservationStart::Now).unwrap();
    rack.load_patch(&path).unwrap();
    let page = rack
        .read_observations(&mut cursor, NonZeroUsize::new(1024).unwrap())
        .unwrap();
    let gap = page.gap.unwrap();
    assert_eq!((gap.from.get(), gap.to.get()), (1, 78));
    assert_eq!(page.records.len(), 1024);
    assert_eq!(page.records.first().unwrap().sequence.get(), 78);
    assert_eq!(page.records.last().unwrap().sequence.get(), 1101);
    assert!(page.records.last().unwrap().last_in_batch);
    std::fs::remove_file(path).unwrap();
}
