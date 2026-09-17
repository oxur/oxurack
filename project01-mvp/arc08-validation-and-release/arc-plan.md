# Validation and release

Status: planned outline; reconstructed 2026-09-17.

## Capability and dependencies

The assembled rack meets deterministic, timing, resource and musical-use requirements.

Dependencies: Composition of arcs01–07; physical MIDI endpoints and an Operator recording session.

## Document-backed slice breakdown

No historical slice document to move here. Candidate work remains at arc level until selected.

Slice numbering records the historical order within this arc. Foundation precedes
dependent work; an existing document is not a newly issued assignment.

## Remaining work and boundaries

Outline only: current workspace checks and inherited coverage target; deterministic saved patches in fresh processes; feature combinations; master/slave hardware jitter; 10/50/100-module benchmarks; 24-hour soak; module docs/examples; errors and Raw(u16) usage; real DAW session. Reconcile public docs before any approved release. No tag, publication, or acceptance is implied by this migration.

The [project roadmap](../project-plan.md) and [backlog](../artifacts/status-and-backlog.md)
govern current interpretation. Historical docs retain their original text and links.
This migration neither reruns implementation validation nor closes this arc.
The sibling [ledger](ledger.md) records the composition checks to establish on resumption.

## Version History

- v1.0 — 2026-09-17: Reconstructed capability and document ownership during the
  Operator-authorized planning migration; future slice detail deliberately deferred.

## Active breakdown — v2.0, 2026-09-17

Was: planned outline only. Now: baseline work is active before product integration.
Final capability and all previous acceptance requirements remain unchanged.

| Slice | Scope | Dependency / exit |
| --- | --- | --- |
| [01 restart baseline](slice01-restart-baseline/slice-plan.md) | Reproducible tool/build/test/feature/coverage observations, row-qualified inherited obligations and source-gap evidence | Existing source and migration; CRC accepts evidence completeness, not product fitness |
| Subsequent validation slices, detailed when ready | Capability-specific reproduction, fresh-process determinism, named hardware timing, benchmarks/soak, musical-use and documentation acceptance | CDC/Operator milestone and available implementations; no directories or assignments yet |

Baseline first is independent of whether MIDI, science-rig or Chord is selected.
A failing check is a retained finding, not authorization to repair code or weaken
a criterion. CRC routes cross-arc remediation to CDC through the Operator.
The baseline consumes historical records without depending on unearned arc closure.

### Version History — restart amendment

- v2.0 — 2026-09-17: Slice01 added because reconstructed arcs01–06 lack current
  validation. Split early baseline observation from final integrated acceptance;
  kept inherited coverage, hardware, soak and Operator requirements intact.

## Operator priority amendment — v2.1, 2026-09-17

The Operator selected **Turing Machine inside the Bevy rack producing MIDI first**,
then **Clock, LFO and Scope as a testing workbench**. This supersedes the unresolved
milestone choice above. Retain all other MVP capabilities as later work. Scope is
an explicitly selected diagnostic addition; a full TUI/GUI, Chord/state extension
or general cable-editing API is not automatically required for the first milestone.

The [dependency and execution map](../arc08-validation-and-release/slice01-restart-baseline/artifacts/turing-machine-dependency-map.md) records source-supported hidden
prerequisites: module system installation, tick/edge/phase ownership, patch seed
handoff, ordered MIDI events and note cleanup, RT lifetime/backpressure, then bounded
Scope capture and a minimal observation interface. The next issued slice remains
arc08/slice01 baseline; it now validates these TM-first dependencies. Following
baseline, CDC details the smallest prerequisite implementation slice(s), then the
adapter and MIDI-out path. CRC may not invent the cross-arc API contracts.

Historical carryover candidates remain pending independent review. Capability
ownership remains with the named domains until a recorded successor assignment;
reopening the same arc with a new slice is allowed and does not resurrect an old
prompt. No original requirement is removed by focusing this sequence.

### Version History — priority amendment

- v2.1 — 2026-09-17: Operator response during arc08/slice01 preparation selected
  TM-to-MIDI, then Clock/LFO/Scope; dependency inspection showed execution/seed/event
  contracts must precede a useful adapter. Original broader backlog retained.
