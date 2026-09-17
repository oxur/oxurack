# Public API

Status: partial implementation; reconstructed 2026-09-17.

## Capability and dependencies

A musician-facing rack API composes runtime patching, parameters, lifecycle and typed/string handles.

Dependencies: arc01 RT transport, arc02 core, arc03 notifications and arc04 graph; enables arc06 integration and arc07 live tools.

## Document-backed slice breakdown

No historical slice document to move here. Candidate work remains at arc level until selected.

Slice numbering records the historical order within this arc. Foundation precedes
dependent work; an existing document is not a newly issued assignment.

## Remaining work and boundaries

Existing umbrella bootstrap/install/load/graph surface and SetParameter dispatch are historical implementation. Remaining: runtime cable operations, queued load/save, unload, transport/reset, handles, enums and ergonomic surfaces. Candidate sequence: cable mutation; lifecycle/transport/reset; handles and value vocabulary. Detail only the next selected unit.

The [project roadmap](../project-plan.md) and [backlog](../artifacts/status-and-backlog.md)
govern current interpretation. Historical docs retain their original text and links.
This migration neither reruns implementation validation nor closes this arc.
The sibling [ledger](ledger.md) records the composition checks to establish on resumption.

## Version History

- v1.0 — 2026-09-17: Reconstructed capability and document ownership during the
  Operator-authorized planning migration; future slice detail deliberately deferred.
