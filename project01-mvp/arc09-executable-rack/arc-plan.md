# Arc09 — executable, reproducible rack

Version 1.3 — 2026-09-18. Status: composition gate; all three slices closed,
CRC assembly ready, independent CDC verification pending.
Depends on: existing core/umbrella source and accepted arc08 baseline observations.
Blocks: arc10 TM/MIDI and arc11 Clock/LFO/Scope workbench. Replaces no original
acceptance criterion; receives the execution/seed portions of historical arcs02/05.
Governing design: [execution-contract.md](slice01-module-behavior-installation/artifacts/execution-contract.md).
Source baseline: `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
Planning predecessor: `390d2aa2c3449f8a6e4bbaff3df6c64d821ce4ea`.

## Capability and boundaries

A module installed in the rack executes once per selected simulation sample,
with predictable input/output visibility and a reproducible patch-seed context.
The implementation remains headless and preserves existing metadata-only modules.
This arc establishes prerequisites; a TM adapter, musical clock, MIDI sink,
physical MIDI, live snapshotting and full transport are subsequent capabilities.

## Slice sequence

| ID | Scope | State / dependency |
| --- | --- | --- |
| slice01-module-behavior-installation | Default behavior hook, once-per-type rack registration, recoverable kind conflict, bounded existing rustdoc repairs | Closed; six rows independently reproduced at source `6a31c0f` |
| slice02-simulation-samples | Explicit sample driver, phase/merge/observe boundary and manual/live exclusivity; maintain legacy step semantics | Closed; six rows independently reproduced at source `5c1ce2c` |
| slice03-seeded-spawn-context | Explicit pre-construction seed handoff, stable seed derivation and fresh-process replay fixtures | Closed; six rows independently reproduced at source `642313a` |

CRC may prepare slice02/03 within the governing design after acceptance of the
previous slice. Substantive API, compatibility, timing or evidence changes require
CDC through Duncan. No automatic advancement into arc10: CDC prepares its first
slice after arc09 composition review. Scope is sized so registration, scheduling
and seed changes each have independent review and recovery headroom.

## Composition and inherited work

[ledger.md](ledger.md) requires an installed two-instance module fixture, exact
sample traces across manual/live drivers, and separate-process seeded patch
reproduction. Children passing separately is insufficient. Original arc02
persistence and arc05 runtime/ergonomic obligations remain with named successors
in [successor routing](../artifacts/cdc-successors03.tsv). Any incompatible old
behavior must be disclosed before implementation, not explained away afterward.

## Version History

- v1.0 — 2026-09-17: CDC response to CRC escalation03; separates TM prerequisite
  contracts into three slices and makes known doc repairs a bounded first-slice gate.
- v1.1 — 2026-09-17: CRC independently closed slice01 and opened slice02 within
  CDC's existing sample/phase contract. Slice01's restricted MIDI/DNS findings
  resolve with host retries and do not alter arc sequence or acceptance scope.
- v1.2 — 2026-09-18: CRC independently closed slice02 after one evidence
  refinement and opened the already planned seed-context slice03. The reported
  weak initial tests required proof repair, not a scope or design amendment.
- v1.3 — 2026-09-18: CRC independently closed slice03 and assembled the
  three-slice composition packet. No structural change was found; arc closure
  and arc10 opening remain gated by independent CDC review.
