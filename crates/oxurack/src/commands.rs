use std::collections::VecDeque;

use oxurack_core::{CoreCommand, CoreError, PatchSource};

use crate::Oxurack;

const COMMAND_CAPACITY: usize = 64;

/// Opaque, monotonically increasing identifier for an accepted facade command.
///
/// IDs begin at zero for each rack and are never reused. Keep the value returned
/// by [`Oxurack::enqueue_command`] to correlate it with a later outcome.
///
/// ```compile_fail
/// let forged = oxurack::CommandId(42);
/// ```
///
/// ```compile_fail
/// fn inspect(id: oxurack::CommandId) -> u64 {
///     id.0
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandId(u64);

/// Final disposition of one accepted facade command.
///
/// A failure is a completed result; reset cancellation is a distinct final
/// outcome. Pending commands are not exposed as results because they have not
/// yet been collected by [`Oxurack::process_commands`].
///
/// A downstream match must allow for variants added in a later release:
///
/// ```
/// use oxurack::{CommandOutcome, CommandResult};
///
/// fn report(outcomes: Vec<CommandOutcome>) {
///     for outcome in outcomes {
///         match outcome.result {
///             CommandResult::Completed(Ok(())) => {}
///             CommandResult::Completed(Err(error)) => eprintln!("{error}"),
///             CommandResult::CancelledByReset => eprintln!("cancelled by reset"),
///             _ => {}
///         }
///     }
/// }
/// ```
///
/// ```compile_fail
/// use oxurack::CommandResult;
///
/// fn report(result: CommandResult) {
///     match result {
///         CommandResult::Completed(_) => {}
///         CommandResult::CancelledByReset => {}
///     }
/// }
/// ```
#[non_exhaustive]
#[derive(Debug)]
pub enum CommandResult {
    /// The command ran and either succeeded or returned its ordinary error.
    Completed(Result<(), CoreError>),
    /// A successful rack reset cancelled this command before execution.
    CancelledByReset,
}

/// One command's opaque identifier and final result.
#[derive(Debug)]
pub struct CommandOutcome {
    /// Identifier returned when this command was accepted.
    pub id: CommandId,
    /// Completed execution or successful-reset cancellation.
    pub result: CommandResult,
}

enum Slot {
    Pending { id: CommandId, command: CoreCommand },
    Completed(CommandOutcome),
}

pub(crate) struct CommandQueue {
    slots: VecDeque<Slot>,
    next_id: u64,
}

impl CommandQueue {
    pub(crate) fn new() -> Self {
        Self {
            slots: VecDeque::with_capacity(COMMAND_CAPACITY),
            next_id: 0,
        }
    }

    fn enqueue(&mut self, command: CoreCommand) -> Result<CommandId, CoreError> {
        if self.slots.len() >= COMMAND_CAPACITY {
            return Err(CoreError::CommandQueueFull {
                capacity: COMMAND_CAPACITY,
            });
        }
        let id = self.next_id;
        let next = id.checked_add(1).ok_or(CoreError::CommandIdExhausted)?;
        let id = CommandId(id);
        self.slots.push_back(Slot::Pending { id, command });
        self.next_id = next;
        Ok(id)
    }

    pub(crate) fn cancel_pending(&mut self) {
        for slot in &mut self.slots {
            let id = match slot {
                Slot::Pending { id, .. } => Some(*id),
                Slot::Completed(_) => None,
            };
            if let Some(id) = id {
                *slot = Slot::Completed(CommandOutcome {
                    id,
                    result: CommandResult::CancelledByReset,
                });
            }
        }
    }
}

impl Oxurack {
    /// Queues a command in the rack's bounded FIFO.
    ///
    /// Command names, values and paths are resolved when the command executes,
    /// not when it is enqueued. Accepted commands occupy a slot until their
    /// outcome is collected, including commands cancelled by reset.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::CommandQueueFull`] when all 64 slots are occupied,
    /// or [`CoreError::CommandIdExhausted`] when another monotonic ID cannot be
    /// assigned. Either error leaves the queue and next ID unchanged.
    pub fn enqueue_command(&mut self, command: CoreCommand) -> Result<CommandId, CoreError> {
        self.command_queue.enqueue(command)
    }

