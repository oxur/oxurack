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
    /// Zero-based sample index at which this event was produced.
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
    ports: [Entity; 7],
}

/// Rack resource holding a finite MIDI event buffer and its first fault.
#[derive(Resource, Debug, Default)]
pub struct MidiOutputBuffer {
    events: Vec<MidiOutputEvent>,
    logical_capacity: usize,
    sink_count: usize,
    channels: HashSet<u8>,
    fault: Option<TickError>,
}

impl MidiOutputBuffer {
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
        if count > 16 {
            return Err(CoreError::Tick(TickError::OutputCapacity {
                reason: "at most 16 MIDI output sinks are supported".to_owned(),
                completed_samples: 0,
            }));
        }
        let capacity = count.checked_mul(5).ok_or_else(|| {
            CoreError::Tick(TickError::OutputCapacity {
                reason: "event capacity arithmetic overflow".to_owned(),
                completed_samples: 0,
            })
        })?;
        self.events
            .try_reserve_exact(capacity.saturating_sub(self.events.len()))
            .map_err(|error| {
                CoreError::Tick(TickError::OutputCapacity {
                    reason: format!("cannot reserve {capacity} MIDI events: {error}"),
                    completed_samples: 0,
                })
            })?;
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
