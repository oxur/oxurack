//! Whole-App lifetime operations for [`super::Oxurack`].

use std::path::Path;
use std::time::Instant;

use bevy_time::prelude::{Fixed, Real, Time, Virtual};
use oxurack_core::{CoreError, ModuleRegistry, Patch, PatchSource, SimulationRunning};
use oxurack_mod_midiout::{MidiOutputBuffer, prepare_silence, silence_outputs};

use crate::{Oxurack, build_base_app};

impl Oxurack {
    /// Appends complete MIDI voice silence while preserving this rack lifetime.
    ///
    /// This does not tick, reseed, stop, or clear a previously latched fault.
    /// The output must still be drained before another sample can be admitted.
    ///
    /// # Errors
    ///
    /// Returns an output capacity/invariant error before changing events or
    /// voice state when the complete silence batch cannot be prepared.
    pub fn reset(&mut self) -> Result<(), CoreError> {
        silence_outputs(self.app.world_mut()).map_err(CoreError::Tick)
    }

    /// Rebuilds the loaded patch from its current live snapshot at sample zero.
    ///
    /// The current rack must be stopped and drained. Installed module hooks run
    /// once in their original successful install order in the new App. Arbitrary
    /// World state is not cloned. User hook/spawner panic and external side
    /// effects are outside the rollback guarantee.
    ///
    /// # Errors
    ///
    /// Returns stopped/pending-output errors before snapshot errors, then any
    /// snapshot, candidate, or capacity error without replacing the old App.
    pub fn restart(&mut self) -> Result<(), CoreError> {
        self.require_stopped_and_drained()?;
        let patch = self.snapshot_patch()?;
        self.replace_configuration(&patch, PatchSource::InMemory)
    }

    /// Replaces the current patch with an empty, stopped rack lifetime.
    ///
    /// Cleanup is carried into the new zero-sink buffer and remains drainable.
    /// Repeated unload of an already empty rack is a no-op after admission.
    ///
    /// # Errors
    ///
    /// Returns stopped/pending-output errors before candidate construction.
    pub fn unload_patch(&mut self) -> Result<(), CoreError> {
        self.require_stopped_and_drained()?;
        if self.loaded_identity.is_none() {
            return Ok(());
        }
        let cleanup = prepare_silence(self.app.world())?;
        let mut next = build_base_app()?;
        self.replay_installations(&mut next);
        validate_candidate(&next)?;
        maintain_candidate(&mut next);
        next.world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .append_silence(&cleanup)?;
        self.commit(next, None);
        Ok(())
    }

    fn require_stopped_and_drained(&self) -> Result<(), CoreError> {
        if self.app.world().resource::<SimulationRunning>().0 {
            return Err(CoreError::LifecycleRequiresStopped);
        }
        let count = self
            .app
            .world()
            .resource::<MidiOutputBuffer>()
            .pending_count();
        if count != 0 {
            return Err(CoreError::PendingMidi { count });
        }
        Ok(())
    }

    fn replay_installations(&self, next: &mut bevy_app::App) {
        for (index, recipe) in self.install_order.iter().enumerate() {
            debug_assert_eq!(self.installed_modules.get(&recipe.kind), Some(&index));
            (recipe.install_into)(next);
        }
    }

    fn replace_configuration(
        &mut self,
        patch: &Patch,
        source: PatchSource,
    ) -> Result<(), CoreError> {
        self.require_stopped_and_drained()?;
        let identity = super::persistence::LoadedIdentity::from_patch(patch);
        let cleanup = prepare_silence(self.app.world())?;
        let mut next = build_base_app()?;
        self.replay_installations(&mut next);
        apply_in_candidate(&mut next, patch, source)?;
        validate_candidate(&next)?;
        maintain_candidate(&mut next);
        next.world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .append_silence(&cleanup)?;
        self.commit(next, Some(identity));
        Ok(())
    }

    fn commit(
        &mut self,
        next: bevy_app::App,
        identity: Option<super::persistence::LoadedIdentity>,
    ) {
        let old = std::mem::replace(&mut self.app, next);
        self.loaded_identity = identity;
        drop(old);
    }
}

pub(super) fn load_patch(rack: &mut Oxurack, path: &Path) -> Result<(), CoreError> {
    rack.require_stopped_and_drained()?;
    let patch = oxurack_core::load_patch_from_file(path)?;
    rack.replace_configuration(&patch, PatchSource::File(path.to_path_buf()))
}

fn apply_in_candidate(
    app: &mut bevy_app::App,
    patch: &Patch,
    source: PatchSource,
) -> Result<(), CoreError> {
    app.world_mut()
        .resource_scope::<ModuleRegistry, _>(|world, registry| {
            let handle = oxurack_core::apply_patch_to_world(patch, &registry, world, source)?;
            world.insert_resource(handle);
            Ok(())
        })
}

fn validate_candidate(app: &bevy_app::App) -> Result<(), CoreError> {
    let buffer = app.world().resource::<MidiOutputBuffer>();
    let count = buffer.pending_count();
    if count != 0 {
        return Err(CoreError::CandidateMidiNotEmpty { count });
    }
    if let Some(fault) = buffer.fault() {
        return Err(CoreError::Tick(fault.clone()));
    }
    Ok(())
}

fn maintain_candidate(app: &mut bevy_app::App) {
    let world = app.world_mut();
    world.resource_mut::<SimulationRunning>().0 = false;
    world.resource_mut::<Time<Virtual>>().pause();
    world
        .resource_mut::<Time<Fixed>>()
        .discard_overstep(std::time::Duration::MAX);
    world
        .resource_mut::<Time<Real>>()
        .update_with_instant(Instant::now());
    #[cfg(feature = "graph")]
    world.run_schedule(oxurack_graph::GraphMaintenance);
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxurack_core::TickError;
    use oxurack_mod_clock::ClockModule;
    use oxurack_mod_lfo::LfoModule;
    use oxurack_mod_turingmachine::TuringMachineModule;

    const PATCH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/patches/turing-machine-midi.ron"
    );

    #[test]
    fn reset_preserves_first_fault_and_replacement_clears_it() {
        let mut rack = Oxurack::new();
        rack.try_install::<ClockModule>().unwrap();
        rack.try_install::<LfoModule>().unwrap();
        rack.try_install::<TuringMachineModule>().unwrap();
        rack.try_install::<oxurack_mod_midiout::MidiOutModule>()
            .unwrap();
        rack.load_patch(Path::new(PATCH)).unwrap();
        let original = TickError::OutputCapacity {
            reason: "controlled lifecycle fault fixture".into(),
            completed_samples: 1,
        };
        rack.world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .latch_fault(original.clone());
        rack.reset().unwrap();
        assert!(matches!(
            rack.midi_output_error(),
            Some(TickError::OutputCapacity { reason, completed_samples: 1 })
                if reason == "controlled lifecycle fault fixture"
        ));
        assert!(
            rack.load_patch(Path::new("/tmp/oxurack-lifecycle-missing-patch.ron"))
                .is_err()
        );
        assert!(matches!(
            rack.midi_output_error(),
            Some(TickError::OutputCapacity { reason, completed_samples: 1 })
                if reason == "controlled lifecycle fault fixture"
        ));
        rack.load_patch(Path::new(PATCH)).unwrap();
        assert!(rack.midi_output_error().is_none());
        assert_eq!(rack.sample_time().completed_samples(), 0);
    }
}
