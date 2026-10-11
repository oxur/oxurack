//! Bounded, sample-indexed observation of selected rack ports and MIDI events.

use std::collections::{TryReserveError, VecDeque};
use std::fmt;
use std::io::{self, Write};

use bevy_app::prelude::{App, FixedUpdate};
use bevy_ecs::prelude::{ChildOf, Entity, Query, Res, ResMut, Resource};
use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_ecs::world::World;
use oxurack_core::{CurrentValue, PatchHandle, Port, SampleTime, TickPhase, Value, ValueKind};

use crate::{MidiOutputEvent, SimulationRunning};

/// Exact module instance and port names to observe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeSelector {
    /// Patch module instance name.
    pub module: String,
    /// Port name on that module instance.
    pub port: String,
}

impl ScopeSelector {
    /// Creates a selector from exact module and port names.
    pub fn new(module: impl Into<String>, port: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            port: port.into(),
        }
    }
}

/// Software delivery evidence for a copied MIDI event.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiDelivery {
    /// The rack generated the event and the runner drained it.
    Generated,
    /// A live sender accepted the event successfully.
    Sent,
}

/// One retained scalar observation or copied MIDI event.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScopeRecord {
    /// A selected port value copied after Consume in its sample.
    Signal {
        /// Zero-based rack sample index.
        sample: u64,
        /// Index into [`ScopeCapture::selectors`].
        selection: usize,
        /// Exact copied value, without coercion or sanitization.
        value: Value,
    },
    /// A MIDI event explicitly copied by the sole drain owner.
    Midi {
        /// Original generated event, including its sample index.
        event: MidiOutputEvent,
        /// Generation or successful software-send evidence.
        delivery: MidiDelivery,
    },
}

#[derive(Debug, Clone, Copy)]
struct Binding {
    entity: Entity,
    kind: ValueKind,
}

/// Bounded mixed FIFO of selected signals and explicitly copied MIDI events.
///
/// Capacity counts records, including both signal and MIDI records. When full,
/// the oldest record is discarded before each append. A capture can therefore
/// retain only part of a sample. Selector labels are stored once and signal
/// records refer to them by index.
#[derive(Resource, Debug)]
pub struct ScopeCapture {
    capacity: usize,
    overwritten: u64,
    selectors: Vec<ScopeSelector>,
    bindings: Vec<Binding>,
    records: VecDeque<ScopeRecord>,
}

impl ScopeCapture {
    /// Maximum number of mixed signal and MIDI records retained.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Total records evicted from the oldest end since configuration or clear.
    #[must_use]
    pub fn overwritten(&self) -> u64 {
        self.overwritten
    }

    /// Stable caller-ordered labels for signal selection indices.
    #[must_use]
    pub fn selectors(&self) -> &[ScopeSelector] {
        &self.selectors
    }

    /// Retained records in insertion order.
    pub fn records(&self) -> impl Iterator<Item = &ScopeRecord> {
        self.records.iter()
    }

    /// Writes retained records as CSV, including a header for an empty capture.
    ///
    /// The caller owns flushing the writer and handling its flush error.
    pub fn write_csv(&self, writer: &mut impl Write) -> io::Result<()> {
        writer.write_all(b"sample,record,module,port,value_kind,value,status,data1,data2\n")?;
        for record in &self.records {
            match *record {
                ScopeRecord::Signal {
                    sample,
                    selection,
                    value,
                } => {
                    let selector = &self.selectors[selection];
                    let (kind, value_text) = match value {
                        Value::Float(value) => ("float", value.to_string()),
                        Value::Bipolar(value) => ("bipolar", value.to_string()),
                        Value::Gate(value) => ("gate", value.to_string()),
                        Value::Raw(value) => ("raw", value.to_string()),
                        Value::Midi(_) => panic!("MIDI values cannot be Scope signal records"),
                        _ => panic!("unsupported value variant in Scope signal record"),
                    };
                    write!(writer, "{sample},signal,")?;
                    write_csv_field(writer, &selector.module)?;
                    writer.write_all(b",")?;
                    write_csv_field(writer, &selector.port)?;
                    writeln!(writer, ",{kind},{value_text},,,")?;
                }
                ScopeRecord::Midi { event, delivery } => {
                    let [status, data1, data2] = event.message.to_bytes();
                    let name = match delivery {
                        MidiDelivery::Generated => "midi_generated",
                        MidiDelivery::Sent => "midi_sent",
                    };
                    writeln!(
                        writer,
                        "{},{},,,midi,,{:02X},{:02X},{:02X}",
                        event.sample, name, status, data1, data2
                    )?;
                }
            }
        }
        Ok(())
    }

