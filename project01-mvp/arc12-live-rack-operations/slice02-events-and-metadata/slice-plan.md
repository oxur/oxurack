# Arc12 Slice02 — live events and metadata

Version1.1 — 2026-10-12. Status: closed; M-01–M-05 CRC accepted at20c60ef.
Current assignment: `arc12-live-rack-operations/slice02-events-and-metadata/cc-prompt.md`.
Author/reviewer CRC Sofie; CC implements; Duncan relays. Three contributors,
Expedited Mode. Depends on accepted Slice01 at
`fabfa1cb4beab5fd79f05b88d4bbec34208a1cac`; Arc12 v1.3 approves this successor.
Main remains a7ce6e6. No arc/project acceptance is inferred.

## Outcome and boundaries

Annotate an advancing Clock/LFO/TM/MIDI rack with live names, descriptions, tags
and rack notes, inspect the edits immediately, save them and replay the unchanged
musical configuration in a fresh rack. A runnable `annotate-live-patch` example
and README recipe deliver this through the public facade. Share core mutation
logic with explicit CoreCommand dispatch; preserve correct notification provenance.

In: owned label selectors/results, rack/module/cable metadata, one chosen parallel
cable by occurrence in effective CableOrder order, validated Tag sets, full-list
rack notes, exact successful events and recoverable failures, immediate graph
labels, persistence and unchanged engine/time/Scope/MIDI ownership.
Out: new schema/IDs, per-note CRUD/IDs, port labels, tag search/hierarchy, formatter
persistence, interactive CLI/REPL, async queues or new dispatch systems, lifecycle
retirement/replacement, cable mutation, dependencies, endpoint audition. Slice03/04
and Arc07/08/13 retain these owners. No edits to engine or accepted fixtures.

The required-full prompt is the detailed contract: signatures, typed error and
clear/equal policies, selector resolution, shared helper wiring, worked sketches,
real test oracles and command recipe. Private mechanics remain CC discretion;
public behavior, ownership and acceptance are resolved here, not delegated.

## Verification and exit

Five M rows in ledger.md; native core/facade tests in both feature modes, explicit
command/direct event observations, graph queries, raw readback, save/fresh reload,
state-preservation control and executable metadata-only trace. Core/patch legacy
checks retained; original Arc03 criteria reconciled individually in the closeout,
with remaining lifecycle/cross-type load limitations assigned to final Arc12
assembly/Slice04. Passing this slice alone does not discharge all old arc rows.
No new checker/probe or coverage campaign. CRC independently reproduces affected
behavior and gates before closing; CC keeps all rows open as proposed done.

Artifacts: none expected beyond source/tests/example/docs and existing close/ledger
records. Any genuinely needed durable support output belongs in this slice's
`artifacts/`, with reason recorded before creation; transient logs/files use /tmp.
CC returns closing-report.md with intake/readback, row walk, exact commands/exits,
failures/retries/unrun gates, deviations, self-review and bubble-up. Preserve issued
prompt; scoped source and separate planning commits only. No merge/push/release.

## Author readiness — CRC 2026-10-12

1. **Source-grounded:** clean fabfa1cb inspected: core label/event/errors/exports,
   PatchHandle/load/metadata and port resolution, facade load/private world/save,
   Scope/pending preservation tests, fresh graph label builders, actual fixtures,
   save example and dependency features. Slice01 native and actual replay accepted.
2. **Design-complete:** prompt resolves raw versus fallback, empty/equal writes,
   rack-only full-list notes, typed tags, active membership, parallel occurrence,
   failure precedence, event provenance and between-sample mutation. Selector is
   view-relative, not a durable global ID. No queue consumer is claimed.
3. **Guideline-applied:** current framework2.6 guide07/template and Rust2.0
   TD-03/06/07 SHOULD, API-02 SHOULD, API-23/EH-01 MUST, AP-42/44 AVOID and
   AP-43 CONSIDER applied to validated/owned types, explicit enum, concrete errors,
   no facade ECS exposure and small synchronous shared implementation.
4. **Executable:** nine explicit paths, shared helper signatures/borrow/emission
   sequence, facade and command wiring, selector sort and annotated example recipe.
   Existing save/graph paths need no change. Sketches uncompiled guidance.
5. **Falsifiable:** raw None versus formatted fallback, actual previous/new,
   equal/clear/failed event counts, opposed cable ordinal/Entity, active-handle
   exclusion, typed tag order/notes order, pending251/continuation control, and
   original70-row/hash trace distinguish plausible wrong implementations.
6. **Coherent/portable:** open set agrees with approved Arc12; required-reading
   manifest gives installed skill paths and bounded prose/source extents. Estimated
   required packet/source30–45k tokens plus implementation/self-review/recovery
   headroom; no entire historical-plan dump. Intake goes in closing-report.md.
   Five rows fit one coherent slice; no unresolved binding choice.

## Assignment history

| Date | Prompt | Predecessor / reason | Disposition |
| --- | --- | --- | --- |
| 2026-10-12 | arc12-live-rack-operations/slice02-events-and-metadata/cc-prompt.md | Slice01 accepted fabfa1cb; next approved Arc12 product increment | CC returned20c60ef/ae2c188/d8761d0; CRC accepted all five at20c60ef; original assignment preserved |

## Version history

- v1.0 — 2026-10-12: CRC detailed/issued the approved metadata/events successor
  after independent persistence closure; no new arc/scope/acceptance amendment.

## Closure and version1.1 — 2026-10-12

Independent source/native/replay acceptance in crc-verification.md. Initial CC
report is preserved. No substantive bubble-up; adjacent event rustdoc clarification
is owned by next Slice03. Arc12 composition and inherited legacy acceptance remain
open. CRC details the approved runtime cable/graph increment next.
