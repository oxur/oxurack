//! Optional ECS adapter for the standalone Turing Machine engine.

use std::collections::HashMap;
use std::sync::OnceLock;

use bevy_app::App;
use bevy_ecs::prelude::{Component, Entity, Query, World};
use bevy_ecs::schedule::{InternedScheduleLabel, IntoScheduleConfigs};
use oxurack_core::{
    CoreError, CurrentValue, MergePolicy, Module, ModuleSpawnContext, OxurackModule,
    ParameterSchema, ParameterValue, PortDirection, PortSchema, TickPhase, Value, ValueKind,
    spawn_module_entity, spawn_port_on_module,
};

use crate::quantizer::Scale as EngineScale;
use crate::{StepOutputs, TuringMachine};

const DEFAULT_WRITE_PROBABILITY: f32 = 0.5;
const DEFAULT_LENGTH: usize = 16;
const DEFAULT_LENGTH_POSITION: usize = 8;
const DEFAULT_ROOT: u8 = 0;
const DEFAULT_NOTE_MIN: u8 = 36;
const DEFAULT_NOTE_MAX: u8 = 84;

const PARAMETER_NAMES: [&str; 9] = [
    "write_probability",
    "length",
    "length_position",
    "scale",
    "root",
    "note_min",
    "note_max",
    "scale_output_scale",
    "scale_output_root",
];

const PULSE_NAMES: [&str; 6] = [
    "pulse_1", "pulse_2", "pulse_3", "pulse_4", "pulse_5", "pulse_6",
];
const GATE_NAMES: [&str; 8] = [
    "gate_1", "gate_2", "gate_3", "gate_4", "gate_5", "gate_6", "gate_7", "gate_8",
];

/// The optional oxurack adapter for [`TuringMachine`].
pub struct TuringMachineModule;

#[derive(Debug, Clone, Copy)]
struct Ports {
    clock: Entity,
    write_modulation: Entity,
    reset: Entity,
    move_step: Entity,
    step: Entity,
    note: Entity,
    note_valid: Entity,
    velocity: Entity,
    velocity_valid: Entity,
    gate: Entity,
    scale_note: Entity,
    scale_note_valid: Entity,
    pulses: [Entity; 6],
    gates: [Entity; 8],
    div2: Entity,
    div4: Entity,
    noise_cc: Entity,
    register_bits: Entity,
    length: Entity,
    write_probability: Entity,
}

#[derive(Component, Debug)]
struct TuringMachineState {
    engine: TuringMachine,
    write_base: f32,
    write_modulation: f32,
    previous_clock: bool,
    previous_reset: bool,
    previous_move: bool,
    pending: Option<StepOutputs>,
    ports: Ports,
}

#[derive(Debug, Clone)]
struct ScaleSpec {
    intervals: Vec<u8>,
    root: u8,
    name: Option<String>,
}

#[derive(Debug, Clone)]
struct ConfigCandidate {
    write_probability: f32,
    length: usize,
    length_position: usize,
    scale: ScaleSpec,
    root: u8,
    note_min: u8,
    note_max: u8,
    note_range_parameter: Option<&'static str>,
    scale_output_scale: ScaleSpec,
    scale_output_root: u8,
    write_changed: bool,
    length_changed: bool,
    length_position_changed: bool,
    scale_changed: bool,
    root_changed: bool,
    note_range_changed: bool,
    scale_output_scale_changed: bool,
    scale_output_root_changed: bool,
}

impl ConfigCandidate {
    fn defaults() -> Self {
        Self {
            write_probability: DEFAULT_WRITE_PROBABILITY,
            length: DEFAULT_LENGTH,
            length_position: DEFAULT_LENGTH_POSITION,
            scale: chromatic_scale_spec(),
            root: DEFAULT_ROOT,
            note_min: DEFAULT_NOTE_MIN,
            note_max: DEFAULT_NOTE_MAX,
            note_range_parameter: None,
            scale_output_scale: chromatic_scale_spec(),
            scale_output_root: DEFAULT_ROOT,
            write_changed: false,
            length_changed: false,
            length_position_changed: false,
            scale_changed: false,
            root_changed: false,
            note_range_changed: false,
            scale_output_scale_changed: false,
            scale_output_root_changed: false,
        }
    }

    fn from_state(state: &TuringMachineState) -> Self {
        Self {
            write_probability: state.write_base,
            length: state.engine.current_length(),
            length_position: state.engine.length_position(),
            scale: scale_spec_from_engine(state.engine.scale(), state.engine.root()),
            root: state.engine.root(),
            note_min: *state.engine.note_range().start(),
            note_max: *state.engine.note_range().end(),
            note_range_parameter: None,
            scale_output_scale: scale_spec_from_engine(
                state.engine.scale_output_scale(),
                state.engine.scale_output_root(),
            ),
            scale_output_root: state.engine.scale_output_root(),
            write_changed: false,
            length_changed: false,
            length_position_changed: false,
            scale_changed: false,
            root_changed: false,
            note_range_changed: false,
            scale_output_scale_changed: false,
            scale_output_root_changed: false,
        }
    }
}

impl OxurackModule for TuringMachineModule {
    const KIND: &'static str = "turing_machine";
    const DISPLAY_NAME: &'static str = "Turing Machine";
    const DESCRIPTION: &'static str =
        "A seeded, sample-driven Turing Machine with scalar ECS ports.";

    fn port_schema() -> &'static [PortSchema] {
        static PORTS: [PortSchema; 32] = [
            PortSchema {
                name: "clock",
                direction: PortDirection::Input,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Master clock rising edge.",
            },
            PortSchema {
                name: "write_modulation",
                direction: PortDirection::Input,
                value_kind: ValueKind::Bipolar,
                merge_policy: MergePolicy::Sum,
                description: "Held bipolar write-probability modulation.",
            },
            PortSchema {
                name: "reset",
                direction: PortDirection::Input,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Reset rising edge.",
            },
            PortSchema {
                name: "move",
                direction: PortDirection::Input,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Move-step rising edge.",
            },
            PortSchema {
                name: "step",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "One-sample delayed step event.",
            },
            PortSchema {
                name: "note",
                direction: PortDirection::Output,
                value_kind: ValueKind::Raw,
                merge_policy: MergePolicy::Reject,
                description: "Quantized note, or zero when invalid.",
            },
            PortSchema {
                name: "note_valid",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Whether note is present.",
            },
            PortSchema {
                name: "velocity",
                direction: PortDirection::Output,
                value_kind: ValueKind::Raw,
                merge_policy: MergePolicy::Reject,
                description: "Velocity, or zero when invalid.",
            },
            PortSchema {
                name: "velocity_valid",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Whether velocity is present.",
            },
            PortSchema {
                name: "gate",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Current step gate.",
            },
            PortSchema {
                name: "scale_note",
                direction: PortDirection::Output,
                value_kind: ValueKind::Raw,
                merge_policy: MergePolicy::Reject,
                description: "Secondary quantized note, or zero when invalid.",
            },
            PortSchema {
                name: "scale_note_valid",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Whether scale_note is present.",
            },
            PortSchema {
                name: "pulse_1",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Adjacent-bit pulse 1.",
            },
            PortSchema {
                name: "pulse_2",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Adjacent-bit pulse 2.",
            },
            PortSchema {
                name: "pulse_3",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Adjacent-bit pulse 3.",
            },
            PortSchema {
                name: "pulse_4",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Adjacent-bit pulse 4.",
            },
            PortSchema {
                name: "pulse_5",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Adjacent-bit pulse 5.",
            },
            PortSchema {
                name: "pulse_6",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Adjacent-bit pulse 6.",
            },
            PortSchema {
                name: "gate_1",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Shift-register gate 1.",
            },
            PortSchema {
                name: "gate_2",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Shift-register gate 2.",
            },
            PortSchema {
                name: "gate_3",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Shift-register gate 3.",
            },
            PortSchema {
                name: "gate_4",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Shift-register gate 4.",
            },
            PortSchema {
                name: "gate_5",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Shift-register gate 5.",
            },
            PortSchema {
                name: "gate_6",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Shift-register gate 6.",
            },
            PortSchema {
                name: "gate_7",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Shift-register gate 7.",
            },
            PortSchema {
                name: "gate_8",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Shift-register gate 8.",
            },
            PortSchema {
                name: "div2",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Clock divided by two.",
            },
            PortSchema {
                name: "div4",
                direction: PortDirection::Output,
                value_kind: ValueKind::Gate,
                merge_policy: MergePolicy::Reject,
                description: "Clock divided by four.",
            },
            PortSchema {
                name: "noise_cc",
                direction: PortDirection::Output,
                value_kind: ValueKind::Raw,
                merge_policy: MergePolicy::Reject,
                description: "Noise controller value.",
            },
            PortSchema {
                name: "register_bits",
                direction: PortDirection::Output,
                value_kind: ValueKind::Raw,
                merge_policy: MergePolicy::Reject,
                description: "Raw shift-register bits.",
            },
            PortSchema {
                name: "length",
                direction: PortDirection::Output,
                value_kind: ValueKind::Raw,
                merge_policy: MergePolicy::Reject,
                description: "Active loop length.",
            },
            PortSchema {
                name: "write_probability",
                direction: PortDirection::Output,
                value_kind: ValueKind::Float,
                merge_policy: MergePolicy::Reject,
                description: "Effective write probability.",
            },
        ];
        &PORTS
    }

    fn parameter_schema() -> &'static [ParameterSchema] {
        static PARAMETERS: OnceLock<Vec<ParameterSchema>> = OnceLock::new();
        PARAMETERS
            .get_or_init(|| {
                vec![
                    ParameterSchema {
                        name: "write_probability",
                        description: "Base write probability.",
                        default: ParameterValue::Float(DEFAULT_WRITE_PROBABILITY),
                    },
                    ParameterSchema {
                        name: "length",
                        description: "Exact loop length.",
                        default: ParameterValue::Int(DEFAULT_LENGTH as i64),
                    },
                    ParameterSchema {
                        name: "length_position",
                        description: "Loop selector position.",
                        default: ParameterValue::Int(DEFAULT_LENGTH_POSITION as i64),
                    },
                    ParameterSchema {
                        name: "scale",
                        description: "Main quantizer scale.",
                        default: ParameterValue::Scale(core_chromatic()),
                    },
                    ParameterSchema {
                        name: "root",
                        description: "Main quantizer root.",
                        default: ParameterValue::Int(i64::from(DEFAULT_ROOT)),
                    },
                    ParameterSchema {
                        name: "note_min",
                        description: "Minimum main note.",
                        default: ParameterValue::Int(i64::from(DEFAULT_NOTE_MIN)),
                    },
                    ParameterSchema {
                        name: "note_max",
                        description: "Maximum main note.",
                        default: ParameterValue::Int(i64::from(DEFAULT_NOTE_MAX)),
                    },
                    ParameterSchema {
                        name: "scale_output_scale",
                        description: "Secondary quantizer scale.",
                        default: ParameterValue::Scale(core_chromatic()),
                    },
                    ParameterSchema {
                        name: "scale_output_root",
                        description: "Secondary quantizer root.",
                        default: ParameterValue::Int(i64::from(DEFAULT_ROOT)),
                    },
                ]
            })
            .as_slice()
    }

    fn spawn(
        world: &mut World,
        instance_name: &str,
        parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        let context = ModuleSpawnContext::new(0, instance_name);
        Self::spawn_with_context(world, &context, parameters)
    }

    fn spawn_with_context(
        world: &mut World,
        context: &ModuleSpawnContext<'_>,
        parameters: &HashMap<String, ParameterValue>,
    ) -> Result<Entity, CoreError> {
        let candidate = decode_spawn_parameters(context.instance_name(), parameters)?;
        let mut engine = TuringMachine::with_seed(context.instance_seed());
        apply_candidate(&mut engine, &candidate);

        let module = spawn_module_entity(world, Self::KIND, context.instance_name());
        let ports = spawn_ports(world, module);
        world.entity_mut(module).insert(TuringMachineState {
            engine,
            write_base: candidate.write_probability,
            write_modulation: 0.0,
            previous_clock: false,
            previous_reset: false,
            previous_move: false,
            pending: None,
            ports,
        });
        Ok(module)
    }

    fn register_systems(app: &mut App, schedule: InternedScheduleLabel) {
        app.add_systems(
            schedule,
            (
                produce_turing_machine_outputs.in_set(TickPhase::Produce),
                consume_turing_machine_inputs.in_set(TickPhase::Consume),
            ),
        );
    }

    fn parameter_setters() -> &'static [(&'static str, oxurack_core::ParameterSetter)] {
        static SETTERS: [(&str, oxurack_core::ParameterSetter); 9] = [
            ("write_probability", set_write_probability),
            ("length", set_length),
            ("length_position", set_length_position),
            ("scale", set_scale),
            ("root", set_root),
            ("note_min", set_note_min),
            ("note_max", set_note_max),
            ("scale_output_scale", set_scale_output_scale),
            ("scale_output_root", set_scale_output_root),
        ];
        &SETTERS
    }

    fn parameter_getters() -> &'static [(&'static str, oxurack_core::ParameterGetter)] {
        static GETTERS: [(&str, oxurack_core::ParameterGetter); 9] = [
            ("write_probability", get_write_probability),
            ("length", get_length),
            ("length_position", get_length_position),
            ("scale", get_scale),
            ("root", get_root),
            ("note_min", get_note_min),
            ("note_max", get_note_max),
            ("scale_output_scale", get_scale_output_scale),
            ("scale_output_root", get_scale_output_root),
        ];
        &GETTERS
    }
}

