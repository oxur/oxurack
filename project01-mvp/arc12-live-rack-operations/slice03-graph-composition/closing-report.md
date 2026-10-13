# CC implementation record — Arc12 Slice03

## Intake (before source edits)

Issued assignment: `cc-prompt.md`, SHA-256 `28b9d8c6676860ae0b17c4b9346fe1479829e56b3a345a79b75c992abbab011e`.
Source checkout S: branch `codex/oxurack-module-behavior-installation`, clean at `20c60efa63c4fbded5c0036a3bcd10ff2c4ba4d9`. Protected `main`: clean at `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. Planning checkout W: branch `planning`, clean at `a8e7d410dc71c16b987fc59880cbf5c032e3a3c7`. Slice02 source predecessor is `20c60ef`; planning predecessor `d8761d0` is superseded by the current CRC acceptance/open set. This intake was written before source changes; no separate intake artifact was created.

All declared required extents were read completely. On the initial output truncation, the cited ranges were reread in bounded calls; the gaps are listed below. `production` means pre-`cfg(test)`. Conditional item 23 was evaluated and is not applicable: only the existing two-`PathBuf` CLI surface is used, with no parser feature or framework added. Required-data: none.

| # | Required files / extent | Intake and recovered gaps |
|---|---|---|
| 1 | S `CLAUDE.md`; W `AGENTS.md`; prompt; D `slice-plan.md`, `ledger.md` — full | Fully read. Prompt was read in contiguous chunks; intake uses live S/W heads and prompt hash above. |
| 2 | Arc12 `arc-plan.md`, `ledger.md`, Slice02 `crc-verification.md` — full | Fully read; predecessor acceptance boundaries and open Slice03 rows captured for this assignment. |
| 3 | P `project-plan.md` — required current-direction-through-current-CRC-advancement section | Required section read; appended historical assignments treated as reference only. |
| 4 | P `cdc-directive08.md` — three named sections | Assessment/decisions, remaining-prompt quality, and Slice03+ check/correction policy read. |
| 5 | P `cdc-directive11.md` — Decision2 and ordering policies | Discriminating ordering proof, append-after-ordered, and fallback read. |
| 6 | K collaboration-framework, Rust-guidelines, testing `SKILL.md` — full | All fully read, including routing and mandatory execution guidance. |
| 7 | PM guides `README.md` — full | Fully read before planning operations. |
| 8 | PM guides 02 and 03 — named sections | Artifact home/iteration preservation and planning authority/open set/iteration issue-execution sections read. |
| 9 | Engineering-methods guide 07 — named section | Required reading, CC intake and preflight section read. |
| 10 | Validation-gates and closing-slices guides — full | Both fully read. |
| 11 | Posture/ethics, structural-pulls, collaborative-rights and engineering-methodology guides — full | All fully read; output truncation was resolved by subsequent bounded reads. |
| 12 | Ledger-discipline and row-closure guides — full | Both fully read before ledger work. |
| 13 | Rust AP-42/43/44, API-02/23, EH-01, TD-03 — full rationale/examples | Required sections read and applied to validation boundary, API and error design. |
| 14 | Core `label.rs`, `cable.rs`, `error.rs`, `event.rs` — all production and named tests | Production fully read. Label active/stale resolution and ordering tests, cable transform/index tests, command dispatch and topology notification sections read. Dispatch test ranges were reread after a truncated combined output. |
| 15 | Core `patch.rs`, `port.rs`, `tick.rs`, `lib.rs` — named sections | Patch types, validation, complete cycle checker, apply/load paths, cycle and topology tests; all port production; propagation order/dirty/rebuild/propagate/merge and ordinal schedule test; exports/plugin registrations read. Cycle, load-event, and ordinal schedule ranges were reread in bounded calls. |
| 16 | Umbrella `lib.rs`, `labels.rs`, `persistence.rs`, `scope.rs` — named sections | Facade production and labels/persistence production plus pending-state, parallel-order, fallback tests read. Scope production read completely: capture records/iteration, overwrite, selector resolution, configuration and observation. |
| 17 | Graph `lib.rs` — all production and named tests | All production and navigation/diagnostic/getter test sections read. Reconciled all 18 queries; current cable list and incoming/outgoing ordering behavior recorded for implementation. |
| 18 | annotate example, labels and persistence integration tests, LFO RON fixture, umbrella Cargo.toml — full | All fully read; existing CLI, public facade, replay and feature patterns identified. |
| 19 | README recipes and Makefile targets/dependencies — named sections | Live persistence/metadata/workbench guidance and test/lint/docs/dependency targets read. |
| 20 | Arc04 plan/ledger and three child ledgers — full | Fully read. Eight inherited Arc04 rows remain deferred carryovers and must be assessed individually, not closed by this implementation. |
| 21 | Arc04 design memo and three child plans — named sections | Reconciliation/v0 scope, navigation catalog and all named locked-decision recaps read. Historical instructions remain reference-only. |
| 22 | Arc05 plan/ledger/design memo — named sections | Current re-entry and both A rows, naming decisions and cable API fragment read. Preserve `patch`/`unpatch`; broader PortRef/handle/lifecycle work remains Slice04. |
| 23 | Rust CLI argument parsing guide — conditional full | Not applicable: no parser feature beyond existing `--patch` and `--save` `PathBuf` arguments is being added. No CLI guide trigger. |

### Contract readback

G-01–G-05 cover a synchronous live topology edit, its command and provenance route, fresh optional graph queries, and preservation/replay evidence. The facade will expose borrowed-name `patch` and `unpatch` methods in both feature configurations; core shared helpers own the semantics. Add validates active `PatchHandle` membership, actual child ports, source then target direction, supported transform parameters and source/result/target kinds, target merge policy including disabled structural cables, active-graph cycles, and checked append ordering before any observable mutation. Remove resolves one exact endpoint-pair occurrence using the same `(present ordinal, ordinal, Entity)` ordering as labels, then captures its cable before mutation. Typed errors leave topology/index, dirty/cache state, metadata, graph, sample clock, driver, Scope, held values and pending MIDI unchanged.

A supplied `Some(transform)` is not sufficient proof of compatibility: different transforms support distinct source/result kinds, and finite/range constraints must be checked explicitly (in particular, an invalid Clamp may panic when applied). A disconnected input with no contributors is intentionally sample-and-hold: the existing merge stage does not write a new value when its contribution list is empty. Cable removal therefore must not force zero or clear the input; a replacement cable can take effect at the next actual sample. Successful edits dirty propagation ordering and emit one topology message with `ChangeSource`; the facade runs only optional `GraphMaintenance` synchronously before return, without ticking or updating the app. Core-only users maintain their graph explicitly. Graph cable list ordering will be endpoint key then the shared present/ordinal/entity fallback, with schema and unrelated queries unchanged.

These edits preserve handles, engines/LFO phase, running/driver state, `SampleTime`, Scope bindings/records, MIDI buffers, parameters, and existing metadata. Two-call remove/add is not transactional. Existing patch-file validation remains compatible and does not gain runtime transform validation here; edited cables remain serializable, while the legacy loader limitation and Slice04 old-entity/lifecycle cleanup remain explicit Arc12 follow-ups. CC evidence will propose completion only; G rows remain open for independent CRC disposition.

## Implementation and evidence

### Source delivery

Source commit: `981964bc5ebc034b42e81dd8d3222f1213c1a9bb` on
`codex/oxurack-module-behavior-installation`. The source worktree is clean after
the path-scoped commit. Only these changed paths were committed:

- `crates/oxurack-core/src/topology.rs`, `error.rs`, `event.rs`, `lib.rs`, `patch.rs`
- `crates/oxurack-graph/src/lib.rs`
- `crates/oxurack/src/cables.rs`, `lib.rs`
- `crates/oxurack/tests/cables.rs`
- `crates/oxurack/examples/rewire-live-patch.rs`
- `README.md`

The source allowlist entry `crates/oxurack-core/src/label.rs` was read but did
not require modification. No Cargo files, fixtures, engine/adapter, tick,
persistence or Scope production, or lifecycle code changed.

Core now exposes shared synchronous `connect_cable`/`disconnect_cable` helpers
with `ChangeSource` provenance and typed direction, transform and order errors.
Connect resolves active PatchHandle membership and actual child ports, validates
direction, transform support/numeric constraints and resulting kinds, structural
merge policy, active topology cycles and checked ordinal capacity before mutation.
It then creates one enabled cable, indexes it, marks propagation ordering dirty
and emits one `CableConnected`. Remove resolves exactly one cable with the label
selector's present-ordinal/absent-order/Entity ordering, clones its metadata,
updates the index/topology and emits one `CableDisconnected`; it can remove a
disabled or malformed legacy cable without revalidating that cable. Command
dispatch uses the same helpers and `Command` provenance; legacy `RemoveCable`
removes occurrence zero. Facade `patch`/`unpatch` works in both feature modes,
uses `DirectApi`, and synchronizes optional graph maintenance before successful
return without ticking. Failed edits do not run that maintenance.

The graph cable, incoming and outgoing views now order by endpoint key and the
shared order fallback. Tests cover exact occurrence selection, present/absent
ordinals and Entity ties, superseded topology, transforms, reject fan-in,
cycles/self-loop, overflow atomicity, event provenance, and immediate graph
freshness. Facade tests cover typed atomic failures, duplicate/one-remove label
selection, no tick, pending MIDI, Scope and engine preservation, next-sample
behavior, and saved-patch replay. Saved header, base parameters, cable membership
and snapshot ordering are asserted. README and `rewire-live-patch` document and
demonstrate a 200-sample run, remove/add, description label, save, and 100-sample
continuation; the example drains MIDI per sample and reports graph counts only
when the graph feature is enabled.

### Native validation

Commands ran in S. Final source revision for all results below is
`981964bc5ebc034b42e81dd8d3222f1213c1a9bb`.

| Command | Result |
|---|---|
| `cargo test --locked -p oxurack-core` | Pass: 423 passed, 0 failed; integration suites 5 and 9 passed; 2 doc tests passed. |
| `cargo test --locked -p oxurack-graph` | Pass: 66 unit tests and 1 integration test passed. All 18 v0 queries remain covered by the existing suite; topology ordering and query-key tests also pass. |
| `cargo test --locked -p oxurack --lib --test cables --test labels --test persistence --test scope --test lfo --test midi_output --example rewire-live-patch --example annotate-live-patch --example save-live-patch --example turing-machine-midi` | Pass after final assertions: 46 library, 3 cables, 4 labels, 2 persistence, 6 Scope, 12 LFO, 2 MIDI-output, and 26 Turing-machine example tests; all example harnesses passed. |
| Same umbrella command with `--no-default-features` | Pass after final assertions: 44 library, 3 cables, 4 labels, 2 persistence, 6 Scope, 12 LFO, 2 MIDI-output, and 26 Turing-machine example tests; all example harnesses passed. Graph remains absent. |
| `make test` in restricted execution | Preserved failure: 146 passed, 4 failed and 3 existing ignored. The four failures were CoreMIDI initialization in `test_open_input_nonexistent_returns_error`, `test_list_output_ports`, `test_open_nonexistent_port_returns_error` and `test_list_input_ports`. |
| Unchanged `make test` with host access | Pass: workspace suite passed; `oxurack-rt` had 150 passed, 0 failed and 3 existing priority/timing ignores. This is a separate authorized retry; the restricted failure remains recorded above. |
| `make lint` | Pass: repository Clippy and formatting targets. |
| `cargo clippy --locked -p oxurack-core -p oxurack-graph -p oxurack --all-features --all-targets -- -D warnings` | Pass after final assertions. |
| `RUSTDOCFLAGS='-D warnings' make docs` | Pass. |
| `cargo build --locked -p oxurack --example rewire-live-patch --example turing-machine-midi` | Pass. |
| `cargo build --locked -p oxurack --no-default-features --example rewire-live-patch` and actual no-default example run | Pass; the example completed 300 samples and reported occurrence, description, transform and save path without graph-specific output. |
| `git diff --check` | Pass before source commit; source worktree clean after commit. |

The crate's existing `block 0.1.6` future-incompatibility advisory remains
visible; no warning suppression or new ignore was added.

### Executable musical recipe and replay

The prompt's executable recipe ran from S using temporary files under
`/tmp/oxurack-rewire.slice03` (no durable artifacts added). The default example
loaded `turing-machine-lfo.ron`, advanced/drained 200 samples, removed and added
the LFO→TM cable with `Affine { factor: 0.0, offset: 0.25 }`, labeled it, saved,
and advanced/drained 100 more. Output reported `completed_samples=300`,
`occurrence=0`, the expected transform/description, and graph counts `8->9`.
The saved RON was inspected and contained the intended transform and description.

Two independent 10,000-sample replays of the saved patch were byte-identical;
their 82-line trace SHA-256 was
`f1849af1c62dd1718a37a29d97fa46cdbe4255ef9e9e12c211a7b76c202deadd`. Scope-view
replay matched that trace byte-for-byte. Scope retained 10,000 selected signal
records, 82 generated MIDI records, zero sent records, zero overwrites, and the
latest target value was Bipolar(0.25); the final 12 sampled values were .25.
The native integration test independently changes only the named original cable
transform and metadata, then verifies the saved replay and exact first-sample
modulation. The rewired sequence differs from the original LFO sequence.

Original fixture controls reproduced their prompt hashes: LFO, 10,000 samples,
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`; MIDI,
2,000 samples, `98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.
Missing patch, malformed RON, and save-to-existing-directory invocations each
returned nonzero with a diagnostic. No endpoint was opened and no hardware result
is claimed.

### Self-review and remaining contract limits

Preflight helpers build an owned candidate against the active patch topology and
do not mutate cable entities, indexes, order-dirty state, metadata, graph,
SampleTime, driver/running state, Scope, held values or pending MIDI. The commit
phase has no recoverable branch after spawn/index/dirty/message. Removal captures
the actual cable before despawn and uses the same selector ordering as labels.
The new ordinal follows present ordered cables and is checked for overflow;
unordered cable ordering and equal-ordinal Entity fallback remain intact. Exactly
one topology event is emitted for each successful operation with its actual
source. Graph freshness is synchronous at facade return when enabled; raw core
owners remain responsible for optional maintenance. No sample is advanced.

Disconnect does not write zero to a target with no contributors: existing merge
semantics preserve the input's held value. A replacement cable takes effect on
the next real sample. Two-call remove/add is explicitly not transactional. Fresh
configuration replay restarts engines and is compared to the independent literal
reference rather than the live rack continuation. Legacy patch-file validation
still treats an arbitrary `Some(transform)` as kind-mismatch accommodation and
does not perform this runtime validation; that compatibility limitation remains
for later Arc12/Slice04 disposition. Old superseded-entity cleanup, lifecycle,
handles/PortRef, queued persistence and transport/reset remain Slice04 ownership.

### Carryover walk and disposition

Slice03 provides runtime cable editing, fresh graph visibility and replay evidence;
it does not close the historical Arc04 rows or claim all historical graph,
representation or lifecycle obligations.

- **A-04-01** remains deferred: Slice03 rechecks current feature-on/off query
  freshness and the live `parameters_of` update without ticking. Broad graph v0
  reconciliation and future graph/representation ownership remain Arc12/CDC work.
- **A-04-02** remains deferred: this report records Slice03 delivery and names
  outstanding lifecycle, handle and legacy-loader boundaries; it does not assemble
  or independently accept all Arc04 composition.
- **Foundation S-01** remains deferred: current graph/core native suites pass and
  active topology behavior is covered; historical foundation behavior has not
  been independently reconciled as a whole.
- **Foundation S-02** remains deferred: current runtime mutation is assigned to
  Arc12 Slice03; future representation/handle and lifecycle work stays with
  Arc12 Slice04 or later CDC assignment.
- **Navigation S-01** remains deferred: all current graph queries and immediate
  freshness are exercised by native suites; this does not retrospectively accept
  historical navigation behavior.
- **Navigation S-02** remains deferred: runtime cable changes are assigned here;
  broader navigation semantics remain owned by Arc04/CDC until a successor
  disposition says otherwise.
- **Diagnostics S-01** remains deferred: active cycle preflight and graph tests
  pass, including the self-loop distinction; whole historical diagnostic
  reconciliation remains unaccepted.
- **Diagnostics S-02** remains deferred: new runtime cycle behavior is in Arc12
  Slice03; remaining diagnostic/representation obligations keep their historical
  owners pending the final Arc12 composition review.
- **A-05-01** remains deferred: public `patch`/`unpatch` now work; queued load/save,
  handles/PortRef, lifecycle, transport and reset remain Slice04 or later assigned
  work.
- **A-05-02** remains deferred: this report accounts for this cable-operation
  subset but does not assemble all Arc05 capability or its remaining findings.

All five Slice03 rows **G-01–G-05 remain open**. CC proposes done based on the
evidence recorded here; CRC must independently reproduce/review and record its
disposition. Slice03 does not close Arc12, Arc04 or Arc05 and does not advance
Slice04. No checks are claimed beyond the commands and executable cases above.

## Iteration01 — CRC evidence correction (2026-10-12)

### Assignment identity, intake and readback

This is correction1/5 under `cc-prompt-iteration01.md`, SHA-256
`add4ddbc9aa421bd58329430af16b440b355a1a2a82ff865ece53315752f2408`.
At intake S was clean on `codex/oxurack-module-behavior-installation` at
`981964bc5ebc034b42e81dd8d3222f1213c1a9bb`; W was on `planning` at
`36a0bd92f666b2b6faa625bcdacc2a347ae6d8dd`. Protected main remained
`a7ce6e6`. The source return is `deafdd228fb44720878c700f8ff62b2a59ddbad6`,
following `09a15f8` and `deafdd2` path-scoped commits. Planning return is recorded
below. The source worktree is clean at return.

Required reading was completed before dependent edits. The numbered manifest
was read at these extents; truncations were recovered with bounded contiguous
reads, and no required-data gaps remain:

| Manifest order | Readback |
|---|---|
| 1–3 | Full: source `CLAUDE.md`, planning `AGENTS.md`, iteration prompt, Slice03 plan/ledger/CRC record, Slice03 initial prompt binding sections, closing report, Arc12 plan/ledger. Initial report and ledger history were preserved. |
| 4–5 | Current-direction-through-CRC-advancement section of `project-plan.md`; directive08 Assessment/decisions, Prompt quality, Checks/correction; directive11 Decision2 ordering proof and fallback. |
| 6–8 | Full collaboration-framework, Rust-guidelines, testing skill entrypoints and PM guides README; named sections in PM guides 02/03 and engineering-methods guide07; full row-closure and testing-discipline/validation-gates guides; independent-verification evidence section. |
| 9 | Rust AP-42/43/44, API-02/23, EH-01 and TD-03 rationale/examples; test observations remain private and concrete, public failure calls exercise the mutator, and no generic harness/API was added. |
| 10 | Required source sections in core topology, label, cable, tick and port: all topology/tests; active/stale label resolution and occurrences; cable/order/index; propagation ordering/rebuild/merge/schedule; port values/policies. |
| 11 | Required sections in facade cables/persistence/lib/scope, cable integration tests and MIDI output: all cable facade tests, TieSource/TieSink and parallel tests, owner/lifecycle APIs, Scope records and selectors, MIDI admission/drain. |
| 12 | Graph production/query and relevant native tests, plus event dispatch/message provenance and patch load/cycle implementation/tests. |
| 13–14 | Full Arc04 plan/ledger and foundation/navigation/diagnostics ledgers; required reconciliation/design memo and each child-plan decision recap; Arc05 plan/ledger and required re-entry/cable-surface design sections. These historical artifacts were used for reconciliation, not to reactivate their old gates. |
| 15 | Full `turing-machine-lfo.ron` fixture and required Makefile validation targets. |

No CLI/async/unsafe/new trait/API/dependency/performance work occurred, so the
conditional domain loads were not applicable. No fixture or supporting artifact
was added. The exact edit fence was the four test regions listed under Source
delivery below; `crates/oxurack/tests/cables.rs` was authorized but unchanged.

The ordering key explains why Entity allocation is not the primary order: active
endpoint names/ports come first, then ordered cables sort ahead of absent-order
cables, then the explicit ordinal, and Entity breaks equal-order ties. Thus a
later-allocated cable with ordinal zero is selected before an earlier-allocated
ordinal-ten cable; an un-ordered legacy cable is not counted in the returned
append occurrence. The LFO same-transform pending-sample test proves a no-op
topology replacement preserves current state, while fresh-load tests rebuild
engines and restart phase; neither proves changed live phase preservation. The
new subject/control Scope comparison instead changes only the subject's live
transform and compares LFO records through sample 1000, while the target records
provide the changed-output oracle.

Atomicity observations now exercise the public core mutator and facade calls.
They snapshot actual cable identities/components/order, both index directions,
propagation order and dirty state after a real rebuild, source/target values and
merge contributions, event counts, graph edge IDs, running/driver/sample state,
Scope records/overwrite count, and MIDI admission. Successful direct and command
paths assert actual cable identity, endpoint IDs, transform and provenance;
graph tests explicitly run `GraphMaintenance` after real `dispatch_core_command`
calls. This establishes the raw-core owner boundary separately from the
facade's synchronous graph owner.

### Iteration01 source evidence

Only these four paths changed, all additions within `#[cfg(test)]` regions:

