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

#[derive(Component, Debug, Clone, PartialEq, Eq)]
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
        let config = validate_spawn_config(instance_name, raw)?;

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
    for parameter in ["bpm", "ppqn", "pulse_width_ms"] {
        let Some(value) = parameters.get(parameter) else {
            continue;
        };
        match parameter {
            "bpm" => raw.bpm = decode_float(instance_name, parameter, value)?,
            "ppqn" => raw.ppqn = decode_int(instance_name, parameter, value)?,
            "pulse_width_ms" => raw.pulse_width_ms = decode_int(instance_name, parameter, value)?,
            _ => unreachable!("the fixed spawn parameter list contains only known names"),
        }
    }

    Ok(raw)
}

fn is_known_parameter(parameter: &str) -> bool {
    matches!(parameter, "bpm" | "ppqn" | "pulse_width_ms")
}

fn spawn_error_parameter(error: ClockError) -> &'static str {
    match error {
        ClockError::NonFiniteBpm | ClockError::BpmOutOfRange => "bpm",
        ClockError::InvalidPpqn => "ppqn",
        ClockError::InvalidPulseWidth | ClockError::PulseOverlap => "pulse_width_ms",
    }
}

fn validate_spawn_config(instance_name: &str, raw: RawConfig) -> Result<ClockConfig, CoreError> {
    raw.into_validated().map_err(|error| {
        invalid_parameter(
            instance_name,
            spawn_error_parameter(error),
            error.to_string(),
        )
    })
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

fn decode_int(
    instance_name: &str,
    parameter: &str,
    value: &ParameterValue,
) -> Result<i64, CoreError> {
    match value {
        ParameterValue::Int(value) => Ok(*value),
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
        "ppqn" => raw.ppqn = decode_int(&instance_name, parameter, &value)?,
        "pulse_width_ms" => raw.pulse_width_ms = decode_int(&instance_name, parameter, &value)?,
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
    use oxurack_core::{CorePlugin, Module, Port};

    fn module_parameters(values: &[(&str, ParameterValue)]) -> HashMap<String, ParameterValue> {
        values
            .iter()
            .map(|(name, value)| ((*name).to_owned(), value.clone()))
            .collect()
    }

    #[test]
    fn failed_spawns_report_the_responsible_parameter_without_entities() {
        fn assert_failed(values: &[(&str, ParameterValue)], expected: &str) {
            let mut world = World::new();
            let before = entity_counts(&mut world);
            let result = ClockModule::spawn(&mut world, "clock", &module_parameters(values));
            assert!(matches!(
                result,
                Err(CoreError::InvalidParameterValue { module, param, .. })
                    if module == "clock" && param == expected
            ));
            assert_eq!(entity_counts(&mut world), before);
        }

        for bpm in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 0.0, 1001.0] {
            assert_failed(&[("bpm", ParameterValue::Float(bpm))], "bpm");
        }
        for ppqn in [0, 97] {
            assert_failed(&[("ppqn", ParameterValue::Int(ppqn))], "ppqn");
        }
        assert_failed(&[("ppqn", ParameterValue::Float(1.0))], "ppqn");
        for pulse_width_ms in [0, 1001] {
            assert_failed(
                &[("pulse_width_ms", ParameterValue::Int(pulse_width_ms))],
                "pulse_width_ms",
            );
        }
        assert_failed(
            &[("pulse_width_ms", ParameterValue::Float(1.0))],
            "pulse_width_ms",
        );
        assert_failed(
            &[
                ("bpm", ParameterValue::Float(1000.0)),
                ("ppqn", ParameterValue::Int(30)),
                ("pulse_width_ms", ParameterValue::Int(2)),
            ],
            "pulse_width_ms",
        );

        let mut world = World::new();
        let entity = ClockModule::spawn(&mut world, "valid", &HashMap::new()).unwrap();
        assert_eq!(
            get_bpm(&world, entity).unwrap(),
            ParameterValue::Float(120.0)
        );
        assert_eq!(get_ppqn(&world, entity).unwrap(), ParameterValue::Int(1));
        assert_eq!(
            get_pulse_width_ms(&world, entity).unwrap(),
            ParameterValue::Int(10)
        );
    }

    #[test]
    fn unknown_spawn_parameters_win_before_invalid_known_values() {
        for values in [
            &[
                ("zeta", ParameterValue::Float(f32::NAN)),
                ("alpha", ParameterValue::Int(1)),
                ("bpm", ParameterValue::Float(f32::NAN)),
            ][..],
            &[
                ("bpm", ParameterValue::Float(f32::NAN)),
                ("alpha", ParameterValue::Int(1)),
                ("zeta", ParameterValue::Float(f32::NAN)),
            ][..],
        ] {
            let mut world = World::new();
            let before = entity_counts(&mut world);
            let result = ClockModule::spawn(&mut world, "clock", &module_parameters(values));
            assert!(matches!(
                result,
                Err(CoreError::UnknownParameter { module, param })
                    if module == "clock" && param == "alpha"
            ));
            assert_eq!(entity_counts(&mut world), before);
        }
    }

    #[test]
    fn setters_decode_each_parameter_and_getters_read_clock_state() {
        let mut world = World::new();
        let entity = ClockModule::spawn(&mut world, "clock", &HashMap::new()).unwrap();
        assert_eq!(
            get_bpm(&world, entity).unwrap(),
            ParameterValue::Float(120.0)
        );

        set_bpm(&mut world, entity, ParameterValue::Float(240.0)).unwrap();
        set_ppqn(&mut world, entity, ParameterValue::Int(2)).unwrap();
        set_pulse_width_ms(&mut world, entity, ParameterValue::Int(5)).unwrap();
        assert_eq!(
            get_bpm(&world, entity).unwrap(),
            ParameterValue::Float(240.0)
        );
        assert_eq!(get_ppqn(&world, entity).unwrap(), ParameterValue::Int(2));
        assert_eq!(
            get_pulse_width_ms(&world, entity).unwrap(),
            ParameterValue::Int(5)
        );

        for (parameter, result) in [
            ("bpm", set_bpm(&mut world, entity, ParameterValue::Int(240))),
            (
                "ppqn",
                set_ppqn(&mut world, entity, ParameterValue::Float(2.0)),
            ),
            (
                "pulse_width_ms",
                set_pulse_width_ms(&mut world, entity, ParameterValue::Bool(true)),
            ),
        ] {
            assert!(matches!(
                result,
                Err(CoreError::InvalidParameterValue { module, param, .. })
                    if module == "clock" && param == parameter
            ));
        }
    }

    #[test]
    fn missing_and_non_clock_entities_are_unknown_modules() {
        let mut world = World::new();
        let missing = world.spawn_empty().id();
        assert!(world.despawn(missing));
        assert!(matches!(
            get_bpm(&world, missing),
            Err(CoreError::UnknownModule(_))
        ));

        let non_clock = world.spawn_empty().id();
        assert!(matches!(
            set_ppqn(&mut world, non_clock, ParameterValue::Int(2)),
            Err(CoreError::UnknownModule(_))
        ));
    }

    #[test]
    fn invalid_edits_preserve_clock_state_and_held_value() {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        let entity = ClockModule::spawn(app.world_mut(), "clock", &HashMap::new()).unwrap();
        ClockModule::register_systems(&mut app, Update.intern());
        app.update();

        let pulse = app.world().get::<ClockState>(entity).unwrap().pulse;
        let before = app.world().get::<ClockState>(entity).unwrap().clone();
        let before_value = *app.world().get::<CurrentValue>(pulse).unwrap();
        set_bpm(app.world_mut(), entity, ParameterValue::Float(240.0)).unwrap();
        let pending = app.world().get::<ClockState>(entity).unwrap().clone();
        let pending_value = *app.world().get::<CurrentValue>(pulse).unwrap();
        assert_ne!(pending, before);
        assert_eq!(pending_value, before_value);
        let error = set_ppqn(app.world_mut(), entity, ParameterValue::Float(2.0)).unwrap_err();
        assert!(matches!(
            error,
            CoreError::InvalidParameterValue { module, param, .. }
                if module == "clock" && param == "ppqn"
        ));
        assert_eq!(*app.world().get::<ClockState>(entity).unwrap(), pending);
        assert_eq!(
            *app.world().get::<CurrentValue>(pulse).unwrap(),
            pending_value
        );

        app.update();
        assert_eq!(
            app.world().get::<CurrentValue>(pulse).unwrap().0,
            Value::Gate(false)
        );
        app.update();
        assert_eq!(
            app.world().get::<CurrentValue>(pulse).unwrap().0,
            Value::Gate(true)
        );

        let before_overlap = app.world().get::<ClockState>(entity).unwrap().clone();
        let before_overlap_value = *app.world().get::<CurrentValue>(pulse).unwrap();
        let error =
            set_pulse_width_ms(app.world_mut(), entity, ParameterValue::Int(1000)).unwrap_err();
        assert!(matches!(
            error,
            CoreError::InvalidParameterValue { module, param, .. }
                if module == "clock" && param == "pulse_width_ms"
        ));
        assert_eq!(
            *app.world().get::<ClockState>(entity).unwrap(),
            before_overlap
        );
        assert_eq!(
            *app.world().get::<CurrentValue>(pulse).unwrap(),
            before_overlap_value
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

    fn entity_counts(world: &mut World) -> (usize, usize) {
        let mut modules = QueryState::<&Module>::new(world);
        let mut ports = QueryState::<&Port>::new(world);
        (modules.iter(world).count(), ports.iter(world).count())
    }
}
