# Slice05 CC implementation record

2026-10-13. Assignment: `cc-prompt.md`, SHA256
`a1ab69c378b51f8b9b9cd003cac59f1a314c9934d39e62d1205337135633db94`.
This is the pre-edit intake and contract readback. Source baseline S was clean
at `ec85ebe99c09b8a2ea64d55da0346af4ae317a67`; planning W was clean at
`4b8119a1ebb2cb8ddf7dfebbdd816925505f8177`; protected main R was clean at
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. No source or implementation edit
had been made when this record was created. The required text was loaded in
bounded contiguous ranges listed below. An earlier combined read hit output
truncation; the identified required sections were reloaded in smaller ranges.
Read receipts establish delivered text, not independent proof of comprehension.

## Required-reading intake

All source paths below are rooted at the assigned S or W unless shown otherwise.
Required-full means the complete file was loaded; required-section means the
complete named section or production/test region was loaded. No data query or
conditional dependency-source lookup applied during intake; no structured
dataset was supplied.

| # | Input and pinned state | Loaded extent and use |
| --- | --- | --- |
| 1 | W `AGENTS.md`; S `CLAUDE.md` | Required-full at issuance. Role, source fence, path and commit discipline. |
| 2 | W Slice05 `cc-prompt.md`, `slice-plan.md`, `ledger.md` | Required-full at issuance. Prompt was reloaded at lines 1–548; plan and six-row ledger loaded in full. |
| 3 | W `project01-mvp/cdc-directive12.md` | Required sections loaded: Decisions and delivery; D1 lifecycle semantics/matrix; all D2; all D3; D4 source boundary, discriminating proof, guidance/readiness. Binding removal, handles, commands, BPM and queue contracts. |
| 4 | W `project01-mvp/cdc-directive08.md` | Required sections Prompt quality and Checks/correction policy loaded in full. Playable-first proof and native gates. |
| 5 | W `project01-mvp/project-plan.md`; Arc12 `arc-plan.md`; Arc12 `ledger.md` | Project Current direction; Arc Capability, five delivery slices, Binding design, Current lifetime/compatibility contract and closure/advancement; full Arc12 ledger. Parent scope and open gates. |
| 6 | W Slice04 `crc-verification.md`; `slice-plan.md` | Required-full Iteration03 independent review/closure section, lines 460–583, reloaded contiguously; Slice04 Boundaries/inherited work and Final review/bubble-up loaded. Six LF rows accepted at ec85ebe; retained evidence and held parent gates. |
| 7 | Installed `collaboration-framework/SKILL.md`; `project-management/guides/README.md` | Required-full; previously loaded and rechecked for this assignment. Workflow, expedited scoped commits and planning artifact routing. |
| 8 | Installed `engineering-methods/guides/07-implementation-prompt-authoring.md`; `work-verification/guides/05-independent-verification.md` | Required-full; guide07 loaded in contiguous ranges through its end; independent verification guide loaded in full. Intake, readback, correction retention and proposed versus accepted evidence. |
| 9 | Installed `testing/SKILL.md`; testing guides `01-testing-discipline.md`, `03-validation-gates.md` | Required-full; all three loaded. Interaction-focused tests and proportionate native proof. |
| 10 | Installed Rust `SKILL.md`; Rust `guides/11-anti-patterns.md` | Required-full; skill lines 1–611 and anti-pattern guide lines 1–2188 delivered in contiguous chunks. No Rust changes before intake completion. |
| 11 | Rust guides `02-api-design.md`, `03-error-handling.md`, `05-type-design.md` | Required sections loaded with complete examples/rationale: API-02/07/23; EH-01/14; TD-02/03/05/06/07/17. Applied to owned references, hidden Arc identity, typed errors, invariant marker and explicit queue states. |
| 12 | S facade `src/lib.rs`, `lifecycle.rs`, `persistence.rs`, `cables.rs`, `labels.rs` | At ec85ebe, production regions through each `mod tests` boundary loaded. Also loaded inline RetainedState/first-fault and disabled-restart tests, cable failure/provenance tests and label provenance tests. Actual owner, replacement, mutation and event seams. |
| 13 | S facade `src/scope.rs` | At ec85ebe, Scope selector/capture fields and accessors, configuration, disable, observer installation and observation loaded through the test boundary; configuration tests loaded. Entity-bound selectors and selective invalidation requirement. |
| 14 | S core `module.rs`, `topology.rs`, `cable.rs`, `event.rs`, `error.rs`, `lib.rs` | At ec85ebe, production regions through test boundaries loaded. Inspected module registry/context-spawner side map, cable ordering and validation helpers, CableIndex methods, all CoreCommand variants/dispatcher and structural notifications, error surface, CorePlugin schedule/message registration and exports. |
| 15 | S `oxurack-mod-midiout/src/lib.rs` | At ec85ebe, production region through test boundary loaded, plus silence/channel-order/fixed-capacity tests. Buffer capacity, reservation accounting, dirty voice state and cleanup preparation. |
| 16 | S `oxurack-graph/src/lib.rs` | At ec85ebe, all production regions through test boundary loaded: graph fields/queries, plugin schedule and four handlers. `graph_test_app` and incident-edge removal regression loaded. Existing handler reads outgoing edges only and removes names unconditionally. |
| 17 | S facade `tests/lifecycle.rs`; `examples/save-live-patch.rs` | At ec85ebe, lifecycle setup, candidate failure, ordered hook replay, normalized structure, seeded replay and sample-300 control regions loaded. Example Args/run_with_output/run_lifecycle/drain and writer tests loaded. |
| 18 | S fixtures `turing-machine-lfo.ron`, `turing-machine-midi.ron`; root and facade Cargo manifests; `Makefile`; `README.md` | At ec85ebe, both fixtures, both manifests, complete Makefile and README loaded. Feature matrix, Clap availability, known runnable examples and native gate definitions confirmed. |
| 19 | W Arc05 `ledger.md`; Slice04 `slice-plan.md` | Arc05 current A-05-01/A-05-02 rows and final CDC disposition loaded; Slice04 Boundaries/inherited work and Final review/bubble-up loaded. Historical transport/catalog obligations remain separately owned. |