- `crates/oxurack-core/src/topology.rs`: real public connect failure matrix
  against missing/stale names/ports, directions, no-transform kind mismatch,
  non-finite/reversed transforms, unsupported transform source, wrong result
  kind, illegal first merge and multi-module cycle; unchanged owner snapshots
  with a rebuilt clean propagation cache and no messages. Command tests identify
  actual added/removed IDs and failed-command silence. Equal-ordinal cables now
  receive opposed Entity/ordinal assignments; cable descriptions prove which
  label occurrence is removed, then insertion before an unordered cable returns
  only the present-ordered count and labels the new identity. The MAX-order
  superseded-topology case remains intact.
- `crates/oxurack-graph/src/lib.rs`: a real core command add/remove/re-add test
  proves message IDs and command provenance, explicit graph maintenance, and
  retirement of the removed edge ID. Endpoint sorting now connects the lexical
  later source first and checks cable IDs. Ordinal/entity fallback derives the
  entity ordering inside the run while forcing ordinal order to oppose it; all
  cable/incoming/outgoing list oracles remain. Diamond topological assertions
  prove predecessor constraints rather than a unique total order.
- `crates/oxurack/src/cables.rs`: the pending-sample failures compare cable
  components/labels, both CableIndex directions, PropagationOrder and dirty,
  held values/contributions, topology messages, graph edge IDs, driver/running,
  clock, Scope and output admission after each invalid call. The changed-live
  transform test proves the next target signal is exactly Bipolar(0.25), keeps
  LFO Scope records equal to the untouched rack to 1000, and shows the control
  target remains dynamic; it does not compare intentionally changed MIDI.
