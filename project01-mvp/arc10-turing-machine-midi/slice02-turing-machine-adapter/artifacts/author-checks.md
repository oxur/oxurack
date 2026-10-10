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

## CRC corrective iteration01 authoring — 2026-10-10

Source baseline `9ba55dd0a4c8aa7d67fe26c30f878d89ead34aa2`, planning
predecessor `2eb5ac138fedf21d4185d1cc2f1dd3df84c4972d`. This is the
author's design/checker self-review; it does not independently accept a future
CC implementation. All A rows are open. The initial author record above is
preserved byte-identically under `iteration00/author-checks.md`.

Loaded framework2.6.0, engineering-methods1.16.0 authoring guide/template,
project-management iteration/closure rules, independent correction-review
rules, testing discipline and Rust2.0.0 entrypoint/complete anti-patterns.
Domain sections actually read: API-23/API-39, TD-05, DC-05. Critical source
reads cover complete adapter/public integration/fixture, engine and quantizer
getter diff, registry and core schedule/merge/Cable/spawn APIs. Omitted portions
of bounded tool output were recovered before reliance; a heading hit alone
was not treated as reading a guide. Bevy0.18.1 resource_scope signature was
checked in the installed dependency source, not inferred from memory.

The new prompt applies concrete typed error matches, strict input populations,
private-state observations, explicit borrow scopes and actual default/length/
scale precedence. Its examples are source-checked sketches, not compiled test
code. The public integration sequence retains the rack until all stop/resume/
reinstall work completes, then consumes it once. The name oracle compares
corresponding row values and independently derived renamed seed output.

The six readiness checks are recorded in the slice plan. Complete per-sample
publication, explicit conflict inputs, summed modulation and continuing seeded
lifecycle oracles address the actual review gaps. A test exposing a production
defect returns to CRC; it cannot authorize production edits. The four source
test regions, protected fixtures/dependencies and narrow CC evidence paths are
explicit. Initial prompts/reports and raw receipts remain historical.

Checker changes are bounded to the existing predicate: current assignment,
canonical close presence and optional current-return identity/source field/
six-row walk/bubble-up. Issue mode passes on the actual draft; return mode must
reject the actual historical close until CC executes. New return predicates
are exercised using a clearly labelled in-memory synthetic structural fixture,
not a fabricated doer return. Detailed receipts live in `crc-review01/`.
