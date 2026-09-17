# RackGraph

Status: historical implementation with follow-ups; reconstructed 2026-09-17.

## Capability and dependencies

A fresh materialized rack graph supports navigation, diagnostics and parameter inspection.

Dependencies: arc02 core and arc03 notifications/labels; consumed by arc05 public API and arc07 interfaces.

## Document-backed slice breakdown

| Historical slice | Scope | Commit evidence (not current acceptance) |
| --- | --- | --- |
| [slice01-foundation](slice01-foundation/slice-plan.md) | Graph resource, maintenance and feature gates | 9a0dda4 |
| [slice02-navigation](slice02-navigation/slice-plan.md) | GraphView, labels and freshness | fc0e000 |
| [slice03-diagnostics](slice03-diagnostics/slice-plan.md) | Cycles, topology, closure and compatibility | 5c49283 |

Slice numbering records the historical order within this arc. Foundation precedes
dependent work; an existing document is not a newly issued assignment.

## Remaining work and boundaries

Getter follow-up 7a6fcdb and d760049 supplies parameters_of after these documents. No new slice directory is created for it because no in-scope standalone doc exists. Revisit CableKind only with modulation semantics; tag queries follow consumers.

The [project roadmap](../project-plan.md) and [backlog](../artifacts/status-and-backlog.md)
govern current interpretation. Historical docs retain their original text and links.
This migration neither reruns implementation validation nor closes this arc.
The sibling [ledger](ledger.md) records the composition checks to establish on resumption.

## Version History

- v1.0 — 2026-09-17: Reconstructed capability and document ownership during the
  Operator-authorized planning migration; future slice detail deliberately deferred.