- `crates/oxurack/src/persistence.rs`: the existing parallel-cable test retains
  its earlier literal snapshot/fresh-load oracle and now also exercises
  runtime `[0.75, 0.25] -> add 0.5 -> [0.75, 0.25, 0.5] -> remove occurrence 2`.
  It checks no-tick calls, actual samples `.25 -> .5 -> .25`, literal saved
  vectors, fresh installed replays, retained descriptions/transforms, and a
  second add/remove on a fresh replay.

Production-region audit: `git diff --unified=0` showed hunks only below the test
module boundaries in the three inline-test files and below `#[cfg(test)] mod
tests` in persistence. The integration test file, production APIs, fixtures,
dependencies and examples are unchanged. No production defect was exposed.

### Native validation at `deafdd228`

| Command | Result |
|---|---|
| `cargo test --locked -p oxurack-core` | Pass: 424 unit tests, 5 integration-cycle tests, 9 seeded-context tests and 2 doctests. |
| `cargo test --locked -p oxurack-graph` | Pass: 67 unit tests, 1 graph-population integration test; 0 doctests. |
| Facade default command from the initial prompt | Pass: 47 library, 3 cables, 4 labels, 12 LFO, 2 MIDI-output, 2 persistence, 6 Scope, and 26 Turing-machine example tests; all selected example harnesses pass. |
| Same facade command with `--no-default-features` | Pass: 45 library and the same selected integration/test counts above; graph remains absent. |
| `make test`, restricted run | Preserved failure: CoreMIDI initialization only in `test_open_input_nonexistent_returns_error`, `test_list_output_ports`, `test_open_nonexistent_port_returns_error`, and `test_list_input_ports`; RT had 146 pass, 4 fail, 3 existing ignored. |
| Unchanged `make test`, host-access retry | Pass: all workspace tests and doctests; `oxurack-rt` 150 passed, 0 failed, 3 existing timing/priority ignores. The restricted failure remains recorded rather than replaced. |
| `make lint` | Pass: Clippy and format checks. |
| `cargo clippy --locked -p oxurack-core -p oxurack-graph -p oxurack --all-features --all-targets -- -D warnings` | Pass. |
| `git diff --check` | Pass. |

