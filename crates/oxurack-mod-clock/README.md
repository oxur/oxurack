# oxurack-mod-clock

`oxurack-mod-clock` is a deterministic, sample-driven musical clock. One
call to `Clock::next_sample` advances one 1 ms sample and returns the Gate
level for that sample.

The default configuration is 120 BPM, 1 pulse per quarter note (PPQN), and a
10 ms pulse width. BPM is canonicalized to 0.001 BPM resolution. A validated
configuration always leaves at least one low sample between pulses. A fresh
clock starts high on sample 0 and holds high for exactly the configured pulse
width.

Changing a configuration schedules one low separator sample before the new
configuration starts a fresh epoch. Equal canonical configurations are no-ops;
multiple successful edits coalesce to the last configuration. Invalid
configurations do not change the running clock. Pausing is owned by the caller
that decides whether to execute a sample, so a stopped clock retains its
state and continues on the next executed sample.

The crate has no rack dependencies by default. Enable the `rack` feature to
export `ClockModule`, which exposes a Gate output named `pulse` and the
parameters `bpm`, `ppqn`, and `pulse_width_ms`. The adapter runs one clock per
module instance in the supplied Produce schedule. Clock tempo is per-module;
the rack patch metadata BPM does not override an omitted Clock `bpm`
parameter.

```rust
use oxurack_mod_clock::{Clock, ClockConfig};

let config = ClockConfig::try_new(120.0, 1, 10)?;
let mut clock = Clock::new(config);
assert!(clock.next_sample());
# Ok::<(), oxurack_mod_clock::ClockError>(())
```