fn spawn_ports(world: &mut World, module: Entity) -> Ports {
    let clock = spawn_input(world, module, "clock", ValueKind::Gate, MergePolicy::Reject);
    let write_modulation = spawn_input(
        world,
        module,
        "write_modulation",
        ValueKind::Bipolar,
        MergePolicy::Sum,
    );
    let reset = spawn_input(world, module, "reset", ValueKind::Gate, MergePolicy::Reject);
    let move_step = spawn_input(world, module, "move", ValueKind::Gate, MergePolicy::Reject);
    let step = spawn_output(world, module, "step", ValueKind::Gate);
    let note = spawn_output(world, module, "note", ValueKind::Raw);
    let note_valid = spawn_output(world, module, "note_valid", ValueKind::Gate);
    let velocity = spawn_output(world, module, "velocity", ValueKind::Raw);
    let velocity_valid = spawn_output(world, module, "velocity_valid", ValueKind::Gate);
    let gate = spawn_output(world, module, "gate", ValueKind::Gate);
    let scale_note = spawn_output(world, module, "scale_note", ValueKind::Raw);
    let scale_note_valid = spawn_output(world, module, "scale_note_valid", ValueKind::Gate);
    let pulses = std::array::from_fn(|index| {
        spawn_output(world, module, PULSE_NAMES[index], ValueKind::Gate)
    });
    let gates = std::array::from_fn(|index| {
        spawn_output(world, module, GATE_NAMES[index], ValueKind::Gate)
    });
    let div2 = spawn_output(world, module, "div2", ValueKind::Gate);
    let div4 = spawn_output(world, module, "div4", ValueKind::Gate);
    let noise_cc = spawn_output(world, module, "noise_cc", ValueKind::Raw);
    let register_bits = spawn_output(world, module, "register_bits", ValueKind::Raw);
    let length = spawn_output(world, module, "length", ValueKind::Raw);
    let write_probability = spawn_output(world, module, "write_probability", ValueKind::Float);
    Ports {
        clock,
        write_modulation,
        reset,
        move_step,
        step,
        note,
        note_valid,
        velocity,
        velocity_valid,
        gate,
        scale_note,
        scale_note_valid,
        pulses,
        gates,
        div2,
        div4,
        noise_cc,
        register_bits,
        length,
        write_probability,
    }
}

fn spawn_input(
    world: &mut World,
    module: Entity,
    name: &str,
    kind: ValueKind,
    merge: MergePolicy,
) -> Entity {
    spawn_port_on_module(world, module, name, PortDirection::Input, kind, merge)
}

fn spawn_output(world: &mut World, module: Entity, name: &str, kind: ValueKind) -> Entity {
    spawn_port_on_module(
        world,
        module,
        name,
        PortDirection::Output,
        kind,
        MergePolicy::Reject,
    )
}

fn produce_turing_machine_outputs(
    mut states: Query<&mut TuringMachineState>,
    mut values: Query<&mut CurrentValue>,
) {
    for mut state in &mut states {
        let Some(outputs) = state.pending.take() else {
            set_value(&mut values, state.ports.step, Value::Gate(false));
            continue;
        };
        publish_step(&mut values, state.ports, &outputs);
    }
}

fn consume_turing_machine_inputs(
    mut states: Query<&mut TuringMachineState>,
    values: Query<&CurrentValue>,
) {
    for mut state in &mut states {
        let clock = gate_value(&values, state.ports.clock);
        let reset = gate_value(&values, state.ports.reset);
        let move_step = gate_value(&values, state.ports.move_step);
        let modulation = bipolar_value(&values, state.ports.write_modulation).clamp(-1.0, 1.0);

        let clock_edge = clock && !state.previous_clock;
        let reset_edge = reset && !state.previous_reset;
        let move_edge = move_step && !state.previous_move;
        state.previous_clock = clock;
        state.previous_reset = reset;
        state.previous_move = move_step;

        state.write_modulation = modulation;
        reconcile_write_probability(&mut state);

        if reset_edge {
            state.engine.reset();
            state.pending = None;
        } else if move_edge {
            state.pending = Some(state.engine.move_step());
        } else if clock_edge {
            state.pending = Some(state.engine.tick());
        }
    }
}

fn reconcile_write_probability(state: &mut TuringMachineState) {
    let target = (state.write_base + state.write_modulation).clamp(0.0, 1.0);
    let delta = target - state.engine.write_probability();
    state.engine.modulate_write(delta);
}

fn publish_step(values: &mut Query<&mut CurrentValue>, ports: Ports, outputs: &StepOutputs) {
    set_value(values, ports.step, Value::Gate(true));
    set_optional_value(values, ports.note, ports.note_valid, outputs.note);
    set_optional_value(
        values,
        ports.velocity,
        ports.velocity_valid,
        outputs.velocity,
    );
    set_value(values, ports.gate, Value::Gate(outputs.gate));
    set_optional_value(
        values,
        ports.scale_note,
        ports.scale_note_valid,
        outputs.scale_note,
    );
    for (port, value) in ports.pulses.into_iter().zip(outputs.pulses) {
        set_value(values, port, Value::Gate(value));
    }
    for (port, value) in ports.gates.into_iter().zip(outputs.gates) {
        set_value(values, port, Value::Gate(value));
    }
    set_value(values, ports.div2, Value::Gate(outputs.div2));
    set_value(values, ports.div4, Value::Gate(outputs.div4));
    set_value(
        values,
        ports.noise_cc,
        Value::Raw(u16::from(outputs.noise_cc)),
    );
    set_value(
        values,
        ports.register_bits,
        Value::Raw(outputs.register_bits),
    );
    let length = u16::try_from(outputs.length).expect("engine loop lengths fit in Raw");
    set_value(values, ports.length, Value::Raw(length));
    set_value(
        values,
        ports.write_probability,
        Value::Float(outputs.write_probability),
    );
}

fn set_optional_value(
    values: &mut Query<&mut CurrentValue>,
    value_port: Entity,
    valid_port: Entity,
    value: Option<u8>,
) {
    let (raw, valid) = optional_output(value);
    set_value(values, value_port, raw);
    set_value(values, valid_port, valid);
}

fn optional_output(value: Option<u8>) -> (Value, Value) {
    match value {
        Some(value) => (Value::Raw(u16::from(value)), Value::Gate(true)),
        None => (Value::Raw(0), Value::Gate(false)),
    }
}

fn set_value(values: &mut Query<&mut CurrentValue>, port: Entity, value: Value) {
    values
        .get_mut(port)
        .expect("TuringMachineState port must have CurrentValue")
        .0 = value;
}

fn gate_value(values: &Query<&CurrentValue>, port: Entity) -> bool {
    match values
        .get(port)
        .expect("TuringMachineState input must exist")
        .0
    {
        Value::Gate(value) => value,
        value => panic!("TuringMachineState gate input has unexpected value {value:?}"),
    }
}

fn bipolar_value(values: &Query<&CurrentValue>, port: Entity) -> f32 {
    match values
        .get(port)
        .expect("TuringMachineState modulation input must exist")
        .0
    {
        Value::Bipolar(value) => value,
        value => panic!("TuringMachineState modulation input has unexpected value {value:?}"),
    }
}

fn core_chromatic() -> oxurack_core::Scale {
    oxurack_core::Scale::chromatic(DEFAULT_ROOT)
}

fn chromatic_scale_spec() -> ScaleSpec {
    ScaleSpec {
        intervals: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        root: DEFAULT_ROOT,
        name: Some("Chromatic".to_owned()),
    }
}

fn scale_spec_from_engine(scale: &EngineScale, root: u8) -> ScaleSpec {
    ScaleSpec {
        intervals: scale.intervals().to_vec(),
        root,
        name: Some(scale.name().to_owned()),
    }
}

fn engine_scale(spec: &ScaleSpec) -> EngineScale {
    EngineScale::new(
        spec.intervals.clone(),
        spec.name.clone().unwrap_or_else(|| "Custom".to_owned()),
    )
}

fn decode_spawn_parameters(
    instance_name: &str,
    parameters: &HashMap<String, ParameterValue>,
) -> Result<ConfigCandidate, CoreError> {
    if let Some(key) = parameters
        .keys()
        .filter(|key| !PARAMETER_NAMES.contains(&key.as_str()))
        .min()
    {
        return Err(CoreError::UnknownParameter {
            module: instance_name.to_owned(),
            param: key.clone(),
        });
    }

    let mut candidate = ConfigCandidate::defaults();
    for parameter in PARAMETER_NAMES {
        let Some(value) = parameters.get(parameter) else {
            continue;
        };
        decode_into_candidate(instance_name, parameter, value, &mut candidate)?;
    }
    validate_candidate(instance_name, &candidate)?;
    Ok(candidate)
}

fn decode_into_candidate(
    instance_name: &str,
    parameter: &str,
    value: &ParameterValue,
    candidate: &mut ConfigCandidate,
) -> Result<(), CoreError> {
    match parameter {
        "write_probability" => {
            candidate.write_probability = decode_float(instance_name, parameter, value)?;
            candidate.write_changed = true;
        }
        "length" => {
            candidate.length = decode_length(instance_name, parameter, value)?;
            candidate.length_changed = true;
        }
        "length_position" => {
            candidate.length_position = decode_position(instance_name, parameter, value)?;
            candidate.length_position_changed = true;
        }
        "scale" => {
            candidate.scale = decode_scale(instance_name, parameter, value)?;
            candidate.scale_changed = true;
        }
        "root" => {
            candidate.root = decode_root(instance_name, parameter, value)?;
            candidate.root_changed = true;
        }
        "note_min" => {
            candidate.note_min = decode_note(instance_name, parameter, value)?;
            candidate.note_range_changed = true;
            candidate.note_range_parameter = Some("note_min");
        }
        "note_max" => {
            candidate.note_max = decode_note(instance_name, parameter, value)?;
            candidate.note_range_changed = true;
            candidate.note_range_parameter = Some("note_max");
        }
        "scale_output_scale" => {
            candidate.scale_output_scale = decode_scale(instance_name, parameter, value)?;
            candidate.scale_output_scale_changed = true;
        }
        "scale_output_root" => {
            candidate.scale_output_root = decode_root(instance_name, parameter, value)?;
            candidate.scale_output_root_changed = true;
        }
        _ => {
            return Err(CoreError::UnknownParameter {
                module: instance_name.to_owned(),
                param: parameter.to_owned(),
            });
        }
    }
    Ok(())
}

