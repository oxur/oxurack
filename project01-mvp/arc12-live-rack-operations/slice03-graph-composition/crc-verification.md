# CRC review — live graph composition

**Current verdict, 2026-10-12:** iteration01 independently accepted at
`deafdd228fb44720878c700f8ff62b2a59ddbad6`; G-01–G-05 done, Slice03 closed.
The initial failed review below is preserved. Slice04 authoring encountered the
separate lifecycle decisions in project-root `crc-escalation12.md`; no Slice04
implementation assignment or ancestor acceptance is implied.

2026-10-12, Sofie/CRC, three contributors, Expedited Mode. **Initial return is
not accepted; Slice03 stays open and iteration01 is issued.** This review found
missing required behavioral evidence, not a reproduced production defect. No
source repair, endpoint connection, scope amendment or Slice04 opening by CRC.

## Candidate and authority

Source clean at `981964bc5ebc034b42e81dd8d3222f1213c1a9bb`; planning intake clean
at `544b89d296ef7635b9a6f15f06a64b6775468277`; protected main unchanged at
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. Assignment reviewed:
`arc12-live-rack-operations/slice03-graph-composition/cc-prompt.md`, SHA-256
`28b9d8c6676860ae0b17c4b9346fe1479829e56b3a345a79b75c992abbab011e`.
Five opening rows and five CC proposed-done rows; none removed or accepted here.

Refreshed current plans, directives08/11, framework2.6.0, guide07/template,
PM closing/iteration mechanics, independent verification and Rust2.0 topic
guidance. Inspected all eleven source-diff paths and the new topology/facade/
integration tests, compared selector resolution with label.rs, and inspected
existing persistence tie-order witnesses. CC's itemized intake/readback is
attestation; coherent prose does not discharge absent execution cases below.

The preflight/owned commit sequence, transform table, checked active ordinal,
one-only removal and optional synchronous maintenance agree with the approved
design on inspection. label.rs was left unchanged: duplicated resolution and
effective-order keys currently agree. Shared extraction was an implementation
recommendation; no refactoring iteration is required merely for that choice.
Graph sorting retains endpoint prefix before ordinal/fallback. Legacy loader
compatibility and old-entity retirement remain Slice04/final Arc12 owners.

## Reproduced checks at 981964b

All commands ran in the assigned source worktree. Transient receipts:
`/var/folders/dt/kc8ndn6d3kx8c1jnxr05drsh0000gn/T/oxurack-crc-981964b-o5_6yw7v`.
These temporary logs are diagnostic support, not new durable slice artifacts.

| Command | Exit and observations |
| --- | --- |
| `cargo test --locked -p oxurack-core` | 0; 423 unit, 5 integration, 9 seeded integration, 2 doctests passed |
| `cargo test --locked -p oxurack-graph` | 0; 66 unit and 1 integration passed |
| `cargo test --locked -p oxurack --lib --test cables --test labels --test persistence --test scope --test lfo --test midi_output --example rewire-live-patch --example annotate-live-patch --example save-live-patch --example turing-machine-midi` | 0; 46 library, 3 cables, 4 labels, 2 persistence, 6 Scope, 12 LFO, 2 MIDI and 26 runner tests passed; other example harnesses 0 tests |
| Same facade command with `--no-default-features` | 0; 44 library, otherwise same counts; graph absent |
| `make test`, restricted | 2; RT 146 passed, 4 CoreMIDI initialization failures, 3 existing ignores |
| Unchanged `make test`, host-access retry | 0; full workspace passed; RT 150 passed, 3 existing ignores |
| `make lint` | 0 |
| `cargo clippy --locked -p oxurack-core -p oxurack-graph -p oxurack --all-features --all-targets -- -D warnings` | 0 |
| `RUSTDOCFLAGS='-D warnings' make docs` | 0 |
| `cargo build --locked -p oxurack --example rewire-live-patch --example turing-machine-midi` | 0 |
| `cargo build --locked -p oxurack --no-default-features --example rewire-live-patch` | 0 |
| `git diff --check` | 0 |

The restricted failures were `midi_io::tests::test_list_input_ports`,
`test_list_output_ports`, `test_open_input_nonexistent_returns_error` and
`test_open_nonexistent_port_returns_error`: CoreMIDI could not initialize. The
host retry is a separate observation, not a rewrite of the failed attempt.
Existing RT priority/timing ignores and block0.1.6 future advisory remain visible.

## Executed musical recipe

