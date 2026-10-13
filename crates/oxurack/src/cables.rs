use oxurack_core::{CableTransform, ChangeSource, CoreError};

use crate::Oxurack;

impl Oxurack {
    /// Adds one enabled cable to the active patch.
    ///
    /// Names are exact and case-sensitive. The returned index is the current
    /// zero-based occurrence for this endpoint pair, suitable for `LabelTarget`.
    /// With the `graph` feature, graph queries reflect the change on return.
    pub fn patch(
        &mut self,
        source: (&str, &str),
        target: (&str, &str),
        transform: Option<CableTransform>,
    ) -> Result<usize, CoreError> {
        let result = oxurack_core::connect_cable(
            self.app.world_mut(),
            source,
            target,
            transform,
            ChangeSource::DirectApi,
        )?;
        #[cfg(feature = "graph")]
        self.app
            .world_mut()
            .run_schedule(oxurack_graph::GraphMaintenance);
        Ok(result)
    }

    /// Removes exactly one cable occurrence from the active patch.
    ///
    /// Names are exact and case-sensitive. The occurrence follows the same
    /// present ordinal, ordinal value, and entity fallback as label selection.
    /// With the `graph` feature, graph queries reflect the change on return.
    pub fn unpatch(
        &mut self,
        source: (&str, &str),
        target: (&str, &str),
        occurrence: usize,
    ) -> Result<(), CoreError> {
        oxurack_core::disconnect_cable(
            self.app.world_mut(),
            source,
            target,
            occurrence,
            ChangeSource::DirectApi,
        )?;
        #[cfg(feature = "graph")]
        self.app
            .world_mut()
            .run_schedule(oxurack_graph::GraphMaintenance);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::Messages;
    use oxurack_core::{CableConnected, CableDisconnected};
    use oxurack_mod_clock::ClockModule;
    use oxurack_mod_lfo::LfoModule;
    use oxurack_mod_turingmachine::TuringMachineModule;
    use pretty_assertions::assert_eq;
    use std::path::Path;

    const FIXTURE: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/patches/turing-machine-lfo.ron"
    );

    fn loaded() -> Oxurack {
        let mut rack = Oxurack::new();
        rack.try_install::<ClockModule>().unwrap();
        rack.try_install::<LfoModule>().unwrap();
        rack.try_install::<TuringMachineModule>().unwrap();
        rack.try_install::<crate::MidiOutModule>().unwrap();
        rack.load_patch(Path::new(FIXTURE)).unwrap();
        rack.set_driver_mode(crate::SimulationDriver::Manual);
        rack
    }

    fn records(rack: &Oxurack) -> Vec<crate::ScopeRecord> {
        rack.scope().unwrap().records().copied().collect()
    }

    fn advance_and_drain(rack: &mut Oxurack) -> Vec<crate::MidiOutputEvent> {
        assert_eq!(rack.advance_samples(1).unwrap(), 1);
        let events: Vec<_> = rack.drain_midi().collect();
        for event in events.iter().copied() {
            rack.record_scope_midi(event, crate::MidiDelivery::Generated);
        }
        events
    }

