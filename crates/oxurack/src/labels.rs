use oxurack_core::{ChangeSource, CoreError, LabelKind, LabelTarget, LabelValue};

impl super::Oxurack {
    /// Reads one raw label value from the active rack, module, or cable.
    ///
    /// Entity-level empty text and tag components are observable until cleared;
    /// this method does not apply graph display-name fallback.
    pub fn label(
        &mut self,
        target: &LabelTarget,
        kind: LabelKind,
    ) -> Result<Option<LabelValue>, CoreError> {
        oxurack_core::label_value(self.app.world_mut(), target, &kind)
    }

    /// Writes one label value synchronously and emits one `LabelChanged` message.
    ///
    /// This call does not advance a sample or alter topology, cable order, or
    /// module parameters.
    pub fn set_label(
        &mut self,
        target: &LabelTarget,
        kind: LabelKind,
        value: LabelValue,
    ) -> Result<(), CoreError> {
        oxurack_core::set_label_value(
            self.app.world_mut(),
            target,
            &kind,
            &value,
            ChangeSource::DirectApi,
        )?;
        self.flush_staging();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use bevy_ecs::prelude::Messages;
    use oxurack_core::{ChangeSource, CoreCommand, LabelChanged, ParameterChanged, ParameterValue};
    use oxurack_mod_clock::ClockModule;
    use oxurack_mod_lfo::LfoModule;
    use oxurack_mod_turingmachine::TuringMachineModule;

    use crate::{
        MidiDelivery, MidiOutModule, MidiOutputBuffer, Oxurack, ScopeRecord, ScopeSelector,
        SimulationDriver,
    };

    const FIXTURE: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/patches/turing-machine-lfo.ron"
    );

    fn loaded_rack() -> Oxurack {
        let mut rack = Oxurack::new();
        rack.try_install::<ClockModule>().unwrap();
        rack.try_install::<LfoModule>().unwrap();
        rack.try_install::<TuringMachineModule>().unwrap();
        rack.try_install::<MidiOutModule>().unwrap();
        rack.load_patch(Path::new(FIXTURE)).unwrap();
        rack.set_driver_mode(SimulationDriver::Manual);
        rack
    }

    fn label_events(rack: &mut Oxurack) -> Vec<LabelChanged> {
        rack.world_mut()
            .resource_mut::<Messages<LabelChanged>>()
            .drain()
            .collect()
    }

    fn parameter_events(rack: &mut Oxurack) -> Vec<ParameterChanged> {
        rack.world_mut()
            .resource_mut::<Messages<ParameterChanged>>()
            .drain()
            .collect()
    }

    fn clear_label_events(rack: &mut Oxurack) {
        rack.world_mut()
            .resource_mut::<Messages<LabelChanged>>()
            .clear();
    }

    fn advance_one_and_drain(rack: &mut Oxurack) -> Vec<crate::MidiOutputEvent> {
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        let events: Vec<_> = rack.drain_midi().collect();
        for event in events.iter().copied() {
            rack.record_scope_midi(event, MidiDelivery::Generated);
        }
        events
    }

    fn records(rack: &Oxurack) -> Vec<ScopeRecord> {
        rack.scope().unwrap().records().copied().collect()
    }

