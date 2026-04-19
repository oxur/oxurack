//! MIDI port discovery and I/O using `midir`.
//!
//! Manages connections to physical and virtual MIDI input/output ports.
//! All port operations run on the RT thread.

/// Manages open MIDI input and output port connections.
///
/// Holds the `midir` connection handles and provides methods for
/// sending and receiving MIDI messages.
#[derive(Debug)]
pub(crate) struct MidiPorts {
    _private: (),
}

impl MidiPorts {
    /// Opens MIDI ports according to the provided configuration.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error::MidiInit`] if the MIDI subsystem cannot be
    /// initialized, or [`crate::Error::PortNotFound`] if a configured port
    /// name does not match any available port.
    pub(crate) fn open(
        _input_configs: &[crate::MidiInputConfig],
        _output_configs: &[crate::MidiOutputConfig],
    ) -> Result<Self, crate::Error> {
        unimplemented!()
    }

    /// Sends a MIDI message on the given output port index.
    ///
    /// # Errors
    ///
    /// Returns an error if the port index is out of range or the port
    /// has been disconnected.
    pub(crate) fn send(
        &mut self,
        _output_port_index: u8,
        _message: &crate::MidiMessage,
    ) -> Result<(), crate::Error> {
        unimplemented!()
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
    unimplemented!()
}

/// Lists the names of all available MIDI input ports on the system.
///
/// # Errors
///
/// Returns [`crate::Error::MidiInit`] if the MIDI subsystem cannot be
/// queried.
pub fn list_midi_input_ports() -> Result<Vec<String>, crate::Error> {
    unimplemented!()
}
