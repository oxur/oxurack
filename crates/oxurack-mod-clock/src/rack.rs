//! Optional oxurack ECS adapter for the standalone clock.

use std::collections::HashMap;

use bevy_app::App;
use bevy_ecs::prelude::{Component, Entity, Query, World};
use bevy_ecs::schedule::{InternedScheduleLabel, IntoScheduleConfigs};
use oxurack_core::{
    CoreError, CurrentValue, MergePolicy, Module, OxurackModule, ParameterSchema, ParameterValue,
    PortDirection, PortSchema, TickPhase, Value, ValueKind, spawn_module_entity,
    spawn_port_on_module,
};

use crate::{Clock, ClockConfig, ClockError};

const DEFAULT_BPM: f32 = 120.0;
const DEFAULT_PPQN: i64 = 1;
const DEFAULT_PULSE_WIDTH_MS: i64 = 10;

/// The oxurack module adapter for [`Clock`].
pub struct ClockModule;

#[derive(Component, Debug)]
struct ClockState {
    clock: Clock,
    pulse: Entity,
}

#[derive(Debug, Clone, Copy)]
struct RawConfig {
    bpm: f32,
    ppqn: i64,
    pulse_width_ms: i64,
}

impl Default for RawConfig {
    fn default() -> Self {
        Self {
            bpm: DEFAULT_BPM,
            ppqn: DEFAULT_PPQN,
            pulse_width_ms: DEFAULT_PULSE_WIDTH_MS,
        }
    }
}

impl RawConfig {
    fn into_validated(self) -> Result<ClockConfig, ClockError> {
        let ppqn = u16::try_from(self.ppqn).map_err(|_| ClockError::InvalidPpqn)?;
        let pulse_width_ms =
            u16::try_from(self.pulse_width_ms).map_err(|_| ClockError::InvalidPulseWidth)?;
        ClockConfig::try_new(self.bpm, ppqn, pulse_width_ms)
    }
}

impl OxurackModule for ClockModule {
    const KIND: &'static str = "clock";
    const DISPLAY_NAME: &'static str = "Clock";
    const DESCRIPTION: &'static str = "A deterministic sample-driven musical clock.";

    fn port_schema() -> &'static [PortSchema] {
        static PORTS: [PortSchema; 1] = [PortSchema {
            name: "pulse",
            direction: PortDirection::Output,
            value_kind: ValueKind::Gate,
            merge_policy: MergePolicy::Reject,
            description: "Clock pulse gate.",
        }];
        &PORTS
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
        static PARAMETERS: [ParameterSchema; 3] = [
            ParameterSchema {
                name: "bpm",
                description: "Tempo in beats per minute.",
                default: ParameterValue::Float(DEFAULT_BPM),
            },
            ParameterSchema {
                name: "ppqn",
                description: "Pulses per quarter note.",
                default: ParameterValue::Int(DEFAULT_PPQN),
            },
            ParameterSchema {
                name: "pulse_width_ms",
                description: "Pulse width in milliseconds.",
                default: ParameterValue::Int(DEFAULT_PULSE_WIDTH_MS),
            },
        ];
        &PARAMETERS
    }

    fn spawn(
        world: &mut World,
        instance_name: &str,
        parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        let raw = decode_spawn_parameters(instance_name, parameters)?;
        let config = raw.into_validated().map_err(|error| {
            invalid_parameter(instance_name, "pulse_width_ms", error.to_string())
        })?;

        let module = spawn_module_entity(world, Self::KIND, instance_name);
        let pulse = spawn_port_on_module(
            world,
            module,
            "pulse",
            PortDirection::Output,
            ValueKind::Gate,
            MergePolicy::Reject,
        );
        world.entity_mut(module).insert(ClockState {
            clock: Clock::new(config),
            pulse,
        });
        Ok(module)
    }

    fn register_systems(app: &mut App, schedule: InternedScheduleLabel) {
        app.add_systems(schedule, produce_clock_pulses.in_set(TickPhase::Produce));
    }

    fn parameter_setters() -> &'static [(&'static str, oxurack_core::ParameterSetter)] {
        static SETTERS: [(&str, oxurack_core::ParameterSetter); 3] = [
            ("bpm", set_bpm),
            ("ppqn", set_ppqn),
            ("pulse_width_ms", set_pulse_width_ms),
        ];
        &SETTERS
    }

    fn parameter_getters() -> &'static [(&'static str, oxurack_core::ParameterGetter)] {
        static GETTERS: [(&str, oxurack_core::ParameterGetter); 3] = [
            ("bpm", get_bpm),
            ("ppqn", get_ppqn),
            ("pulse_width_ms", get_pulse_width_ms),
        ];
        &GETTERS
    }
}