Ran the actual cargo-run example with original LFO input, temporary saved RON,
then two fresh 10000-sample runner processes and Scope-view replay. Successful
commands exited0. Default example reached300 samples, occurrence0, expected
description/constant transform and graph counts8→9; no-default reached300 without
graph output. Saved RON contains the constant+.25 connection and description.
All three rewired traces agree, 82 rows, SHA-256
`f1849af1c62dd1718a37a29d97fa46cdbe4255ef9e9e12c211a7b76c202deadd`.
Scope retained10082, signals10000, generated82, sent0, overwritten0; observed
target modulation+.25. Original LFO10000 remains70 rows/hash
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`;
MIDI2000 remains13 rows/hash
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.
Missing patch, malformed patch and save-to-directory exited1 with diagnosis.
No endpoint opened; offline output is not hardware or lifecycle acceptance.

## Findings and row walk

**F-01 — G-01/G-04: required runtime parallel-order proof is absent.**
`tests/cables.rs::live_patch_unpatch_and_labels_follow_exact_occurrences` proves
occurrence shifting, labels and graph counts but never ticks parallel candidates
to assert their LastWins winner. Existing
`persistence::tests::parallel_cable_tie_order_survives_snapshot_and_fresh_load`
only saves loaded [.75,.25]; it never adds .5 then removes that runtime cable.
The required [.75,.25]→[.75,.25,.5]→[.75,.25] effective-value/save/reload sequence
is therefore missing. The raw topology fallback test removes one equal ordinal
but does not label/select the same opposed-ordinal/absent cable or test insertion
before an absent-order cable. The new graph fallback test also hardcodes allocator
order for equal ordinals instead of deriving the actual Entity comparison.

**F-02 — G-02/G-03: call-boundary failure and identity evidence is incomplete.**
`topology::tests::transforms_validate_all_supported_kind_pairs_and_numeric_edges`
mostly exercises private validation, not the public mutator's atomic failure
boundary. Cycle/fan-in/overflow tests observe selected index/dirty/message totals,
but don't cover both index directions, cached order, actual cable values or
exact event identities. Facade pending252 negative checks observe time/Scope/
admission but not graph/topology/messages/driver/running/continuation. Successful
command tests check provenance/ports but not actual created/removed cable IDs;
the explicit command→GraphMaintenance path has no actual command witness. These
are missing checks prescribed by the initial contract, not claims that mutation
currently leaks or messages are currently wrong.

**F-03 — G-04: changed-topology engine continuity is not exercised.**
`cables::tests::live_rewire_preserves_engines_scope_and_pending_sample` restores
the original Affine(.5,0), providing useful original-continuation evidence.
The constant+.25 integration test loads fresh engines; the example continues live
but does not compare LFO output to an untouched control. Neither establishes the
explicit required live constant+.25 target with unchanged LFO phase/Scope
continuation against that control. Preserve the passing same-transform test and
add the missing differing-transform comparison.

**F-04 — G-05: historical reconciliation remains too general.**
The report names eight Arc04 rows, but says whole foundation/navigation/diagnostic
reconciliation remains future work and asserts all18 queries are covered without
the requested query→native-test→observable mapping. The initial assignment makes
the concrete delivered-subset/remaining-owner walk part of this slice, while
final historical acceptance stays CDC's Arc12 gate. Complete that bounded walk
in the existing report; do not invent tests for already witnessed queries or
close historical ledgers. Arc05 lifecycle/handles/queue obligations remain Slice04.

| Row | Current disposition |
| --- | --- |
| G-01 | open; ordinary public occurrences reproduced; F-01 ordering/selector interaction remains |
| G-02 | open; inspected preflight and existing negatives pass; F-02 complete atomic boundary proof remains |
| G-03 | open; inspected provenance/immediate graph and existing tests pass; F-02 actual IDs/command maintenance/failure messages remain |
| G-04 | open; pending sample251 and fresh literal replay reproduce; F-01/F-03 interactions remain |
| G-05 | open; executable/native/hash checks reproduce; F-04 concrete inherited reconciliation remains |

## Bubble-up and next assignment

No architecture, slice breakdown or acceptance amendment is required. This is
one collected correction of the existing proof contract, iteration1/5. Source
production can remain unchanged unless the new tests reveal a real defect;
report that result before repair. The five rows, initial prompt and CC report
remain preserved. Current assignment is `cc-prompt-iteration01.md`; Slice04 and
Arc12 composition stay held. Future review may retain the independently reproduced
unchanged production/docs/build/demo claims above at this revision/date after
inspecting the test-only delta and inputs; affected tests must execute fresh.


## Iteration01 independent review — 2026-10-12

Sofie/CRC, three contributors, Expedited Mode. **Accepted, five of five rows.**
Assignment `cc-prompt-iteration01.md`, independently hashed SHA-256
`add4ddbc9aa421bd58329430af16b440b355a1a2a82ff865ece53315752f2408`.
Candidate `deafdd228fb44720878c700f8ff62b2a59ddbad6`, comprising
`09a15f8927e7340c9be9ae9f4c3ba222f5ff7ce2` and `deafdd228` after981964b;
planning intake `9b156a416d85b5ce8f6517cccbae0e4337b675c2`, both clean.
Main remains clean at `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.