    #[test]
    fn live_rewire_preserves_engines_scope_and_pending_sample() {
        let mut subject = loaded();
        let mut control = loaded();
        let selectors = [
            crate::ScopeSelector::new("lfo_main", "bipolar_out"),
            crate::ScopeSelector::new("tm_main", "write_modulation"),
        ];
        subject.configure_scope(&selectors, 4_000).unwrap();
        control.configure_scope(&selectors, 4_000).unwrap();
        subject.start();
        control.start();
        for _ in 0..200 {
            assert_eq!(
                advance_and_drain(&mut subject),
                advance_and_drain(&mut control)
            );
        }
        let source = ("lfo_main", "bipolar_out");
        let target = ("tm_main", "write_modulation");
        let before_time = subject.sample_time();
        let before_driver = subject.driver_mode();
        let before_running = subject.app.world().resource::<crate::SimulationRunning>().0;
        let before_records = records(&subject);
        let before_overwritten = subject.scope().unwrap().overwritten();
        subject
            .app
            .world_mut()
            .resource_mut::<Messages<CableConnected>>()
            .clear();
        subject
            .app
            .world_mut()
            .resource_mut::<Messages<CableDisconnected>>()
            .clear();

        subject.unpatch(source, target, 0).unwrap();
        subject
            .patch(
                source,
                target,
                Some(CableTransform::Affine {
                    factor: 0.5,
                    offset: 0.0,
                }),
            )
            .unwrap();
        let disconnected: Vec<_> = subject
            .app
            .world()
            .resource::<Messages<CableDisconnected>>()
            .iter_current_update_messages()
            .collect();
        let connected: Vec<_> = subject
            .app
            .world()
            .resource::<Messages<CableConnected>>()
            .iter_current_update_messages()
            .collect();
        assert_eq!(disconnected.len(), 1);
        assert_eq!(disconnected[0].source, ChangeSource::DirectApi);
        assert_eq!(connected.len(), 1);
        assert_eq!(connected[0].source, ChangeSource::DirectApi);
        assert_eq!(
            connected[0].transform,
            Some(CableTransform::Affine {
                factor: 0.5,
                offset: 0.0
            })
        );
        assert_eq!(subject.sample_time(), before_time);
        assert_eq!(subject.driver_mode(), before_driver);
        assert_eq!(
            subject.app.world().resource::<crate::SimulationRunning>().0,
            before_running
        );
        assert_eq!(records(&subject), before_records);
        assert_eq!(subject.scope().unwrap().overwritten(), before_overwritten);
        #[cfg(feature = "graph")]
        assert_eq!(subject.graph().cables().len(), 9);

        for _ in 0..51 {
            assert_eq!(
                advance_and_drain(&mut subject),
                advance_and_drain(&mut control)
            );
        }
        assert_eq!(subject.advance_samples(1).unwrap(), 1);
        assert_eq!(control.advance_samples(1).unwrap(), 1);
        assert_eq!(subject.sample_time().completed_samples(), 252);
        assert_eq!(control.sample_time().completed_samples(), 252);
        assert!(matches!(
            subject
                .app
                .world()
                .resource::<crate::MidiOutputBuffer>()
                .check_admission(),
            Err(oxurack_core::TickError::OutputBackpressure {
                pending_messages: 1,
                ..
            })
        ));
        let pending_before = subject.sample_time();
        let pending_records = records(&subject);
        let pending_overwritten = subject.scope().unwrap().overwritten();
        assert!(matches!(
            subject.patch(
                source,
                target,
                Some(CableTransform::Clamp { min: 1.0, max: 0.0 })
            ),
            Err(CoreError::InvalidCableTransform { .. })
        ));
        assert!(matches!(
            subject.unpatch(source, target, usize::MAX),
            Err(CoreError::UnknownCable { .. })
        ));
        assert_eq!(subject.sample_time(), pending_before);
        assert_eq!(records(&subject), pending_records);
        assert_eq!(subject.scope().unwrap().overwritten(), pending_overwritten);
        assert!(matches!(
            subject
                .app
                .world()
                .resource::<crate::MidiOutputBuffer>()
                .check_admission(),
            Err(oxurack_core::TickError::OutputBackpressure {
                pending_messages: 1,
                ..
            })
        ));

        subject.unpatch(source, target, 0).unwrap();
        subject
            .patch(
                source,
                target,
                Some(CableTransform::Affine {
                    factor: 0.5,
                    offset: 0.0,
                }),
            )
            .unwrap();
        assert_eq!(subject.sample_time(), pending_before);
        assert_eq!(records(&subject), pending_records);
        assert_eq!(subject.scope().unwrap().overwritten(), pending_overwritten);
        assert!(matches!(
            subject
                .app
                .world()
                .resource::<crate::MidiOutputBuffer>()
                .check_admission(),
            Err(oxurack_core::TickError::OutputBackpressure {
                pending_messages: 1,
                ..
            })
        ));

        let pending_subject: Vec<_> = subject.drain_midi().collect();
        let pending_control: Vec<_> = control.drain_midi().collect();
        assert_eq!(pending_subject, pending_control);
        assert_eq!(pending_subject.len(), 1);
        assert_eq!(pending_subject[0].sample, 251);
        for event in pending_subject.iter().copied() {
            subject.record_scope_midi(event, crate::MidiDelivery::Generated);
        }
        for event in pending_control.iter().copied() {
            control.record_scope_midi(event, crate::MidiDelivery::Generated);
        }
        while subject.sample_time().completed_samples() < 1_000 {
            assert_eq!(
                advance_and_drain(&mut subject),
                advance_and_drain(&mut control)
            );
            assert_eq!(subject.sample_time(), control.sample_time());
        }
        assert_eq!(records(&subject), records(&control));
        subject.stop();
        control.stop();
    }

    #[test]
    fn configured_parameter_is_visible_through_facade_without_a_tick() {
        let mut rack = loaded();
        let before = rack.sample_time();
        rack.set_parameter(
            "tm_main",
            "write_probability",
            crate::ParameterValue::Float(0.25),
        )
        .unwrap();
        assert_eq!(rack.sample_time(), before);
        assert_eq!(
            rack.parameter("tm_main", "write_probability").unwrap(),
            crate::ParameterValue::Float(0.25)
        );
        #[cfg(feature = "graph")]
        {
            let module = rack
                .graph()
                .module_by_name("tm_main")
                .expect("Turing Machine module is loaded");
            let entity = bevy_ecs::entity::Entity::from_bits(module.entity_bits);
            let parameters = rack.graph().parameters_of(entity);
            assert_eq!(
                parameters
                    .iter()
                    .find(|parameter| parameter.name == "write_probability")
                    .and_then(|parameter| parameter.current.as_ref()),
                Some(&crate::ParameterValue::Float(0.25))
            );
        }
    }
}