    /// Executes and collects all queued commands in FIFO order between samples.
    ///
    /// Each pending slot is removed before its command executes. Errors are
    /// returned in that command's outcome and do not stop later commands.
    /// Outcomes are returned exactly once; no sample tick acknowledges a
    /// command. A successful queued Panic cancels only the still-pending tail.
    pub fn process_commands(&mut self) -> Vec<CommandOutcome> {
        let mut outcomes = Vec::with_capacity(self.command_queue.slots.len());
        while let Some(slot) = self.command_queue.slots.pop_front() {
            let outcome = match slot {
                Slot::Completed(outcome) => outcome,
                Slot::Pending { id, command } => CommandOutcome {
                    id,
                    result: CommandResult::Completed(self.execute_command(&command)),
                },
            };
            outcomes.push(outcome);
        }
        outcomes
    }

    /// Executes one command synchronously through the corresponding facade API.
    ///
    /// Command parameter, label and cable mutations retain `Command` event
    /// provenance. Command loads use the real candidate path and identify their
    /// source as `command load: <path>`; initialization events retain
    /// `PatchLoad`. `SetBpm` changes patch metadata and does not edit a Clock
    /// module's parameter.
    ///
    /// # Errors
    ///
    /// Returns the error from the selected command's real operation. This
    /// includes invalid input, lookup, lifecycle/admission, I/O and patch errors.
    /// A `CoreCommand` variant without a facade implementation returns
    /// [`CoreError::NotImplemented`].
    pub fn execute_command(&mut self, command: &CoreCommand) -> Result<(), CoreError> {
        let result = match command {
            CoreCommand::LoadPatch(path) => crate::lifecycle::load_patch_with_source(
                self,
                path,
                PatchSource::Other(format!("command load: {}", path.display())),
            ),
            CoreCommand::SavePatch(path) => self.save_patch(path),
            CoreCommand::SetParameter {
                module,
                param,
                value,
            } => self.set_parameter_with_source(
                module,
                param,
                value.clone(),
                oxurack_core::ChangeSource::Command,
            ),
            CoreCommand::SetLabel {
                target,
                kind,
                value,
            } => oxurack_core::set_label_value(
                self.app.world_mut(),
                target,
                kind,
                value,
                oxurack_core::ChangeSource::Command,
            ),
            CoreCommand::AddCable {
                source,
                target,
                transform,
            } => {
                let result = oxurack_core::connect_cable(
                    self.app.world_mut(),
                    (&source.0, &source.1),
                    (&target.0, &target.1),
                    *transform,
                    oxurack_core::ChangeSource::Command,
                )
                .map(|_| ());
                if result.is_ok() {
                    self.maintain_graph();
                }
                result
            }
            CoreCommand::RemoveCable { source, target } => {
                let result = oxurack_core::disconnect_cable(
                    self.app.world_mut(),
                    (&source.0, &source.1),
                    (&target.0, &target.1),
                    0,
                    oxurack_core::ChangeSource::Command,
                );
                if result.is_ok() {
                    self.maintain_graph();
                }
                result
            }
            CoreCommand::SetBpm(bpm) => {
                self.set_bpm_with_source(*bpm, oxurack_core::ChangeSource::Command)
            }
            CoreCommand::Panic => self.reset_with_source(oxurack_core::ChangeSource::Command),
            _ => Err(CoreError::NotImplemented("unknown CoreCommand")),
        };
        if result.is_ok() {
            self.flush_staging();
        }
        result
    }

    /// Returns the loaded patch's global beats-per-minute value.
    ///
    /// This metadata value is independent of every Clock module's `bpm`
    /// parameter.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::NoPatchLoaded`] if the rack has no loaded patch.
    pub fn bpm(&self) -> Result<f32, CoreError> {
        self.loaded_identity
            .as_ref()
            .map(crate::persistence::LoadedIdentity::bpm)
            .ok_or(CoreError::NoPatchLoaded)
    }

    /// Sets the loaded patch's global BPM metadata without changing a module.
    ///
    /// Snapshots, saves and restarts retain this value. It does not change the
    /// Clock module parameter used for audible tempo.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::NoPatchLoaded`] without a loaded patch, or
    /// [`CoreError::Patch`] wrapping [`oxurack_core::PatchError::InvalidBpm`]
    /// unless `bpm` is finite and greater than zero.
    pub fn set_bpm(&mut self, bpm: f32) -> Result<(), CoreError> {
        self.set_bpm_with_source(bpm, oxurack_core::ChangeSource::DirectApi)
    }

