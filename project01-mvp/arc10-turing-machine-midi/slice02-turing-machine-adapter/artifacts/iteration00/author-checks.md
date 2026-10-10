# CRC author checks — Slice02 initial issue, 2026-09-19

Source `a19cfd1`; planning predecessor `ef9d99f`. This is design self-review,
not implementation evidence or acceptance.

CRC inventoried all 12 `StepOutputs` fields (including both arrays), all engine
constructors/actions/setters/getters, LengthSelector and both Quantizers. The
complete adapter schema is 4 inputs + 28 outputs = 32 ports and 9 parameters.
No actual capability is silently deferred.

Core's `ParameterValue::Scale` owns root/intervals/optional name while the TM
Scale owns intervals/String name and Quantizer owns root/range. The specified
conversion preserves supported intervals and roots, rejects malformed
deserialized core values, and canonically maps absent name to `Custom`. Six
read-only engine getters plus three Quantizer getters eliminate stale shadow
state for framework getters.

The modulation design was checked against `modulate_write`'s additive behavior:
the adapter computes a desired absolute `clamp(base + held Bipolar input)` and
passes only `target - actual` to the engine. A constant modulation therefore
does not accumulate, while the existing standalone action remains unchanged.

The phase trace follows the accepted pipeline: Clock Produce sample 0 reaches
TM Consume sample 0; TM pending snapshot reaches Produce sample 1. Produce
updates all fields and step in one system iteration. Reset > move > clock plus
upfront edge-history updates prevents simultaneous lower edges replaying later.

Prompt examples cover exact getter shapes, Scale conversion, private state,
candidate application order, mutation-safe setter flow, write reconciliation,
edge priority and Option publication. Tests include fixtures where common old or
wrong policies disagree. The source fence requires no core/RT/MIDI change.

The structural checker passes assignment, six-row denominator, mappings,
source allowlist and link checks, and rejects its missing-row, stale-assignment,
missing-mapping and invalid-status controls. These are structural checks only.
