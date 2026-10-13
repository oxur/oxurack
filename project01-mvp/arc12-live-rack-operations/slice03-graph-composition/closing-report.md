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
