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
    use oxurack_core::{CorePlugin, ParameterRegistry, Port};

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
            _ => panic!("unknown test output {name}"),
        };
        app.world()
            .get::<CurrentValue>(port)
            .expect("output should exist")
            .0
    }

    #[test]
    fn schema_declares_complete_port_and_parameter_inventory() {
        let port_names: Vec<_> = TuringMachineModule::port_schema()
            .iter()
            .map(|schema| schema.name)
            .collect();
        assert_eq!(port_names.len(), 32);
        assert_eq!(
            port_names,
            vec![
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
            ]
        );
        assert_eq!(
            TuringMachineModule::parameter_schema()
                .iter()
                .map(|schema| schema.name)
                .collect::<Vec<_>>(),
            PARAMETER_NAMES.to_vec()
        );
        assert!(TuringMachineModule::port_schema()[1].merge_policy == MergePolicy::Sum);
        assert!(TuringMachineModule::port_schema()[0].merge_policy == MergePolicy::Reject);
    }

    #[test]
    fn unknown_keys_win_lexically_and_failed_spawns_leave_world_empty() {
        let mut world = World::new();
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
    fn invalid_setters_preserve_dynamic_state_pending_and_held_ports() {
        let (mut app, entity) = app_with_tm(&[]);
        app.update();
        let clock = module_input(&app, entity, "clock");
        set_input(&mut app, entity, clock, Value::Gate(true));
        app.update();
        let before = {
            let state = app.world().get::<TuringMachineState>(entity).unwrap();
            (
                state.engine.register_bits(),
                state.engine.step_count(),
                state.pending.is_some(),
                *app.world().get::<CurrentValue>(state.ports.step).unwrap(),
            )
        };
        let error = set_root(app.world_mut(), entity, ParameterValue::Int(12)).unwrap_err();
        assert!(matches!(
            error,
            CoreError::InvalidParameterValue { module, param, .. }
                if module == "tm" && param == "root"
        ));
        let after = {
            let state = app.world().get::<TuringMachineState>(entity).unwrap();
            (
                state.engine.register_bits(),
                state.engine.step_count(),
                state.pending.is_some(),
                *app.world().get::<CurrentValue>(state.ports.step).unwrap(),
            )
        };
        assert_eq!(after, before);

        let error = set_note_min(app.world_mut(), entity, ParameterValue::Int(127)).unwrap_err();
        assert!(matches!(
            error,
            CoreError::InvalidParameterValue { param, .. } if param == "note_min"
        ));
        let wrong = app.world_mut().spawn_empty().id();
        assert!(matches!(
            get_length(app.world(), wrong),
            Err(CoreError::UnknownModule(_))
        ));
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
    fn reset_move_clock_priority_and_modulation_are_explicit() {
        let (mut app, entity) = app_with_tm(&[]);
        app.update();
        let clock = module_input(&app, entity, "clock");
        let reset = module_input(&app, entity, "reset");
        let move_step = module_input(&app, entity, "move");
        let modulation = module_input(&app, entity, "write_modulation");
        set_input(&mut app, entity, modulation, Value::Bipolar(0.3));
        set_input(&mut app, entity, clock, Value::Gate(true));
        set_input(&mut app, entity, reset, Value::Gate(true));
        set_input(&mut app, entity, move_step, Value::Gate(true));
        app.update();
        let state = app.world().get::<TuringMachineState>(entity).unwrap();
        assert_eq!(state.engine.write_probability(), 0.8);
        assert_eq!(state.engine.step_count(), 0);
        assert!(state.pending.is_none());
        set_input(&mut app, entity, clock, Value::Gate(false));
        set_input(&mut app, entity, reset, Value::Gate(false));
        set_input(&mut app, entity, move_step, Value::Gate(false));
        app.update();
        let state = app.world().get::<TuringMachineState>(entity).unwrap();
        assert_eq!(state.engine.step_count(), 0);
        assert!(state.pending.is_none());
        set_input(&mut app, entity, modulation, Value::Bipolar(0.3));
        app.update();
        assert_eq!(
            app.world()
                .get::<TuringMachineState>(entity)
                .unwrap()
                .engine
                .write_probability(),
            0.8
        );
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

    fn entity_counts(world: &mut World) -> (usize, usize) {
        let mut modules = QueryState::<&Module>::new(world);
        let mut ports = QueryState::<&Port>::new(world);
        (modules.iter(world).count(), ports.iter(world).count())
    }
}