The prompt's locked-dependency conditional was not triggered by an unfamiliar
dependency API during intake. Before relying on hierarchy or graph behavior in
implementation, I verified the exact local lockfile source: Bevy `0.18.1`
`hierarchy.rs:151` marks `ChildOf` with `linked_spawn`; `World::despawn` docs at
`world/mod.rs:1456` specify linked-descendant despawn; petgraph `0.7.1`
`stable_graph/mod.rs:708,723` confirms `edges` is outgoing and
`edges_directed` accepts incoming/outgoing. The new owned-grandchild and direct
graph-notification tests exercise those exact boundaries. No dependency or
manifest changed.

## Contract readback

The active patch is `PatchHandle` membership, not every entity left in the ECS.
`remove_module` must first enforce stopped-then-drained precedence and preflight
module kind, active ownership, nested active modules, hierarchy and every cable
touching descendant ports. Cable cleanup is deduplicated and ordered by the
existing `(missing order, CableOrder, Entity)` policy. Any recoverable preflight
or selected-output capacity failure leaves messages, entities, indexes, output,
Scope and handles unchanged. For a MIDI sink, prepare its own silence batch and
prove all fixed output room before appending; only then disconnect with
`DirectApi`, invoke the copied infallible removal hook once, release that sink's
channel reservation, remove active membership, despawn its owned tree, emit one
`ModuleRemoved`, invalidate only its token, and refresh graph synchronously.
This is the IC-01/IC-05 boundary in prompt sections 1 and 5. The easily missed
ordering constraint is that cleanup must be safely enqueued before teardown;
after append the remainder is an infallible commit. Arbitrary user hook panic is
outside recoverable rollback. Whole-App destruction does not invoke per-instance
hooks.