    fn push(&mut self, record: ScopeRecord) {
        if self.records.len() == self.capacity {
            self.records.pop_front();
            self.overwritten = self
                .overwritten
                .checked_add(1)
                .expect("Scope overwrite counter overflow");
        }
        self.records.push_back(record);
    }
}

/// Errors while validating or allocating a Scope capture.
#[non_exhaustive]
#[derive(Debug)]
pub enum ScopeError {
    /// The rack is running and capture configuration is frozen.
    Running,
    /// Record capacity must be positive.
    ZeroCapacity,
    /// No patch has been loaded.
    NoPatch,
    /// The same exact module/port pair appears more than once.
    DuplicateSelector { module: String, port: String },
    /// No exact module instance name exists in the loaded patch.
    UnknownModule { module: String },
    /// No exact child port name exists on the selected module.
    UnknownPort { module: String, port: String },
    /// The selected port kind cannot be captured as a scalar signal.
    UnsupportedKind {
        module: String,
        port: String,
        kind: ValueKind,
    },
    /// The selected port is missing a matching CurrentValue component.
    InvalidPortState { module: String, port: String },
    /// The bounded deque could not reserve the requested record capacity.
    Allocation(TryReserveError),
}

impl fmt::Display for ScopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running => f.write_str("Scope must be configured while the rack is stopped"),
            Self::ZeroCapacity => f.write_str("Scope record capacity must be positive"),
            Self::NoPatch => f.write_str("Scope requires a loaded patch"),
            Self::DuplicateSelector { module, port } => {
                write!(f, "duplicate Scope selector {module}.{port}")
            }
            Self::UnknownModule { module } => write!(f, "unknown Scope module {module:?}"),
            Self::UnknownPort { module, port } => {
                write!(f, "unknown Scope port {module}.{port}")
            }
            Self::UnsupportedKind { module, port, kind } => {
                write!(
                    f,
                    "unsupported Scope port kind {kind:?} for {module}.{port}"
                )
            }
            Self::InvalidPortState { module, port } => {
                write!(
                    f,
                    "invalid CurrentValue state for Scope port {module}.{port}"
                )
            }
            Self::Allocation(error) => write!(f, "cannot reserve Scope records: {error}"),
        }
    }
}

impl std::error::Error for ScopeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Allocation(error) => Some(error),
            _ => None,
        }
    }
}

pub(crate) fn install_observer(app: &mut App) {
    app.add_systems(FixedUpdate, observe_scope.in_set(TickPhase::Observe));
}

pub(crate) fn configure(
    world: &mut World,
    selectors: &[ScopeSelector],
    capacity: usize,
) -> Result<(), ScopeError> {
    if world.resource::<SimulationRunning>().0 {
        return Err(ScopeError::Running);
    }
    if capacity == 0 {
        return Err(ScopeError::ZeroCapacity);
    }
    if !world.contains_resource::<PatchHandle>() {
        return Err(ScopeError::NoPatch);
    }

    let mut owned_selectors = Vec::with_capacity(selectors.len());
    let mut bindings = Vec::with_capacity(selectors.len());
    for (index, selector) in selectors.iter().enumerate() {
        if selectors[..index]
            .iter()
            .any(|prior| prior.module == selector.module && prior.port == selector.port)
        {
            return Err(ScopeError::DuplicateSelector {
                module: selector.module.clone(),
                port: selector.port.clone(),
            });
        }
        let module_entity = world
            .get_resource::<PatchHandle>()
            .ok_or(ScopeError::NoPatch)?
            .modules
            .get(&selector.module)
            .copied()
            .ok_or_else(|| ScopeError::UnknownModule {
                module: selector.module.clone(),
            })?;
        let port_entity = {
            let mut ports = world.query::<(Entity, &Port, &ChildOf)>();
            ports
                .iter(world)
                .find(|(_, port, parent)| {
                    parent.parent() == module_entity && port.name.as_ref() == selector.port
                })
                .map(|(entity, _, _)| entity)
        }
        .ok_or_else(|| ScopeError::UnknownPort {
            module: selector.module.clone(),
            port: selector.port.clone(),
        })?;
        let port = world.get::<Port>(port_entity).expect("matched Scope port");
        let kind = port.value_kind;
        if !matches!(
            kind,
            ValueKind::Float | ValueKind::Bipolar | ValueKind::Gate | ValueKind::Raw
        ) {
            return Err(ScopeError::UnsupportedKind {
                module: selector.module.clone(),
                port: selector.port.clone(),
                kind,
            });
        }
        let current_value_matches = match world.get::<CurrentValue>(port_entity) {
            Some(value) => value.0.kind() == kind,
            None => false,
        };
        if !current_value_matches {
            return Err(ScopeError::InvalidPortState {
                module: selector.module.clone(),
                port: selector.port.clone(),
            });
        }
        owned_selectors.push(selector.clone());
        bindings.push(Binding {
            entity: port_entity,
            kind,
        });
    }

    let mut records = VecDeque::new();
    records
        .try_reserve_exact(capacity)
        .map_err(ScopeError::Allocation)?;
    world.insert_resource(ScopeCapture {
        capacity,
        overwritten: 0,
        selectors: owned_selectors,
        bindings,
        records,
    });
    Ok(())
}