Inspected the complete four-path diff: core topology.rs, graph lib.rs, facade
cables.rs and persistence.rs. Compared production prefixes before `#[cfg(test)]`
against981964b: all four byte-identical. No integration-test, example, fixture,
manifest, dependency or lockfile change. No source repair by CRC. Checked CC's
manifest/readback and new report against actual tests rather than treating its
attestation as verification. Framework2.6.0, Rust2.0 and the existing guide07
contracts remain applicable; private test observations add no public API.

### Fresh execution at deafdd2

Commands ran in the assigned implementation worktree. Transient receipts:
`/var/folders/dt/kc8ndn6d3kx8c1jnxr05drsh0000gn/T/oxurack-crc-deafdd2-9aa57kcj`.
`results.tsv` records the restricted sequence; `workspace-host.log` is the
separate successful retry. These are diagnostic logs, not durable artifacts.

| Command | CRC result |
| --- | --- |
| `cargo test --locked -p oxurack-core` | exit0; 424 unit, 5 integration, 9 seeded integration, 2 doctests passed |
| `cargo test --locked -p oxurack-graph` | exit0; 67 unit, 1 integration passed |
| `cargo test --locked -p oxurack --lib --test cables --test labels --test persistence --test scope --test lfo --test midi_output --example rewire-live-patch --example annotate-live-patch --example save-live-patch --example turing-machine-midi` | exit0; 47 library, 3 cables, 4 labels, 2 persistence, 6 Scope, 12 LFO, 2 MIDI, 26 runner tests; remaining example harnesses0 tests |
| Same facade command with `--no-default-features` | exit0; 45 library, same integration/runner counts; graph absent |
| `make test`, restricted | exit2; RT146 passed, four CoreMIDI initialization failures, three existing ignores |
| Unchanged `make test`, host-access retry | exit0; full workspace and doctests passed, RT150 passed/3 existing ignores; feature-unified core448 passed |
| `make lint` | exit0 |
| `cargo clippy --locked -p oxurack-core -p oxurack-graph -p oxurack --all-features --all-targets -- -D warnings` | exit0 |
| `git diff --check` | exit0 |

The four restricted failures are the same input/output listing and nonexistent
input/output initialization tests named in the initial review. No failure was
removed; no test ignore or warning suppression was added. The existing block0.1.6
future-incompatibility advisory remains. No hardware or MIDI endpoint was opened.

### Retained independent evidence

Retain the initial CRC's strict docs, default/no-default example builds,
300-sample actual example, saved RON, repeated82-row replay/Scope and original
fixture hashes, plus CLI error checks, **at981964b, executed2026-10-12**. They were
not rerun or relabelled fresh. This decision follows the correction-review rule:
all production code, fixtures, examples, docs, dependencies and acceptance inputs
are unchanged; the inspected delta only strengthens native tests. No new finding
undermines those claims. The new lifecycle witness below concerns already-owned
Slice04 behavior and does not negate single-load/fresh-rack replay. The current
assignment explicitly permits this retention. Affected native checks ran fresh.

### Findings resolved and final row walk

| Row | Status and independent evidence |
| --- | --- |
| G-01 | done/reproduced. Public occurrence tests plus opposed ordinal/Entity and absent-order selector tests pass. The actual LastWins schedule yields .25→.5→.25 through add/remove2; literal saved vectors and installed fresh replays preserve that sequence and existing metadata. |
| G-02 | done/reproduced. Public core failures now compare rebuilt propagation cache, cable identities/components, both index directions, values/contributions and messages. Facade pending252 failures additionally compare graph, clock, driver/running, Scope and admission; existing disabled fan-in, self-loop and order-exhaustion cases pass. Inspection confirms preflight precedes owned mutation. |
| G-03 | done/reproduced. Real core command add/remove/re-add checks actual IDs, endpoints and Command provenance; explicit GraphMaintenance removes the old edge and admits only the new one. Direct facade return-boundary graph checks and failed-call silence pass. Patch-load provenance remains unchanged and natively tested. |
| G-04 | done/reproduced. Actual subject changes Affine to constant+.25 at sample200; next target signal is .25 while its entire LFO Scope sequence through1000 matches an untouched control whose target remains dynamic. Existing pending251 single-drain/original continuation, literal Update/FixedUpdate and new runtime save/reload order tests pass. |
| G-05 | done/reproduced plus retained independent evidence. Checked the18-query mapping against actual native assertions, all current graph/facade gates passed, and retained musical/docs/build evidence has the boundary above. The ten inherited rows are individually accounted for below; no historical row is promoted by child acceptance. |