Names in `ModuleRef`/`PortRef`/`KnobRef` are owned lookup data. A bound `Handle<M>`
and references derived from it carry private rack-lifetime and per-instance Arc
identity, never an Entity. Validate lifetime and instance identity before any
shared string/name helper; then validate active membership, module kind and the
installed Rust `TypeId`. A failed candidate preserves tokens; successful new
lifetime rotates them; removing one instance invalidates only that instance.
Exact tuple names may contain dots; dotted parsing accepts exactly one separator
with nonempty sides. These are IC-02's cross-rack, wrong-type, stale, reuse,
failed-load and survivor tests.

All eight existing `CoreCommand` variants must perform their real operation.
Parameter/label/cable mutations use `Command` provenance, direct facade APIs keep
`DirectApi`, and load initialization keeps `PatchLoad`; a command load labels its
`PatchLoaded` source as `Other("command load: …")`. `SetBpm` writes only
`LoadedIdentity.bpm`, so save/snapshot/restart retain it while Clock's own
parameter stays at its configured value. Low-level core dispatch remains explicit
`NotImplemented` for facade-only operations. These are IC-03's event, no-tick,
real-save and BPM independence oracles.

The facade owns a 64-slot FIFO outside `App`. Each slot is Pending or an
uncollected Completed outcome, including `CancelledByReset`; capacity counts both.
IDs start at zero and use checked increment before acceptance. Full/exhausted
admission leaves slots and counter untouched. Processing pops one slot before
executing, resolves paths/names at execution time, continues after command errors,
and returns every result once in submission order. Successful reset converts only
remaining Pending slots and preserves Completed results; reset failure converts
nothing. Thus queued Panic preserves its own completed result and cancels only its
still-pending tail. This pop-before-execute seam is the IC-04 interaction oracle.
There is no message consumer, worker or sample acknowledgment; consuming
`into_app` abandons facade queue state unless outcomes were collected first.

The implementation is limited to the source allowlist in the issued prompt and
planning changes to this report and the Slice05 ledger. The new tests must prove
the six IC rows through real facade paths, inline access only for private
invariants, graph notification without pre-disconnecting, the sample-300 literal
MIDI retirement, survivor/control continuity, bounded queue/error behavior, and
the runnable default/no-graph/fresh-replay session. Required fresh gates include
the affected core, MIDI-out, graph and facade feature modes, repository test/lint,
strict Clippy/docs, example builds, original MIDI/LFO trace hashes and the exact
session recipe. Existing restricted CoreMIDI failures remain separately reported;
Slice05 rows stay open pending CRC independent acceptance. Arc05 A-05-01/A-05-02,
Arc12 R-01–R-04 and project/release gates are not accepted by this child result.

## Execution evidence and disposition

Implementation was performed only in source S after required intake and contract
readback. `rustfmt --all` was applied. Pinned Rust guidance was followed: names
are borrowed at the API boundary; `Handle<M>` has private invariant
`PhantomData<fn(M) -> M>` and private Arc lifetime/instance identities; failures
use typed `CoreError` variants; removal is preflight/commit with selected cleanup
appended before teardown; command execution pops before effects and returns typed
`CoreError` outcomes; reset failure does not cancel. Handles use manual Clone and
Debug without type bounds. No public World, Entity or pointer escaped.

### Implemented paths

- Core removal validates active `PatchHandle` membership, registry kind, nested
  active membership, hierarchy descendants, every incident cable and both
  CableIndex endpoint rosters. It owns ordered cable snapshots, then commits
  `CableDisconnected(DirectApi)`, one copied module hook, membership removal,
  linked-descendant despawn, one `ModuleRemoved`, and dirty propagation.
  `OxurackModule::on_remove` defaults to no-op; MIDI-out releases its channel and
  logical capacity without touching fixed storage or pending cleanup.
- Selected MIDI silence retains whole-world accounting checks and emits only the
  requested dirty sink. Facade removal follows stopped/drained precedence,
  preflight, selected silence and room admission, Scope invalidation only when
  selected, core commit, one instance-token retirement and immediate graph
  maintenance. Sample-300 cleanup was Note Off 62, CC123, CC120 in order, all at
  sample 300. Last removal snapshots as a loaded empty patch.
- `Handle<M>`, `ModuleRef`, `PortRef` and `KnobRef` are exported. Bound values use
  private Arc lifetime/instance identity; module lookup checks installed Rust
  `TypeId`; failed replacements preserve handles, successful lifetime commits
  rotate identities, and in-place removal retires one token. Tuple names preserve
  dots; ambiguous dotted input is rejected.