    #[test]
    fn direct_and_command_events_have_exact_fields_and_call_order() {
        let mut rack = loaded_rack();
        clear_label_events(&mut rack);
        rack.world_mut()
            .resource_mut::<Messages<ParameterChanged>>()
            .clear();
        let module = super::LabelTarget::Module("tm_main".into());

        rack.set_label(
            &module,
            super::LabelKind::DisplayName,
            super::LabelValue::Text("Lead".into()),
        )
        .unwrap();
        let direct = label_events(&mut rack);
        assert_eq!(direct.len(), 1);
        assert!(direct[0].entity.is_some());
        assert_eq!(direct[0].instance_name, "tm_main");
        assert_eq!(direct[0].kind, super::LabelKind::DisplayName);
        assert_eq!(direct[0].previous, None);
        assert_eq!(direct[0].new, super::LabelValue::Text("Lead".into()));
        assert_eq!(direct[0].source, ChangeSource::DirectApi);
        clear_label_events(&mut rack);

        rack.set_label(
            &module,
            super::LabelKind::DisplayName,
            super::LabelValue::Text("Lead".into()),
        )
        .unwrap();
        let equal = label_events(&mut rack);
        assert_eq!(equal.len(), 1);
        assert_eq!(
            equal[0].previous,
            Some(super::LabelValue::Text("Lead".into()))
        );
        assert_eq!(equal[0].new, super::LabelValue::Text("Lead".into()));
        assert_eq!(equal[0].source, ChangeSource::DirectApi);
        clear_label_events(&mut rack);

        rack.set_label(
            &module,
            super::LabelKind::DisplayName,
            super::LabelValue::Text(String::new()),
        )
        .unwrap();
        let cleared = label_events(&mut rack);
        assert_eq!(cleared.len(), 1);
        assert_eq!(
            cleared[0].previous,
            Some(super::LabelValue::Text("Lead".into()))
        );
        assert_eq!(cleared[0].new, super::LabelValue::Text(String::new()));
        assert_eq!(cleared[0].source, ChangeSource::DirectApi);
        clear_label_events(&mut rack);

        rack.set_label(
            &module,
            super::LabelKind::DisplayName,
            super::LabelValue::Text(String::new()),
        )
        .unwrap();
        let cleared_again = label_events(&mut rack);
        assert_eq!(cleared_again.len(), 1);
        assert_eq!(cleared_again[0].previous, None);
        assert_eq!(cleared_again[0].new, super::LabelValue::Text(String::new()));

        rack.set_label(
            &super::LabelTarget::Rack,
            super::LabelKind::Description,
            super::LabelValue::Text(String::new()),
        )
        .unwrap();
        let rack_event = label_events(&mut rack).pop().unwrap();
        assert_eq!(rack_event.entity, None);
        assert_eq!(rack_event.instance_name, "");
        assert_eq!(
            rack_event.previous,
            Some(super::LabelValue::Text(String::new()))
        );

        clear_label_events(&mut rack);
        let mut observed = Vec::new();
        rack.set_label(
            &module,
            super::LabelKind::Description,
            super::LabelValue::Text("direct label".into()),
        )
        .unwrap();
        let direct_label = label_events(&mut rack);
        assert_eq!(direct_label.len(), 1);
        assert_eq!(direct_label[0].source, ChangeSource::DirectApi);
        observed.push(("label", direct_label[0].source.clone()));

        let parameter = CoreCommand::SetParameter {
            module: "lfo_main".into(),
            param: "frequency_hz".into(),
            value: ParameterValue::Float(0.5),
        };
        oxurack_core::dispatch_core_command(rack.world_mut(), &parameter).unwrap();
        let param_observed = parameter_events(&mut rack);
        assert_eq!(param_observed.len(), 1);
        assert_eq!(param_observed[0].source, ChangeSource::Command);
        observed.push(("parameter", param_observed[0].source.clone()));

        let command = CoreCommand::SetLabel {
            target: module.clone(),
            kind: super::LabelKind::Description,
            value: super::LabelValue::Text("command label".into()),
        };
        oxurack_core::dispatch_core_command(rack.world_mut(), &command).unwrap();
        let command_events = label_events(&mut rack);
        assert_eq!(command_events.len(), 1);
        assert_eq!(command_events[0].source, ChangeSource::Command);
        assert_eq!(
            command_events[0].new,
            super::LabelValue::Text("command label".into())
        );
        assert!(parameter_events(&mut rack).is_empty());
        observed.push(("label", command_events[0].source.clone()));
        assert_eq!(
            observed,
            vec![
                ("label", ChangeSource::DirectApi),
                ("parameter", ChangeSource::Command),
                ("label", ChangeSource::Command),
            ]
        );
    }