The existing `block 0.1.6` future-incompatibility advisory remains; no warning
suppression or new ignored test was added. The late assertion tightening was
rerun in both touched focused tests and strict Clippy; the complete default and
no-default facade command and full core suite were then rerun at the final source
revision. Graph source was unchanged after its 67+1 passing package run.

### F-04 — v0 query-to-test reconciliation

This table maps the eighteen query names to native test symbols and the concrete
observation each makes. It records current witness and limits, not blanket
historical acceptance.

| Query | Native witness and assertion | Limit |
|---|---|---|
| `module_kinds` | `test_module_kinds_histogram`: literal VCO/filter counts. | Histogram fixture only. |
| `modules` | `test_modules_returns_all`, `test_modules_empty_for_empty_graph`, `test_modules_sorted_by_instance_name`: membership, empty set and name ordering. | Current graph contents only. |
| `ports_of` | `test_ports_of_returns_inputs_and_outputs`, `test_ports_of_returns_empty_for_unknown_module`: both directions/names/kinds and unknown empty result. | Does not exercise runtime port editing. |
| `parameters_of` | `test_parameters_of_returns_schema`, `test_parameters_of_includes_current_value`, `test_parameters_of_current_none_when_getter_missing`, empty-unknown and sorted-by-name cases: schema, current `.42`, missing getter `None`, and deterministic ordering. | Does not add a getter or cover every parameter type. |
| `upstream_of` | `test_upstream_of_returns_source_modules` and empty-neighborhood coverage: exact source ancestry and no-source result. | One chain fixture. |
| `downstream_of` | `test_downstream_of_returns_target_modules` and empty-neighborhood coverage: exact target descendants and no-target result. | One chain fixture. |
| `incoming_cables` | `test_incoming_cables` checks two incoming edges; `cable_lists_sort_endpoint_prefix_then_ordinals_and_entity_fallback` checks ordered entity IDs. | Count fixture plus shared ordering fixture. |
| `outgoing_cables` | `test_outgoing_cables` checks two outgoing edges; shared list test checks exact IDs. | Count fixture plus shared ordering fixture. |
| `cables` | `test_cables_returns_all` checks membership/count; `test_cables_sorted_by_endpoints` checks exact endpoint-key order and cable identities. | Does not add Signal/Modulation edge kind. |
| `dangling_modules` | Empty-graph, all-connected and isolated-module tests; isolated fixture returns `lonely_1`. | Three structural fixtures. |
| `dangling_ports` | Empty, all-wired, unwired-output and unwired-input tests identify each result class. | Minimal fixtures only. |
| `topological_order` | Linear chain asserts exact order; diamond asserts `a` before both branches and both before `d`; empty graph; cycle error names `a,b`. | `CycleError` carries no partial ordering; diamond branch tie is intentionally unspecified. |
| `cycles` | Empty/acyclic, two-node cycle, self-loop singleton SCC and multiple disjoint-cycle tests. | SCC fixtures do not claim all diagnostics semantics. |
| `downstream_closure` | Linear chain includes self/descendants; diamond includes both branches; isolated contains self. | No large-graph/performance claim. |
| `upstream_closure` | Linear chain includes self/ancestors; unknown entity returns empty. | No large-graph/performance claim. |
| `compatible_inputs_for` | Matching kind, kind mismatch exclusion, same-module candidate and unknown-port tests. | Value-kind discovery only; it does not validate fan-in, cycles or transform legality. |
| `module_by_name` | Resolves exact module/kind; missing name returns `None`. | Does not establish lifecycle retirement after unload. |
| `modules_of_kind` | VCO filter returns the two matching VCO modules. | One kind and fixture. |