- All eight `CoreCommand` variants execute at the facade. Mutations retain
  Command/DirectApi provenance; command load records
  `PatchSource::Other("command load: …")`; global BPM snapshots/saves/restarts
  independently of Clock BPM. The 64-slot queue has checked zero-based IDs,
  FIFO execution-time lookup, typed outcomes, continues after errors and
  success-only reset cancellation. No queue state lives in `App`.
- README documents lifetime-bound handles, stopped/drained removal, selective
  cleanup, queue/reset outcomes and global BPM. `rack-session` runs a real
  300-sample session, removes LFO, exercises stale/surviving handles and the
  save/fail-load/reload/edit/BPM/save FIFO, writes only the final MIDI trace to
  stdout, and compares trace, signals and copied MIDI with a fresh saved-patch
  run.

### Test and gate receipts

Focused new checks passed: facade `tests/instance_commands.rs` (14 tests),
facade private command state (4 tests), owned-grandchild/nested-owner lifecycle
checks, first-fault removal, selected-sink MIDI reservation/channel reuse, and
graph incoming/outgoing/self/parallel-edge plus same-name preservation tests.
The executable example's 2 in-process tests passed. Existing accepted lifecycle
tests were retained.

| Command | Result |
| --- | --- |
| `cargo test --locked -p oxurack-core` | pass, 426 unit + 14 integration tests + 2 doctests (442 total) |
| `cargo test --locked -p oxurack-mod-midiout` | pass, 10 unit tests |
| `cargo test --locked -p oxurack-graph` | pass, 68 unit + 1 integration test |
| `cargo test --locked -p oxurack --all-targets` | pass; facade unit, integration and example targets |
| `cargo test --locked -p oxurack --no-default-features --all-targets` | pass; no-graph matrix; existing unused-mut advisory retained |
| `make test` restricted host | first run: 146 RT pass, 4 CoreMIDI init failures, 3 ignored; no test changed/suppressed |
| `make test` with host MIDI access | final-source pass; RT 150 pass, 0 fail, 3 ignored; workspace and doctests green |
| `make lint` | pass; Clippy and format checks |
| strict all-features/all-targets Clippy with `-D warnings` | pass |
| `RUSTDOCFLAGS='-D warnings' make docs` | pass |
| default/no-default-features `cargo build --locked -p oxurack --examples` | both pass |
| `git diff --check` | final-source pass |