    #[test]
    fn metadata_calls_preserve_running_engines_scope_and_pending_midi() {
        let mut subject = loaded_rack();
        let mut control = loaded_rack();
        let selectors = [ScopeSelector::new("lfo_main", "bipolar_out")];
        subject.configure_scope(&selectors, 4_000).unwrap();
        control.configure_scope(&selectors, 4_000).unwrap();
        subject.start();
        control.start();

        for _ in 0..200 {
            assert_eq!(
                advance_one_and_drain(&mut subject),
                advance_one_and_drain(&mut control)
            );
        }
        let before_time = subject.sample_time();
        let before_driver = subject.driver_mode();
        let before_running = subject.world().resource::<crate::SimulationRunning>().0;
        let before_records = records(&subject);
        let before_overwritten = subject.scope().unwrap().overwritten();
        let module = super::LabelTarget::Module("tm_main".into());

        assert_eq!(
            subject
                .label(&module, super::LabelKind::DisplayName)
                .unwrap(),
            None
        );
        subject
            .set_label(
                &module,
                super::LabelKind::DisplayName,
                super::LabelValue::Text("Melody".into()),
            )
            .unwrap();
        subject
            .set_label(
                &module,
                super::LabelKind::DisplayName,
                super::LabelValue::Text("Melody".into()),
            )
            .unwrap();
        subject
            .set_label(
                &module,
                super::LabelKind::DisplayName,
                super::LabelValue::Text(String::new()),
            )
            .unwrap();
        clear_label_events(&mut subject);
        let before_rejection = subject
            .label(&module, super::LabelKind::DisplayName)
            .unwrap();
        assert!(matches!(
            subject.set_label(
                &module,
                super::LabelKind::Notes,
                super::LabelValue::Notes(vec!["unsupported".into()]),
            ),
            Err(crate::CoreError::InvalidLabelValue { .. })
        ));
        assert_eq!(
            subject
                .label(&module, super::LabelKind::DisplayName)
                .unwrap(),
            before_rejection
        );
        assert!(label_events(&mut subject).is_empty());
        let save =
            std::env::temp_dir().join(format!("label-live-state-{}.ron", std::process::id()));
        subject.save_patch(&save).unwrap();

        assert_eq!(subject.sample_time(), before_time);
        assert_eq!(subject.driver_mode(), before_driver);
        assert_eq!(
            subject.world().resource::<crate::SimulationRunning>().0,
            before_running
        );
        assert_eq!(records(&subject), before_records);
        assert_eq!(subject.scope().unwrap().overwritten(), before_overwritten);
        std::fs::remove_file(save).unwrap();

        // Keep both runners in lockstep and leave sample 251's real event pending
        // after sample 252 completes.
        for _ in 0..51 {
            let left = advance_one_and_drain(&mut subject);
            let right = advance_one_and_drain(&mut control);
            assert_eq!(left, right);
        }
        assert_eq!(subject.advance_samples(1).unwrap(), 1);
        assert_eq!(control.advance_samples(1).unwrap(), 1);
        assert_eq!(subject.sample_time().completed_samples(), 252);
        assert_eq!(control.sample_time().completed_samples(), 252);
        let pending = subject
            .world()
            .resource::<MidiOutputBuffer>()
            .check_admission()
            .unwrap_err();
        assert!(matches!(
            pending,
            oxurack_core::TickError::OutputBackpressure {
                pending_messages: 1,
                ..
            }
        ));
        let pending_before = subject.sample_time();
        subject
            .set_label(
                &super::LabelTarget::Rack,
                super::LabelKind::Description,
                super::LabelValue::Text("after sample 252".into()),
            )
            .unwrap();
        clear_label_events(&mut subject);
        assert!(matches!(
            subject.set_label(
                &module,
                super::LabelKind::Notes,
                super::LabelValue::Notes(vec![]),
            ),
            Err(crate::CoreError::InvalidLabelValue { .. })
        ));
        assert!(label_events(&mut subject).is_empty());
        assert_eq!(subject.sample_time(), pending_before);
        assert!(matches!(
            subject
                .world()
                .resource::<MidiOutputBuffer>()
                .check_admission(),
            Err(oxurack_core::TickError::OutputBackpressure {
                pending_messages: 1,
                ..
            })
        ));
        let pending_records = records(&subject);
        let pending_overwritten = subject.scope().unwrap().overwritten();
        assert_eq!(
            subject
                .label(&module, super::LabelKind::DisplayName)
                .unwrap(),
            None
        );
        let pending_save =
            std::env::temp_dir().join(format!("label-pending-state-{}.ron", std::process::id()));
        subject.save_patch(&pending_save).unwrap();
        assert_eq!(records(&subject), pending_records);
        assert_eq!(subject.scope().unwrap().overwritten(), pending_overwritten);
        std::fs::remove_file(pending_save).unwrap();

        let pending_subject: Vec<_> = subject.drain_midi().collect();
        let pending_control: Vec<_> = control.drain_midi().collect();
        assert_eq!(pending_subject.len(), 1);
        assert_eq!(pending_subject, pending_control);
        assert_eq!(pending_subject[0].sample, 251);
        for event in pending_subject.iter().copied() {
            subject.record_scope_midi(event, MidiDelivery::Generated);
        }
        for event in pending_control.iter().copied() {
            control.record_scope_midi(event, MidiDelivery::Generated);
        }
        assert!(subject.drain_midi().next().is_none());
        assert!(control.drain_midi().next().is_none());

        while subject.sample_time().completed_samples() < 1_000 {
            assert_eq!(
                advance_one_and_drain(&mut subject),
                advance_one_and_drain(&mut control)
            );
            assert_eq!(subject.sample_time(), control.sample_time());
        }
        assert_eq!(records(&subject), records(&control));
    }
}
