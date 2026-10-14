# Arc12 — independent CDC acceptance

2026-10-14, Sofie/CDC, Three-Contributor Workflow / Expedited Mode.
**Accepted at `27ca00f64c902b77c2c16d943f1a2b5f650f7f67`.** Planning intake
`a5a8e65`; source/main/planning clean at intake, main remains a7ce6e6.
CRC's [closing report](closing-report.md) and [operational record](crc-verification.md)
cover this same candidate. CDC authored no implementation repair. All four R rows
are done; five children/27 accepted rows reconcile. This accepts the defined live
rack capability, not the original whole MVP, historical deferred rows or release.

## Independent method and results

Read the arc packet, current plans/ledger/AGENTS, directive12, child ledger row
walks and inherited notification obligations. Inspected current facade lifecycle,
commands, handle validation, core removal/loader, MIDI silence/reservation code,
graph retirement and runnable frontend code. Specifically checked candidate
commit versus preflight, surviving instance tokens, queue survival/cancellation,
selected cleanup before hook/despawn, both graph edge directions and name guarding,
and PatchLoaded's final synchronous emission. No current composition defect found.

Reviewed CRC's complete transient composition witness, then independently built
current artifacts and compiled/executed it in **both** feature modes. Artifact
paths came from each fresh Cargo JSON build, not a possibly stale top-level rlib.
This reuses an inspected experiment; it is independent execution, not a claim that
CDC invented a second experimental method. The durable setup/actions/oracles are
CRC record's eight-step protocol; this record preserves observed values and limits.

| Row | CDC reproduced observation | Disposition |
| --- | --- | --- |
| R-01 | Subject/control200 samples, identical parameter/label/topology edits, queued save only on subject; next 1000 MIDI and full Scope equal, 1200 samples/4807 records. Saved configuration and fresh reference then agree for 10000 samples, 82 MIDI/30082 records, no overwrite. | done |
| R-02 | Graph 4/9 after bound rewiring and metadata; rejected missing-source edit preserves snapshot/graph. Removing LFO gives 3/8 immediately and disables affected capture; actual saved reload yields 4 ModuleAdded/9 CableConnected/4 LabelChanged/1 PatchLoaded, with correct provenance and summary counts. | done |
| R-03 | LFO handle stales, TM survives removal; unload stales old lifetime, preserves cleanup at 1200. Queued Load→SetBpm137→Save preserves queue tail across App swap; Clock stays120. Restart yields stopped sample 0/ScopeNone/stale old handles and fresh-reference1000-sample equality. Failed load preserves snapshot/time/handle. Reset preserves sample 1000 and returns cancellation once. | done |
| R-04 | Five child ledgers total 27 done; eighteen distinct inherited rows in CRC report retained with the residual decisions below. Fresh feature/native/docs checks pass within stated host boundary; no hidden source fix or scope deletion. | done |

Default and no-graph ordinary traces each 82 rows, identical SHA256
`c9f365651430895ddfa131f639344859ea463e70baa482ceb30da6e3210d46f5`.
Observed lifecycle cleanup separately: unload at 1200 = NoteOff67, CC123, CC120;
restart old10000 = CC123, CC120; reset1000 = NoteOff62, CC123, CC120. Never compare
old-lifetime cleanup as though it were new-lifetime musical output.

Fresh commands from the source worktree:

- `cargo build --locked -p oxurack [--no-default-features] --example rack-session --message-format=json`: both exit 0; actual rlibs linked by rustc edition 2024; both witness runs exit 0.
- `cargo test --locked -p oxurack --all-targets`: exit 0; all target groups pass.
- Same facade command with `--no-default-features`: exit 0; all target groups pass.
- `make test` restricted: exit 2, four oxurack-rt CoreMIDI initialization failures,
 146 passed/3 ignored in that failing group. Rack/core/graph/module groups pass.
- Exact `make test` with approved host access: exit 0, RT150 passed/0 failed/3
 existing ignored; workspace and doctests pass. `block v0.1.6` future-incompatibility
 advisory remains. This is native test success, not a hardware timing measurement.
