# Arc12 Slice03 — graph composition and live rewiring

Version1.0 — 2026-10-12. Status: open; initial implementation issued by CRC Sofie.
Current assignment: arc12-live-rack-operations/slice03-graph-composition/cc-prompt.md.
Three contributors, Expedited Mode. Depends on Slice02 CRC accepted at20c60ef;
Arc12 approves runtime cables/validation/events/fresh graph/save/reload. Maina7ce6e6
protected. No ancestor acceptance inferred.

## Outcome, contract and boundaries

Rewire LFO→TM in a running real musical rack, inspect before a sample, save current
configuration and replay it fresh. Synchronous patch/unpatch facade plus shared
explicit core command helpers, exact occurrence selection, atomic typed failures,
logical order, correct provenance and immediate optional graph maintenance. The
required-full prompt supplies authoritative signatures/table/sequence/test recipe.

No enabled/schema field, global ID, queued command consumer, async owner, engine
checkpoint, broad PortRef/handle/lifecycle design or endpoint gate. Legacy file
validation compatibility is preserved: runtime transform/direction validation is
stricter; final Arc12/Slice04 owns loader-policy disposition. Held input values
retain existing sample-and-hold after disconnect, not implicitly zeroed.
Historical Arc04 eight rows and Arc05 cable subset retain explicit reconciliation.
Original deferred rows aren't certified simply by this child acceptance.

## Verification and artifact homes

Five G rows in ledger.md; real core schedules Update/FixedUpdate, private owner
preservation, public graph/no-graph and fresh seeded replay, native graph18-query
and getter suite, runnable rewire example and original hashes. No new checker,
coverage platform or hardware witness. CRC independently gates closure.
CC return uses closing-report.md and ledger.md, itemized intake/readback and actual
results/failures/retries. Supporting artifacts none expected; explain necessary
exceptions under artifacts/. /tmp for transient receipts. Scoped source and separate
planning commits; preserve prompt. No merge/push/release.

## Author readiness — CRC 2026-10-12

1. Source-grounded: clean20c60ef inspected core labels/ports/order/index/commands/
   cycle checker/propagation, facade owner/save/pending/Scope, graph production,
   complete native query/getter tests, actual LFO fixture and example. Slice02
   independently reproduced core+both facade modes+workspace host+replay gates.
2. Design-complete: prompt resolves public names/signatures, returned occurrence,
   active membership, exact parallel removal, supported transforms/finite bounds,
   cycle/self-loop/fan-in, max-order overflow, no-mutation errors, provenance and
   optional synchronous graph. Legacy loading and sample-and-hold explicit.
3. Guideline-applied: guide07/template and Rust API-02 SHOULD, API-23/EH-01 MUST,
   TD-03 SHOULD, AP-42/44 AVOID and AP-43 CONSIDER support borrowed facade, typed
   failures, reused ordering type, shared owner and no ECS/async abstraction leak.
4. Executable: twelve actual paths, preflight/commit sketch, short-borrow wiring,
   existing index/dirty/cycle helper, explicit command and graph integration,
   runnable200→rewire→100 recipe with compiled implementation left to CC.
5. Falsifiable: wrong-direction/transform/numeric/merge/cycle/overflow cases;
   opposed Entity/ordinal and absent ties, literal .75/held/.25 schedules, pending251
   and unchanged original continuation, real immediate graph, independent literal
   fresh configuration and original hashes. Native query witnesses reused by scope.
6. Coherent/portable: manifest exact roots/sections includes governing methods and
   inherited rows, itemized intake in existing return record; estimated50–70k
   tokens reading/source plus execution/recovery headroom. No unresolved contract
   delegated to CC or extra proof platform. Future API/lifecycle remains Slice04.

## Assignment history

| Date | Prompt | Predecessor / reason | Disposition |
| --- | --- | --- | --- |
| 2026-10-12 | cc-prompt.md | Slice02 accepted20c60ef; approved Arc12 cable increment | Initial CC implementation issued; G-01–G-05 open; return to CRC |

## Version history

- v1.0 — 2026-10-12: CRC detailed next approved product increment after metadata
  acceptance. Necessary event provenance extension and bounded runtime validation
  are explicit; no schema/scheduler or architecture/scope amendment.
