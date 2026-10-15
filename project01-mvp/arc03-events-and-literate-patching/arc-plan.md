# Events and literate patching

Status: historical implementation with follow-ups; reconstructed 2026-09-17.

## Capability and dependencies

Observable rack changes and human-readable patch metadata compose into one substrate.

Dependencies: arc02 core and persistence; consumed by arc04 graph and arc05/07 runtime interfaces.

## Document-backed slice breakdown

| Historical slice | Scope | Commit evidence (not current acceptance) |
| --- | --- | --- |
| [slice01-notifications](slice01-notifications/slice-plan.md) | Notification tier (legacy bucket 3.25) | 229b616; 229eceb; getter follow-up 7a6fcdb |
| [slice02-literate-patching](slice02-literate-patching/slice-plan.md) | Labels, tags, notes and patch round trips (legacy bucket 3.5) | a894e0a; ce10c39 |

Slice numbering records the historical order within this arc. Foundation precedes
dependent work; an existing document is not a newly issued assignment.

## Remaining work and boundaries

Runtime label editing remains. Event failure reporting, audit subscriber, ordering regression checks and literate extensions are tracked individually in the project backlog.

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
remain the scope record. Child inventory: slice01-notifications, slice02-literate-patching

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

## Current disposition — v3.0, 2026-09-17

**Closed, carried over.** [CDC verification](cdc-verification.md) independently
checked CRC's assembly and 2 child dispositions. Both A rows are deferred;
capability delivery remains unaccepted. [Successor routing](../artifacts/cdc-successors03.tsv)
assigns all inherited rows explicitly. Current successor IDs: `arc12-live-rack-operations/slice02-events-and-metadata`.
Re-entry: Before adding runtime removal/label/note mutation or claiming live metadata round trips; include original slice specifications. Historical requirements above are preserved.

### Version History — CDC closure

- v3.0 — 2026-09-17: CRC escalation03 and child carryover reviews completed the
  administrative evidence; CDC closed this work as carried over and assigned
  successors without accepting its unproved capability or weakening its criteria.


## Successor reconciliation — v3.1, 2026-10-14

[Arc12 CDC acceptance](../arc12-live-rack-operations/cdc-verification.md) accepts
current labels, direct/command/load notifications and synchronous graph composition.
Historical ledgers stay deferred. PatchUnloaded and runtime-observed common
cross-type ordering now re-enter Arc07/Slice02 session observation before public
watch/subscriber claims; source summary-last plus typed counts is not that stronger
proof. Per-note/authorship/history/tag queries also re-enter 02; expanded/hierarchical
tags and formatter/user config re-enter 03. Port labels re-enter Arc13 catalog/API
reconciliation. See directive13 and Arc07 v2.0; these explicit owners supersede
register03's single Slice02 destination without rewriting its history.

## Successor refinement — v3.2, 2026-10-14

[Directive14](../cdc-directive14.md), prompted by escalation14 after accepted
Arc07/01, supersedes the combined 02 metadata assignment: actual cross-lifetime
unload/common-order observations and exact tag search remain Arc07/02; persistent
per-note IDs/CRUD/authorship/revision history move to Arc07/05 before rich-note
claims or Arc07 closure. Configuration/tag hierarchy remains03; port labels
remain Arc13. Historical deferred rows retain their original status and evidence.
Version history: v3.2 — separate the note schema migration from session observation
while preserving explicit ownership and re-entry; no capability acceptance added.