The session command, fresh `turing-machine-midi` replay and no-graph session ran
for 2000 samples. Both `cmp` checks passed. Session replay had 14 MIDI rows and
4014 mixed Scope records; global BPM was 133 and Clock BPM remained 120. Original
MIDI2000 remained 13 rows with SHA256
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`; LFO10000
remained 70 rows with SHA256
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.

Some entries above preceded the final small test/example additions; the complete
final-source gate rerun below supersedes those receipts. No external MIDI
endpoint was opened. No dependency or source fixture changed. Temporary traces
and saved RON are in `/tmp/oxurack-slice05.WKfxRi`, outside the repository.

### Six-row self-walk and boundary

| Row | Proposed CC evidence; CRC status remains open |
| --- | --- |
| IC-01 | Removal checks admission precedence, survivor Scope, loaded empty header, child/grandchild cascade, nested-active preflight preservation, full CableIndex and graph direct-notification regressions. Core removal exercises A→B→C plus B→B and ordered parallel incident cables; an unconnected-LFO control test compares continuing survivor Scope and MIDI. |
| IC-02 | Same-kind different Rust type and wrong installed kind, cross-rack, removed instance, restart, unload/reload with same name, failed replacement, stale bound knob/port, surviving handle, tuple names with dots and rejected dotted forms. |
| IC-03 | All eight variants are implemented; tests exercise parameter/label/cable/BPM/Panic, save/load in the executable sequence, load provenance, invalid BPM and Clock independence. Low-level unsupported variants remain explicit errors. |
| IC-04 | Zero-based checked IDs, 64/65 capacity, direct and queued reset cancellation, reset-failure preservation, FIFO outcomes and execution-time save-then-load path are tested. |
| IC-05 | Literal sample-300 cleanup, selected other-dirty-sink preservation, accounting release, immediate channel reuse, fixed storage and fault preservation. No hardware send/silence is claimed. |
| IC-06 | CLI runner has writer-oriented execution, zero/missing-input errors, real live session, fresh trace/signal/MIDI equality, default/no-graph comparison and locked docs/lint/test/build matrix. |

The custom topology removal test records four incident cable IDs ordered by
`CableOrder` (including the self-edge and parallel edges), observes all four
`CableDisconnected(DirectApi)` messages before the removal hook, confirms no
`ModuleRemoved` exists at hook time, then confirms one event and surviving A/C
owners. A separate uninterrupted control compares 200 subsequent samples,
Scope and MIDI after removing an unconnected LFO at sample 300. CRC should still
independently examine the breadth and evidence against each criterion. Inherited
Arc05 A-05-01 runtime/removal/handles/queued-operations and A-05-02 reconciliation
remain separately carried; Arc07 transport and Arc13 typed vocabulary remain
unchanged. Arc12 R-01–R-04 and project P-01–P-06/release gates remain open. IC rows
are CC proposed evidence only, never acceptance.

### Final-source rerun and commits

After the last integration test addition, the final rerun passed:
`cargo test --locked -p oxurack-core` (426 unit, 14 integration, 2 doctests),
`cargo test --locked -p oxurack-mod-midiout` (10),
`cargo test --locked -p oxurack-graph` (68 unit, 1 integration), facade
`--all-targets` and `--no-default-features --all-targets`, strict all-features /
all-targets Clippy, `make lint`, warning-denying docs, both example build modes,
and `git diff --check`. Final `make test` with host MIDI access passed; the RT
crate reported 150 passed, 0 failed and 3 ignored, with workspace doctests green.
The first restricted-host `make test` result remains recorded above: four
CoreMIDI initialization failures occurred because host MIDI access was denied;
the unchanged host-access retry passed. No errors were skipped or suppressed.

The final-source 2000-sample live session, fresh `turing-machine-midi` replay,
and no-graph session all passed. Fresh and no-graph traces matched the live
session. Session evidence reported 14 MIDI rows, 4014 Scope records, global BPM
133, and Clock BPM 120. Source S changes remain within the prompt allowlist;
planning W changes remain limited to this report and the Slice05 ledger. The
Source commit: `7404fe227e561746dbda76543b59893feeacfe15`. No acceptance is
claimed. All six IC rows remain open for CRC review.

## Applied Rust guidance

| Guidance | Applied decision and proof |
| --- | --- |
| API-02 SHOULD; API-07 MUST | Borrow names at call boundaries and return owned refs/results; keep Arc identity and ECS Entity private. Exercise cross-rack and stale-bound operations. |
| API-23 MUST; EH-01 MUST | Use named `CoreError` variants for recoverable stale/type/ownership/queue/admission failures. Keep only invariant failures/panics outside recoverable guarantees. Compare complete pre/post state on failures. |
| TD-02/03 SHOULD; TD-05 MUST | Use opaque `CommandId`, owned reference types and private token fields; keep module/port strings as exact configuration lookup values. Validate constructors/parsing. |
| TD-06/07 SHOULD | Represent queue slots/outcomes as explicit states and mark the public outcome enum non-exhaustive. Test cancellation/capacity/collection transitions. |
| TD-17 SHOULD | Use `PhantomData<fn(M) -> M>` for invariant `Handle<M>` tagging; manual Clone/Debug so `M: Clone/Debug` is not imposed. Compile and use actual module handles. |
| EH-14 MUST | Document public errors, panic boundary, token invalidation, queue admission/result behavior, reset cancellation, `into_app`, and global BPM distinction; strict docs gate. |
| Anti-pattern review | No public smart pointers/Entity, no untyped error strings, no silent command success or swallowed results, no unnecessary Box/worker, no public invariant-bearing token fields; use short borrows and named constants for queue bounds. |
