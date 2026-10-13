//! Installed, bounded MIDI output for sample-driven oxurack patches.

use std::collections::{HashMap, HashSet};

use bevy_app::App;
use bevy_ecs::prelude::{Component, Entity, Query, Res, ResMut, Resource, World};
use bevy_ecs::schedule::{InternedScheduleLabel, IntoScheduleConfigs};
use oxurack_core::{
    CoreError, CurrentValue, MergePolicy, Module, OxurackModule, ParameterSchema, ParameterValue,
    PortDirection, PortSchema, SampleStart, SampleTime, SimulationRunning, TickError, TickPhase,
    Value, ValueKind, spawn_module_entity, spawn_port_on_module,
};
use oxurack_midi::MidiWire;

const PORTS: [PortSchema; 7] = [
    PortSchema {
        name: "step",
        direction: PortDirection::Input,
        value_kind: ValueKind::Gate,
        merge_policy: MergePolicy::Reject,
        description: "One MIDI event opportunity for this sample.",
    },
    PortSchema {
        name: "note_valid",
        direction: PortDirection::Input,
        value_kind: ValueKind::Gate,
        merge_policy: MergePolicy::Reject,
        description: "Whether the note input is present.",
    },
    PortSchema {
        name: "velocity_valid",
        direction: PortDirection::Input,
        value_kind: ValueKind::Gate,
        merge_policy: MergePolicy::Reject,
        description: "Whether the velocity input is present.",
    },
    PortSchema {
        name: "gate",
        direction: PortDirection::Input,
        value_kind: ValueKind::Gate,
        merge_policy: MergePolicy::Reject,
        description: "Whether a valid note should sound.",
    },
    PortSchema {
        name: "note",
        direction: PortDirection::Input,
        value_kind: ValueKind::Raw,
        merge_policy: MergePolicy::Reject,
        description: "MIDI note number, 0 through 127.",
    },
    PortSchema {
        name: "velocity",
        direction: PortDirection::Input,
        value_kind: ValueKind::Raw,
        merge_policy: MergePolicy::Reject,
        description: "MIDI velocity, 0 through 127.",
    },
    PortSchema {
        name: "noise_cc",
        direction: PortDirection::Input,
        value_kind: ValueKind::Raw,
        merge_policy: MergePolicy::Reject,
        description: "Optional noise controller value, 0 through 127.",
    },
];

const MAX_SINKS: usize = 16;
const NORMAL_EVENTS_PER_SINK: usize = 5;
const CLEANUP_EVENTS: usize = MAX_SINKS * 3;
const TOTAL_EVENTS: usize = MAX_SINKS * NORMAL_EVENTS_PER_SINK + CLEANUP_EVENTS;

const PARAMETERS: [ParameterSchema; 4] = [
    ParameterSchema {
        name: "channel",
        description: "MIDI channel, 0 through 15.",
        default: ParameterValue::Int(0),
    },
    ParameterSchema {
        name: "noise_cc",
        description: "Noise CC number, or -1 to disable.",
        default: ParameterValue::Int(-1),
    },
    ParameterSchema {
        name: "note_cc",
        description: "Note CC number, or -1 to disable.",
        default: ParameterValue::Int(-1),
    },
    ParameterSchema {
        name: "velocity_cc",
        description: "Velocity CC number, or -1 to disable.",
        default: ParameterValue::Int(-1),
    },
];

/// One ordered MIDI message produced at a completed rack sample.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiOutputEvent {
    /// Zero-based sample index at which this event was produced. Indices are
    /// local to the owning rack lifetime; carried cleanup may have an index
    /// from the retired lifetime.
    pub sample: u64,
    /// Validated MIDI channel message.
    pub message: MidiWire,
}

/// Installed module that turns published scalar ports into MIDI events.
pub struct MidiOutModule;

#[derive(Debug, Clone, Copy)]
struct Config {
    channel: u8,
    noise_cc: Option<u8>,
    note_cc: Option<u8>,
    velocity_cc: Option<u8>,
}

#[derive(Component, Debug)]
struct MidiOutState {
    config: Config,
    active_note: Option<u8>,
    dirty: bool,
    ports: [Entity; 7],
}

/// Rack resource holding a finite MIDI event buffer and its first fault.
#[derive(Resource, Debug)]
pub struct MidiOutputBuffer {
    events: Vec<MidiOutputEvent>,
    logical_capacity: usize,
    sink_count: usize,
    channels: HashSet<u8>,
    fault: Option<TickError>,
}

impl Default for MidiOutputBuffer {
    /// Creates the buffer with its bounded ordinary and cleanup storage.
    ///
    /// # Panics
    ///
    /// Panics if the fixed storage reservation cannot be allocated.
    fn default() -> Self {
        Self::try_new().expect("cannot reserve fixed MIDI output event storage")
    }
}

/// Owned, fixed-size silence events and the old sink entities they retire.
#[derive(Debug)]
pub struct SilenceBatch {
    events: [Option<MidiOutputEvent>; CLEANUP_EVENTS],
    len: usize,
    sinks: [Option<Entity>; MAX_SINKS],
    sink_len: usize,
}

impl SilenceBatch {
    fn empty() -> Self {
        Self {
            events: [None; CLEANUP_EVENTS],
            len: 0,
            sinks: [None; MAX_SINKS],
            sink_len: 0,
        }
    }

    fn push(&mut self, event: MidiOutputEvent) -> Result<(), TickError> {
        if self.len == self.events.len() {
            return Err(output_capacity(
                "silence batch exceeds fixed cleanup capacity",
            ));
        }
        self.events[self.len] = Some(event);
        self.len += 1;
        Ok(())
    }

    fn remember_sink(&mut self, entity: Entity) -> Result<(), TickError> {
        if self.sink_len == self.sinks.len() {
            return Err(output_capacity("silence batch exceeds fixed sink capacity"));
        }
        self.sinks[self.sink_len] = Some(entity);
        self.sink_len += 1;
        Ok(())
    }

    /// Returns the complete ordered silence sequence.
    pub fn events(&self) -> impl Iterator<Item = MidiOutputEvent> + '_ {
        self.events[..self.len].iter().filter_map(|event| *event)
    }
}

impl MidiOutputBuffer {
    /// Creates an output buffer with 128 pre-reserved event slots.
    pub fn try_new() -> Result<Self, TickError> {
        let mut events = Vec::new();
        events.try_reserve_exact(TOTAL_EVENTS).map_err(|error| {
            output_capacity(format!(
                "cannot reserve {TOTAL_EVENTS} MIDI events: {error}"
            ))
        })?;
        Ok(Self {
            events,
            logical_capacity: 0,
            sink_count: 0,
            channels: HashSet::new(),
            fault: None,
        })
    }

    /// Returns the number of unread ordinary and cleanup events.
    #[must_use]
    pub fn pending_count(&self) -> usize {
        self.events.len()
    }