- `make lint`: exit 0.
- `cargo clippy --locked -p oxurack -p oxurack-core -p oxurack-graph -p oxurack-mod-midiout --all-features --all-targets -- -D warnings`: exit 0.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p oxurack -p oxurack-core -p oxurack-graph -p oxurack-mod-midiout --all-features --no-deps`: exit 0.

Transient logs/source/result vectors:
`/var/folders/dt/kc8ndn6d3kx8c1jnxr05drsh0000gn/T/oxurack-cdc-arc12-a3us5u80`,
`/tmp/oxurack-cdc-arc12-host.log`, `-clippy.log`, `-docs.log` with the same stem.
They are supporting execution traces, not durable dependencies or a new tool suite.
CRC's child-specific review evidence remains at its original revisions/dates;
CDC did not rewrite those records as fresh reviews. Current native tests and the
composed run reproduce integration; they do not claim every historical proof was
separately reconstructed. No endpoint/controller/coverage/benchmark/soak run here.

## Historical decisions and remaining owners

The eighteen-row accounting in CRC's closing report is accepted as reconciliation,
with these explicit residual dispositions. Original ledgers and register03 remain
historical deferred records, not retroactively changed to done.

1. **PatchUnloaded:** the old “when unload exists” trigger has arrived. Its
   original single-World observer assumption does not match directive12's owner
   replacement. Retain the capability, defer to **Arc07/Slice02 session observation**,
   before introducing watch/subscriber UI that survives patch replacement. That
   slice must define a facade-lifetime notification/receipt with actual unload
   identity, delivery and error semantics. Do not emit stale Entity removals into
   a new World or claim the missing event is delivered. Slice01's synchronous
   command replies need no PatchUnloaded subscription.
2. **Cross-type ordering:** current producer control flow emits granular load
   notifications before PatchLoaded; real typed counts/provenance and synchronous
   graph freshness reproduce. This is sufficient for Arc12's current direct-owner
   API. It does **not** fulfill the old requirement for a runtime-observed common
   cross-type timeline. Retain that stronger observer proof with Arc07/Slice02,
   before any public ordered subscription/watch claim. Do not create an event bus
   merely to close the present arc or report that counts measured temporal order.
3. **Literate extensions:** full-list notes/labels/tags are delivered. Per-note
   IDs/CRUD events, richer authorship/version history and tag search/filtering
   belong to **Arc07/Slice02**, re-entering with persisted session observation/edit
   UX; hierarchical/expanded tag policy and formatter/user-configuration persistence
   belong to **Arc07/Slice03 controller/configuration**, before that richer user
   configuration is exposed. Port labels belong to **Arc13 catalog/API reconciliation**,
   before typed per-port presentation. These are retained features, not prerequisites
   for numeric live knobs.
4. **Graph extensions:** current v0 combined graph and 18-query contract remain
   accepted as scoped; CableKind Signal/Modulation, knob edges, signal-only filters,
   richer provenance and corresponding diagnostics remain **Arc13 catalog/API
   reconciliation**, before distinguishing connection kinds in the module vocabulary.
   No deterministic diamond tie beyond existing v0 is invented at this gate.
5. **Arc05:** directive12's split is retained. Arc07/Slice04 owns distinct external
   transport/power and START/STOP/CONTINUE before external transport controls;
   Arc13 owns module-specific Port/Knob/value enums and convenience styles. Current
   start/stop/reset/restart, generic handles and queued persistence are delivered.
6. Exact performance checkpoints remain Arc13/backlog. Arc08 retains physical
   timing, full musical/controller acceptance, coverage/resource provenance,
   benchmarks and soak. P-01–P-06 stay open; P-07 keeps prior acceptance.

Arc13 catalog/API re-entry is owned by the already planned
`arc13-remaining-module-catalog/slice01-catalog-reentry` reconciliation, which must
place the retained port/graph/typed API work before affected catalog interfaces.
This names responsibility, not an issued implementation assignment.

These decisions continue identified historical obligations with concrete owners
and re-entry; they do not waive a failing Arc12 runtime criterion. See
[directive13](../cdc-directive13.md) for the Operator-mediated return and next work.

## Bubble-up and next work

Arc12's amendments (getter/order repair; isolated lifetime ownership;04/05 split)
produced the promised executable editing/persistence/lifecycle capability.
The lifecycle and public-command proof needed repeated corrections; retain that
history and improve frontloaded test examples rather than expanding proof tooling.

Duncan selected **live controls for the existing workbench** during this review.
Open the existing Arc07, ahead of catalog expansion, with its first executable
live-control slice. Numeric order does not determine dependency order. CRC reviews
that implementation and advances only the approved, sufficiently detailed scope;
CDC retains design and arc acceptance. No merge/push/release authorization added.