    fn set_bpm_with_source(
        &mut self,
        bpm: f32,
        source: oxurack_core::ChangeSource,
    ) -> Result<(), CoreError> {
        if !(bpm > 0.0 && bpm.is_finite()) {
            return Err(CoreError::Patch(oxurack_core::PatchError::InvalidBpm(bpm)));
        }
        let identity = self
            .loaded_identity
            .as_mut()
            .ok_or(CoreError::NoPatchLoaded)?;
        let previous = identity.bpm();
        identity.set_bpm(bpm);
        if let Some(info) = self.observation.current.as_mut() {
            info.bpm = bpm;
        }
        self.observation.publish(
            self.observation.lifetime,
            self.sample_time().completed_samples(),
            vec![crate::ObservationChange::GlobalBpmChanged {
                previous,
                bpm,
                source,
            }],
        );
        Ok(())
    }

    fn maintain_graph(&mut self) {
        #[cfg(feature = "graph")]
        self.app
            .world_mut()
            .run_schedule(oxurack_graph::GraphMaintenance);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxurack_mod_clock::ClockModule;
    use oxurack_mod_lfo::LfoModule;
    use oxurack_mod_turingmachine::TuringMachineModule;

    fn loaded() -> Oxurack {
        let mut rack = Oxurack::new();
        rack.try_install::<ClockModule>().unwrap();
        rack.try_install::<LfoModule>().unwrap();
        rack.try_install::<TuringMachineModule>().unwrap();
        rack.try_install::<crate::MidiOutModule>().unwrap();
        let path = std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/patches/turing-machine-lfo.ron"
        ));
        rack.load_patch(path).unwrap();
        rack
    }

    #[test]
    fn direct_reset_retains_cancellation_until_one_fifo_collection() {
        let mut rack = loaded();
        rack.enqueue_command(CoreCommand::SetBpm(133.0)).unwrap();
        rack.reset().unwrap();
        let outcomes = rack.process_commands();
        assert_eq!(outcomes.len(), 1);
        assert!(matches!(
            outcomes[0].result,
            CommandResult::CancelledByReset
        ));
        assert!(rack.process_commands().is_empty());
    }

    #[test]
    fn failed_reset_preserves_pending_command_and_all_observed_state_immediately() {
        use std::time::Duration;

        use crate::{ScopeRecord, ScopeSelector};
        use bevy_ecs::message::{Message, Messages};
        use bevy_time::prelude::{Fixed, Time, Virtual};
        use oxurack_core::{
            CableConnected, CableDisconnected, LabelChanged, ModuleAdded, ModuleRemoved,
            ParameterChanged, PatchLoaded, SimulationRunning, TickError,
        };

        #[derive(Debug, PartialEq)]
        struct Projection {
            queued: Vec<String>,
            next_id: u64,
            sample_time: oxurack_core::SampleTime,
            driver: crate::SimulationDriver,
            running: bool,
            fixed_elapsed: Duration,
            fixed_overstep: Duration,
            virtual_elapsed: Duration,
            virtual_delta: Duration,
            virtual_paused: bool,
            pending_midi_count: usize,
            pending_midi_and_fault: String,
            fault: Option<String>,
            selectors: Vec<ScopeSelector>,
            scope_capacity: usize,
            scope_overwritten: u64,
            scope_records: Vec<ScopeRecord>,
            patch_loaded: Vec<String>,
            module_added: Vec<String>,
            module_removed: Vec<String>,
            parameters: Vec<String>,
            labels: Vec<String>,
            cables_connected: Vec<String>,
            cables_disconnected: Vec<String>,
        }

        fn current_messages<M: Message + std::fmt::Debug>(
            world: &bevy_ecs::world::World,
        ) -> Vec<String> {
            world
                .resource::<Messages<M>>()
                .iter_current_update_messages()
                .map(|message| format!("{message:?}"))
                .collect()
        }

        fn project(rack: &Oxurack) -> Projection {
            let world = rack.app.world();
            let fixed = world.resource::<Time<Fixed>>();
            let virtual_time = world.resource::<Time<Virtual>>();
            let output = world.resource::<crate::MidiOutputBuffer>();
            let scope = rack.scope().expect("Scope configured for reset test");
            let queued = rack
                .command_queue
                .slots
                .iter()
                .map(|slot| match slot {
                    Slot::Pending { id, command } => {
                        format!("pending {id:?} {command:?}")
                    }
                    Slot::Completed(outcome) => format!("completed {outcome:?}"),
                })
                .collect();
            Projection {
                queued,
                next_id: rack.command_queue.next_id,
                sample_time: rack.sample_time(),
                driver: rack.driver_mode(),
                running: world.resource::<SimulationRunning>().0,
                fixed_elapsed: fixed.elapsed(),
                fixed_overstep: fixed.overstep(),
                virtual_elapsed: virtual_time.elapsed(),
                virtual_delta: virtual_time.delta(),
                virtual_paused: virtual_time.is_paused(),
                pending_midi_count: output.pending_count(),
                pending_midi_and_fault: format!("{output:?}"),
                fault: output.fault().map(|fault| format!("{fault:?}")),
                selectors: scope.selectors().to_vec(),
                scope_capacity: scope.capacity(),
                scope_overwritten: scope.overwritten(),
                scope_records: scope.records().copied().collect(),
                patch_loaded: current_messages::<PatchLoaded>(world),
                module_added: current_messages::<ModuleAdded>(world),
                module_removed: current_messages::<ModuleRemoved>(world),
                parameters: current_messages::<ParameterChanged>(world),
                labels: current_messages::<LabelChanged>(world),
                cables_connected: current_messages::<CableConnected>(world),
                cables_disconnected: current_messages::<CableDisconnected>(world),
            }
        }

        let mut rack = loaded();
        rack.set_driver_mode(crate::SimulationDriver::Manual);
        rack.configure_scope(
            &[
                ScopeSelector::new("clock_main", "pulse"),
                ScopeSelector::new("tm_main", "write_probability"),
            ],
            64,
        )
        .unwrap();
        rack.start();
        for _ in 0..16 {
            assert_eq!(rack.advance_samples(1).unwrap(), 1);
        }
        let initial_midi: Vec<_> = rack.drain_midi().collect();
        for event in initial_midi {
            rack.record_scope_midi(event, crate::MidiDelivery::Generated);
        }
        rack.stop();
        assert!(!rack.scope().unwrap().records().next().is_none());

        let first = rack.enqueue_command(CoreCommand::SetBpm(133.0)).unwrap();
        let second = rack
            .enqueue_command(CoreCommand::SetParameter {
                module: "clock_main".into(),
                param: "bpm".into(),
                value: oxurack_core::ParameterValue::Float(121.0),
            })
            .unwrap();
        let marker = TickError::OutputCapacity {
            reason: "test-injected latched fault marker".to_owned(),
            completed_samples: 16,
        };
        rack.app
            .world_mut()
            .resource_mut::<crate::MidiOutputBuffer>()
            .latch_fault(marker.clone());
        let sink = rack
            .app
            .world()
            .resource::<oxurack_core::PatchHandle>()
            .modules["midi_main"];
        rack.app.world_mut().despawn(sink);

        let before = project(&rack);
        assert!(matches!(
            rack.reset(),
            Err(CoreError::Tick(TickError::OutputCapacity { ref reason, .. }))
                if reason == "MIDI sink and reservation accounting disagree"
        ));
        assert_eq!(
            project(&rack),
            before,
            "failed reset changed observed rack state"
        );

        let outcomes = rack.process_commands();
        assert_eq!(outcomes.len(), 2);
        assert_eq!(outcomes[0].id, first);
        assert_eq!(outcomes[1].id, second);
        assert!(matches!(
            outcomes[0].result,
            CommandResult::Completed(Ok(()))
        ));
        assert!(matches!(
            outcomes[1].result,
            CommandResult::Completed(Ok(()))
        ));
        assert_eq!(rack.bpm().unwrap(), 133.0);
        assert_eq!(
            rack.parameter("clock_main", "bpm").unwrap(),
            oxurack_core::ParameterValue::Float(121.0)
        );
        assert_eq!(
            rack.app
                .world()
                .resource::<crate::MidiOutputBuffer>()
                .fault()
                .map(|fault| format!("{fault:?}")),
            Some(format!("{marker:?}"))
        );
    }

    #[test]
    fn identifier_exhaustion_does_not_accept_or_wrap() {
        let mut queue = CommandQueue::new();
        queue.next_id = u64::MAX;
        assert!(matches!(
            queue.enqueue(CoreCommand::Panic),
            Err(CoreError::CommandIdExhausted)
        ));
        assert!(queue.slots.is_empty());
        assert_eq!(queue.next_id, u64::MAX);
    }

    #[test]
    fn command_load_uses_command_provenance() {
        let mut rack = loaded();
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples/patches/turing-machine-lfo.ron");
        rack.execute_command(&CoreCommand::LoadPatch(path.clone()))
            .unwrap();
        let events: Vec<_> = rack
            .app
            .world()
            .resource::<bevy_ecs::message::Messages<oxurack_core::PatchLoaded>>()
            .iter_current_update_messages()
            .collect();
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0].source,
            PatchSource::Other(format!("command load: {}", path.display()))
        );
    }
}