fn produce_clock_pulses(mut clocks: Query<&mut ClockState>, mut values: Query<&mut CurrentValue>) {
    for mut state in &mut clocks {
        let pulse = state.clock.next_sample();
        values
            .get_mut(state.pulse)
            .expect("ClockState pulse must be a CurrentValue")
            .0 = Value::Gate(pulse);
    }
}

fn decode_spawn_parameters(
    instance_name: &str,
    parameters: &HashMap<String, ParameterValue>,
) -> Result<RawConfig, CoreError> {
    let mut raw = RawConfig::default();
    let mut keys: Vec<&String> = parameters.keys().collect();
    keys.sort_unstable();

    for key in keys {
        let value = parameters
            .get(key)
            .expect("parameter key collected from the same map");
        match key.as_str() {
            "bpm" => raw.bpm = decode_float(instance_name, key, value)?,
            "ppqn" => raw.ppqn = decode_bounded_int(instance_name, key, value, 1, 96)?,
            "pulse_width_ms" => {
                raw.pulse_width_ms = decode_bounded_int(instance_name, key, value, 1, 1000)?
            }
            _ => {
                return Err(CoreError::UnknownParameter {
                    module: instance_name.to_owned(),
                    param: key.clone(),
                });
            }
        }
    }

    Ok(raw)
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

fn decode_bounded_int(
    instance_name: &str,
    parameter: &str,
    value: &ParameterValue,
    minimum: i64,
    maximum: i64,
) -> Result<i64, CoreError> {
    match value {
        ParameterValue::Int(value) if (*value >= minimum) && (*value <= maximum) => Ok(*value),
        ParameterValue::Int(_) => Err(invalid_parameter(
            instance_name,
            parameter,
            format!("must be an integer in {minimum}..={maximum}"),
        )),
        _ => Err(invalid_parameter(instance_name, parameter, "expected Int")),
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

fn current_config(world: &World, entity: Entity) -> Result<ClockConfig, CoreError> {
    world
        .get::<ClockState>(entity)
        .map(|state| state.clock.config())
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
        bpm: current.bpm(),
        ppqn: i64::from(current.ppqn()),
        pulse_width_ms: i64::from(current.pulse_width_ms()),
    };

    match parameter {
        "bpm" => raw.bpm = decode_float(&instance_name, parameter, &value)?,
        "ppqn" => raw.ppqn = decode_bounded_int(&instance_name, parameter, &value, 1, 96)?,
        "pulse_width_ms" => {
            raw.pulse_width_ms = decode_bounded_int(&instance_name, parameter, &value, 1, 1000)?
        }
        _ => {
            return Err(CoreError::UnknownParameter {
                module: instance_name,
                param: parameter.to_owned(),
            });
        }
    }

    let candidate = raw
        .into_validated()
        .map_err(|error| invalid_parameter(&instance_name, parameter, error.to_string()))?;
    world
        .get_mut::<ClockState>(entity)
        .ok_or_else(|| CoreError::UnknownModule(format!("{entity:?}")))?
        .clock
        .set_config(candidate);
    Ok(())
}

fn set_bpm(world: &mut World, entity: Entity, value: ParameterValue) -> Result<(), CoreError> {
    set_parameter(world, entity, "bpm", value)
}

fn set_ppqn(world: &mut World, entity: Entity, value: ParameterValue) -> Result<(), CoreError> {
    set_parameter(world, entity, "ppqn", value)
}

fn set_pulse_width_ms(
    world: &mut World,
    entity: Entity,
    value: ParameterValue,
) -> Result<(), CoreError> {
    set_parameter(world, entity, "pulse_width_ms", value)
}

fn get_bpm(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Float(current_config(world, entity)?.bpm()))
}

fn get_ppqn(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Int(i64::from(
        current_config(world, entity)?.ppqn(),
    )))
}

