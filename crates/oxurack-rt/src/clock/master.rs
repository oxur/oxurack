//! Master clock: generates MIDI clock ticks at a configurable tempo.

use super::TickSchedule;

/// A master MIDI clock that generates ticks at a configurable tempo.
///
/// The master clock owns the tempo and produces a steady stream of
/// 24-PPQN clock pulses. It is used when this system is the clock
/// source for the MIDI network.
#[derive(Debug)]
pub struct MasterClock {
    _private: (), // prevent external construction
}

impl MasterClock {
    /// Creates a new master clock at the given initial tempo.
    ///
    /// # Arguments
    ///
    /// * `tempo_bpm` - Initial tempo in beats per minute.
    /// * `now_ns` - Current monotonic timestamp in nanoseconds to anchor the
    ///   first tick.
    pub fn new(_tempo_bpm: f64, _now_ns: u64) -> Self {
        unimplemented!()
    }

    /// Returns the schedule for the next tick without advancing state.
    ///
    /// This is used by the RT loop to determine when to sleep until.
    pub fn next_tick(&self) -> TickSchedule {
        unimplemented!()
    }

    /// Advances the clock by one tick, updating the internal beat and
    /// subdivision counters.
    ///
    /// Returns the schedule that was consumed (i.e., the tick that just
    /// fired).
    pub fn advance(&mut self) -> TickSchedule {
        unimplemented!()
    }

    /// Changes the tempo, taking effect from the next tick.
    ///
    /// The current tick's timing is not retroactively adjusted.
    ///
    /// # Arguments
    ///
    /// * `bpm` - New tempo in beats per minute.
    pub fn set_tempo(&mut self, _bpm: f64) {
        unimplemented!()
    }

    /// Resets the clock to beat 0, subdivision 0.
    ///
    /// Used when a transport Start message is issued.
    ///
    /// # Arguments
    ///
    /// * `now_ns` - Current monotonic timestamp to anchor the reset.
    pub fn reset(&mut self, _now_ns: u64) {
        unimplemented!()
    }
}
