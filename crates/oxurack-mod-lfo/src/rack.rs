//! Optional oxurack ECS adapter for the standalone LFO.

use std::collections::HashMap;

use bevy_app::App;
use bevy_ecs::prelude::{Component, Entity, Query, World};
use bevy_ecs::schedule::{InternedScheduleLabel, IntoScheduleConfigs};
use oxurack_core::{
    CoreError, CurrentValue, MergePolicy, Module, OxurackModule, ParameterSchema, ParameterValue,
    PortDirection, PortSchema, TickPhase, Value, ValueKind, spawn_module_entity,
    spawn_port_on_module,
};

use crate::{Lfo, LfoConfig, LfoError, Waveform};

const DEFAULT_WAVEFORM: &str = "sine";
const DEFAULT_FREQUENCY_HZ: f32 = 1.0;
const DEFAULT_PHASE_OFFSET: f32 = 0.0;
const DEFAULT_PULSE_WIDTH: f32 = 0.5;

/// The oxurack module adapter for [`Lfo`].
pub struct LfoModule;

#[derive(Component, Debug)]
struct LfoState {
    lfo: Lfo,
    frequency_cv: f32,
    sync: bool,
    out: Entity,
    bipolar_out: Entity,
    frequency_in: Entity,
    sync_in: Entity,
}

#[derive(Debug, Clone, Copy)]
struct RawConfig {
    waveform: Waveform,
    frequency_hz: f32,
    phase_offset: f32,
    pulse_width: f32,
}

impl Default for RawConfig {
    fn default() -> Self {
        Self {
            waveform: Waveform::Sine,
            frequency_hz: DEFAULT_FREQUENCY_HZ,
            phase_offset: DEFAULT_PHASE_OFFSET,
            pulse_width: DEFAULT_PULSE_WIDTH,
        }
    }
}

impl RawConfig {
    fn into_validated(self) -> Result<LfoConfig, LfoError> {
        LfoConfig::try_new(
            self.waveform,
            self.frequency_hz,
            self.phase_offset,
            self.pulse_width,
        )
    }
}

impl OxurackModule for LfoModule {
    const KIND: &'static str = "lfo";
    const DISPLAY_NAME: &'static str = "LFO";
    const DESCRIPTION: &'static str = "A deterministic sample-driven low-frequency oscillator.";

    fn port_schema() -> &'static [PortSchema] {
        static PORTS: [PortSchema; 4] = [
            PortSchema {
                name: "out",
                direction: PortDirection::Output,
                value_kind: ValueKind::Float,
                merge_policy: MergePolicy::Reject,
                description: "Unipolar oscillator output in the range 0 through 1.",
            },
            PortSchema {
                name: "bipolar_out",
                direction: PortDirection::Output,
                value_kind: ValueKind::Bipolar,
                merge_policy: MergePolicy::Reject,
                description: "Bipolar oscillator output in the range -1 through 1.",
            },
            PortSchema {
                name: "freq_in",
                direction: PortDirection::Input,
                value_kind: ValueKind::Float,
                merge_policy: MergePolicy::Reject,
                description: "Held frequency control voltage in the range 0 through 1.",
            },
            PortSchema {
                name: "sync_in",
                direction: PortDirection::Input,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Held sync gate; its rising edge resets phase on the next sample.",
            },
        ];
        &PORTS
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
        static PARAMETERS: std::sync::OnceLock<Vec<ParameterSchema>> = std::sync::OnceLock::new();
        PARAMETERS.get_or_init(|| {
            vec![
                ParameterSchema {
                    name: "waveform",
                    description: "Waveform name: sine, tri, saw_up, saw_down, or square.",
                    default: ParameterValue::String(DEFAULT_WAVEFORM.to_owned()),
                },
                ParameterSchema {
                    name: "frequency_hz",
                    description: "Base frequency in hertz, from 0 through 100.",
                    default: ParameterValue::Float(DEFAULT_FREQUENCY_HZ),
                },
                ParameterSchema {
                    name: "phase_offset",
                    description: "Phase offset in cycles, from 0 inclusive to 1 exclusive.",
                    default: ParameterValue::Float(DEFAULT_PHASE_OFFSET),
                },
                ParameterSchema {
                    name: "pulse_width",
                    description: "Square-wave positive width in cycles, greater than 0 and less than 1.",
                    default: ParameterValue::Float(DEFAULT_PULSE_WIDTH),
                },
            ]
        })
    }

    fn spawn(
        world: &mut World,
        instance_name: &str,
        parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        let raw = decode_spawn_parameters(instance_name, parameters)?;
        let config = raw.into_validated().map_err(|error| {
            invalid_parameter(instance_name, error_parameter(&error), error.to_string())
        })?;

        let module = spawn_module_entity(world, Self::KIND, instance_name);
        let out = spawn_port_on_module(
            world,
            module,
            "out",
            PortDirection::Output,
            ValueKind::Float,
            MergePolicy::Reject,
        );
        let bipolar_out = spawn_port_on_module(
            world,
            module,
            "bipolar_out",
            PortDirection::Output,
            ValueKind::Bipolar,
            MergePolicy::Reject,
        );
        let frequency_in = spawn_port_on_module(
            world,
            module,
            "freq_in",
            PortDirection::Input,
            ValueKind::Float,
            MergePolicy::Reject,
        );
        let sync_in = spawn_port_on_module(
            world,
            module,
            "sync_in",
            PortDirection::Input,
            ValueKind::Gate,
            MergePolicy::Reject,
        );
        world.entity_mut(module).insert(LfoState {
            lfo: Lfo::new(config),
            frequency_cv: 0.0,
            sync: false,
            out,
            bipolar_out,
            frequency_in,
            sync_in,
        });
        Ok(module)
    }

    fn register_systems(app: &mut App, schedule: InternedScheduleLabel) {
        app.add_systems(
            schedule,
            (
                produce_lfo_outputs.in_set(TickPhase::Produce),
                consume_lfo_inputs.in_set(TickPhase::Consume),
            ),
        );
    }

    fn parameter_setters() -> &'static [(&'static str, oxurack_core::ParameterSetter)] {
        static SETTERS: [(&str, oxurack_core::ParameterSetter); 4] = [
            ("waveform", set_waveform),
            ("frequency_hz", set_frequency_hz),
            ("phase_offset", set_phase_offset),
            ("pulse_width", set_pulse_width),
        ];
        &SETTERS
    }

    fn parameter_getters() -> &'static [(&'static str, oxurack_core::ParameterGetter)] {
        static GETTERS: [(&str, oxurack_core::ParameterGetter); 4] = [
            ("waveform", get_waveform),
            ("frequency_hz", get_frequency_hz),
            ("phase_offset", get_phase_offset),
            ("pulse_width", get_pulse_width),
        ];
        &GETTERS
    }
}