fn get_pulse_width_ms(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Int(i64::from(
        current_config(world, entity)?.pulse_width_ms(),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_app::{App, Update};
    use bevy_ecs::prelude::QueryState;
    use bevy_ecs::schedule::ScheduleLabel;
    use oxurack_core::{CorePlugin, Module};

    fn module_parameters(values: &[(&str, ParameterValue)]) -> HashMap<String, ParameterValue> {
        values
            .iter()
            .map(|(name, value)| ((*name).to_owned(), value.clone()))
            .collect()
    }

    #[test]
    fn aggregate_spawn_validation_happens_before_entity_creation() {
        let mut world = World::new();
        let parameters = module_parameters(&[
            ("bpm", ParameterValue::Float(1000.0)),
            ("ppqn", ParameterValue::Int(30)),
            ("pulse_width_ms", ParameterValue::Int(2)),
        ]);
        let result = ClockModule::spawn(&mut world, "clock", &parameters);
        assert!(matches!(
            result,
            Err(CoreError::InvalidParameterValue {
                module,
                param,
                ..
            }) if module == "clock" && param == "pulse_width_ms"
        ));

        let mut modules = QueryState::<&Module>::new(&mut world);
        assert_eq!(modules.iter(&world).count(), 0);

        let valid = module_parameters(&[
            ("bpm", ParameterValue::Float(1000.0)),
            ("ppqn", ParameterValue::Int(30)),
            ("pulse_width_ms", ParameterValue::Int(1)),
        ]);
        let entity = ClockModule::spawn(&mut world, "valid", &valid).unwrap();
        assert_eq!(
            get_bpm(&world, entity).unwrap(),
            ParameterValue::Float(1000.0)
        );
        assert_eq!(get_ppqn(&world, entity).unwrap(), ParameterValue::Int(30));
        assert_eq!(
            get_pulse_width_ms(&world, entity).unwrap(),
            ParameterValue::Int(1)
        );
    }

    #[test]
    fn unknown_spawn_parameters_are_reported_in_lexical_order() {
        let mut world = World::new();
        let parameters = module_parameters(&[
            ("zeta", ParameterValue::Int(1)),
            ("alpha", ParameterValue::Int(1)),
        ]);
        let result = ClockModule::spawn(&mut world, "clock", &parameters);
        assert!(matches!(
            result,
            Err(CoreError::UnknownParameter { module, param })
                if module == "clock" && param == "alpha"
        ));
    }

    #[test]
    fn setters_validate_before_commit_and_getters_read_clock_state() {
        let mut world = World::new();
        let entity = ClockModule::spawn(&mut world, "clock", &HashMap::new()).unwrap();
        assert_eq!(
            get_bpm(&world, entity).unwrap(),
            ParameterValue::Float(120.0)
        );

        let error = set_bpm(&mut world, entity, ParameterValue::Float(f32::NAN)).unwrap_err();
        assert!(matches!(
            error,
            CoreError::InvalidParameterValue { module, param, .. }
                if module == "clock" && param == "bpm"
        ));
        assert_eq!(
            get_bpm(&world, entity).unwrap(),
            ParameterValue::Float(120.0)
        );

        set_bpm(&mut world, entity, ParameterValue::Float(240.0)).unwrap();
        assert_eq!(
            get_bpm(&world, entity).unwrap(),
            ParameterValue::Float(240.0)
        );
        assert_eq!(get_ppqn(&world, entity).unwrap(), ParameterValue::Int(1));
        assert_eq!(
            get_pulse_width_ms(&world, entity).unwrap(),
            ParameterValue::Int(10)
        );
    }

    #[test]
    fn adapter_uses_the_supplied_update_schedule() {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        let entity = ClockModule::spawn(app.world_mut(), "clock", &HashMap::new()).unwrap();
        ClockModule::register_systems(&mut app, Update.intern());
        app.update();

        let pulse = app.world().get::<ClockState>(entity).unwrap().pulse;
        assert_eq!(
            app.world().get::<CurrentValue>(pulse).unwrap().0,
            Value::Gate(true)
        );
    }
}
