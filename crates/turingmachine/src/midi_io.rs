//! Feature-gated MIDI I/O layer for the Turing Machine.
//!
//! This module is only compiled when the `midi-io` feature is enabled.
//! It wraps a [`TuringMachine`] engine and a [`midir::MidiOutputConnection`]
//! to send MIDI Note On/Off and CC messages on each tick.

use std::fmt;

use midir::MidiOutputConnection;

use crate::engine::TuringMachine;
use crate::error::Error;
use crate::outputs::StepOutputs;

/// A [`TuringMachine`] wired to a live MIDI output connection.
///
/// On each [`tick`](Self::tick) the engine advances one step and the
/// resulting note, velocity, gate, and CC values are sent as MIDI
/// messages over the wrapped [`MidiOutputConnection`].
pub struct MidiTuringMachine {
    engine: TuringMachine,
    conn_out: MidiOutputConnection,
    channel: u8,
    note_cc: Option<u8>,
    velocity_cc: Option<u8>,
    noise_cc_num: Option<u8>,
    gate_note: u8,
    last_note: Option<u8>,
}

// `MidiOutputConnection` does not implement `Debug`, so we provide a
// manual implementation that uses a placeholder for that field.
impl fmt::Debug for MidiTuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MidiTuringMachine")
            .field("engine", &self.engine)
            .field("channel", &self.channel)
            .field("note_cc", &self.note_cc)
            .field("velocity_cc", &self.velocity_cc)
            .field("noise_cc_num", &self.noise_cc_num)
            .field("gate_note", &self.gate_note)
            .field("last_note", &self.last_note)
            .field("conn_out", &"<MidiOutputConnection>")
            .finish()
    }
}

impl MidiTuringMachine {
    /// Creates a new `MidiTuringMachine`.
    ///
    /// `channel` is clamped to the valid MIDI range 0--15.  All CC
    /// routing options start disabled and `gate_note` defaults to 60
    /// (middle C).
    #[must_use]
    pub fn new(engine: TuringMachine, conn_out: MidiOutputConnection, channel: u8) -> Self {
        Self {
            engine,
            conn_out,
            channel: channel.min(15),
            note_cc: None,
            velocity_cc: None,
            noise_cc_num: None,
            gate_note: 60,
            last_note: None,
        }
    }

    /// Advances the engine by one step and sends the resulting MIDI
    /// messages over the output connection.
    ///
    /// # Signal flow
    ///
    /// 1. Tick the inner [`TuringMachine`] to obtain [`StepOutputs`].
    /// 2. Send Note Off for the previously sounding note (if any).
    /// 3. If the gate is high and a note is available, send Note On and
    ///    record the note for future Note Off.
    /// 4. If the gate is low, clear the last-note tracker.
    /// 5. Send CC messages for any configured routings.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if any MIDI send operation fails.
    pub fn tick(&mut self) -> Result<StepOutputs, Error> {
        let outputs = self.engine.tick();

        // Note Off for the previous note.
        if let Some(prev) = self.last_note {
            self.conn_out.send(&[0x80 | self.channel, prev, 0])?;
        }

        // Note On / gate tracking.
        if outputs.gate {
            if let Some(note) = outputs.note {
                let velocity = outputs.velocity.unwrap_or(64);
                self.conn_out.send(&[0x90 | self.channel, note, velocity])?;
                self.last_note = Some(note);
            }
        } else {
            self.last_note = None;
        }

        // CC routing.
        if let Some(cc_num) = self.noise_cc_num {
            self.conn_out
                .send(&[0xB0 | self.channel, cc_num, outputs.noise_cc])?;
        }
        if let Some(cc_num) = self.note_cc
            && let Some(note) = outputs.note
        {
            self.conn_out.send(&[0xB0 | self.channel, cc_num, note])?;
        }
        if let Some(cc_num) = self.velocity_cc
            && let Some(velocity) = outputs.velocity
        {
            self.conn_out
                .send(&[0xB0 | self.channel, cc_num, velocity])?;
        }

        Ok(outputs)
    }

    /// Sends a Note Off for the currently sounding note (if any) and
    /// clears the last-note tracker.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if the MIDI send operation fails.
    pub fn all_notes_off(&mut self) -> Result<(), Error> {
        if let Some(note) = self.last_note.take() {
            self.conn_out.send(&[0x80 | self.channel, note, 0])?;
        }
        Ok(())
    }

    /// Sets the MIDI channel (clamped to 0--15).
    pub fn set_channel(&mut self, channel: u8) {
        self.channel = channel.min(15);
    }

    /// Routes the noise CC output to the given MIDI CC number.
    ///
    /// The CC number is masked to the valid 7-bit range (0--127).
    pub fn route_noise_to_cc(&mut self, cc: u8) {
        self.noise_cc_num = Some(cc & 0x7F);
    }

    /// Routes the note output to the given MIDI CC number.
    ///
    /// The CC number is masked to the valid 7-bit range (0--127).
    pub fn route_note_to_cc(&mut self, cc: u8) {
        self.note_cc = Some(cc & 0x7F);
    }

    /// Routes the velocity output to the given MIDI CC number.
    ///
    /// The CC number is masked to the valid 7-bit range (0--127).
    pub fn route_velocity_to_cc(&mut self, cc: u8) {
        self.velocity_cc = Some(cc & 0x7F);
    }

    /// Returns a shared reference to the inner [`TuringMachine`] engine.
    #[must_use]
    pub fn engine(&self) -> &TuringMachine {
        &self.engine
    }

    /// Returns an exclusive reference to the inner [`TuringMachine`]
    /// engine, allowing parameter changes between ticks.
    pub fn engine_mut(&mut self) -> &mut TuringMachine {
        &mut self.engine
    }
}