fn produce_lfo_outputs(mut states: Query<&mut LfoState>, mut values: Query<&mut CurrentValue>) {
    for mut state in &mut states {
        let frequency_cv = state.frequency_cv;
        let sync = state.sync;
        let sample = state.lfo.next_sample(frequency_cv, sync);
        set_value(&mut values, state.out, Value::Float(sample.out));
        set_value(
            &mut values,
            state.bipolar_out,
            Value::Bipolar(sample.bipolar_out),
        );
    }
}

fn consume_lfo_inputs(states: Query<&mut LfoState>, values: Query<&CurrentValue>) {
    for mut state in states {
        state.frequency_cv = float_value(&values, state.frequency_in);
        state.sync = gate_value(&values, state.sync_in);
    }
}

fn set_value(values: &mut Query<&mut CurrentValue>, entity: Entity, value: Value) {
    values
        .get_mut(entity)
        .expect("LfoState port must have a CurrentValue")
        .0 = value;
}

fn float_value(values: &Query<&CurrentValue>, entity: Entity) -> f32 {
    match values
        .get(entity)
        .expect("LfoState input must have a CurrentValue")
        .0
    {
        Value::Float(value) => value,
        _ => 0.0,
    }
}

fn gate_value(values: &Query<&CurrentValue>, entity: Entity) -> bool {
    match values
        .get(entity)
        .expect("LfoState input must have a CurrentValue")
        .0
    {
        Value::Gate(value) => value,
        _ => false,
    }
}

fn decode_spawn_parameters(
    instance_name: &str,
    parameters: &HashMap<String, ParameterValue>,
) -> Result<RawConfig, CoreError> {
    if let Some(key) = parameters
        .keys()
        .filter(|key| !is_known_parameter(key))
        .min()
    {
        return Err(CoreError::UnknownParameter {
            module: instance_name.to_owned(),
            param: key.clone(),
        });
    }

    let mut raw = RawConfig::default();
    for parameter in ["waveform", "frequency_hz", "phase_offset", "pulse_width"] {
        let Some(value) = parameters.get(parameter) else {
            continue;
        };
        match parameter {
            "waveform" => raw.waveform = decode_waveform(instance_name, parameter, value)?,
            "frequency_hz" => raw.frequency_hz = decode_float(instance_name, parameter, value)?,
            "phase_offset" => raw.phase_offset = decode_float(instance_name, parameter, value)?,
            "pulse_width" => raw.pulse_width = decode_float(instance_name, parameter, value)?,
            _ => unreachable!("the fixed spawn parameter list contains only known names"),
        }
    }
    Ok(raw)
}

