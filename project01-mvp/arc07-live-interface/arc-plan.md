# Live interface

Status: planned outline; reconstructed 2026-09-17.

## Capability and dependencies

A CLI/REPL and controller mapping let a musician load, inspect and change a running rack.

Dependencies: arc04 navigation, arc05 runtime operations and usable arc06 modules.

## Document-backed slice breakdown

No historical slice document to move here. Candidate work remains at arc level until selected.

Slice numbering records the historical order within this arc. Foundation precedes
dependent work; an existing document is not a newly issued assignment.

## Remaining work and boundaries

Outline only: patch load/save CLI and clock selection; REPL parser/commands/completion/watch; nanoKONTROL2 or generic MIDI-CC configuration; tag queries and user configuration where selected. No dedicated legacy slice document exists; plan slices when this arc is near.

The [project roadmap](../project-plan.md) and [backlog](../artifacts/status-and-backlog.md)
govern current interpretation. Historical docs retain their original text and links.
This migration neither reruns implementation validation nor closes this arc.
The sibling [ledger](ledger.md) records the composition checks to establish on resumption.

## Version History

- v1.0 — 2026-09-17: Reconstructed capability and document ownership during the
  Operator-authorized planning migration; future slice detail deliberately deferred.

## Operator priority amendment — v1.1, 2026-09-17

Following audible TM integration, prioritize the smallest workbench that can
control and inspect Clock/LFO/Scope with simulation-time capture and MIDI traces.
Scope capture belongs to arc06; presentation belongs here. Full four-pane TUI,
GUI and general REPL/controller features remain later unless specifically selected.
See the [dependency map](../arc08-validation-and-release/slice01-restart-baseline/artifacts/turing-machine-dependency-map.md).

### Version History — priority amendment

- v1.1 — 2026-09-17: Operator's TM-first/workbench-second decision sets interface
  sequencing; arc08 planning exposed the need to decouple display from capture.