    /// Returns a borrowed drain of pending events in production order.
    ///
    /// Dropping this drain before consuming it discards the remaining events.
    pub fn drain(&mut self) -> std::vec::Drain<'_, MidiOutputEvent> {
        self.events.drain(..)
    }

    /// Returns the first terminal output fault, if one has been latched.
    #[must_use]
    pub fn fault(&self) -> Option<&TickError> {
        self.fault.as_ref()
    }

    /// Checks whether the next sample can be admitted without losing output.
    ///
    /// # Errors
    ///
    /// Returns the first latched terminal fault, or backpressure when pending
    /// events have not been drained.
    pub fn check_admission(&self) -> Result<(), TickError> {
        if let Some(fault) = &self.fault {
            return Err(fault.clone());
        }
        if !self.events.is_empty() {
            return Err(TickError::OutputBackpressure {
                pending_messages: self.events.len(),
                completed_samples: 0,
            });
        }
        Ok(())
    }

    /// Retains the first fault. Later checks never conceal its original cause.
    pub fn latch_fault(&mut self, fault: TickError) {
        if self.fault.is_none() {
            self.fault = Some(fault);
        }
    }

    fn reserve_sink(&mut self, instance_name: &str, channel: u8) -> Result<(), CoreError> {
        if self.channels.contains(&channel) {
            return Err(invalid_parameter(
                instance_name,
                "channel",
                format!("channel {channel} is already assigned to another midi_out sink"),
            ));
        }
        let count = self.sink_count.checked_add(1).ok_or_else(|| {
            CoreError::Tick(TickError::OutputCapacity {
                reason: "static sink count overflow".to_owned(),
                completed_samples: 0,
            })
        })?;
        if count > MAX_SINKS {
            return Err(CoreError::Tick(TickError::OutputCapacity {
                reason: "at most 16 MIDI output sinks are supported".to_owned(),
                completed_samples: 0,
            }));
        }
        let capacity = count.checked_mul(NORMAL_EVENTS_PER_SINK).ok_or_else(|| {
            CoreError::Tick(TickError::OutputCapacity {
                reason: "event capacity arithmetic overflow".to_owned(),
                completed_samples: 0,
            })
        })?;
        // The fixed 128-slot reservation is independent of logical capacity.
        self.sink_count = count;
        self.logical_capacity = capacity;
        self.channels.insert(channel);
        Ok(())
    }

    fn ensure_room(&self, required: usize) -> Result<(), TickError> {
        if self.events.len().saturating_add(required) > self.logical_capacity {
            return Err(TickError::OutputCapacity {
                reason: format!(
                    "event buffer capacity {} cannot admit {required} more messages",
                    self.logical_capacity
                ),
                completed_samples: 0,
            });
        }
        Ok(())
    }

    fn push(&mut self, event: MidiOutputEvent) -> Result<(), TickError> {
        self.ensure_room(1)?;
        self.events.push(event);
        Ok(())
    }

    /// Appends a complete silence batch when the fixed storage has room.
    pub fn append_silence(&mut self, batch: &SilenceBatch) -> Result<(), TickError> {
        if self.events.len().saturating_add(batch.len) > TOTAL_EVENTS {
            return Err(output_capacity(
                "MIDI cleanup batch exceeds fixed event storage",
            ));
        }
        for event in batch.events() {
            self.events.push(event);
        }
        Ok(())
    }

    fn ensure_total_room(&self, required: usize) -> Result<(), TickError> {
        if self.events.len().saturating_add(required) > TOTAL_EVENTS {
            return Err(output_capacity(
                "MIDI cleanup batch exceeds fixed event storage",
            ));
        }
        Ok(())
    }
}

/// Collects channel-ordered cleanup for every dirty sink without mutating it.
pub fn prepare_silence(world: &World) -> Result<SilenceBatch, TickError> {
    let boundary = world.resource::<SampleTime>().completed_samples();
    let buffer = world.resource::<MidiOutputBuffer>();
    let mut by_channel: [Option<(Entity, Option<u8>)>; 16] = [None; 16];
    let mut seen = [false; 16];
    let mut count = 0usize;
    if let Some(mut query) = world.try_query::<(Entity, &MidiOutState)>() {
        for (entity, state) in query.iter(world) {
            count += 1;
            let channel = state.config.channel as usize;
            if channel >= 16 || seen[channel] {
                return Err(output_capacity(
                    "MIDI sink channel accounting is inconsistent",
                ));
            }
            seen[channel] = true;
            if state.active_note.is_some() && !state.dirty {
                return Err(output_capacity("active MIDI note is not marked dirty"));
            }
            if state.dirty {
                by_channel[channel] = Some((entity, state.active_note));
            }
        }
    }
    if count != buffer.sink_count
        || count != buffer.channels.len()
        || seen
            .iter()
            .enumerate()
            .any(|(channel, present)| *present != buffer.channels.contains(&(channel as u8)))
    {
        return Err(output_capacity(
            "MIDI sink and reservation accounting disagree",
        ));
    }
    let mut batch = SilenceBatch::empty();
    for (channel, entry) in by_channel.into_iter().enumerate() {
        let Some((entity, active)) = entry else {
            continue;
        };
        if let Some(note) = active {
            batch.push(MidiOutputEvent {
                sample: boundary,
                message: MidiWire::note_off(channel as u8, note, 0),
            })?;
        }
        batch.push(MidiOutputEvent {
            sample: boundary,
            message: MidiWire::cc(channel as u8, 123, 0),
        })?;
        batch.push(MidiOutputEvent {
            sample: boundary,
            message: MidiWire::cc(channel as u8, 120, 0),
        })?;
        batch.remember_sink(entity)?;
    }
    Ok(batch)
}

/// Appends complete cleanup to a World and then clears its dirty voice state.
pub fn silence_outputs(world: &mut World) -> Result<(), TickError> {
    let batch = prepare_silence(world)?;
    world
        .resource::<MidiOutputBuffer>()
        .ensure_total_room(batch.len)?;
    {
        let mut buffer = world.resource_mut::<MidiOutputBuffer>();
        buffer.append_silence(&batch)?;
    }
    for entity in batch.sinks[..batch.sink_len].iter().flatten().copied() {
        if let Some(mut state) = world.get_mut::<MidiOutState>(entity) {
            state.active_note = None;
            state.dirty = false;
        }
    }
    Ok(())
}

fn output_capacity(reason: impl Into<String>) -> TickError {
    TickError::OutputCapacity {
        reason: reason.into(),
        completed_samples: 0,
    }
}

impl OxurackModule for MidiOutModule {
    const KIND: &'static str = "midi_out";
    const DISPLAY_NAME: &'static str = "MIDI Output";
    const DESCRIPTION: &'static str = "A bounded sample-quantized MIDI event sink.";