No additional native query test was needed for this reconciliation. A mismatch
not delivered remains assigned to Arc04/CDC; in particular `CableKind::Signal`
versus `Modulation` stays a historical representation deferral (the graph edge
comment names D-25.7), not an implemented capability.

### F-04 — inherited row walk and retained ownership

Each row below is assessed separately. The cited current witness is not a
retroactive closure of its historical ledger.

| Historical row | Delivered subset/current witness | Specific remaining requirement and owner |
|---|---|---|
| A-04-01 | Current feature-on/off query behavior, active topology freshness, and `parameters_of` current-value/missing-getter behavior; graph package and facade variants pass. | Reconcile the complete graph-v0 requirement set and edge representation, including deferred Signal/Modulation distinction; Arc04/CDC owns any new assignment. |
| A-04-02 | This report maps all eighteen queries to actual tests and records this live-composition subset. | Assemble and independently disposition the whole Arc04 composition and historical semantics; Arc12 CRC assembly/CDC gate. |
| Foundation S-01 | Core active-topology mutators, cable index, schedule cache and graph query tests are current witnesses. | Historical foundation requirements outside this changed behavior have not been replayed as a whole; Arc04 carryover/CDC. |
| Foundation S-02 | Current synchronous live cable add/remove and label occurrence behavior are evidenced here. | Broader representations, handles and lifecycle cleanup remain outside this slice; Arc12 Slice04 or later named CDC assignment. |
| Navigation S-01 | Current navigation query test symbols/observations are mapped above. | Historical navigation ledger is not independently reconciled or accepted; Arc04 carryover/CDC. |
| Navigation S-02 | Runtime cable changes refresh the graph through explicit raw-owner maintenance and facade synchronous maintenance. | Remaining navigation semantics beyond these cable edits retain Arc04/CDC ownership. |
| Diagnostics S-01 | Cycle preflight and graph cycle tests cover two-node, self-loop and disjoint-cycle cases. | Complete historical diagnostic behavior remains open to Arc04/CDC reconciliation. |
| Diagnostics S-02 | This live cycle rejection is added to Arc12 Slice03 evidence. | Remaining diagnostic and representation obligations remain with Arc04/CDC pending the Arc12 composition gate. |
| A-05-01 | Public `patch`/`unpatch`, actual occurrence, metadata preservation, Scope and replay are evidenced. | Queued operations, handles/PortRef, unload/replacement retirement, lifecycle, transport and reset belong to Arc12 Slice04. |
| A-05-02 | This report identifies and tests the cable-operation/metadata subset. | Assemble the complete Arc05 API and disposition its remaining findings; Arc12 CRC assembly/CDC gate. |

Unload/replacement map retirement therefore remains Slice04; final child and
carryover reconciliation remains Arc12 CRC assembly plus CDC; broader historical
coverage/release gates remain Arc08-owned. The prior runnable recipe, 300-sample
demo, fresh 82-row replay, Scope output, fixture hashes and CLI error behavior
remain evidence from `981964b` at their original date. They were not regenerated
or represented as fresh. Because the correction changed only tests, CRC may
retain those unaffected claims if it confirms the source/fixture dependency
boundary; the earlier CRC record was not edited.

### Proposed status and handback

Iteration01 closes the four identified evidence gaps for CC review, subject to
CRC reproducing its stated criteria. All five rows remain **open** in the ledger:
CC's proposal is not CRC acceptance. No product behavior changed, no Slice03
closure is claimed, and Slice04 does not advance. There is no reported
production defect or additional Arc08 bubble-up. No supporting artifact was
created; all validation output was transient.