fn validate_candidate(instance_name: &str, candidate: &ConfigCandidate) -> Result<(), CoreError> {
    if !candidate.write_probability.is_finite()
        || !(0.0..=1.0).contains(&candidate.write_probability)
    {
        return Err(invalid_parameter(
            instance_name,
            "write_probability",
            "must be finite and in 0.0..=1.0",
        ));
    }
    if !crate::LengthSelector::VALID_LENGTHS.contains(&candidate.length) {
        return Err(invalid_parameter(
            instance_name,
            "length",
            "must be one of 2, 3, 4, 5, 6, 8, 10, 12, or 16",
        ));
    }
    if candidate.length_position > 8 {
        return Err(invalid_parameter(
            instance_name,
            "length_position",
            "must be in 0..=8",
        ));
    }
    if candidate.note_min > candidate.note_max {
        return Err(invalid_parameter(
            instance_name,
            candidate.note_range_parameter.unwrap_or("note_min"),
            "must not be greater than note_max",
        ));
    }
    validate_scale_spec(instance_name, "scale", &candidate.scale)?;
    validate_scale_spec(
        instance_name,
        "scale_output_scale",
        &candidate.scale_output_scale,
    )?;
    if candidate.root > 11 {
        return Err(invalid_parameter(
            instance_name,
            "root",
            "must be in 0..=11",
        ));
    }
    if candidate.scale_output_root > 11 {
        return Err(invalid_parameter(
            instance_name,
            "scale_output_root",
            "must be in 0..=11",
        ));
    }
    Ok(())
}

fn validate_scale_spec(
    instance_name: &str,
    parameter: &str,
    scale: &ScaleSpec,
) -> Result<(), CoreError> {
    if scale.root > 11 {
        return Err(invalid_parameter(
            instance_name,
            parameter,
            "root must be in 0..=11",
        ));
    }
    if scale.intervals.is_empty() {
        return Err(invalid_parameter(
            instance_name,
            parameter,
            "intervals must not be empty",
        ));
    }
    if scale.intervals.iter().any(|&interval| interval >= 12) {
        return Err(invalid_parameter(
            instance_name,
            parameter,
            "intervals must be below 12",
        ));
    }
    if scale.intervals.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(invalid_parameter(
            instance_name,
            parameter,
            "intervals must be strictly increasing and unique",
        ));
    }
    Ok(())
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

fn decode_length(
    instance_name: &str,
    parameter: &str,
    value: &ParameterValue,
) -> Result<usize, CoreError> {
    let value = decode_int(instance_name, parameter, value)?;
    let value = usize::try_from(value)
        .map_err(|_| invalid_parameter(instance_name, parameter, "must be a valid length"))?;
    if crate::LengthSelector::VALID_LENGTHS.contains(&value) {
        Ok(value)
    } else {
        Err(invalid_parameter(
            instance_name,
            parameter,
            "must be one of 2, 3, 4, 5, 6, 8, 10, 12, or 16",
        ))
    }
}

fn decode_position(
    instance_name: &str,
    parameter: &str,
    value: &ParameterValue,
) -> Result<usize, CoreError> {
    let value = decode_int(instance_name, parameter, value)?;
    if (0..=8).contains(&value) {
        Ok(value as usize)
    } else {
        Err(invalid_parameter(
            instance_name,
            parameter,
            "must be in 0..=8",
        ))
    }
}

fn decode_root(
    instance_name: &str,
    parameter: &str,
    value: &ParameterValue,
) -> Result<u8, CoreError> {
    let value = decode_int(instance_name, parameter, value)?;
    if (0..=11).contains(&value) {
        Ok(value as u8)
    } else {
        Err(invalid_parameter(
            instance_name,
            parameter,
            "must be in 0..=11",
        ))
    }
}

fn decode_note(
    instance_name: &str,
    parameter: &str,
    value: &ParameterValue,
) -> Result<u8, CoreError> {
    let value = decode_int(instance_name, parameter, value)?;
    if (0..=127).contains(&value) {
        Ok(value as u8)
    } else {
        Err(invalid_parameter(
            instance_name,
            parameter,
            "must be in 0..=127",
        ))
    }
}

fn decode_scale(
    instance_name: &str,
    parameter: &str,
    value: &ParameterValue,
) -> Result<ScaleSpec, CoreError> {
    let scale = match value {
        ParameterValue::Scale(scale) => scale,
        _ => {
            return Err(invalid_parameter(
                instance_name,
                parameter,
                "expected Scale",
            ));
        }
    };
    let spec = ScaleSpec {
        intervals: scale.intervals.clone(),
        root: scale.root,
        name: scale.name.clone(),
    };
    validate_scale_spec(instance_name, parameter, &spec)?;
    Ok(spec)
}

fn set_parameter(
    world: &mut World,
    entity: Entity,
    parameter: &str,
    value: ParameterValue,
) -> Result<(), CoreError> {
    let instance_name = module_instance_name(world, entity)?;
    let mut candidate = {
        let state = world
            .get::<TuringMachineState>(entity)
            .ok_or_else(|| CoreError::UnknownModule(instance_name.clone()))?;
        ConfigCandidate::from_state(state)
    };
    decode_into_candidate(&instance_name, parameter, &value, &mut candidate)?;
    validate_candidate(&instance_name, &candidate)?;
    let mut state = world
        .get_mut::<TuringMachineState>(entity)
        .ok_or_else(|| CoreError::UnknownModule(instance_name.clone()))?;
    apply_candidate(&mut state.engine, &candidate);
    if candidate.write_changed {
        state.write_base = candidate.write_probability;
    }
    if candidate.write_changed {
        reconcile_write_probability(&mut state);
    }
    Ok(())
}

fn apply_candidate(engine: &mut TuringMachine, candidate: &ConfigCandidate) {
    if candidate.write_changed {
        engine.set_write(candidate.write_probability);
    }
    if candidate.length_changed {
        engine.set_length(candidate.length);
    }
    if candidate.length_position_changed {
        engine.set_length_position(candidate.length_position);
    }
    if candidate.scale_changed {
        engine.set_scale(engine_scale(&candidate.scale));
        engine.set_root(candidate.scale.root);
    }
    if candidate.root_changed {
        engine.set_root(candidate.root);
    }
    if candidate.note_range_changed {
        engine.set_note_range(candidate.note_min..=candidate.note_max);
    }
    if candidate.scale_output_scale_changed {
        engine.set_scale_output_scale(engine_scale(&candidate.scale_output_scale));
        engine.set_scale_output_root(candidate.scale_output_scale.root);
    }
    if candidate.scale_output_root_changed {
        engine.set_scale_output_root(candidate.scale_output_root);
    }
}

fn module_instance_name(world: &World, entity: Entity) -> Result<String, CoreError> {
    let module = world
        .get::<Module>(entity)
        .ok_or_else(|| CoreError::UnknownModule(format!("{entity:?}")))?;
    if world.get::<TuringMachineState>(entity).is_none() {
        return Err(CoreError::UnknownModule(module.instance_name.clone()));
    }
    Ok(module.instance_name.clone())
}

fn state_ref(world: &World, entity: Entity) -> Result<&TuringMachineState, CoreError> {
    module_instance_name(world, entity)?;
    world
        .get::<TuringMachineState>(entity)
        .ok_or_else(|| CoreError::UnknownModule(format!("{entity:?}")))
}

fn set_write_probability(
    world: &mut World,
    entity: Entity,
    value: ParameterValue,
) -> Result<(), CoreError> {
    set_parameter(world, entity, "write_probability", value)
}
fn set_length(world: &mut World, entity: Entity, value: ParameterValue) -> Result<(), CoreError> {
    set_parameter(world, entity, "length", value)
}
fn set_length_position(
    world: &mut World,
    entity: Entity,
    value: ParameterValue,
) -> Result<(), CoreError> {
    set_parameter(world, entity, "length_position", value)
}
fn set_scale(world: &mut World, entity: Entity, value: ParameterValue) -> Result<(), CoreError> {
    set_parameter(world, entity, "scale", value)
}
fn set_root(world: &mut World, entity: Entity, value: ParameterValue) -> Result<(), CoreError> {
    set_parameter(world, entity, "root", value)
}
fn set_note_min(world: &mut World, entity: Entity, value: ParameterValue) -> Result<(), CoreError> {
    set_parameter(world, entity, "note_min", value)
}
fn set_note_max(world: &mut World, entity: Entity, value: ParameterValue) -> Result<(), CoreError> {
    set_parameter(world, entity, "note_max", value)
}
fn set_scale_output_scale(
    world: &mut World,
    entity: Entity,
    value: ParameterValue,
) -> Result<(), CoreError> {
    set_parameter(world, entity, "scale_output_scale", value)
}
fn set_scale_output_root(
    world: &mut World,
    entity: Entity,
    value: ParameterValue,
) -> Result<(), CoreError> {
    set_parameter(world, entity, "scale_output_root", value)
}

fn get_write_probability(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Float(
        state_ref(world, entity)?.engine.write_probability(),
    ))
}
fn get_length(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Int(
        state_ref(world, entity)?.engine.current_length() as i64,
    ))
}
fn get_length_position(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Int(
        state_ref(world, entity)?.engine.length_position() as i64,
    ))
}
fn get_scale(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    let state = state_ref(world, entity)?;
    Ok(ParameterValue::Scale(core_scale_from_engine(
        state.engine.scale(),
        state.engine.root(),
    )))
}
fn get_root(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Int(i64::from(
        state_ref(world, entity)?.engine.root(),
    )))
}
fn get_note_min(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Int(i64::from(
        *state_ref(world, entity)?.engine.note_range().start(),
    )))
}
fn get_note_max(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Int(i64::from(
        *state_ref(world, entity)?.engine.note_range().end(),
    )))
}
fn get_scale_output_scale(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    let state = state_ref(world, entity)?;
    Ok(ParameterValue::Scale(core_scale_from_engine(
        state.engine.scale_output_scale(),
        state.engine.scale_output_root(),
    )))
}
fn get_scale_output_root(world: &World, entity: Entity) -> Result<ParameterValue, CoreError> {
    Ok(ParameterValue::Int(i64::from(
        state_ref(world, entity)?.engine.scale_output_root(),
    )))
}

fn core_scale_from_engine(scale: &EngineScale, root: u8) -> oxurack_core::Scale {
    oxurack_core::Scale {
        root,
        intervals: scale.intervals().to_vec(),
        name: Some(scale.name().to_owned()),
    }
}