    fn port_schema() -> &'static [PortSchema] {
        &PORTS
    }
    fn parameter_schema() -> &'static [ParameterSchema] {
        &PARAMETERS
    }

    fn spawn(
        world: &mut World,
        instance_name: &str,
        parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        let config = decode_config(instance_name, parameters)?;
        world.init_resource::<MidiOutputBuffer>();
        world
            .resource_mut::<MidiOutputBuffer>()
            .reserve_sink(instance_name, config.channel)?;

        let entity = spawn_module_entity(world, Self::KIND, instance_name);
        let ports = PORTS.map(|schema| {
            spawn_port_on_module(
                world,
                entity,
                schema.name,
                schema.direction,
                schema.value_kind,
                schema.merge_policy,
            )
        });
        world.entity_mut(entity).insert(MidiOutState {
            config,
            active_note: None,
            dirty: false,
            ports,
        });
        Ok(entity)
    }

    fn register_systems(app: &mut App, schedule: InternedScheduleLabel) {
        app.init_resource::<MidiOutputBuffer>()
            .add_systems(schedule, check_output_admission.before(SampleStart))
            .add_systems(schedule, consume_midi_outputs.in_set(TickPhase::Consume));
    }

    fn parameter_getters() -> &'static [(&'static str, oxurack_core::ParameterGetter)] {
        static GETTERS: [(&str, oxurack_core::ParameterGetter); 4] = [
            ("channel", get_channel),
            ("noise_cc", get_noise_cc),
            ("note_cc", get_note_cc),
            ("velocity_cc", get_velocity_cc),
        ];
        &GETTERS
    }
}

fn decode_config(
    instance_name: &str,
    values: &HashMap<String, ParameterValue>,
) -> Result<Config, CoreError> {
    if let Some(key) = values
        .keys()
        .filter(|key| {
            !matches!(
                key.as_str(),
                "channel" | "noise_cc" | "note_cc" | "velocity_cc"
            )
        })
        .min()
    {
        return Err(CoreError::UnknownParameter {
            module: instance_name.to_owned(),
            param: key.clone(),
        });
    }
    let channel = int_parameter(instance_name, values, "channel", 0)?;
    if !(0..=15).contains(&channel) {
        return Err(invalid_parameter(
            instance_name,
            "channel",
            "must be in 0..=15",
        ));
    }
    let noise_cc = cc_parameter(instance_name, values, "noise_cc")?;
    let note_cc = cc_parameter(instance_name, values, "note_cc")?;
    let velocity_cc = cc_parameter(instance_name, values, "velocity_cc")?;
    let routes = [
        ("noise_cc", noise_cc),
        ("note_cc", note_cc),
        ("velocity_cc", velocity_cc),
    ];
    let mut seen = HashSet::new();
    for (parameter, cc) in routes {
        if let Some(cc) = cc
            && !seen.insert(cc)
        {
            return Err(invalid_parameter(
                instance_name,
                parameter,
                format!("enabled CC {cc} duplicates another route"),
            ));
        }
    }
    Ok(Config {
        channel: channel as u8,
        noise_cc,
        note_cc,
        velocity_cc,
    })
}

fn int_parameter(
    instance: &str,
    values: &HashMap<String, ParameterValue>,
    name: &str,
    default: i64,
) -> Result<i64, CoreError> {
    match values.get(name) {
        None => Ok(default),
        Some(ParameterValue::Int(value)) => Ok(*value),
        Some(_) => Err(invalid_parameter(instance, name, "must be an integer")),
    }
}

fn cc_parameter(
    instance: &str,
    values: &HashMap<String, ParameterValue>,
    name: &str,
) -> Result<Option<u8>, CoreError> {
    let value = int_parameter(instance, values, name, -1)?;
    match value {
        -1 => Ok(None),
        0..=119 => Ok(Some(value as u8)),
        _ => Err(invalid_parameter(
            instance,
            name,
            "must be -1 or in 0..=119",
        )),
    }
}

fn invalid_parameter(instance: &str, parameter: &str, reason: impl Into<String>) -> CoreError {
    CoreError::InvalidParameterValue {
        module: instance.to_owned(),
        param: parameter.to_owned(),
        reason: reason.into(),
    }
}

fn get_config(world: &World, entity: Entity) -> Result<Config, CoreError> {
    world
        .get::<MidiOutState>(entity)
        .map(|state| state.config)
        .ok_or_else(|| {
            CoreError::UnknownModule(format!("entity {entity:?} is not a midi_out module"))
        })
}

fn parameter_value(world: &World, entity: Entity, name: &str) -> Result<ParameterValue, CoreError> {
    let config = get_config(world, entity)?;
    let value = match name {
        "channel" => i64::from(config.channel),
        "noise_cc" => config.noise_cc.map_or(-1, i64::from),
        "note_cc" => config.note_cc.map_or(-1, i64::from),
        "velocity_cc" => config.velocity_cc.map_or(-1, i64::from),
        _ => {
            return Err(CoreError::UnknownParameter {
                module: "midi_out".into(),
                param: name.into(),
            });
        }
    };
    Ok(ParameterValue::Int(value))
}

fn get_channel(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    parameter_value(world, entity, "channel")
}
fn get_noise_cc(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    parameter_value(world, entity, "noise_cc")
}
fn get_note_cc(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    parameter_value(world, entity, "note_cc")
}
fn get_velocity_cc(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    parameter_value(world, entity, "velocity_cc")
}

fn check_output_admission(
    mut buffer: ResMut<MidiOutputBuffer>,
    mut running: ResMut<SimulationRunning>,
) {
    if !running.0 {
        return;
    }
    if let Err(error) = buffer.check_admission() {
        buffer.latch_fault(error);
        running.0 = false;
    }
}

#[derive(Clone, Copy)]
struct Inputs {
    note_valid: bool,
    gate: bool,
    note: Option<u8>,
    velocity: Option<u8>,
    noise: Option<u8>,
}

fn read_gate(
    values: &Query<&CurrentValue>,
    entity: Entity,
    module: &str,
    port: &str,
) -> Result<bool, TickError> {
    match values.get(entity).map(|value| value.0) {
        Ok(Value::Gate(value)) => Ok(value),
        Ok(other) => Err(invalid_output(
            module,
            port,
            format!("expected Gate, found {other:?}"),
        )),
        Err(_) => Err(invalid_output(module, port, "port has no current value")),
    }
}

fn read_midi_data(
    values: &Query<&CurrentValue>,
    entity: Entity,
    module: &str,
    port: &str,
) -> Result<u8, TickError> {
    match values.get(entity).map(|value| value.0) {
        Ok(Value::Raw(value @ 0..=127)) => Ok(value as u8),
        Ok(Value::Raw(value)) => Err(invalid_output(
            module,
            port,
            format!("Raw value {value} exceeds 127"),
        )),
        Ok(other) => Err(invalid_output(
            module,
            port,
            format!("expected Raw, found {other:?}"),
        )),
        Err(_) => Err(invalid_output(module, port, "port has no current value")),
    }
}

