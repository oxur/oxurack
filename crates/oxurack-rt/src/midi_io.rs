//! MIDI port discovery and I/O using `midir`.
//!
//! Manages connections to physical and virtual MIDI input and output
//! ports. Output port operations run directly on the RT thread; input
//! ports use `midir`'s callback threads to capture events into per-port
//! `rtrb` queues, which the RT thread drains each iteration.

/// Manages open MIDI output port connections.
///
/// Holds the `midir` connection handles and provides methods for
/// sending raw MIDI bytes to connected output ports. Tracks port
/// health via `port_lost` flags: once a send fails on a port, it is
/// marked as lost and all subsequent sends to that port are skipped.
pub(crate) struct MidiPorts {
    outputs: Vec<midir::MidiOutputConnection>,
    /// Per-port lost flag. When `true`, sends to that port are skipped.
    port_lost: Vec<bool>,
}

impl std::fmt::Debug for MidiPorts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MidiPorts")
            .field("output_count", &self.outputs.len())
            .field("ports_lost", &self.port_lost.iter().filter(|&&v| v).count())
            .finish()
    }
}

impl MidiPorts {
    /// Opens MIDI output connections matching the given configs.
    ///
    /// Each config's `name` is matched by case-insensitive substring
    /// against the available system MIDI output port names. A new
    /// `MidiOutput` client is created for each connection because
    /// `midir::MidiOutput::connect` consumes the client.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::MidiInit`] if the MIDI subsystem cannot be
    /// initialized, or [`crate::Error::PortNotFound`] if a configured port
    /// name does not match any available port.
    pub(crate) fn open_outputs(configs: &[crate::MidiOutputConfig]) -> Result<Self, crate::Error> {
        if configs.is_empty() {
            return Ok(Self {
                outputs: Vec::new(),
                port_lost: Vec::new(),
            });
        }

        let mut outputs = Vec::with_capacity(configs.len());

        for config in configs {
            // Create a temporary MidiOutput to enumerate ports.
            let enumerator = midir::MidiOutput::new("oxurack-rt-enum")
                .map_err(|e| crate::Error::MidiInit(e.to_string()))?;

            let ports = enumerator.ports();
            let target_lower = config.name.to_lowercase();

            let port = ports
                .iter()
                .find(|p| {
                    enumerator
                        .port_name(p)
                        .map(|name| name.to_lowercase().contains(&target_lower))
                        .unwrap_or(false)
                })
                .ok_or_else(|| crate::Error::PortNotFound {
                    name: config.name.clone(),
                })?
                .clone();

            // Create a fresh MidiOutput for the connection (connect consumes it).
            let conn_out = midir::MidiOutput::new("oxurack-rt")
                .map_err(|e| crate::Error::MidiInit(e.to_string()))?;

            let connection = conn_out
                .connect(&port, &config.name)
                .map_err(|e| crate::Error::MidiInit(e.to_string()))?;

            outputs.push(connection);
        }

        let port_lost = vec![false; outputs.len()];
        Ok(Self { outputs, port_lost })
    }