fn invalid_parameter(instance_name: &str, parameter: &str, reason: impl Into<String>) -> CoreError {
    CoreError::InvalidParameterValue {
        module: instance_name.to_owned(),
        param: parameter.to_owned(),
        reason: reason.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bevy_app::{App, Update};
    use bevy_ecs::prelude::QueryState;
    use bevy_ecs::schedule::ScheduleLabel;
    use oxurack_core::{Cable, CableIndex, CorePlugin, ParameterRegistry, Port};

    #[derive(Debug, Clone, PartialEq)]
    struct DeepObservation {
        length: usize,
        length_position: usize,
        write_probability: f32,
        root: u8,
        scale_intervals: Vec<u8>,
        scale_name: String,
        note_range: std::ops::RangeInclusive<u8>,
        scale_output_root: u8,
        scale_output_intervals: Vec<u8>,
        scale_output_name: String,
        register_bits: u16,
        step_count: u64,
        write_base: f32,
        write_modulation: f32,
        previous_gates: [bool; 3],
        pending: Option<StepOutputs>,
        inputs: Vec<(String, Value)>,
        ports: Vec<(String, Value)>,
    }

    fn parameters(values: &[(&str, ParameterValue)]) -> HashMap<String, ParameterValue> {
        values
            .iter()
            .map(|(name, value)| ((*name).to_owned(), value.clone()))
            .collect()
    }

    fn app_with_tm(values: &[(&str, ParameterValue)]) -> (App, Entity) {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        let entity = TuringMachineModule::spawn(app.world_mut(), "tm", &parameters(values))
            .expect("test Turing Machine should spawn");
        TuringMachineModule::register_systems(&mut app, Update.intern());
        (app, entity)
    }

    fn set_input(app: &mut App, _entity: Entity, port: Entity, value: Value) {
        app.world_mut()
            .get_mut::<CurrentValue>(port)
            .expect("input should exist")
            .0 = value;
    }

    fn module_input(app: &App, entity: Entity, name: &str) -> Entity {
        let state = app
            .world()
            .get::<TuringMachineState>(entity)
            .expect("state should exist");
        match name {
            "clock" => state.ports.clock,
            "write_modulation" => state.ports.write_modulation,
            "reset" => state.ports.reset,
            "move" => state.ports.move_step,
            _ => panic!("unknown test input {name}"),
        }
    }

    fn output_value(app: &App, entity: Entity, name: &str) -> Value {
        let state = app
            .world()
            .get::<TuringMachineState>(entity)
            .expect("state should exist");
        let port = match name {
            "step" => state.ports.step,
            "note" => state.ports.note,
            "note_valid" => state.ports.note_valid,
            "length" => state.ports.length,
            "write_probability" => state.ports.write_probability,
            "velocity" => state.ports.velocity,
            "velocity_valid" => state.ports.velocity_valid,
            "gate" => state.ports.gate,
            "scale_note" => state.ports.scale_note,
            "scale_note_valid" => state.ports.scale_note_valid,
            "div2" => state.ports.div2,
            "div4" => state.ports.div4,
            "noise_cc" => state.ports.noise_cc,
            "register_bits" => state.ports.register_bits,
            _ if PULSE_NAMES.contains(&name) => {
                state.ports.pulses[PULSE_NAMES
                    .iter()
                    .position(|port_name| *port_name == name)
                    .unwrap()]
            }
            _ if GATE_NAMES.contains(&name) => {
                state.ports.gates[GATE_NAMES
                    .iter()
                    .position(|port_name| *port_name == name)
                    .unwrap()]
            }
            _ => panic!("unknown test output {name}"),
        };
        app.world()
            .get::<CurrentValue>(port)
            .expect("output should exist")
            .0
    }

    fn output_port_entities(ports: Ports) -> [(&'static str, Entity); 28] {
        [
            ("step", ports.step),
            ("note", ports.note),
            ("note_valid", ports.note_valid),
            ("velocity", ports.velocity),
            ("velocity_valid", ports.velocity_valid),
            ("gate", ports.gate),
            ("scale_note", ports.scale_note),
            ("scale_note_valid", ports.scale_note_valid),
            ("pulse_1", ports.pulses[0]),
            ("pulse_2", ports.pulses[1]),
            ("pulse_3", ports.pulses[2]),
            ("pulse_4", ports.pulses[3]),
            ("pulse_5", ports.pulses[4]),
            ("pulse_6", ports.pulses[5]),
            ("gate_1", ports.gates[0]),
            ("gate_2", ports.gates[1]),
            ("gate_3", ports.gates[2]),
            ("gate_4", ports.gates[3]),
            ("gate_5", ports.gates[4]),
            ("gate_6", ports.gates[5]),
            ("gate_7", ports.gates[6]),
            ("gate_8", ports.gates[7]),
            ("div2", ports.div2),
            ("div4", ports.div4),
            ("noise_cc", ports.noise_cc),
            ("register_bits", ports.register_bits),
            ("length", ports.length),
            ("write_probability", ports.write_probability),
        ]
    }

    fn input_port_entities(ports: Ports) -> [(&'static str, Entity); 4] {
        [
            ("clock", ports.clock),
            ("write_modulation", ports.write_modulation),
            ("reset", ports.reset),
            ("move", ports.move_step),
        ]
    }

    fn deep_observation(world: &World, entity: Entity) -> DeepObservation {
        let state = world.get::<TuringMachineState>(entity).expect("TM state");
        let ports = output_port_entities(state.ports)
            .into_iter()
            .map(|(name, entity)| {
                (
                    name.to_owned(),
                    world.get::<CurrentValue>(entity).expect("port value").0,
                )
            })
            .collect();
        let inputs = input_port_entities(state.ports)
            .into_iter()
            .map(|(name, entity)| {
                (
                    name.to_owned(),
                    world.get::<CurrentValue>(entity).expect("input value").0,
                )
            })
            .collect();
        DeepObservation {
            length: state.engine.current_length(),
            length_position: state.engine.length_position(),
            write_probability: state.engine.write_probability(),
            root: state.engine.root(),
            scale_intervals: state.engine.scale().intervals().to_vec(),
            scale_name: state.engine.scale().name().to_owned(),
            note_range: state.engine.note_range(),
            scale_output_root: state.engine.scale_output_root(),
            scale_output_intervals: state.engine.scale_output_scale().intervals().to_vec(),
            scale_output_name: state.engine.scale_output_scale().name().to_owned(),
            register_bits: state.engine.register_bits(),
            step_count: state.engine.step_count(),
            write_base: state.write_base,
            write_modulation: state.write_modulation,
            previous_gates: [
                state.previous_clock,
                state.previous_reset,
                state.previous_move,
            ],
            pending: state.pending.clone(),
            inputs,
            ports,
        }
    }

    fn assert_matches_standalone(
        world: &World,
        entity: Entity,
        engine: &TuringMachine,
        pending: &Option<StepOutputs>,
        published: &StepOutputs,
        step: bool,
    ) {
        let actual = deep_observation(world, entity);
        assert_eq!(actual.length, engine.current_length());
        assert_eq!(actual.length_position, engine.length_position());
        assert_eq!(actual.write_probability, engine.write_probability());
        assert_eq!(actual.root, engine.root());
        assert_eq!(actual.scale_intervals, engine.scale().intervals());
        assert_eq!(actual.scale_name, engine.scale().name());
        assert_eq!(actual.note_range, engine.note_range());
        assert_eq!(actual.scale_output_root, engine.scale_output_root());
        assert_eq!(
            actual.scale_output_intervals,
            engine.scale_output_scale().intervals()
        );
        assert_eq!(actual.scale_output_name, engine.scale_output_scale().name());
        assert_eq!(actual.register_bits, engine.register_bits());
        assert_eq!(actual.step_count, engine.step_count());
        assert_eq!(&actual.pending, pending);
        assert_eq!(actual.ports, values_for_snapshot(published, step));
    }

    fn values_for_snapshot(outputs: &StepOutputs, step: bool) -> Vec<(String, Value)> {
        let mut values = vec![
            ("step".to_owned(), Value::Gate(step)),
            (
                "note".to_owned(),
                Value::Raw(u16::from(outputs.note.unwrap_or_default())),
            ),
            ("note_valid".to_owned(), Value::Gate(outputs.note.is_some())),
            (
                "velocity".to_owned(),
                Value::Raw(u16::from(outputs.velocity.unwrap_or_default())),
            ),
            (
                "velocity_valid".to_owned(),
                Value::Gate(outputs.velocity.is_some()),
            ),
            ("gate".to_owned(), Value::Gate(outputs.gate)),
            (
                "scale_note".to_owned(),
                Value::Raw(u16::from(outputs.scale_note.unwrap_or_default())),
            ),
            (
                "scale_note_valid".to_owned(),
                Value::Gate(outputs.scale_note.is_some()),
            ),
        ];
        for (index, value) in outputs.pulses.iter().copied().enumerate() {
            values.push((format!("pulse_{}", index + 1), Value::Gate(value)));
        }
        for (index, value) in outputs.gates.iter().copied().enumerate() {
            values.push((format!("gate_{}", index + 1), Value::Gate(value)));
        }
        values.extend([
            ("div2".to_owned(), Value::Gate(outputs.div2)),
            ("div4".to_owned(), Value::Gate(outputs.div4)),
            (
                "noise_cc".to_owned(),
                Value::Raw(u16::from(outputs.noise_cc)),
            ),
            (
                "register_bits".to_owned(),
                Value::Raw(outputs.register_bits),
            ),
            ("length".to_owned(), Value::Raw(outputs.length as u16)),
            (
                "write_probability".to_owned(),
                Value::Float(outputs.write_probability),
            ),
        ]);
        values
    }

    fn register_parameters(world: &mut World) {
        let kind = oxurack_core::ModuleKind::from(TuringMachineModule::KIND);
        let mut registry = world.resource_mut::<ParameterRegistry>();
        for &(name, setter) in TuringMachineModule::parameter_setters() {
            registry.register(kind.clone(), name, setter);
        }
        for &(name, getter) in TuringMachineModule::parameter_getters() {
            registry.register_getter(kind.clone(), name, getter);
        }
    }

    fn registered_set(
        world: &mut World,
        entity: Entity,
        name: &str,
        value: ParameterValue,
    ) -> Result<(), CoreError> {
        let kind = oxurack_core::ModuleKind::from(TuringMachineModule::KIND);
        world.resource_scope::<ParameterRegistry, _>(|world, registry| {
            registry.set_parameter(world, entity, &kind, name, value)
        })
    }

    fn registered_get(
        world: &World,
        entity: Entity,
        name: &str,
    ) -> Result<ParameterValue, CoreError> {
        let kind = oxurack_core::ModuleKind::from(TuringMachineModule::KIND);
        world
            .resource::<ParameterRegistry>()
            .get_parameter(world, entity, &kind, name)
    }

    fn invalid_parameter_values() -> Vec<(&'static str, ParameterValue)> {
        let valid_scale = || {
            ParameterValue::Scale(oxurack_core::Scale {
                root: 0,
                intervals: vec![0, 2, 5],
                name: Some("well formed".to_owned()),
            })
        };
        let mut invalid = Vec::new();
        for parameter in PARAMETER_NAMES {
            let expected = match parameter {
                "write_probability" => 0,
                "scale" | "scale_output_scale" => 4,
                _ => 1,
            };
            let wrong = [
                ParameterValue::Float(0.5),
                ParameterValue::Int(7),
                ParameterValue::Bool(true),
                ParameterValue::String("well formed".to_owned()),
                valid_scale(),
            ];
            invalid.extend(
                wrong
                    .into_iter()
                    .enumerate()
                    .filter_map(|(variant, value)| {
                        (variant != expected).then_some((parameter, value))
                    }),
            );
        }
        invalid.extend([
            ("write_probability", ParameterValue::Float(-0.01)),
            ("write_probability", ParameterValue::Float(1.01)),
            ("write_probability", ParameterValue::Float(f32::NAN)),
            ("write_probability", ParameterValue::Float(f32::INFINITY)),
            (
                "write_probability",
                ParameterValue::Float(f32::NEG_INFINITY),
            ),
            ("length", ParameterValue::Int(-1)),
            ("length", ParameterValue::Int(0)),
            ("length", ParameterValue::Int(1)),
            ("length", ParameterValue::Int(7)),
            ("length", ParameterValue::Int(9)),
            ("length", ParameterValue::Int(17)),
            ("length", ParameterValue::Int(i64::MIN)),
            ("length", ParameterValue::Int(i64::MAX)),
            ("length_position", ParameterValue::Int(-1)),
            ("length_position", ParameterValue::Int(9)),
            ("length_position", ParameterValue::Int(i64::MIN)),
            ("length_position", ParameterValue::Int(i64::MAX)),
            ("root", ParameterValue::Int(-1)),
            ("root", ParameterValue::Int(12)),
            ("scale_output_root", ParameterValue::Int(-1)),
            ("scale_output_root", ParameterValue::Int(12)),
            ("note_min", ParameterValue::Int(-1)),
            ("note_min", ParameterValue::Int(128)),
            ("note_max", ParameterValue::Int(-1)),
            ("note_max", ParameterValue::Int(128)),
            ("note_min", ParameterValue::Int(85)),
            ("note_max", ParameterValue::Int(35)),
        ]);
        for parameter in ["scale", "scale_output_scale"] {
            invalid.extend([
                (
                    parameter,
                    ParameterValue::Scale(oxurack_core::Scale {
                        root: 12,
                        intervals: vec![0],
                        name: None,
                    }),
                ),
                (
                    parameter,
                    ParameterValue::Scale(oxurack_core::Scale {
                        root: 0,
                        intervals: vec![],
                        name: None,
                    }),
                ),
                (
                    parameter,
                    ParameterValue::Scale(oxurack_core::Scale {
                        root: 0,
                        intervals: vec![2, 1],
                        name: None,
                    }),
                ),
                (
                    parameter,
                    ParameterValue::Scale(oxurack_core::Scale {
                        root: 0,
                        intervals: vec![1, 1],
                        name: None,
                    }),
                ),
                (
                    parameter,
                    ParameterValue::Scale(oxurack_core::Scale {
                        root: 0,
                        intervals: vec![12],
                        name: None,
                    }),
                ),
            ]);
        }
        invalid
    }

    #[test]
    fn schema_declares_complete_port_and_parameter_inventory() {
        let expected_names = [
            "clock",
            "write_modulation",
            "reset",
            "move",
            "step",
            "note",
            "note_valid",
            "velocity",
            "velocity_valid",
            "gate",
            "scale_note",
            "scale_note_valid",
            "pulse_1",
            "pulse_2",
            "pulse_3",
            "pulse_4",
            "pulse_5",
            "pulse_6",
            "gate_1",
            "gate_2",
            "gate_3",
            "gate_4",
            "gate_5",
            "gate_6",
            "gate_7",
            "gate_8",
            "div2",
            "div4",
            "noise_cc",
            "register_bits",
            "length",
            "write_probability",
        ];
        let expected_ports = expected_names.map(|name| {
            let (direction, value_kind, merge_policy) = match name {
                "clock" | "reset" | "move" => {
                    (PortDirection::Input, ValueKind::Gate, MergePolicy::Reject)
                }
                "write_modulation" => (PortDirection::Input, ValueKind::Bipolar, MergePolicy::Sum),
                "note" | "velocity" | "scale_note" | "noise_cc" | "register_bits" | "length" => {
                    (PortDirection::Output, ValueKind::Raw, MergePolicy::Reject)
                }
                "write_probability" => {
                    (PortDirection::Output, ValueKind::Float, MergePolicy::Reject)
                }
                _ => (PortDirection::Output, ValueKind::Gate, MergePolicy::Reject),
            };
            (name, direction, value_kind, merge_policy)
        });
        let actual_ports: Vec<_> = TuringMachineModule::port_schema()
            .iter()
            .map(|schema| {
                (
                    schema.name,
                    schema.direction,
                    schema.value_kind,
                    schema.merge_policy,
                )
            })
            .collect();
        assert_eq!(actual_ports, expected_ports);

        let expected_parameters = [
            ("write_probability", ParameterValue::Float(0.5)),
            ("length", ParameterValue::Int(16)),
            ("length_position", ParameterValue::Int(8)),
            ("scale", ParameterValue::Scale(core_chromatic())),
            ("root", ParameterValue::Int(0)),
            ("note_min", ParameterValue::Int(36)),
            ("note_max", ParameterValue::Int(84)),
            (
                "scale_output_scale",
                ParameterValue::Scale(core_chromatic()),
            ),
            ("scale_output_root", ParameterValue::Int(0)),
        ];
        let actual_parameters: Vec<_> = TuringMachineModule::parameter_schema()
            .iter()
            .map(|schema| (schema.name, schema.default.clone()))
            .collect();
        assert_eq!(actual_parameters, expected_parameters);

        let setters: Vec<_> = TuringMachineModule::parameter_setters()
            .iter()
            .map(|(name, _)| *name)
            .collect();
        let getters: Vec<_> = TuringMachineModule::parameter_getters()
            .iter()
            .map(|(name, _)| *name)
            .collect();
        assert_eq!(setters, PARAMETER_NAMES);
        assert_eq!(getters, PARAMETER_NAMES);
        assert_eq!(
            setters
                .iter()
                .copied()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            9
        );
        assert_eq!(
            getters
                .iter()
                .copied()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            9
        );
    }

    #[test]
    fn unknown_keys_win_lexically_and_failed_spawns_leave_world_empty() {
        let mut world = World::new();
        let unrelated = world.spawn_empty().id();
        let before = entity_counts(&mut world);
        let result = TuringMachineModule::spawn(
            &mut world,
            "tm",
            &parameters(&[
                ("zeta", ParameterValue::Float(f32::NAN)),
                ("alpha", ParameterValue::Int(1)),
            ]),
        );
        assert!(matches!(
            result,
            Err(CoreError::UnknownParameter { module, param })
                if module == "tm" && param == "alpha"
        ));
        assert_eq!(entity_counts(&mut world), before);
        assert!(world.get_entity(unrelated).is_ok());

        for values in [
            parameters(&[
                ("zeta", ParameterValue::Float(f32::NAN)),
                ("alpha", ParameterValue::Int(1)),
                ("write_probability", ParameterValue::Float(f32::NAN)),
            ]),
            parameters(&[
                ("alpha", ParameterValue::Int(1)),
                ("zeta", ParameterValue::Float(f32::NAN)),
                ("write_probability", ParameterValue::Float(f32::NAN)),
            ]),
        ] {
            let error = TuringMachineModule::spawn(&mut world, "tm", &values).unwrap_err();
            assert!(
                matches!(error, CoreError::UnknownParameter { module, param }
                if module == "tm" && param == "alpha")
            );
            assert_eq!(entity_counts(&mut world), before);
        }
        let error = TuringMachineModule::spawn(
            &mut world,
            "tm",
            &parameters(&[
                ("zeta", ParameterValue::Float(0.0)),
                ("write_probability", ParameterValue::Float(f32::NAN)),
            ]),
        )
        .unwrap_err();
        assert!(
            matches!(error, CoreError::UnknownParameter { module, param }
            if module == "tm" && param == "zeta")
        );
        assert_eq!(entity_counts(&mut world), before);

        let malformed = oxurack_core::Scale {
            root: 0,
            intervals: vec![0, 0],
            name: Some("bad".to_owned()),
        };
        let result = TuringMachineModule::spawn(
            &mut world,
            "tm",
            &parameters(&[("scale", ParameterValue::Scale(malformed))]),
        );
        assert!(matches!(
            result,
            Err(CoreError::InvalidParameterValue { param, .. }) if param == "scale"
        ));
        assert_eq!(entity_counts(&mut world), before);
    }

    #[test]
    fn every_invalid_spawn_variant_and_range_is_rejected_without_orphans() {
        let mut world = World::new();
        let unrelated = world.spawn_empty().id();
        for (parameter, invalid) in invalid_parameter_values() {
            let before = entity_counts(&mut world);
            let error = TuringMachineModule::spawn(
                &mut world,
                "invalid_spawn",
                &parameters(&[(parameter, invalid.clone())]),
            )
            .unwrap_err();
            assert!(
                matches!(&error,
                CoreError::InvalidParameterValue { module, param, .. }
                    if module == "invalid_spawn" && param == parameter),
                "spawn {parameter} accepted or misattributed {invalid:?}: {error:?}"
            );
            assert_eq!(
                entity_counts(&mut world),
                before,
                "orphan after {parameter}={invalid:?}"
            );
            assert!(world.get_entity(unrelated).is_ok());
        }

        for (values, expected) in [
            (vec![("note_min", ParameterValue::Int(85))], "note_min"),
            (vec![("note_max", ParameterValue::Int(35))], "note_max"),
            (
                vec![
                    ("note_min", ParameterValue::Int(100)),
                    ("note_max", ParameterValue::Int(90)),
                ],
                "note_max",
            ),
        ] {
            let before = entity_counts(&mut world);
            let error = TuringMachineModule::spawn(&mut world, "range_spawn", &parameters(&values))
                .unwrap_err();
            assert!(matches!(error,
                CoreError::InvalidParameterValue { module, param, .. }
                    if module == "range_spawn" && param == expected));
            assert_eq!(entity_counts(&mut world), before);
        }
    }

    #[test]
    fn spawn_presence_precedence_and_embedded_roots_are_preserved() {
        let scale = oxurack_core::Scale {
            root: 4,
            intervals: vec![0, 2, 5, 7, 9],
            name: None,
        };
        let secondary = oxurack_core::Scale {
            root: 6,
            intervals: vec![0, 3, 5, 7, 10],
            name: Some("Secondary".to_owned()),
        };
        let mut world = World::new();
        let length_only = TuringMachineModule::spawn(
            &mut world,
            "length_only",
            &parameters(&[("length", ParameterValue::Int(12))]),
        )
        .unwrap();
        assert_eq!(
            get_length(&world, length_only).unwrap(),
            ParameterValue::Int(12)
        );
        assert_eq!(
            get_length_position(&world, length_only).unwrap(),
            ParameterValue::Int(7)
        );

        let both_length_forms = TuringMachineModule::spawn(
            &mut world,
            "both_lengths",
            &parameters(&[
                ("length", ParameterValue::Int(12)),
                ("length_position", ParameterValue::Int(2)),
            ]),
        )
        .unwrap();
        assert_eq!(
            get_length(&world, both_length_forms).unwrap(),
            ParameterValue::Int(4)
        );
        assert_eq!(
            get_length_position(&world, both_length_forms).unwrap(),
            ParameterValue::Int(2)
        );

        let embedded = TuringMachineModule::spawn(
            &mut world,
            "embedded",
            &parameters(&[
                ("scale", ParameterValue::Scale(scale.clone())),
                (
                    "scale_output_scale",
                    ParameterValue::Scale(secondary.clone()),
                ),
            ]),
        )
        .unwrap();
        assert_eq!(get_root(&world, embedded).unwrap(), ParameterValue::Int(4));
        assert_eq!(
            get_scale(&world, embedded).unwrap(),
            ParameterValue::Scale(oxurack_core::Scale {
                root: 4,
                intervals: scale.intervals.clone(),
                name: Some("Custom".to_owned())
            })
        );
        assert_eq!(
            get_scale_output_root(&world, embedded).unwrap(),
            ParameterValue::Int(6)
        );

        let explicit = TuringMachineModule::spawn(
            &mut world,
            "explicit",
            &parameters(&[
                ("scale", ParameterValue::Scale(scale)),
                ("root", ParameterValue::Int(7)),
                ("scale_output_scale", ParameterValue::Scale(secondary)),
                ("scale_output_root", ParameterValue::Int(9)),
            ]),
        )
        .unwrap();
        assert_eq!(get_root(&world, explicit).unwrap(), ParameterValue::Int(7));
        assert_eq!(
            get_scale_output_root(&world, explicit).unwrap(),
            ParameterValue::Int(9)
        );
    }

    #[test]
    fn custom_scales_roots_ranges_and_both_length_forms_round_trip() {
        let scale = oxurack_core::Scale {
            root: 7,
            intervals: vec![0, 2, 4, 7, 9],
            name: Some("Custom Five".to_owned()),
        };
        let output_scale = oxurack_core::Scale {
            root: 3,
            intervals: vec![0, 3, 5, 7, 10],
            name: Some("Output Five".to_owned()),
        };
        let (mut app, entity) = app_with_tm(&[
            ("write_probability", ParameterValue::Float(0.25)),
            ("length", ParameterValue::Int(12)),
            ("length_position", ParameterValue::Int(2)),
            ("scale", ParameterValue::Scale(scale.clone())),
            ("root", ParameterValue::Int(9)),
            ("note_min", ParameterValue::Int(41)),
            ("note_max", ParameterValue::Int(73)),
            (
                "scale_output_scale",
                ParameterValue::Scale(output_scale.clone()),
            ),
            ("scale_output_root", ParameterValue::Int(5)),
        ]);
        let state = app.world().get::<TuringMachineState>(entity).unwrap();
        assert_eq!(state.engine.current_length(), 4);
        assert_eq!(state.engine.length_position(), 2);
        assert_eq!(state.engine.root(), 9);
        assert_eq!(state.engine.note_range(), 41..=73);
        assert_eq!(state.engine.scale().intervals(), scale.intervals.as_slice());
        assert_eq!(state.engine.scale().name(), "Custom Five");
        assert_eq!(state.engine.scale_output_root(), 5);
        assert_eq!(
            state.engine.scale_output_scale().intervals(),
            output_scale.intervals.as_slice()
        );
        assert_eq!(
            get_scale(app.world(), entity).unwrap(),
            ParameterValue::Scale(oxurack_core::Scale {
                root: 9,
                intervals: scale.intervals,
                name: Some("Custom Five".to_owned()),
            })
        );
        set_length(app.world_mut(), entity, ParameterValue::Int(8)).unwrap();
        assert_eq!(
            get_length(app.world(), entity).unwrap(),
            ParameterValue::Int(8)
        );
    }

    #[test]
    fn registered_setters_and_getters_cover_every_valid_value_and_alias() {
        let (mut app, entity) = app_with_tm(&[]);
        register_parameters(app.world_mut());

        for probability in [0.0, 0.25, 1.0] {
            registered_set(
                app.world_mut(),
                entity,
                "write_probability",
                ParameterValue::Float(probability),
            )
            .unwrap();
            assert_eq!(
                registered_get(app.world(), entity, "write_probability").unwrap(),
                ParameterValue::Float(probability)
            );
        }

        let lengths = [2, 3, 4, 5, 6, 8, 10, 12, 16];
        for (position, length) in lengths.into_iter().enumerate() {
            registered_set(
                app.world_mut(),
                entity,
                "length_position",
                ParameterValue::Int(position as i64),
            )
            .unwrap();
            assert_eq!(
                registered_get(app.world(), entity, "length_position").unwrap(),
                ParameterValue::Int(position as i64)
            );
            assert_eq!(
                registered_get(app.world(), entity, "length").unwrap(),
                ParameterValue::Int(length)
            );
            registered_set(
                app.world_mut(),
                entity,
                "length",
                ParameterValue::Int(length),
            )
            .unwrap();
            assert_eq!(
                registered_get(app.world(), entity, "length").unwrap(),
                ParameterValue::Int(length)
            );
            assert_eq!(
                registered_get(app.world(), entity, "length_position").unwrap(),
                ParameterValue::Int(position as i64)
            );
        }

        let main_scale = oxurack_core::Scale {
            root: 4,
            intervals: vec![0, 2, 5, 7, 9],
            name: None,
        };
        registered_set(
            app.world_mut(),
            entity,
            "scale",
            ParameterValue::Scale(main_scale.clone()),
        )
        .unwrap();
        assert_eq!(
            registered_get(app.world(), entity, "scale").unwrap(),
            ParameterValue::Scale(oxurack_core::Scale {
                root: 4,
                intervals: vec![0, 2, 5, 7, 9],
                name: Some("Custom".to_owned()),
            })
        );
        registered_set(app.world_mut(), entity, "root", ParameterValue::Int(7)).unwrap();
        assert_eq!(
            registered_get(app.world(), entity, "root").unwrap(),
            ParameterValue::Int(7)
        );
        assert_eq!(
            registered_get(app.world(), entity, "scale").unwrap(),
            ParameterValue::Scale(oxurack_core::Scale {
                root: 7,
                intervals: vec![0, 2, 5, 7, 9],
                name: Some("Custom".to_owned()),
            })
        );

        let secondary = oxurack_core::Scale {
            root: 4,
            intervals: vec![0, 3, 5, 7, 10],
            name: Some("Secondary Custom".to_owned()),
        };
        registered_set(
            app.world_mut(),
            entity,
            "scale_output_scale",
            ParameterValue::Scale(secondary.clone()),
        )
        .unwrap();
        assert_eq!(
            registered_get(app.world(), entity, "scale_output_scale").unwrap(),
            ParameterValue::Scale(secondary.clone())
        );
        registered_set(
            app.world_mut(),
            entity,
            "scale_output_root",
            ParameterValue::Int(11),
        )
        .unwrap();
        assert_eq!(
            registered_get(app.world(), entity, "scale_output_root").unwrap(),
            ParameterValue::Int(11)
        );
        assert_eq!(
            registered_get(app.world(), entity, "scale_output_scale").unwrap(),
            ParameterValue::Scale(oxurack_core::Scale {
                root: 11,
                intervals: secondary.intervals,
                name: secondary.name,
            })
        );

        registered_set(app.world_mut(), entity, "note_min", ParameterValue::Int(0)).unwrap();
        registered_set(
            app.world_mut(),
            entity,
            "note_max",
            ParameterValue::Int(127),
        )
        .unwrap();
        assert_eq!(
            registered_get(app.world(), entity, "note_min").unwrap(),
            ParameterValue::Int(0)
        );
        assert_eq!(
            registered_get(app.world(), entity, "note_max").unwrap(),
            ParameterValue::Int(127)
        );
        registered_set(app.world_mut(), entity, "note_min", ParameterValue::Int(64)).unwrap();
        registered_set(app.world_mut(), entity, "note_max", ParameterValue::Int(64)).unwrap();
        assert_eq!(
            registered_get(app.world(), entity, "note_min").unwrap(),
            ParameterValue::Int(64)
        );
        assert_eq!(
            registered_get(app.world(), entity, "note_max").unwrap(),
            ParameterValue::Int(64)
        );
    }

    #[test]
    fn every_invalid_spawn_and_registered_setter_is_atomic_with_seeded_continuation() {
        let config = [
            ("write_probability", ParameterValue::Float(0.75)),
            ("length", ParameterValue::Int(12)),
            ("length_position", ParameterValue::Int(3)),
            (
                "scale",
                ParameterValue::Scale(oxurack_core::Scale {
                    root: 4,
                    intervals: vec![0, 2, 5, 7, 9],
                    name: Some("Configured".to_owned()),
                }),
            ),
            ("root", ParameterValue::Int(7)),
            ("note_min", ParameterValue::Int(40)),
            ("note_max", ParameterValue::Int(84)),
            (
                "scale_output_scale",
                ParameterValue::Scale(oxurack_core::Scale {
                    root: 3,
                    intervals: vec![0, 3, 5, 7, 10],
                    name: Some("Output".to_owned()),
                }),
            ),
            ("scale_output_root", ParameterValue::Int(8)),
        ];
        for (parameter, invalid) in invalid_parameter_values() {
            let mut spawn_world = World::new();
            let unrelated = spawn_world.spawn_empty().id();
            let before_counts = entity_counts(&mut spawn_world);
            let spawn_error = TuringMachineModule::spawn(
                &mut spawn_world,
                "spawn_target",
                &parameters(&[(parameter, invalid.clone())]),
            )
            .unwrap_err();
            assert!(
                matches!(&spawn_error,
                CoreError::InvalidParameterValue { module, param, .. }
                    if module == "spawn_target" && param == parameter),
                "spawn {parameter} accepted or misattributed {invalid:?}: {spawn_error:?}"
            );
            assert_eq!(
                entity_counts(&mut spawn_world),
                before_counts,
                "spawn {parameter}"
            );
            assert!(spawn_world.get_entity(unrelated).is_ok());

            let (mut subject, subject_entity) = app_with_tm(&config);
            let (mut control, control_entity) = app_with_tm(&config);
            register_parameters(subject.world_mut());
            register_parameters(control.world_mut());
            for (app, instance) in [
                (&mut subject, subject_entity),
                (&mut control, control_entity),
            ] {
                app.update();
                let modulation = module_input(app, instance, "write_modulation");
                let clock = module_input(app, instance, "clock");
                set_input(app, instance, modulation, Value::Bipolar(0.25));
                set_input(app, instance, clock, Value::Gate(true));
                app.update();
            }
            let mut oracle = TuringMachine::with_seed(
                oxurack_core::ModuleSpawnContext::new(0, "tm").instance_seed(),
            );
            oracle.set_write(0.75);
            oracle.set_length(12);
            oracle.set_length_position(3);
            oracle.set_scale(EngineScale::new(vec![0, 2, 5, 7, 9], "Configured"));
            oracle.set_root(7);
            oracle.set_note_range(40..=84);
            oracle.set_scale_output_scale(EngineScale::new(vec![0, 3, 5, 7, 10], "Output"));
            oracle.set_scale_output_root(8);
            oracle.modulate_write(0.25);
            let mut expected_pending = Some(oracle.tick());
            let mut expected_published = StepOutputs::default();
            let before = deep_observation(subject.world(), subject_entity);
            assert!(before.pending.is_some(), "precondition: pending snapshot");
            assert_eq!(before, deep_observation(control.world(), control_entity));
            assert_matches_standalone(
                subject.world(),
                subject_entity,
                &oracle,
                &expected_pending,
                &expected_published,
                false,
            );
            let error = registered_set(
                subject.world_mut(),
                subject_entity,
                parameter,
                invalid.clone(),
            )
            .unwrap_err();
            assert!(
                matches!(&error,
                CoreError::InvalidParameterValue { module, param, .. }
                    if module == "tm" && param == parameter),
                "setter {parameter} accepted or misattributed {invalid:?}: {error:?}"
            );
            assert_eq!(
                deep_observation(subject.world(), subject_entity),
                before,
                "setter {parameter}"
            );

            // Keep the consumed high edge and compare the original pending
            // snapshot against the separately configured standalone engine.
            subject.update();
            control.update();
            expected_published = expected_pending.take().unwrap();
            assert_matches_standalone(
                subject.world(),
                subject_entity,
                &oracle,
                &expected_pending,
                &expected_published,
                true,
            );
            assert_eq!(
                deep_observation(subject.world(), subject_entity),
                deep_observation(control.world(), control_entity)
            );
            for value in [false, true] {
                let subject_clock = module_input(&subject, subject_entity, "clock");
                let control_clock = module_input(&control, control_entity, "clock");
                set_input(
                    &mut subject,
                    subject_entity,
                    subject_clock,
                    Value::Gate(value),
                );
                set_input(
                    &mut control,
                    control_entity,
                    control_clock,
                    Value::Gate(value),
                );
                subject.update();
                control.update();
                if value {
                    expected_pending = Some(oracle.tick());
                }
                assert_matches_standalone(
                    subject.world(),
                    subject_entity,
                    &oracle,
                    &expected_pending,
                    &expected_published,
                    false,
                );
                assert_eq!(
                    deep_observation(subject.world(), subject_entity),
                    deep_observation(control.world(), control_entity),
                    "continuation after rejected {parameter}={invalid:?}"
                );
            }
            subject.update();
            control.update();
            expected_published = expected_pending.take().unwrap();
            assert_matches_standalone(
                subject.world(),
                subject_entity,
                &oracle,
                &expected_pending,
                &expected_published,
                true,
            );
            assert_eq!(
                deep_observation(subject.world(), subject_entity),
                deep_observation(control.world(), control_entity),
                "published continuation after rejected {parameter}={invalid:?}",
            );
        }
    }

    #[test]
    fn validation_covers_boundaries_wrong_variants_and_all_registered_accessors() {
        let valid_scale = oxurack_core::Scale {
            root: 11,
            intervals: vec![11],
            name: None,
        };
        let valid = [
            ("write_probability", ParameterValue::Float(0.0)),
            ("length", ParameterValue::Int(2)),
            ("length_position", ParameterValue::Int(0)),
            ("scale", ParameterValue::Scale(valid_scale.clone())),
            ("root", ParameterValue::Int(0)),
            ("note_min", ParameterValue::Int(0)),
            ("note_max", ParameterValue::Int(127)),
            (
                "scale_output_scale",
                ParameterValue::Scale(valid_scale.clone()),
            ),
            ("scale_output_root", ParameterValue::Int(11)),
        ];
        let mut world = World::new();
        let entity = TuringMachineModule::spawn(&mut world, "tm", &parameters(&valid))
            .expect("boundary values should be accepted");
        assert_eq!(get_length(&world, entity).unwrap(), ParameterValue::Int(2));
        assert_eq!(
            get_length_position(&world, entity).unwrap(),
            ParameterValue::Int(0)
        );
        assert_eq!(get_root(&world, entity).unwrap(), ParameterValue::Int(0));
        assert_eq!(
            get_note_min(&world, entity).unwrap(),
            ParameterValue::Int(0)
        );
        assert_eq!(
            get_note_max(&world, entity).unwrap(),
            ParameterValue::Int(127)
        );
        assert_eq!(
            get_scale_output_root(&world, entity).unwrap(),
            ParameterValue::Int(11)
        );

        for (parameter, value) in [
            ("write_probability", ParameterValue::Bool(true)),
            ("length", ParameterValue::Bool(true)),
            ("length_position", ParameterValue::Bool(true)),
            ("scale", ParameterValue::Bool(true)),
            ("root", ParameterValue::Bool(true)),
            ("note_min", ParameterValue::Bool(true)),
            ("note_max", ParameterValue::Bool(true)),
            ("scale_output_scale", ParameterValue::Bool(true)),
            ("scale_output_root", ParameterValue::Bool(true)),
        ] {
            let error = set_parameter(&mut world, entity, parameter, value).unwrap_err();
            assert!(matches!(
                error,
                CoreError::InvalidParameterValue { param, .. } if param == parameter
            ));
        }

        for (parameter, value) in [
            ("write_probability", ParameterValue::Float(-0.01)),
            ("write_probability", ParameterValue::Float(1.01)),
            ("write_probability", ParameterValue::Float(f32::NAN)),
            ("length", ParameterValue::Int(1)),
            ("length", ParameterValue::Int(17)),
            ("length_position", ParameterValue::Int(-1)),
            ("length_position", ParameterValue::Int(9)),
            ("root", ParameterValue::Int(-1)),
            ("root", ParameterValue::Int(12)),
            ("note_min", ParameterValue::Int(-1)),
            ("note_max", ParameterValue::Int(128)),
            ("scale_output_root", ParameterValue::Int(12)),
        ] {
            let error = set_parameter(&mut world, entity, parameter, value).unwrap_err();
            assert!(matches!(
                error,
                CoreError::InvalidParameterValue { param, .. } if param == parameter
            ));
        }

        for malformed in [
            oxurack_core::Scale {
                root: 0,
                intervals: vec![],
                name: None,
            },
            oxurack_core::Scale {
                root: 0,
                intervals: vec![2, 1],
                name: None,
            },
            oxurack_core::Scale {
                root: 0,
                intervals: vec![1, 1],
                name: None,
            },
            oxurack_core::Scale {
                root: 0,
                intervals: vec![12],
                name: None,
            },
            oxurack_core::Scale {
                root: 12,
                intervals: vec![0],
                name: None,
            },
        ] {
            let error = set_parameter(
                &mut world,
                entity,
                "scale",
                ParameterValue::Scale(malformed),
            )
            .unwrap_err();
            assert!(matches!(
                error,
                CoreError::InvalidParameterValue { param, .. } if param == "scale"
            ));
        }

        let kind = oxurack_core::ModuleKind::from("turing_machine");
        let mut registry = ParameterRegistry::default();
        for &(parameter, setter) in TuringMachineModule::parameter_setters() {
            registry.register(kind.clone(), parameter, setter);
        }
        for &(parameter, getter) in TuringMachineModule::parameter_getters() {
            registry.register_getter(kind.clone(), parameter, getter);
        }
        world.insert_resource(registry);
        let registry = world.resource::<ParameterRegistry>();
        for parameter in PARAMETER_NAMES {
            registry
                .get_parameter(&world, entity, &kind, parameter)
                .unwrap_or_else(|error| panic!("registered getter {parameter} failed: {error}"));
        }
        assert_eq!(
            optional_output(Some(127)),
            (Value::Raw(127), Value::Gate(true))
        );
        assert_eq!(optional_output(None), (Value::Raw(0), Value::Gate(false)));
    }

    #[test]
    fn every_registered_accessor_rejects_missing_and_non_turing_entities() {
        let mut world = World::new();
        world.insert_resource(ParameterRegistry::default());
        register_parameters(&mut world);
        let missing = world.spawn_empty().id();
        assert!(world.despawn(missing));
        let bare = world.spawn_empty().id();
        let other_module = spawn_module_entity(&mut world, "other", "other-instance");
        let valid_values = [
            ParameterValue::Float(0.5),
            ParameterValue::Int(8),
            ParameterValue::Int(8),
            ParameterValue::Scale(core_chromatic()),
            ParameterValue::Int(0),
            ParameterValue::Int(36),
            ParameterValue::Int(84),
            ParameterValue::Scale(core_chromatic()),
            ParameterValue::Int(0),
        ];
        for (index, name) in PARAMETER_NAMES.into_iter().enumerate() {
            for entity in [missing, bare, other_module] {
                assert!(
                    matches!(
                        registered_get(&world, entity, name),
                        Err(CoreError::UnknownModule(_))
                    ),
                    "getter {name} accepted entity {entity:?}"
                );
                assert!(
                    matches!(
                        registered_set(&mut world, entity, name, valid_values[index].clone()),
                        Err(CoreError::UnknownModule(_))
                    ),
                    "setter {name} accepted entity {entity:?}"
                );
            }
        }
    }

    #[test]
    fn consume_publishes_one_sample_later_and_wide_gates_do_not_retrigger() {
        let (mut app, entity) = app_with_tm(&[]);
        app.update();
        let clock = module_input(&app, entity, "clock");
        set_input(&mut app, entity, clock, Value::Gate(true));
        app.update();
        assert_eq!(output_value(&app, entity, "step"), Value::Gate(false));
        app.update();
        assert_eq!(output_value(&app, entity, "step"), Value::Gate(true));
        let steps = app
            .world()
            .get::<TuringMachineState>(entity)
            .unwrap()
            .engine
            .step_count();
        app.update();
        assert_eq!(output_value(&app, entity, "step"), Value::Gate(false));
        assert_eq!(
            app.world()
                .get::<TuringMachineState>(entity)
                .unwrap()
                .engine
                .step_count(),
            steps
        );
    }

    #[test]
    fn real_produce_publication_maps_options_and_holds_all_values_on_idle() {
        let (mut app, entity) = app_with_tm(&[]);
        let snapshot = StepOutputs {
            note: Some(0),
            velocity: None,
            gate: true,
            scale_note: Some(127),
            pulses: [true, false, true, false, false, true],
            gates: [false, true, false, true, false, true, false, true],
            div2: true,
            div4: false,
            noise_cc: 83,
            register_bits: 0xA35C,
            length: 12,
            write_probability: 0.375,
        };
        app.world_mut()
            .get_mut::<TuringMachineState>(entity)
            .unwrap()
            .pending = Some(snapshot);
        app.update();
        let expected = [
            ("step", Value::Gate(true)),
            ("note", Value::Raw(0)),
            ("note_valid", Value::Gate(true)),
            ("velocity", Value::Raw(0)),
            ("velocity_valid", Value::Gate(false)),
            ("gate", Value::Gate(true)),
            ("scale_note", Value::Raw(127)),
            ("scale_note_valid", Value::Gate(true)),
            ("pulse_1", Value::Gate(true)),
            ("pulse_2", Value::Gate(false)),
            ("pulse_3", Value::Gate(true)),
            ("pulse_4", Value::Gate(false)),
            ("pulse_5", Value::Gate(false)),
            ("pulse_6", Value::Gate(true)),
            ("gate_1", Value::Gate(false)),
            ("gate_2", Value::Gate(true)),
            ("gate_3", Value::Gate(false)),
            ("gate_4", Value::Gate(true)),
            ("gate_5", Value::Gate(false)),
            ("gate_6", Value::Gate(true)),
            ("gate_7", Value::Gate(false)),
            ("gate_8", Value::Gate(true)),
            ("div2", Value::Gate(true)),
            ("div4", Value::Gate(false)),
            ("noise_cc", Value::Raw(83)),
            ("register_bits", Value::Raw(0xA35C)),
            ("length", Value::Raw(12)),
            ("write_probability", Value::Float(0.375)),
        ];
        for (name, value) in expected {
            assert_eq!(output_value(&app, entity, name), value, "published {name}");
        }
        let mut held = deep_observation(app.world(), entity).ports;
        held[0].1 = Value::Gate(false);
        app.update();
        assert_eq!(output_value(&app, entity, "step"), Value::Gate(false));
        assert_eq!(
            deep_observation(app.world(), entity).ports,
            held,
            "idle Produce holds snapshot fields"
        );

        app.world_mut()
            .get_mut::<TuringMachineState>(entity)
            .unwrap()
            .pending = Some(StepOutputs {
            note: None,
            velocity: None,
            scale_note: None,
            ..StepOutputs::default()
        });
        app.update();
        for (name, value) in [
            ("note", Value::Raw(0)),
            ("note_valid", Value::Gate(false)),
            ("velocity", Value::Raw(0)),
            ("velocity_valid", Value::Gate(false)),
            ("scale_note", Value::Raw(0)),
            ("scale_note_valid", Value::Gate(false)),
        ] {
            assert_eq!(output_value(&app, entity, name), value, "absent {name}");
        }
    }

    #[test]
    fn consume_priority_consumes_losing_edges_and_matches_scripted_engine() {
        let (mut app, entity) = app_with_tm(&[]);
        let clock = module_input(&app, entity, "clock");
        let reset = module_input(&app, entity, "reset");
        let move_step = module_input(&app, entity, "move");
        let modulation = module_input(&app, entity, "write_modulation");
        let script = [
            (true, false, false),
            (true, false, false),
            (false, false, false),
            (true, false, true),
            (true, false, true),
            (false, false, false),
            (true, true, true),
            (true, true, true),
            (false, false, false),
            (true, false, false),
            (false, false, false),
        ];
        let mut reference = TuringMachine::with_seed(
            oxurack_core::ModuleSpawnContext::new(0, "tm").instance_seed(),
        );
        let mut pending = None;
        let mut published = StepOutputs::default();
        let (mut previous_clock, mut previous_reset, mut previous_move) = (false, false, false);
        for (sample, (clock_level, reset_level, move_level)) in script.into_iter().enumerate() {
            let published_step = if let Some(snapshot) = pending.take() {
                published = snapshot;
                true
            } else {
                false
            };
            set_input(&mut app, entity, clock, Value::Gate(clock_level));
            set_input(&mut app, entity, reset, Value::Gate(reset_level));
            set_input(&mut app, entity, move_step, Value::Gate(move_level));
            set_input(&mut app, entity, modulation, Value::Bipolar(0.0));
            app.update();

            let clock_edge = clock_level && !previous_clock;
            let reset_edge = reset_level && !previous_reset;
            let move_edge = move_level && !previous_move;
            previous_clock = clock_level;
            previous_reset = reset_level;
            previous_move = move_level;
            if reset_edge {
                reference.reset();
                pending = None;
            } else if move_edge {
                pending = Some(reference.move_step());
            } else if clock_edge {
                pending = Some(reference.tick());
            }

            let actual = deep_observation(app.world(), entity);
            assert_eq!(actual.pending, pending, "pending at sample {sample}");
            assert_eq!(
                actual.step_count,
                reference.step_count(),
                "count at sample {sample}"
            );
            assert_eq!(
                actual.register_bits,
                reference.register_bits(),
                "register at sample {sample}"
            );
            assert_eq!(actual.length, reference.current_length());
            assert_eq!(actual.write_probability, reference.write_probability());
            assert_eq!(
                actual.ports,
                values_for_snapshot(&published, published_step),
                "published sample {sample}"
            );
        }
    }

    #[test]
    fn standalone_move_and_reset_actions_work_without_competing_edges() {
        let (mut app, entity) = app_with_tm(&[
            ("length", ParameterValue::Int(8)),
            ("root", ParameterValue::Int(5)),
        ]);
        let clock = module_input(&app, entity, "clock");
        let reset = module_input(&app, entity, "reset");
        let move_step = module_input(&app, entity, "move");
        let mut reference = TuringMachine::with_seed(
            oxurack_core::ModuleSpawnContext::new(0, "tm").instance_seed(),
        );
        reference.set_length(8);
        reference.set_root(5);

        set_input(&mut app, entity, move_step, Value::Gate(true));
        app.update();
        assert_eq!(
            deep_observation(app.world(), entity).pending,
            Some(reference.move_step())
        );
        assert_eq!(deep_observation(app.world(), entity).step_count, 0);

        set_input(&mut app, entity, move_step, Value::Gate(false));
        app.update();
        let before_reset = deep_observation(app.world(), entity);
        set_input(&mut app, entity, reset, Value::Gate(true));
        app.update();
        let after_reset = deep_observation(app.world(), entity);
        reference.reset();
        assert_eq!(after_reset.pending, None);
        assert_eq!(after_reset.step_count, reference.step_count());
        assert_eq!(after_reset.register_bits, reference.register_bits());
        assert_eq!(after_reset.length, before_reset.length);
        assert_eq!(after_reset.length_position, before_reset.length_position);
        assert_eq!(after_reset.root, before_reset.root);
        let mut held = before_reset.ports.clone();
        held[0].1 = Value::Gate(false);
        assert_eq!(
            after_reset.ports, held,
            "reset retains published fields and clears only the step pulse"
        );

        set_input(&mut app, entity, reset, Value::Gate(false));
        app.update();
        set_input(&mut app, entity, clock, Value::Gate(true));
        app.update();
        assert_eq!(
            deep_observation(app.world(), entity).pending,
            Some(reference.tick()),
            "reset retains the existing RNG stream and settings"
        );
    }

    #[test]
    fn held_modulation_reconciles_targets_and_registered_base_edits() {
        let (mut app, entity) = app_with_tm(&[("write_probability", ParameterValue::Float(0.5))]);
        register_parameters(app.world_mut());
        let modulation = module_input(&app, entity, "write_modulation");
        let clock = module_input(&app, entity, "clock");
        for offset in [-1.0, -0.25, 0.0, 0.25, 1.0] {
            let expected = (0.5_f32 + offset).clamp(0.0, 1.0);
            set_input(&mut app, entity, modulation, Value::Bipolar(offset));
            for _ in 0..3 {
                app.update();
                assert_eq!(
                    app.world()
                        .get::<TuringMachineState>(entity)
                        .unwrap()
                        .engine
                        .write_probability(),
                    expected,
                    "held offset {offset} must not accumulate",
                );
            }
            set_input(&mut app, entity, clock, Value::Gate(false));
            app.update();
            set_input(&mut app, entity, clock, Value::Gate(true));
            app.update();
            let pending = app
                .world()
                .get::<TuringMachineState>(entity)
                .unwrap()
                .pending
                .as_ref()
                .unwrap();
            assert_eq!(
                pending.write_probability, expected,
                "snapshot at offset {offset}"
            );
            set_input(&mut app, entity, clock, Value::Gate(false));
            app.update();
        }

        set_input(&mut app, entity, modulation, Value::Bipolar(0.25));
        app.update();
        set_input(&mut app, entity, clock, Value::Gate(true));
        app.update();
        let old_pending = app
            .world()
            .get::<TuringMachineState>(entity)
            .unwrap()
            .pending
            .clone()
            .unwrap();
        assert_eq!(old_pending.write_probability, 0.75);
        registered_set(
            app.world_mut(),
            entity,
            "write_probability",
            ParameterValue::Float(0.25),
        )
        .unwrap();
        assert_eq!(
            registered_get(app.world(), entity, "write_probability").unwrap(),
            ParameterValue::Float(0.5)
        );
        assert_eq!(
            app.world()
                .get::<TuringMachineState>(entity)
                .unwrap()
                .pending
                .as_ref(),
            Some(&old_pending)
        );
        set_input(&mut app, entity, clock, Value::Gate(false));
        app.update();
        set_input(&mut app, entity, clock, Value::Gate(true));
        app.update();
        assert_eq!(
            app.world()
                .get::<TuringMachineState>(entity)
                .unwrap()
                .pending
                .as_ref()
                .unwrap()
                .write_probability,
            0.5
        );
    }

    #[test]
    fn two_real_sum_cables_drive_write_modulation_through_core_phases() {
        let (mut app, entity) = app_with_tm(&[("write_probability", ParameterValue::Float(0.5))]);
        let target = module_input(&app, entity, "write_modulation");
        let (source_a, source_b) = {
            let world = app.world_mut();
            let module_a = spawn_module_entity(world, "bipolar_test_source", "source_a");
            let module_b = spawn_module_entity(world, "bipolar_test_source", "source_b");
            let port_a = spawn_port_on_module(
                world,
                module_a,
                "out",
                PortDirection::Output,
                ValueKind::Bipolar,
                MergePolicy::Reject,
            );
            let port_b = spawn_port_on_module(
                world,
                module_b,
                "out",
                PortDirection::Output,
                ValueKind::Bipolar,
                MergePolicy::Reject,
            );
            let cable_a = Cable {
                source_port: port_a,
                target_port: target,
                transform: None,
                enabled: true,
            };
            let cable_b = Cable {
                source_port: port_b,
                target_port: target,
                transform: None,
                enabled: true,
            };
            let cable_a_entity = world.spawn(cable_a.clone()).id();
            let cable_b_entity = world.spawn(cable_b.clone()).id();
            world
                .resource_mut::<CableIndex>()
                .add_cable(cable_a_entity, &cable_a);
            world
                .resource_mut::<CableIndex>()
                .add_cable(cable_b_entity, &cable_b);
            oxurack_core::mark_propagation_order_dirty(world);
            (port_a, port_b)
        };

        for (left, right, expected_modulation) in
            [(0.75, 0.75, 1.0), (-0.75, -0.75, -1.0), (0.25, -0.25, 0.0)]
        {
            for _ in 0..3 {
                app.world_mut().get_mut::<CurrentValue>(source_a).unwrap().0 = Value::Bipolar(left);
                app.world_mut().get_mut::<CurrentValue>(source_b).unwrap().0 =
                    Value::Bipolar(right);
                app.update();
                let merged = app
                    .world()
                    .get::<CurrentValue>(module_input(&app, entity, "write_modulation"))
                    .unwrap()
                    .0;
                assert_eq!(merged, Value::Bipolar(expected_modulation));
                assert_eq!(
                    app.world()
                        .get::<TuringMachineState>(entity)
                        .unwrap()
                        .engine
                        .write_probability(),
                    (0.5_f32 + expected_modulation).clamp(0.0, 1.0),
                );
            }
        }
    }

    #[test]
    fn pause_preserves_consumed_high_edges_pending_outputs_and_sample_time() {
        for input_name in ["clock", "reset", "move"] {
            let (mut subject, subject_entity) = app_with_tm(&[]);
            let (mut reference, reference_entity) = app_with_tm(&[]);
            let subject_input = module_input(&subject, subject_entity, input_name);
            let reference_input = module_input(&reference, reference_entity, input_name);
            set_input(
                &mut subject,
                subject_entity,
                subject_input,
                Value::Gate(true),
            );
            set_input(
                &mut reference,
                reference_entity,
                reference_input,
                Value::Gate(true),
            );
            subject.update();
            reference.update();
            assert_eq!(
                deep_observation(subject.world(), subject_entity),
                deep_observation(reference.world(), reference_entity),
            );
            let before_pause = deep_observation(subject.world(), subject_entity);
            let before_time = *subject.world().resource::<oxurack_core::SampleTime>();
            subject
                .world_mut()
                .resource_mut::<oxurack_core::SimulationRunning>()
                .0 = false;
            for _ in 0..4 {
                subject.update();
            }
            assert_eq!(
                *subject.world().resource::<oxurack_core::SampleTime>(),
                before_time
            );
            assert_eq!(
                deep_observation(subject.world(), subject_entity),
                before_pause
            );

            subject
                .world_mut()
                .resource_mut::<oxurack_core::SimulationRunning>()
                .0 = true;
            subject.update();
            reference.update();
            assert_eq!(
                deep_observation(subject.world(), subject_entity),
                deep_observation(reference.world(), reference_entity),
                "resume with {input_name} held high must not invent an edge",
            );
            subject.update();
            reference.update();
            assert_eq!(
                deep_observation(subject.world(), subject_entity),
                deep_observation(reference.world(), reference_entity),
                "continued run with {input_name} held high",
            );
        }
    }

    #[test]
    fn module_has_all_cached_ports() {
        let mut world = World::new();
        let entity = TuringMachineModule::spawn(&mut world, "tm", &HashMap::new()).unwrap();
        let port_count = {
            let mut ports = QueryState::<&Port>::new(&mut world);
            ports.iter(&world).count()
        };
        let state = world.get::<TuringMachineState>(entity).unwrap();
        assert_eq!(port_count, 32);
        assert_eq!(state.ports.pulses.len(), 6);
        assert_eq!(state.ports.gates.len(), 8);
    }

    fn entity_counts(world: &mut World) -> (usize, usize, usize) {
        let mut modules = QueryState::<&Module>::new(world);
        let mut ports = QueryState::<&Port>::new(world);
        let mut entities = QueryState::<Entity>::new(world);
        (
            modules.iter(world).count(),
            ports.iter(world).count(),
            entities.iter(world).count(),
        )
    }
}