fn invalid_output(module: &str, port: &str, reason: impl Into<String>) -> TickError {
    TickError::InvalidMidiOutput {
        module: module.to_owned(),
        port: port.to_owned(),
        reason: reason.into(),
        completed_samples: 0,
    }
}

fn consume_midi_outputs(
    mut sinks: Query<(Entity, &Module, &mut MidiOutState)>,
    values: Query<&CurrentValue>,
    time: Res<SampleTime>,
    mut buffer: ResMut<MidiOutputBuffer>,
    mut running: ResMut<SimulationRunning>,
) {
    let mut order: Vec<(String, Entity)> = sinks
        .iter_mut()
        .map(|(entity, module, _)| (module.instance_name.clone(), entity))
        .collect();
    order.sort_by(|left, right| left.0.cmp(&right.0));
    let Some(sample) = time.last_index() else {
        buffer.latch_fault(TickError::OutputCapacity {
            reason: "MIDI sink ran without an active sample index".into(),
            completed_samples: 0,
        });
        running.0 = false;
        return;
    };

    for (instance_name, entity) in order {
        let Ok((_, _, mut state)) = sinks.get_mut(entity) else {
            continue;
        };
        let [
            step_port,
            note_valid_port,
            velocity_valid_port,
            gate_port,
            note_port,
            velocity_port,
            noise_port,
        ] = state.ports;
        let step = match read_gate(&values, step_port, &instance_name, "step") {
            Ok(value) => value,
            Err(error) => {
                buffer.latch_fault(error);
                running.0 = false;
                return;
            }
        };
        if !step {
            continue;
        }

        let parsed = (|| {
            let note_valid = read_gate(&values, note_valid_port, &instance_name, "note_valid")?;
            let velocity_valid = read_gate(
                &values,
                velocity_valid_port,
                &instance_name,
                "velocity_valid",
            )?;
            let gate = read_gate(&values, gate_port, &instance_name, "gate")?;
            let note = if note_valid {
                Some(read_midi_data(&values, note_port, &instance_name, "note")?)
            } else {
                None
            };
            let velocity = if velocity_valid {
                Some(read_midi_data(
                    &values,
                    velocity_port,
                    &instance_name,
                    "velocity",
                )?)
            } else {
                None
            };
            let noise = if state.config.noise_cc.is_some() {
                Some(read_midi_data(
                    &values,
                    noise_port,
                    &instance_name,
                    "noise_cc",
                )?)
            } else {
                None
            };
            if state.config.noise_cc.is_some() && noise.is_none() {
                return Err(invalid_output(
                    &instance_name,
                    "noise_cc",
                    "configured route has no value",
                ));
            }
            Ok::<Inputs, TickError>(Inputs {
                note_valid,
                gate,
                note,
                velocity,
                noise,
            })
        })();
        let input = match parsed {
            Ok(input) => input,
            Err(error) => {
                buffer.latch_fault(error);
                running.0 = false;
                return;
            }
        };
        if let Err(error) = buffer.ensure_room(5) {
            buffer.latch_fault(error);
            running.0 = false;
            return;
        }

        let mut messages = [None; 5];
        let mut count = 0;
        if let Some(old) = state.active_note.take() {
            messages[count] = Some(MidiWire::note_off(state.config.channel, old, 0));
            count += 1;
        }
        if let (true, true, Some(note), Some(velocity)) = (
            input.note_valid,
            input.gate,
            input.note,
            Some(input.velocity.unwrap_or(64)),
        ) && velocity > 0
        {
            messages[count] = Some(MidiWire::note_on(state.config.channel, note, velocity));
            count += 1;
            state.active_note = Some(note);
        }
        if let (Some(cc), Some(value)) = (state.config.noise_cc, input.noise) {
            messages[count] = Some(MidiWire::cc(state.config.channel, cc, value));
            count += 1;
        }
        if let (Some(cc), Some(note)) = (state.config.note_cc, input.note) {
            messages[count] = Some(MidiWire::cc(state.config.channel, cc, note));
            count += 1;
        }
        if let (Some(cc), Some(velocity)) = (state.config.velocity_cc, input.velocity) {
            messages[count] = Some(MidiWire::cc(state.config.channel, cc, velocity));
            count += 1;
        }
        for message in messages.into_iter().flatten().take(count) {
            if let Err(error) = buffer.push(MidiOutputEvent { sample, message }) {
                buffer.latch_fault(error);
                running.0 = false;
                return;
            }
        }
        if count != 0 {
            state.dirty = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_app::{FixedUpdate, Update};
    use bevy_ecs::schedule::ScheduleLabel;
    use oxurack_core::{CorePlugin, CorePluginFixed, ModuleRegistry, ParameterRegistry, Port};

    fn params(entries: &[(&str, ParameterValue)]) -> HashMap<String, ParameterValue> {
        entries
            .iter()
            .map(|(name, value)| ((*name).to_owned(), value.clone()))
            .collect()
    }

    fn app_with_sinks(configs: &[(&str, &[(&str, ParameterValue)])]) -> (App, Vec<Entity>) {
        let mut app = App::new();
        app.add_plugins(CorePluginFixed);
        app.world_mut()
            .resource_mut::<ModuleRegistry>()
            .register::<MidiOutModule>();
        {
            let mut registry = app.world_mut().resource_mut::<ParameterRegistry>();
            for &(name, getter) in MidiOutModule::parameter_getters() {
                registry.register_getter(
                    oxurack_core::ModuleKind::from(MidiOutModule::KIND),
                    name,
                    getter,
                );
            }
        }
        MidiOutModule::register_systems(&mut app, FixedUpdate.intern());
        let entities = configs
            .iter()
            .map(|(name, values)| {
                MidiOutModule::spawn(app.world_mut(), name, &params(values)).unwrap()
            })
            .collect();
        (app, entities)
    }

    fn set_input(world: &mut World, entity: Entity, name: &str, value: Value) {
        let state = world.get::<MidiOutState>(entity).unwrap();
        let port = state
            .ports
            .iter()
            .copied()
            .find(|port| world.get::<Port>(*port).unwrap().name.as_ref() == name)
            .unwrap();
        world.get_mut::<CurrentValue>(port).unwrap().0 = value;
    }

    fn set_step(world: &mut World, entity: Entity, values: &[(&str, Value)]) {
        for (name, value) in values {
            set_input(world, entity, name, *value);
        }
    }

    fn run(app: &mut App) {
        app.world_mut().run_schedule(FixedUpdate);
    }

    fn bytes(app: &mut App) -> Vec<[u8; 3]> {
        app.world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .drain()
            .map(|event| event.message.to_bytes())
            .collect()
    }

    #[test]
    fn silence_is_channel_ordered_fixed_capacity_and_idempotent() {
        let configs = [
            ("z_channel_one", &[("channel", ParameterValue::Int(1))][..]),
            ("a_channel_zero", &[("channel", ParameterValue::Int(0))][..]),
        ];
        let (mut app, entities) = app_with_sinks(&configs);
        assert!(app.world().resource::<MidiOutputBuffer>().events.capacity() >= TOTAL_EVENTS);
        assert_eq!(
            app.world().resource::<MidiOutputBuffer>().logical_capacity,
            10
        );
        for (entity, note) in entities.iter().copied().zip([61, 60]) {
            let mut state = app.world_mut().get_mut::<MidiOutState>(entity).unwrap();
            state.active_note = Some(note);
            state.dirty = true;
        }
        let batch = prepare_silence(app.world()).unwrap();
        assert_eq!(
            batch
                .events()
                .map(|event| event.message.to_bytes())
                .collect::<Vec<_>>(),
            vec![
                [0x80, 60, 0],
                [0xB0, 123, 0],
                [0xB0, 120, 0],
                [0x81, 61, 0],
                [0xB1, 123, 0],
                [0xB1, 120, 0],
            ]
        );
        silence_outputs(app.world_mut()).unwrap();
        let emitted = bytes(&mut app);
        assert_eq!(
            emitted,
            batch
                .events()
                .map(|event| event.message.to_bytes())
                .collect::<Vec<_>>()
        );
        assert!(entities.iter().all(|entity| {
            let state = app.world().get::<MidiOutState>(*entity).unwrap();
            state.active_note.is_none() && !state.dirty
        }));
        silence_outputs(app.world_mut()).unwrap();
        assert!(bytes(&mut app).is_empty());
        {
            let mut released = app
                .world_mut()
                .get_mut::<MidiOutState>(entities[0])
                .unwrap();
            released.active_note = None;
            released.dirty = true;
        }
        silence_outputs(app.world_mut()).unwrap();
        assert_eq!(bytes(&mut app), vec![[0xB1, 123, 0], [0xB1, 120, 0]]);
    }

    #[test]
    fn failed_silence_room_check_preserves_pending_and_dirty_state_for_retry() {
        let (mut app, entities) = app_with_sinks(&[("sink", &[])]);
        let sink = entities[0];
        {
            let mut state = app.world_mut().get_mut::<MidiOutState>(sink).unwrap();
            state.active_note = Some(60);
            state.dirty = true;
        }
        let filler = MidiOutputEvent {
            sample: 0,
            message: MidiWire::cc(0, 1, 1),
        };
        let original_pending = {
            let mut buffer = app.world_mut().resource_mut::<MidiOutputBuffer>();
            buffer.events.resize(TOTAL_EVENTS, filler);
            buffer.events.clone()
        };
        assert!(silence_outputs(app.world_mut()).is_err());
        assert_eq!(
            app.world().resource::<MidiOutputBuffer>().events.as_slice(),
            original_pending.as_slice()
        );
        let state = app.world().get::<MidiOutState>(sink).unwrap();
        assert_eq!(state.active_note, Some(60));
        assert!(state.dirty);
        app.world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .events
            .clear();
        silence_outputs(app.world_mut()).unwrap();
        assert_eq!(
            bytes(&mut app),
            vec![[0x80, 60, 0], [0xB0, 123, 0], [0xB0, 120, 0]]
        );
    }

    #[test]
    fn silence_invariant_failure_keeps_events_voice_state_and_fault_unchanged() {
        let (mut app, entities) = app_with_sinks(&[("sink", &[])]);
        let sink = entities[0];
        let pending = MidiOutputEvent {
            sample: 0,
            message: MidiWire::cc(0, 7, 9),
        };
        app.world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .push(pending)
            .unwrap();
        app.world_mut()
            .get_mut::<MidiOutState>(sink)
            .unwrap()
            .active_note = Some(60);
        let before = app.world().resource::<MidiOutputBuffer>().events.clone();
        let fault = app.world().resource::<MidiOutputBuffer>().fault().cloned();
        assert!(silence_outputs(app.world_mut()).is_err());
        assert_eq!(app.world().resource::<MidiOutputBuffer>().events, before);
        assert_eq!(
            app.world()
                .resource::<MidiOutputBuffer>()
                .fault()
                .map(|error| format!("{error:?}")),
            fault.as_ref().map(|error| format!("{error:?}"))
        );
        let state = app.world().get::<MidiOutState>(sink).unwrap();
        assert_eq!(state.active_note, Some(60));
        assert!(!state.dirty);
        app.world_mut().get_mut::<MidiOutState>(sink).unwrap().dirty = true;
        silence_outputs(app.world_mut()).unwrap();
        let emitted = bytes(&mut app);
        assert_eq!(
            emitted,
            vec![[0xB0, 7, 9], [0x80, 60, 0], [0xB0, 123, 0], [0xB0, 120, 0]]
        );
    }

    #[test]
    fn generated_cc_only_and_released_note_both_receive_cleanup() {
        let config = [
            ("channel", ParameterValue::Int(0)),
            ("noise_cc", ParameterValue::Int(74)),
        ];
        let (mut app, entities) = app_with_sinks(&[("sink", &config)]);
        let sink = entities[0];
        set_step(
            app.world_mut(),
            sink,
            &[
                ("step", Value::Gate(true)),
                ("note_valid", Value::Gate(false)),
                ("velocity_valid", Value::Gate(false)),
                ("gate", Value::Gate(false)),
                ("noise_cc", Value::Raw(23)),
            ],
        );
        run(&mut app);
        assert_eq!(bytes(&mut app), vec![[0xB0, 74, 23]]);
        let state = app.world().get::<MidiOutState>(sink).unwrap();
        assert_eq!(state.active_note, None);
        assert!(state.dirty);
        silence_outputs(app.world_mut()).unwrap();
        assert_eq!(bytes(&mut app), vec![[0xB0, 123, 0], [0xB0, 120, 0]]);

        set_step(
            app.world_mut(),
            sink,
            &[
                ("note_valid", Value::Gate(true)),
                ("velocity_valid", Value::Gate(true)),
                ("gate", Value::Gate(true)),
                ("note", Value::Raw(60)),
                ("velocity", Value::Raw(100)),
            ],
        );
        run(&mut app);
        assert_eq!(bytes(&mut app), vec![[0x90, 60, 100], [0xB0, 74, 23]]);
        set_input(app.world_mut(), sink, "gate", Value::Gate(false));
        run(&mut app);
        assert_eq!(bytes(&mut app), vec![[0x80, 60, 0], [0xB0, 74, 23]]);
        let state = app.world().get::<MidiOutState>(sink).unwrap();
        assert_eq!(state.active_note, None);
        assert!(state.dirty);
        silence_outputs(app.world_mut()).unwrap();
        assert_eq!(bytes(&mut app), vec![[0xB0, 123, 0], [0xB0, 120, 0]]);
    }

    #[test]
    fn sixteen_sinks_fit_eighty_normal_plus_forty_eight_cleanup_events() {
        let parameters = (0..16)
            .map(|channel| {
                let entries = vec![
                    ("channel", ParameterValue::Int(channel)),
                    ("noise_cc", ParameterValue::Int(74)),
                    ("note_cc", ParameterValue::Int(75)),
                    ("velocity_cc", ParameterValue::Int(76)),
                ];
                (format!("sink_{channel:02}"), entries)
            })
            .collect::<Vec<_>>();
        let configs = parameters
            .iter()
            .map(|(name, values)| (name.as_str(), values.as_slice()))
            .collect::<Vec<_>>();
        let (mut app, sinks) = app_with_sinks(&configs);
        for (index, sink) in sinks.iter().copied().enumerate() {
            set_step(
                app.world_mut(),
                sink,
                &[
                    ("step", Value::Gate(true)),
                    ("note_valid", Value::Gate(true)),
                    ("velocity_valid", Value::Gate(true)),
                    ("gate", Value::Gate(true)),
                    ("note", Value::Raw(60 + index as u16)),
                    ("velocity", Value::Raw(100)),
                    ("noise_cc", Value::Raw(23)),
                ],
            );
        }
        run(&mut app);
        let first: Vec<_> = app
            .world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .drain()
            .collect();
        let expected_first: Vec<_> = (0..16)
            .flat_map(|channel| {
                let ch = channel as u8;
                let note = 60 + channel as u8;
                vec![
                    MidiOutputEvent {
                        sample: 0,
                        message: MidiWire::note_on(ch, note, 100),
                    },
                    MidiOutputEvent {
                        sample: 0,
                        message: MidiWire::cc(ch, 74, 23),
                    },
                    MidiOutputEvent {
                        sample: 0,
                        message: MidiWire::cc(ch, 75, note),
                    },
                    MidiOutputEvent {
                        sample: 0,
                        message: MidiWire::cc(ch, 76, 100),
                    },
                ]
            })
            .collect();
        assert_eq!(first, expected_first);
        let capacity = app.world().resource::<MidiOutputBuffer>().events.capacity();
        run(&mut app);
        let expected_ordinary: Vec<_> = (0..16)
            .flat_map(|channel| {
                let ch = channel as u8;
                let note = 60 + channel as u8;
                vec![
                    MidiOutputEvent {
                        sample: 1,
                        message: MidiWire::note_off(ch, note, 0),
                    },
                    MidiOutputEvent {
                        sample: 1,
                        message: MidiWire::note_on(ch, note, 100),
                    },
                    MidiOutputEvent {
                        sample: 1,
                        message: MidiWire::cc(ch, 74, 23),
                    },
                    MidiOutputEvent {
                        sample: 1,
                        message: MidiWire::cc(ch, 75, note),
                    },
                    MidiOutputEvent {
                        sample: 1,
                        message: MidiWire::cc(ch, 76, 100),
                    },
                ]
            })
            .collect();
        assert_eq!(
            app.world().resource::<MidiOutputBuffer>().pending_count(),
            80
        );
        assert_eq!(
            app.world().resource::<MidiOutputBuffer>().events.as_slice(),
            expected_ordinary.as_slice()
        );
        silence_outputs(app.world_mut()).unwrap();
        assert_eq!(
            app.world().resource::<MidiOutputBuffer>().pending_count(),
            TOTAL_EVENTS
        );
        assert!(
            app.world()
                .resource::<MidiOutputBuffer>()
                .check_admission()
                .is_err()
        );
        let expected_cleanup: Vec<_> = (0..16)
            .flat_map(|channel| {
                let ch = channel as u8;
                let note = 60 + channel as u8;
                vec![
                    MidiOutputEvent {
                        sample: 2,
                        message: MidiWire::note_off(ch, note, 0),
                    },
                    MidiOutputEvent {
                        sample: 2,
                        message: MidiWire::cc(ch, 123, 0),
                    },
                    MidiOutputEvent {
                        sample: 2,
                        message: MidiWire::cc(ch, 120, 0),
                    },
                ]
            })
            .collect();
        let expected = [expected_ordinary, expected_cleanup].concat();
        let before_failed_append = app.world().resource::<MidiOutputBuffer>().events.clone();
        assert_eq!(before_failed_append.len(), TOTAL_EVENTS);
        assert_eq!(before_failed_append, expected);
        assert_eq!(
            app.world()
                .resource::<oxurack_core::SampleTime>()
                .completed_samples(),
            2
        );
        let storage_capacity_before_failed_append =
            app.world().resource::<MidiOutputBuffer>().events.capacity();
        assert!(app.world().resource::<MidiOutputBuffer>().fault().is_none());
        let states_before_failed_append = sinks
            .iter()
            .map(|sink| {
                let state = app.world().get::<MidiOutState>(*sink).unwrap();
                (
                    state.config.channel,
                    state.config.noise_cc,
                    state.config.note_cc,
                    state.config.velocity_cc,
                    state.active_note,
                    state.dirty,
                )
            })
            .collect::<Vec<_>>();
        assert!(
            states_before_failed_append
                .iter()
                .all(|state| state.4.is_none() && !state.5)
        );
        let fault_before_failed_append = app
            .world()
            .resource::<MidiOutputBuffer>()
            .fault()
            .map(|error| format!("{error:?}"));
        let mut one_event = SilenceBatch::empty();
        one_event
            .push(MidiOutputEvent {
                sample: 2,
                message: MidiWire::cc(0, 1, 1),
            })
            .unwrap();
        assert!(matches!(
            app.world_mut()
                .resource_mut::<MidiOutputBuffer>()
                .append_silence(&one_event),
            Err(TickError::OutputCapacity { .. })
        ));
        assert_eq!(
            app.world().resource::<MidiOutputBuffer>().events,
            before_failed_append
        );
        assert_eq!(
            app.world().resource::<MidiOutputBuffer>().events.capacity(),
            storage_capacity_before_failed_append
        );
        assert_eq!(
            app.world()
                .resource::<MidiOutputBuffer>()
                .fault()
                .map(|error| format!("{error:?}")),
            fault_before_failed_append
        );
        assert_eq!(
            sinks
                .iter()
                .map(|sink| {
                    let state = app.world().get::<MidiOutState>(*sink).unwrap();
                    (
                        state.config.channel,
                        state.config.noise_cc,
                        state.config.note_cc,
                        state.config.velocity_cc,
                        state.active_note,
                        state.dirty,
                    )
                })
                .collect::<Vec<_>>(),
            states_before_failed_append
        );
        silence_outputs(app.world_mut()).unwrap();
        assert_eq!(
            app.world().resource::<MidiOutputBuffer>().pending_count(),
            TOTAL_EVENTS
        );
        assert_eq!(
            app.world().resource::<MidiOutputBuffer>().events,
            before_failed_append
        );
        let all: Vec<_> = app
            .world_mut()
            .resource_mut::<MidiOutputBuffer>()
            .drain()
            .collect();
        assert_eq!(all.len(), expected.len());
        for (index, (actual, wanted)) in all.iter().zip(&expected).enumerate() {
            assert_eq!(actual, wanted, "event {index}");
        }
        let buffer = app.world().resource::<MidiOutputBuffer>();
        assert_eq!(buffer.pending_count(), 0);
        assert_eq!(buffer.logical_capacity, 80);
        assert_eq!(buffer.events.capacity(), capacity);
        assert!(buffer.check_admission().is_ok());
        assert!(sinks.iter().all(|sink| {
            let state = app.world().get::<MidiOutState>(*sink).unwrap();
            !state.dirty && state.active_note.is_none()
        }));
        silence_outputs(app.world_mut()).unwrap();
        assert!(bytes(&mut app).is_empty());
    }

    #[test]
    fn schema_defaults_getters_and_strict_spawn_validation() {
        let (mut app, entities) = app_with_sinks(&[("sink", &[])]);
        let schema = MidiOutModule::port_schema();
        assert_eq!(
            schema
                .iter()
                .map(|port| (
                    port.name,
                    port.direction,
                    port.value_kind,
                    port.merge_policy
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    "step",
                    PortDirection::Input,
                    ValueKind::Gate,
                    MergePolicy::Reject
                ),
                (
                    "note_valid",
                    PortDirection::Input,
                    ValueKind::Gate,
                    MergePolicy::Reject
                ),
                (
                    "velocity_valid",
                    PortDirection::Input,
                    ValueKind::Gate,
                    MergePolicy::Reject
                ),
                (
                    "gate",
                    PortDirection::Input,
                    ValueKind::Gate,
                    MergePolicy::Reject
                ),
                (
                    "note",
                    PortDirection::Input,
                    ValueKind::Raw,
                    MergePolicy::Reject
                ),
                (
                    "velocity",
                    PortDirection::Input,
                    ValueKind::Raw,
                    MergePolicy::Reject
                ),
                (
                    "noise_cc",
                    PortDirection::Input,
                    ValueKind::Raw,
                    MergePolicy::Reject
                ),
            ]
        );
        assert_eq!(MidiOutModule::parameter_schema().len(), 4);
        let registry = app.world().resource::<ParameterRegistry>();
        let kind = oxurack_core::ModuleKind::from(MidiOutModule::KIND);
        for (name, expected) in [
            ("channel", 0),
            ("noise_cc", -1),
            ("note_cc", -1),
            ("velocity_cc", -1),
        ] {
            assert_eq!(
                registry
                    .get_parameter(app.world(), entities[0], &kind, name)
                    .unwrap(),
                ParameterValue::Int(expected)
            );
        }
        let world = app.world_mut();
        let bad = [
            params(&[("channel", ParameterValue::Int(16))]),
            params(&[("channel", ParameterValue::Int(-1))]),
            params(&[("channel", ParameterValue::Int(i64::MIN))]),
            params(&[("channel", ParameterValue::Int(i64::MAX))]),
            params(&[("channel", ParameterValue::Bool(false))]),
            params(&[("noise_cc", ParameterValue::Int(-2))]),
            params(&[("note_cc", ParameterValue::Int(120))]),
            params(&[("velocity_cc", ParameterValue::Int(i64::MAX))]),
            params(&[
                ("noise_cc", ParameterValue::Int(7)),
                ("note_cc", ParameterValue::Int(7)),
            ]),
            params(&[
                ("z_unknown", ParameterValue::Bool(false)),
                ("channel", ParameterValue::String("bad".into())),
            ]),
        ];
        for values in bad {
            let before = world.entities().len();
            let error = MidiOutModule::spawn(world, "bad_sink", &values).unwrap_err();
            assert!(matches!(
                error,
                CoreError::InvalidParameterValue { .. } | CoreError::UnknownParameter { .. }
            ));
            assert_eq!(
                world.entities().len(),
                before,
                "invalid config must fail before creating entities"
            );
        }
        let second = MidiOutModule::spawn(
            world,
            "second",
            &params(&[("channel", ParameterValue::Int(15))]),
        )
        .unwrap();
        assert!(world.get::<MidiOutState>(second).is_some());
        let edge_routes = MidiOutModule::spawn(
            world,
            "edge_routes",
            &params(&[
                ("channel", ParameterValue::Int(14)),
                ("noise_cc", ParameterValue::Int(0)),
                ("note_cc", ParameterValue::Int(119)),
            ]),
        )
        .unwrap();
        assert_eq!(get_config(world, edge_routes).unwrap().noise_cc, Some(0));
        assert_eq!(get_config(world, edge_routes).unwrap().note_cc, Some(119));
        let before = world.entities().len();
        assert!(
            matches!(MidiOutModule::spawn(world, "duplicate", &params(&[("channel", ParameterValue::Int(0))])), Err(CoreError::InvalidParameterValue { param, .. }) if param == "channel")
        );
        assert_eq!(world.entities().len(), before);
    }

    #[test]
    fn transitions_retrigger_release_hold_idle_and_order_ccs() {
        let routes = [
            ("noise_cc", ParameterValue::Int(74)),
            ("note_cc", ParameterValue::Int(75)),
            ("velocity_cc", ParameterValue::Int(76)),
        ];
        let (mut app, entities) = app_with_sinks(&[("sink", &routes)]);
        let sink = entities[0];
        set_step(
            app.world_mut(),
            sink,
            &[
                ("step", Value::Gate(true)),
                ("note_valid", Value::Gate(true)),
                ("velocity_valid", Value::Gate(true)),
                ("gate", Value::Gate(true)),
                ("note", Value::Raw(60)),
                ("velocity", Value::Raw(100)),
                ("noise_cc", Value::Raw(23)),
            ],
        );
        run(&mut app);
        assert_eq!(
            bytes(&mut app),
            vec![
                [0x90, 60, 100],
                [0xB0, 74, 23],
                [0xB0, 75, 60],
                [0xB0, 76, 100]
            ]
        );

        run(&mut app); // Consecutive true is another event with held inputs.
        assert_eq!(
            bytes(&mut app),
            vec![
                [0x80, 60, 0],
                [0x90, 60, 100],
                [0xB0, 74, 23],
                [0xB0, 75, 60],
                [0xB0, 76, 100]
            ]
        );

        set_input(app.world_mut(), sink, "step", Value::Gate(false));
        set_input(app.world_mut(), sink, "note", Value::Float(f32::NAN));
        run(&mut app);
        assert!(
            bytes(&mut app).is_empty(),
            "idle ignores unused scalar values and retains note"
        );

        set_step(
            app.world_mut(),
            sink,
            &[
                ("step", Value::Gate(true)),
                ("note_valid", Value::Gate(false)),
                ("velocity_valid", Value::Gate(false)),
                ("gate", Value::Gate(false)),
            ],
        );
        run(&mut app);
        assert_eq!(bytes(&mut app), vec![[0x80, 60, 0], [0xB0, 74, 23]]);
        assert_eq!(
            app.world().get::<MidiOutState>(sink).unwrap().active_note,
            None
        );

        set_step(
            app.world_mut(),
            sink,
            &[
                ("step", Value::Gate(true)),
                ("note_valid", Value::Gate(true)),
                ("velocity_valid", Value::Gate(true)),
                ("gate", Value::Gate(true)),
                ("note", Value::Raw(0)),
                ("velocity", Value::Raw(127)),
                ("noise_cc", Value::Raw(127)),
            ],
        );
        run(&mut app);
        assert_eq!(
            bytes(&mut app),
            vec![
                [0x90, 0, 127],
                [0xB0, 74, 127],
                [0xB0, 75, 0],
                [0xB0, 76, 127]
            ]
        );

        set_step(
            app.world_mut(),
            sink,
            &[
                ("step", Value::Gate(true)),
                ("note_valid", Value::Gate(true)),
                ("velocity_valid", Value::Gate(true)),
                ("gate", Value::Gate(true)),
                ("note", Value::Raw(127)),
                ("velocity", Value::Raw(127)),
                ("noise_cc", Value::Raw(0)),
            ],
        );
        run(&mut app);
        assert_eq!(
            bytes(&mut app),
            vec![
                [0x80, 0, 0],
                [0x90, 127, 127],
                [0xB0, 74, 0],
                [0xB0, 75, 127],
                [0xB0, 76, 127]
            ]
        );

        set_step(
            app.world_mut(),
            sink,
            &[
                ("step", Value::Gate(true)),
                ("note_valid", Value::Gate(true)),
                ("velocity_valid", Value::Gate(false)),
                ("gate", Value::Gate(true)),
                ("note", Value::Raw(64)),
                ("velocity", Value::Raw(1000)),
            ],
        );
        run(&mut app);
        assert_eq!(
            bytes(&mut app),
            vec![
                [0x80, 127, 0],
                [0x90, 64, 64],
                [0xB0, 74, 0],
                [0xB0, 75, 64]
            ]
        );

        set_step(
            app.world_mut(),
            sink,
            &[
                ("step", Value::Gate(true)),
                ("note_valid", Value::Gate(true)),
                ("velocity_valid", Value::Gate(true)),
                ("gate", Value::Gate(true)),
                ("note", Value::Raw(65)),
                ("velocity", Value::Raw(0)),
            ],
        );
        run(&mut app);
        assert_eq!(
            bytes(&mut app),
            vec![[0x80, 64, 0], [0xB0, 74, 0], [0xB0, 75, 65], [0xB0, 76, 0]]
        );
        assert_eq!(
            app.world().get::<MidiOutState>(sink).unwrap().active_note,
            None
        );
    }

    #[test]
    fn guard_precedes_the_existing_latch_in_update_and_fixedupdate() {
        for fixed in [false, true] {
            let mut app = App::new();
            let schedule = if fixed {
                app.add_plugins(CorePluginFixed);
                FixedUpdate.intern()
            } else {
                app.add_plugins(CorePlugin);
                Update.intern()
            };
            app.world_mut()
                .resource_mut::<ModuleRegistry>()
                .register::<MidiOutModule>();
            MidiOutModule::register_systems(&mut app, schedule);
            let sink = MidiOutModule::spawn(app.world_mut(), "sink", &HashMap::new()).unwrap();
            set_step(
                app.world_mut(),
                sink,
                &[
                    ("step", Value::Gate(true)),
                    ("note_valid", Value::Gate(true)),
                    ("velocity_valid", Value::Gate(true)),
                    ("gate", Value::Gate(true)),
                    ("note", Value::Raw(60)),
                    ("velocity", Value::Raw(100)),
                ],
            );
            app.world_mut().run_schedule(schedule);
            assert_eq!(app.world().resource::<SampleTime>().completed_samples(), 1);
            assert_eq!(app.world().resource::<MidiOutputBuffer>().events.len(), 1);
            app.world_mut().run_schedule(schedule);
            assert_eq!(app.world().resource::<SampleTime>().completed_samples(), 1);
            assert_eq!(app.world().resource::<MidiOutputBuffer>().events.len(), 1);
            assert!(!app.world().resource::<SimulationRunning>().0);
        }
    }

    #[test]
    fn lexical_order_and_invalid_data_preserve_prior_prefix_and_failing_state() {
        let configs = [
            ("z_sink", &[][..]),
            ("a_sink", &[("channel", ParameterValue::Int(1))][..]),
        ];
        let (mut app, entities) = app_with_sinks(&configs);
        for &entity in &entities {
            set_step(
                app.world_mut(),
                entity,
                &[
                    ("step", Value::Gate(true)),
                    ("note_valid", Value::Gate(true)),
                    ("velocity_valid", Value::Gate(true)),
                    ("gate", Value::Gate(true)),
                    ("note", Value::Raw(60)),
                    ("velocity", Value::Raw(100)),
                ],
            );
        }
        run(&mut app);
        assert_eq!(
            bytes(&mut app),
            vec![[0x91, 60, 100], [0x90, 60, 100]],
            "lexical name order beats spawn order"
        );

        let (mut app, entities) = app_with_sinks(&[
            ("a_sink", &[]),
            ("b_sink", &[("channel", ParameterValue::Int(1))]),
        ]);
        for &entity in &entities {
            set_step(
                app.world_mut(),
                entity,
                &[
                    ("step", Value::Gate(true)),
                    ("note_valid", Value::Gate(true)),
                    ("velocity_valid", Value::Gate(true)),
                    ("gate", Value::Gate(true)),
                    ("note", Value::Raw(60)),
                    ("velocity", Value::Raw(100)),
                ],
            );
        }
        let second = entities[1];
        set_input(app.world_mut(), second, "note", Value::Raw(128));
        run(&mut app);
        assert_eq!(
            bytes(&mut app),
            vec![[0x90, 60, 100]],
            "earlier lexical output prefix remains"
        );
        assert_eq!(
            app.world().get::<MidiOutState>(second).unwrap().active_note,
            None
        );
        assert!(
            matches!(app.world().resource::<MidiOutputBuffer>().fault(), Some(TickError::InvalidMidiOutput { module, port, .. }) if module == "b_sink" && port == "note")
        );
        assert_eq!(app.world().resource::<SampleTime>().completed_samples(), 1);
        assert!(!app.world().resource::<SimulationRunning>().0);
    }
}