pub(crate) fn disable(world: &mut World) {
    world.remove_resource::<ScopeCapture>();
}

pub(crate) fn clear(world: &mut World) {
    if let Some(mut capture) = world.get_resource_mut::<ScopeCapture>() {
        capture.records.clear();
        capture.overwritten = 0;
    }
}

pub(crate) fn record_midi(world: &mut World, event: MidiOutputEvent, delivery: MidiDelivery) {
    if let Some(mut capture) = world.get_resource_mut::<ScopeCapture>() {
        capture.push(ScopeRecord::Midi { event, delivery });
    }
}

fn observe_scope(
    capture: Option<ResMut<ScopeCapture>>,
    values: Query<&CurrentValue>,
    time: Res<SampleTime>,
) {
    let Some(mut capture) = capture else { return };
    let sample = time.last_index().expect("Observe requires a begun sample");
    for selection in 0..capture.bindings.len() {
        let (entity, kind) = {
            let binding = capture.bindings[selection];
            (binding.entity, binding.kind)
        };
        let value = values
            .get(entity)
            .expect("Scope selected port disappeared")
            .0;
        assert_eq!(value.kind(), kind, "Scope selected port changed value type");
        capture.push(ScopeRecord::Signal {
            sample,
            selection,
            value,
        });
    }
}

fn write_csv_field(writer: &mut impl Write, value: &str) -> io::Result<()> {
    if value.contains([',', '"', '\r', '\n']) {
        writer.write_all(b"\"")?;
        for character in value.chars() {
            if character == '"' {
                writer.write_all(b"\"\"")?;
            } else {
                let mut encoded = [0; 4];
                writer.write_all(character.encode_utf8(&mut encoded).as_bytes())?;
            }
        }
        writer.write_all(b"\"")
    } else {
        writer.write_all(value.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::prelude::World;
    use oxurack_core::{MergePolicy, PatchHandle, PortDirection, spawn_port_on_module};

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("injected writer failure"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn empty_capture(selectors: Vec<ScopeSelector>, capacity: usize) -> ScopeCapture {
        ScopeCapture {
            capacity,
            overwritten: 0,
            selectors,
            bindings: Vec::new(),
            records: VecDeque::new(),
        }
    }

    #[test]
    fn csv_has_header_for_empty_capture() {
        let mut output = Vec::new();
        empty_capture(Vec::new(), 1).write_csv(&mut output).unwrap();
        assert_eq!(
            output,
            b"sample,record,module,port,value_kind,value,status,data1,data2\n"
        );
    }

    #[test]
    fn csv_escapes_signal_labels() {
        let mut capture = empty_capture(vec![ScopeSelector::new("mod,\"\nule", "port\r\nname")], 1);
        capture.push(ScopeRecord::Signal {
            sample: 7,
            selection: 0,
            value: Value::Bipolar(-0.5),
        });
        let mut output = Vec::new();
        capture.write_csv(&mut output).unwrap();
        let csv = String::from_utf8(output).unwrap();
        assert!(csv.contains("7,signal,\"mod,\"\"\nule\",\"port\r\nname\",bipolar,-0.5,,,\n"));
    }

    #[test]
    fn csv_propagates_writer_errors() {
        let capture = empty_capture(Vec::new(), 1);
        let error = capture.write_csv(&mut FailingWriter).unwrap_err();
        assert_eq!(error.to_string(), "injected writer failure");
    }

    #[test]
    fn configuration_rejects_missing_and_wrong_current_values() {
        let mut world = World::new();
        world.insert_resource(SimulationRunning(false));
        let module = world.spawn_empty().id();
        let port = spawn_port_on_module(
            &mut world,
            module,
            "signal",
            PortDirection::Output,
            ValueKind::Float,
            MergePolicy::Reject,
        );
        world.insert_resource(PatchHandle {
            modules: [("module".to_owned(), module)].into_iter().collect(),
        });
        world.entity_mut(port).remove::<CurrentValue>();
        let selector = [ScopeSelector::new("module", "signal")];
        assert!(matches!(
            configure(&mut world, &selector, 2),
            Err(ScopeError::InvalidPortState { .. })
        ));
        world
            .entity_mut(port)
            .insert(CurrentValue(Value::Gate(false)));
        assert!(matches!(
            configure(&mut world, &selector, 2),
            Err(ScopeError::InvalidPortState { .. })
        ));
        assert!(world.get_resource::<ScopeCapture>().is_none());
    }
}
