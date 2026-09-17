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
