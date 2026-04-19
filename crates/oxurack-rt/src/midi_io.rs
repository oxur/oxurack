//! MIDI port discovery and I/O using `midir`.
//!
//! Manages connections to physical and virtual MIDI output ports.
//! All port operations run on the RT thread.

/// Manages open MIDI output port connections.
///
/// Holds the `midir` connection handles and provides methods for
/// sending raw MIDI bytes to connected output ports.
pub(crate) struct MidiPorts {
    outputs: Vec<midir::MidiOutputConnection>,
}

impl std::fmt::Debug for MidiPorts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MidiPorts")
            .field("output_count", &self.outputs.len())
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

        Ok(Self { outputs })
    }

    /// Sends raw MIDI bytes to the port at the given index.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::PortNotFound`] if `port_index` is out of
    /// bounds, or [`crate::Error::MidiInit`] if the send fails (e.g. the
    /// port has been disconnected).
    pub(crate) fn send(&mut self, port_index: u8, bytes: &[u8]) -> Result<(), crate::Error> {
        let port = self
            .outputs
            .get_mut(port_index as usize)
            .ok_or(crate::Error::PortNotFound {
                name: format!("index {port_index}"),
            })?;
        port.send(bytes)
            .map_err(|e| crate::Error::MidiInit(e.to_string()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_no_ports_succeeds() {
        let result = MidiPorts::open_outputs(&[]);
        assert!(result.is_ok(), "opening with no configs should succeed");
        let ports = result.unwrap();
        assert!(ports.outputs.is_empty());
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
    #[ignore]
    fn test_list_output_ports() {
        let result = list_midi_output_ports();
        assert!(
            result.is_ok(),
            "listing output ports should succeed: {result:?}"
        );
    }

    #[test]
    #[ignore]
    fn test_list_input_ports() {
        let result = list_midi_input_ports();
        assert!(
            result.is_ok(),
            "listing input ports should succeed: {result:?}"
        );
    }
}
