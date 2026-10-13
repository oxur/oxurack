use std::collections::VecDeque;

use oxurack_core::{CoreCommand, CoreError, PatchSource};

use crate::Oxurack;

const COMMAND_CAPACITY: usize = 64;

/// Monotonic identifier assigned when a facade command is accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandId(pub u64);

/// Final disposition of one accepted facade command.
#[derive(Debug)]
pub enum CommandStatus {
    Pending,
    Completed(Result<(), CoreError>),
    CancelledByReset,
}

/// One command's identifier and current disposition.
#[derive(Debug)]
pub struct CommandResult {
    pub id: CommandId,
    pub status: CommandStatus,
}

struct Slot {
    id: CommandId,
    command: CoreCommand,
    status: CommandStatus,
}

pub(crate) struct CommandQueue {
    slots: VecDeque<Slot>,
    next_id: u64,
}

impl CommandQueue {
    pub(crate) fn new() -> Self {
        Self {
            slots: VecDeque::new(),
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
        self.slots.push_back(Slot {
            id: CommandId(id),
            command,
            status: CommandStatus::Pending,
        });
        self.next_id = next;
        Ok(CommandId(id))
    }
    pub(crate) fn cancel_pending(&mut self) {
        for slot in &mut self.slots {
            if matches!(slot.status, CommandStatus::Pending) {
                slot.status = CommandStatus::CancelledByReset;
            }
        }
    }
}

impl Oxurack {
    /// Queues a command in the rack's bounded FIFO.
    pub fn enqueue_command(&mut self, command: CoreCommand) -> Result<CommandId, CoreError> {
        self.command_queue.enqueue(command)
    }

    /// Executes and collects queued commands in FIFO order between samples.
    pub fn process_commands(&mut self) -> Vec<CommandResult> {
        let mut output = Vec::new();
        while let Some(slot) = self.command_queue.slots.pop_front() {
            let status = match slot.status {
                CommandStatus::Pending => {
                    CommandStatus::Completed(self.execute_command(&slot.command))
                }
                completed => completed,
            };
            output.push(CommandResult {
                id: slot.id,
                status,
            });
        }
        output
    }

    /// Executes one command synchronously using the same facade behavior as
    /// the corresponding direct API.
    pub fn execute_command(&mut self, command: &CoreCommand) -> Result<(), CoreError> {
        match command {
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
            CoreCommand::SetBpm(bpm) => self.set_bpm(*bpm),
            CoreCommand::Panic => self.reset(),
            _ => Err(CoreError::NotImplemented("unknown CoreCommand")),
        }
    }

    /// Returns the loaded patch's global beats-per-minute value.
    pub fn bpm(&self) -> Result<f32, CoreError> {
        self.loaded_identity
            .as_ref()
            .map(crate::persistence::LoadedIdentity::bpm)
            .ok_or(CoreError::NoPatchLoaded)
    }

    /// Sets global patch BPM without changing any module parameter.
    pub fn set_bpm(&mut self, bpm: f32) -> Result<(), CoreError> {
        if !(bpm > 0.0 && bpm.is_finite()) {
            return Err(CoreError::Patch(oxurack_core::PatchError::InvalidBpm(bpm)));
        }
        self.loaded_identity
            .as_mut()
            .ok_or(CoreError::NoPatchLoaded)?
            .set_bpm(bpm);
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
            outcomes[0].status,
            CommandStatus::CancelledByReset
        ));
        assert!(rack.process_commands().is_empty());
    }

    #[test]
    fn failed_reset_preserves_pending_command() {
        let mut rack = loaded();
        rack.enqueue_command(CoreCommand::SetBpm(133.0)).unwrap();
        let sink = rack
            .app
            .world()
            .resource::<oxurack_core::PatchHandle>()
            .modules["midi_main"];
        rack.app.world_mut().despawn(sink);
        assert!(rack.reset().is_err());
        let outcomes = rack.process_commands();
        assert!(matches!(
            outcomes[0].status,
            CommandStatus::Completed(Ok(()))
        ));
        assert_eq!(rack.bpm().unwrap(), 133.0);
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
