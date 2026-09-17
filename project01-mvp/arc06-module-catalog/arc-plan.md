# Module catalog

Status: historical implementation with follow-ups; reconstructed 2026-09-17.

## Capability and dependencies

Standalone musical engines and rack adapters provide the inherited v1 module palette.

Dependencies: arc01 timing/I/O and arc02 module infrastructure; arc05 runtime controls as needed.

## Document-backed slice breakdown

| Historical slice | Scope | Commit evidence (not current acceptance) |
| --- | --- | --- |
| [slice01-turing-machine](slice01-turing-machine/slice-plan.md) | Standalone Turing Machine engine and its historical design | c0155c3; 355962f; 7158cb4 |

Slice numbering records the historical order within this arc. Foundation precedes
dependent work; an existing document is not a newly issued assignment.

## Remaining work and boundaries

First usable rack needs a Turing Machine Bevy adapter plus a MIDI-out module. Then Clock, Noise, LFO, Sequencer, Euclidean, Quantizer, Range, Sample-and-Hold and Arpeggiator. Original catalog is a project-level artifact because it also plans infrastructure and interfaces. MIDI-in stays post-MVP; chord/Lykn remain candidates. No absent-module slice directories are pre-created.

The [project roadmap](../project-plan.md) and [backlog](../artifacts/status-and-backlog.md)
govern current interpretation. Historical docs retain their original text and links.
This migration neither reruns implementation validation nor closes this arc.
The sibling [ledger](ledger.md) records the composition checks to establish on resumption.

## Version History

- v1.0 — 2026-09-17: Reconstructed capability and document ownership during the
  Operator-authorized planning migration; future slice detail deliberately deferred.

## Current restart disposition — v2.0, 2026-09-17

**Carryover closure proposed; arc remains unaccepted.** Was: retrospective
implementation/reconciliation with no active assignment. Operator authorized
carrying unfinished work forward on 2026-09-17. Historical requirements above
remain the scope record. Child inventory: slice01-turing-machine

Both arc ledger rows remain open. CRC first reviews any child carryover reports,
then assembles this arc's `closing-report.md` and `crc-verification.md`; CDC
independently checks the actual transfer/composition disposition in
`cdc-verification.md`. Reports are created when those checks occur. No hardware,
feature or integrated-capability acceptance is inferred from administrative closure.

The receiving reconciliation home is
[arc08/slice01](../arc08-validation-and-release/slice01-restart-baseline/slice-plan.md), R-02, with every existing ledger
row retained in its carryover register. Capability responsibility remains this
arc's named domain; CDC must assign each remaining feature to a concrete successor
arc/slice before implementation. Until then, the original criterion and source
ledger remain authoritative, owned by CDC for design and CRC for tracking.
Re-entry: restart baseline and milestone decision, then recorded successor plan.

### Version History — restart amendment

- v2.0 — 2026-09-17: Historical children and migration evidence surfaced missing
  acceptance; proposed carried-over closure and transferred reconciliation to
  arc08/slice01 without claiming delivered capability or removing scope.

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

## Current disposition — v3.0, 2026-09-17

**Closed, carried over.** [CDC verification](cdc-verification.md) independently
checked CRC's assembly and 1 child dispositions. Both A rows are deferred;
capability delivery remains unaccepted. [Successor routing](../artifacts/cdc-successors03.tsv)
assigns all inherited rows explicitly. Current successor IDs: `arc10-turing-machine-midi/slice02-turing-machine-adapter; arc11-observation-workbench/slice01-lfo; arc11-observation-workbench/slice02-scope-capture; arc13-remaining-module-catalog/slice01-catalog-reentry`.
Re-entry: TM after executable-rack contracts; workbench after M1; remaining catalog after M2 with original eleven-module inventory. Historical requirements above are preserved.

### Version History — CDC closure

- v3.0 — 2026-09-17: CRC escalation03 and child carryover reviews completed the
  administrative evidence; CDC closed this work as carried over and assigned
  successors without accepting its unproved capability or weakening its criteria.