    /// Sends raw MIDI bytes to the port at the given index.
    ///
    /// If the port was previously marked as lost (due to a prior send
    /// failure), the send is silently skipped and the same error is
    /// returned. Once a port is lost, it remains lost until the
    /// runtime is restarted (future versions will support
    /// re-enumeration).
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::PortNotFound`] if `port_index` is out of
    /// bounds or the port has been marked as lost, or
    /// [`crate::Error::MidiInit`] if the send fails (e.g. the port has
    /// been disconnected).
    pub(crate) fn send(&mut self, port_index: u8, bytes: &[u8]) -> Result<(), crate::Error> {
        let idx = port_index as usize;

        // Check bounds.
        if idx >= self.outputs.len() {
            return Err(crate::Error::PortNotFound {
                name: format!("index {port_index}"),
            });
        }

        // Skip sends to lost ports.
        if self.port_lost[idx] {
            return Err(crate::Error::PortNotFound {
                name: format!("index {port_index} (lost)"),
            });
        }

        if let Err(e) = self.outputs[idx].send(bytes) {
            self.port_lost[idx] = true;
            return Err(crate::Error::MidiInit(e.to_string()));
        }

        Ok(())
    }

    /// Returns `true` if the port at the given index has been marked as
    /// lost due to a prior send failure.
    #[cfg(test)]
    pub(crate) fn is_port_lost(&self, port_index: u8) -> bool {
        self.port_lost
            .get(port_index as usize)
            .copied()
            .unwrap_or(true)
    }
}

/// Lists the names of all available MIDI output ports on the system.
///
/// Useful for configuration UIs that let the user select which port
/// to connect to.
///
/// # Errors
///
/// Returns [`crate::Error::MidiInit`] if the MIDI subsystem cannot be
/// queried.
pub fn list_midi_output_ports() -> Result<Vec<String>, crate::Error> {
    let midi_out = midir::MidiOutput::new("oxurack-rt-enum")
        .map_err(|e| crate::Error::MidiInit(e.to_string()))?;
    let ports = midi_out.ports();
    let names: Vec<String> = ports
        .iter()
        .filter_map(|p| midi_out.port_name(p).ok())
        .collect();
    Ok(names)
}

/// Lists the names of all available MIDI input ports on the system.
///
/// # Errors
///
/// Returns [`crate::Error::MidiInit`] if the MIDI subsystem cannot be
/// queried.
pub fn list_midi_input_ports() -> Result<Vec<String>, crate::Error> {
    let midi_in = midir::MidiInput::new("oxurack-rt-enum")
        .map_err(|e| crate::Error::MidiInit(e.to_string()))?;
    let ports = midi_in.ports();
    let names: Vec<String> = ports
        .iter()
        .filter_map(|p| midi_in.port_name(p).ok())
        .collect();
    Ok(names)
}

/// Raw MIDI event from the `midir` callback, before classification.
///
/// Transferred from the callback thread to the RT thread via an
/// internal per-port SPSC queue. Kept deliberately small and `Copy`
/// to minimise overhead in the callback.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RawMidiEvent {
    /// Index of the input port that received this event.
    pub(crate) port_index: u8,
    /// Monotonic timestamp in nanoseconds (from [`crate::timing::MonotonicClock`])
    /// captured at the moment the callback fires.
    pub(crate) timestamp_ns: u64,
    /// Raw MIDI bytes (up to 3; zero-padded if shorter).
    pub(crate) bytes: [u8; 3],
    /// Number of valid bytes in `bytes` (1, 2, or 3).
    pub(crate) length: u8,
}

/// Manages open MIDI input port connections.
///
/// Each configured input port gets its own `midir::MidiInputConnection`
/// and a dedicated `rtrb` queue. The `midir` callback writes
/// [`RawMidiEvent`]s into the producer end; the RT thread drains from
/// the consumer end via [`MidiInputPorts::drain_all`].
///
/// Dropping this struct closes all input connections.
pub(crate) struct MidiInputPorts {
    /// Kept alive to hold the connections open; dropping closes the port.
    _connections: Vec<midir::MidiInputConnection<()>>,
    /// One consumer per input port, polled by the RT thread.
    consumers: Vec<rtrb::Consumer<RawMidiEvent>>,
}

impl std::fmt::Debug for MidiInputPorts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MidiInputPorts")
            .field("input_count", &self._connections.len())
            .finish()
    }
}

impl MidiInputPorts {
    /// Opens MIDI input connections matching the given configs.
    ///
    /// Each config's `name` is matched by case-insensitive substring
    /// against available system MIDI input port names. A dedicated
    /// per-port `rtrb` queue (capacity 256) is created so that the
    /// `midir` callback can push events without contention.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::MidiInit`] if the MIDI subsystem cannot be
    /// initialized, or [`crate::Error::PortNotFound`] if a configured port
    /// name does not match any available port.
    pub(crate) fn open(configs: &[crate::MidiInputConfig]) -> Result<Self, crate::Error> {
        if configs.is_empty() {
            return Ok(Self {
                _connections: Vec::new(),
                consumers: Vec::new(),
            });
        }

        let mut connections = Vec::with_capacity(configs.len());
        let mut consumers = Vec::with_capacity(configs.len());

        for (port_index, config) in configs.iter().enumerate() {
            // Create a per-port queue (256 slots is plenty for MIDI rates).
            let (mut producer, consumer) = rtrb::RingBuffer::new(256);
            consumers.push(consumer);

            // Create a MidiInput to enumerate available ports.
            let midi_in = midir::MidiInput::new("oxurack-rt-in-enum")
                .map_err(|e| crate::Error::MidiInit(e.to_string()))?;

            let ports = midi_in.ports();
            let target_lower = config.name.to_lowercase();

            let port = ports
                .iter()
                .find(|p| {
                    midi_in
                        .port_name(p)
                        .map(|name| name.to_lowercase().contains(&target_lower))
                        .unwrap_or(false)
                })
                .ok_or_else(|| crate::Error::PortNotFound {
                    name: config.name.clone(),
                })?
                .clone();

            // Create a fresh MidiInput for the connection (connect consumes it).
            let conn_in = midir::MidiInput::new("oxurack-rt-in")
                .map_err(|e| crate::Error::MidiInit(e.to_string()))?;

            let idx = port_index as u8;
            // Each callback gets its own MonotonicClock for consistent timestamping.
            let callback_clock = crate::timing::MonotonicClock::new();

            let connection = conn_in
                .connect(
                    &port,
                    &config.name,
                    move |_timestamp_us, data, _| {
                        // Minimal work in callback: timestamp + copy bytes + push.
                        let mut bytes = [0u8; 3];
                        let len = data.len().min(3);
                        bytes[..len].copy_from_slice(&data[..len]);

                        let event = RawMidiEvent {
                            port_index: idx,
                            timestamp_ns: callback_clock.now(),
                            bytes,
                            length: len as u8,
                        };
                        // Drop on overflow — better to lose a message than block.
                        let _ = producer.push(event);
                    },
                    (),
                )
                .map_err(|e| crate::Error::MidiInit(e.to_string()))?;

            connections.push(connection);
        }

        Ok(Self {
            _connections: connections,
            consumers,
        })
    }

    /// Drains all pending raw MIDI events from all input port queues.
    ///
    /// Call this from the RT thread loop each iteration. The returned
    /// iterator is allocation-free: it pops directly from the ring
    /// buffers.
    pub(crate) fn drain_all(&mut self) -> impl Iterator<Item = RawMidiEvent> + '_ {
        self.consumers
            .iter_mut()
            .flat_map(|consumer| std::iter::from_fn(move || consumer.pop().ok()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_no_ports_succeeds() {
        let result = MidiPorts::open_outputs(&[]);
        assert!(result.is_ok(), "opening with no configs should succeed");
        let ports = result.unwrap();
        assert!(ports.outputs.is_empty());
        assert!(ports.port_lost.is_empty());
    }

    #[test]
    fn test_open_nonexistent_port_returns_error() {
        let configs = vec![crate::MidiOutputConfig {
            name: "__nonexistent_test_port__".to_string(),
        }];
        let result = MidiPorts::open_outputs(&configs);
        assert!(result.is_err(), "opening a nonexistent port should fail");
        let err = result.unwrap_err();
        match &err {
            crate::Error::PortNotFound { name } => {
                assert_eq!(name, "__nonexistent_test_port__");
            }
            other => panic!("expected PortNotFound, got: {other}"),
        }
    }

    #[test]
    fn test_list_output_ports() {
        let result = list_midi_output_ports();
        assert!(
            result.is_ok(),
            "listing output ports should succeed: {result:?}"
        );
    }

    #[test]
    fn test_list_input_ports() {
        let result = list_midi_input_ports();
        assert!(
            result.is_ok(),
            "listing input ports should succeed: {result:?}"
        );
    }

    // ── Input port tests ────────────────────────────────────────────

    #[test]
    fn test_open_input_no_ports_succeeds() {
        let result = MidiInputPorts::open(&[]);
        assert!(
            result.is_ok(),
            "opening with no input configs should succeed"
        );
        let mut ports = result.unwrap();
        // drain_all on an empty set of consumers yields nothing.
        assert_eq!(ports.drain_all().count(), 0);
    }

    #[test]
    fn test_send_out_of_bounds_returns_port_not_found() {
        let mut ports = MidiPorts::open_outputs(&[]).unwrap();
        let result = ports.send(0, &[0xF8]);
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::Error::PortNotFound { name } => {
                assert!(name.contains("index 0"), "expected index in name: {name}");
            }
            other => panic!("expected PortNotFound, got: {other}"),
        }
    }

    #[test]
    fn test_port_lost_tracked_on_empty() {
        let ports = MidiPorts::open_outputs(&[]).unwrap();
        // No ports exist, so is_port_lost should return true (out of bounds).
        assert!(ports.is_port_lost(0));
    }

    #[test]
    fn test_open_input_nonexistent_returns_error() {
        let configs = vec![crate::MidiInputConfig {
            name: "__nonexistent_test_port__".to_string(),
        }];
        let result = MidiInputPorts::open(&configs);
        assert!(
            result.is_err(),
            "opening a nonexistent input port should fail"
        );
        let err = result.unwrap_err();
        match &err {
            crate::Error::PortNotFound { name } => {
                assert_eq!(name, "__nonexistent_test_port__");
            }
            other => panic!("expected PortNotFound, got: {other}"),
        }
    }

    // ── Debug impl tests ───────────────────────────────────────────

    #[test]
    fn test_midi_ports_debug() {
        let ports = MidiPorts::open_outputs(&[]).unwrap();
        let debug = format!("{ports:?}");
        assert!(
            debug.contains("output_count: 0"),
            "expected 'output_count: 0' in debug output, got: {debug}"
        );
    }

    #[test]
    fn test_midi_input_ports_debug() {
        let ports = MidiInputPorts::open(&[]).unwrap();
        let debug = format!("{ports:?}");
        assert!(
            debug.contains("input_count: 0"),
            "expected 'input_count: 0' in debug output, got: {debug}"
        );
    }

    // ── Port-lost tracking ─────────────────────────────────────────

    #[test]
    fn test_port_lost_out_of_bounds_returns_true() {
        let ports = MidiPorts::open_outputs(&[]).unwrap();
        // Out-of-bounds index should return true (treated as lost).
        assert!(ports.is_port_lost(0));
        assert!(ports.is_port_lost(255));
    }
}