F-01–F-04 are resolved. Five opening rows, five CC proposed rows and five final
done rows; zero drops, deferrals or no-ops. The accepted close does not claim
all native observations are exhaustive or new lifecycle behavior exists.

### Historical delivered subset and remaining owner

The report's generic historical remainder language is sharpened here by actual
locked decisions and native witnesses. This is bounded reconciliation, not a
reissue of historical prompts or retrospective acceptance of those ledgers.

| Inherited row | Concrete current witness | Remaining disposition / owner |
| --- | --- | --- |
| A-04-01 | Separate optional graph crate, all18 query witnesses in CC's map, current getters/labels and synchronous cable freshness; default/no-default runs above | Unload/replacement freshness and stale map retirement: Slice04, then Arc12 assembly/CDC. Undifferentiated edges are the explicitly locked v0 design, not a missing prerequisite. |
| A-04-02 | All six historical child rows individually walked below | Final inherited composition verdict remains Arc12 CRC assembly/CDC; child acceptance does not change historical deferred rows. |
| Foundation S-01 | L1–L10: crate/features; StableDiGraph/Entity maps; push message maintenance; fresh ECS reads; synchronous GraphMaintenance; native add/remove and serde round trips for ModuleInfo/CableInfo/PortInfo | Actual incoming/outgoing map retirement and same-name replacement event order need Slice04 witnesses. Existing module-removal test covers an outgoing edge only. |
| Foundation S-02 | L6 undifferentiated CableKind deferral D-25.7 retained; ModuleId is not substituted for process-local Entity; no new wire identity/schema | CableKind stays the existing catalog/design backlog item; stale-map cleanup is Slice04. These are explicit owners, not new Slice03 work. |
| Navigation S-01 | Locked GraphView borrowing, feature-gated facade accessor, fresh display-name/description/tags/parameter builders, port/neighborhood/list and unknown/empty behavior in native tests; query map records assertions | Lifecycle removal of modules, names and edges requires Slice04. No independent historical ledger disposition until Arc12 CDC composition. |
| Navigation S-02 | Locked dedicated maintenance and endpoint ordering retained; corrected ordinal/fallback list test identifies exact IDs | Same-name replacement must not erase the new name mapping when old removal is delivered; Slice04 event order/maintenance contract. No separate navigation redesign identified. |
| Diagnostics S-01 | Native empty/dangling, chain/diamond/cycle-error, SCC/self-loop/disjoint cycles, self-inclusive closures, unknown-empty and direct ValueKind compatibility tests; deterministic cable list; getter follow-up present | No new diagnostic implementation gap identified within v0; final historical reconciliation remains Arc12 assembly/CDC. Compatibility discovery is not connection admission. |
| Diagnostics S-02 | CableKind combined-edge interpretation retained; historical missing-getter deferral is now represented by parameters_of current/missing-getter tests | CableKind backlog stays deferred; coverage/performance/release claims remain Arc08, outside this query proof. |
| A-05-01 | Actual patch/unpatch occurrence, metadata, pending MIDI/Scope continuation, save/fresh replay | Unload/replacement/reset/transport, ergonomic handles and queued-operation disposition: Slice04 under CDC decisions in escalation12. |
| A-05-02 | Delivered Slice01/02/03 API subsets and explicit unresolved lifecycle requirements | Final Arc05 API accounting is Arc12 assembly/CDC after Slice04. No transfer or waiver is enacted here. |

### Bubble-up, preservation and next action

Delivered the approved runtime cable/graph increment, including required ordering,
atomic errors, notification/graph identity, changed live phase continuity and
bounded inherited reconciliation. No production defect was exposed by iteration01;
no Slice03 acceptance amendment or roadmap restructuring is needed. Preserve both
issued prompts and CC's full closing report. Artifact inventory: none beyond the
first-class close/planning documents; validated against source/planning diff.

What worked: actual opposing order and literal LastWins values discriminate the
ordering bug; private before/after ownership records make negative cases useful;
an unchanged live LFO control distinguishes phase continuity from fresh replay.

Next-slice investigation reproduced the known replacement gap concretely: same
MIDI channel cannot reload; a later failing candidate leaves an orphan sink and
reserved channel while the old handle survives. Source/docs also disagree about
transport/reset semantics. These belong to Slice04 and need CDC's consequential
contract decisions, not source repairs or a second Slice03 correction. See
project-root crc-escalation12.md. Routine close/status changes are recorded now;
Slice04 implementation is held at that design handoff. Arc12 four rows, historical
ledgers and projectP-01–P-06 remain open/deferred at their existing gates.