fn is_known_parameter(parameter: &str) -> bool {
    matches!(
        parameter,
        "waveform" | "frequency_hz" | "phase_offset" | "pulse_width"
    )
}

fn decode_waveform(
    instance_name: &str,
    parameter: &str,
    value: &ParameterValue,
) -> Result<Waveform, CoreError> {
    match value {
        ParameterValue::String(value) => value.parse().map_err(|error: LfoError| {
            invalid_parameter(instance_name, parameter, error.to_string())
        }),
        _ => Err(invalid_parameter(
            instance_name,
            parameter,
            "expected String",
        )),
    }
}

fn decode_float(
    instance_name: &str,
    parameter: &str,
    value: &ParameterValue,
) -> Result<f32, CoreError> {
    match value {
        ParameterValue::Float(value) => Ok(*value),
        _ => Err(invalid_parameter(
            instance_name,
            parameter,
            "expected Float",
        )),
    }
}

fn error_parameter(error: &LfoError) -> &'static str {
    match error {
        LfoError::NonFiniteFrequency | LfoError::FrequencyOutOfRange => "frequency_hz",
        LfoError::NonFinitePhaseOffset | LfoError::PhaseOffsetOutOfRange => "phase_offset",
        LfoError::NonFinitePulseWidth | LfoError::PulseWidthOutOfRange => "pulse_width",
        LfoError::UnknownWaveform(_) => "waveform",
    }
}

fn invalid_parameter(instance_name: &str, parameter: &str, reason: impl Into<String>) -> CoreError {
    CoreError::InvalidParameterValue {
        module: instance_name.to_owned(),
        param: parameter.to_owned(),
        reason: reason.into(),
    }
}

fn module_instance_name(world: &World, entity: Entity) -> Result<String, CoreError> {
    world
        .get::<Module>(entity)
        .map(|module| module.instance_name.clone())
        .ok_or_else(|| CoreError::UnknownModule(format!("{entity:?}")))
}

fn current_config(world: &World, entity: Entity) -> Result<LfoConfig, CoreError> {
    world
        .get::<LfoState>(entity)
        .map(|state| state.lfo.config())
        .ok_or_else(|| CoreError::UnknownModule(format!("{entity:?}")))
}

fn set_parameter(
    world: &mut World,
    entity: Entity,
    parameter: &str,
    value: ParameterValue,
) -> Result<(), CoreError> {
    let instance_name = module_instance_name(world, entity)?;
    let current = current_config(world, entity)?;
    let mut raw = RawConfig {
        waveform: current.waveform(),
        frequency_hz: current.frequency_hz(),
        phase_offset: current.phase_offset(),
        pulse_width: current.pulse_width(),
    };

    match parameter {
        "waveform" => raw.waveform = decode_waveform(&instance_name, parameter, &value)?,
        "frequency_hz" => raw.frequency_hz = decode_float(&instance_name, parameter, &value)?,
        "phase_offset" => raw.phase_offset = decode_float(&instance_name, parameter, &value)?,
        "pulse_width" => raw.pulse_width = decode_float(&instance_name, parameter, &value)?,
        _ => {
            return Err(CoreError::UnknownParameter {
                module: instance_name,
                param: parameter.to_owned(),
            });
        }
    }

    let candidate = raw.into_validated().map_err(|error| {
        invalid_parameter(&instance_name, error_parameter(&error), error.to_string())
    })?;
    world
        .get_mut::<LfoState>(entity)
        .ok_or_else(|| CoreError::UnknownModule(format!("{entity:?}")))?
        .lfo
        .set_config(candidate);
    Ok(())
}

fn set_waveform(world: &mut World, entity: Entity, value: ParameterValue) -> Result<(), CoreError> {
    set_parameter(world, entity, "waveform", value)
}

fn set_frequency_hz(
    world: &mut World,
    entity: Entity,
    value: ParameterValue,
) -> Result<(), CoreError> {
    set_parameter(world, entity, "frequency_hz", value)
}

fn set_phase_offset(
    world: &mut World,
    entity: Entity,
    value: ParameterValue,
) -> Result<(), CoreError> {
    set_parameter(world, entity, "phase_offset", value)
}

fn set_pulse_width(
    world: &mut World,
    entity: Entity,
    value: ParameterValue,
) -> Result<(), CoreError> {
    set_parameter(world, entity, "pulse_width", value)
}

fn get_waveform(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::String(
        current_config(world, entity)?.waveform().to_string(),
    ))
}

fn get_frequency_hz(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Float(
        current_config(world, entity)?.frequency_hz(),
    ))
}

fn get_phase_offset(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Float(
        current_config(world, entity)?.phase_offset(),
    ))
}

fn get_pulse_width(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Float(
        current_config(world, entity)?.pulse_width(),
    ))
}
