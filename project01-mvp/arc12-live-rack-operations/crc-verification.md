# Arc12 — CRC operational composition record

2026-10-14, Sofie/CRC. Three contributors, Expedited Mode. Candidate source
`27ca00f64c902b77c2c16d943f1a2b5f650f7f67`, CC planning818d146; protected
main a7ce6e6. [Closing report](closing-report.md) and [ledger](ledger.md) describe
the same candidate. All five children/27 rows accepted; four arc rows open.

**Runtime composition reproduced; assembled packet ready for CDC's independent
review and historical-disposition decisions. Arc12 is not closed.** CRC's
experiments are operational evidence independent of CC implementation; CRC's
assembly, inherited interpretation and proposed closure are not independently
accepted by their own author. CDC must reproduce composition and check the
remaining historical questions; no prefilled CDC verdict file was created.

## Methods, scope and authority

Reviewed current status/project/arc plan and ledgers, CDC12 retained lifecycle/
instance/command design, all five child CRC verdicts and actual current source
delta. Read PM README/closing-arcs, applicable verification/testing discipline
and relevant Rust API/EH/TD guidance. Framework2.6.0/Rust2.0 remain governing.
Child Slice05 iteration01 review contains fresh gates and its row-specific
retention rationale. Source and main remain clean, no product repair by CRC.

Historical inputs: project cdc-successors03.tsv; Arc02/03/04/05 parent ledgers;
Arc03 notification locked decisions and literate/runtime reconciliation;
Arc04 memo's Post-3.25/3.5 reconciliation, all three child locked recaps and
Slice03's18-query mapping/ten inherited-row walk; CDC12's transport/typed-form
split. The18-row table in the closing report is accounting, not retroactive
acceptance of historical ledgers or a new future-feature plan.

## Fresh mandatory checks and current public recipes

The final-source commands/results are recorded in
[Slice05 iteration01 verification](slice05-instance-commands/crc-verification.md#iteration01-independent-review-and-closure--2026-10-14).
Both facade all-target feature modes, doctests (one positive/three compile-fail),
make lint, strict four-package/all-feature/all-target Clippy, warning-denying docs,
both example builds and diff check pass. Restricted make test retains four known
CoreMIDI failures (146 passed/3 existing ignored); exact host retry passes
(150 passed/0 failed/3 existing ignored), workspace and doctests included.
These results are actual current-candidate executions, not child-count inference.

Fresh native CLI runs from S use:

```bash
cargo run --locked -p oxurack --example rack-session -- --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --save "$OUT/session.ron" --samples 2000
cargo run --locked -p oxurack --example turing-machine-midi -- --patch "$OUT/session.ron" --samples 2000
cargo run --locked -p oxurack --no-default-features --example rack-session -- --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --save "$OUT/no-graph.ron" --samples 2000
cargo run --locked -p oxurack --example annotate-live-patch -- --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --save "$OUT/annotated.ron"
cargo run --locked -p oxurack --example rewire-live-patch -- --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --save "$OUT/rewired.ron"
cargo run --locked -p oxurack --example save-live-patch -- --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --save "$OUT/lifecycle.ron" --lifecycle
```

Here OUT denotes a fresh temporary directory; actual executed argument vectors,
stdout/stderr and summaries are under `oxurack-crc-27ca00f-recipes-zjt2n6jn`.
All exit0. Session/fresh/no-graph stdout equals14 rows with SHA256
`40965cc5f2031cbd3b26bd7275e0012359800c959d9c08f3dd5139f3707a76a2`;
post-cleanup Scope4016/global133/Clock120. Actual300-sample valid-sibling case
preserves bytes and records both exact three-event cleanup vectors, Scope604.
Annotation and rewiring reach300 samples; --lifecycle emits72 rows/hash
`20d86b836d7540a2ac6bf725f5045dfe6304fa7a027d703f22e1fe93f27613ba`.
Original MIDI2000/LFO10000 also reproduce their pinned13/70-row hashes.

## One composed current-owner experiment, both feature modes

A transient direct public-API Rust program (not a new crate, dependency, product
validator or permanent harness) tests the property missing from isolated child
receipts: metadata/topology/save continuation followed by instance retirement,
queued replacement and seeded restart in the same owner. Source copy and logs:
tmpdir `oxurack-crc-arc12-composition-go1xjkk5`; original convenience source
`/tmp/oxurack-arc12-composition.rs`. These are not durable gate dependencies;
the complete setup/action/assertion protocol follows for independent reproduction.

For each mode, build actual locked artifacts with:

```bash
cargo build --locked -p oxurack --example rack-session --message-format=json
cargo build --locked -p oxurack --no-default-features --example rack-session --message-format=json
```

Select the actual compiler-artifact rlib paths for oxurack/core/Clock/LFO/TM/
bevy_ecs from that build, then rustc --edition=2024 with --extern and
`-L dependency=S/target/debug/deps`. Default witness enables its graph assertions;
no-default omits them. Both compile/execute exit0. No stale facade rlib or
simulated executor. Both use the same immutable seed42 LFO patch and real modules.

1. Create subject/control with Clock/LFO/TM/MIDI, Manual, capacity60000 Scope
   selecting Clock pulse, TM probability/note and LFO bipolar. Advance both200,
   draining/copying every sample, assert initial MIDI equality.
2. Apply identical live edits: LFO1 Hz/TM base.25; bound unpatch/repatch LFO→TM
   with Affine(0,.25); rack Notes=["Arc12 composed performance"], TM tag crc,
   cable description constant0.25. Subject queue: command clock description,
   SetBpm133, Save. Control executes the same non-save edits without saving.
   Assert outcome IDs/order/Ok, same sample200, snapshots equal, graph4/9.
3. Reject missing-source patch on subject, assert snapshot and graph unchanged.
   Continue both1000 samples, compare all MIDI and mixed Scope (not just totals),
   zero overwrites, same time1200. Observed continuation7 MIDI/4807 total Scope.
   This rejects saving/resetting or topology edits that rewind the saving owner.
4. Stop/drain; bind LFO/TM; remove LFO, verify LFO knob stale/TM valid, selected
   capture disabled, graph immediately3/8. Save current patch; unload and record
   all retired cleanup at1200. Empty snapshot fails and sample resets0; old TM
   handle stale. Queue Load(saved)→SetBpm137→Save(saved); all three original IDs
   finish Ok despite App swap; global137 and Clock120.
5. Independently load final saved file, compare complete current snapshots.
   Configure both three-selector Scope and play10000 with per-sample drain/copy.
   All MIDI and complete records equal, zero overwrites. Observed82 MIDI/30082
   Scope. Emit ordinary trace to stdout; both process/feature variants match.
6. Stop; bind Clock; restart subject from current snapshot, drain old10000-stamped
   cleanup. Verify sample0, ScopeNone, old Clock stale, graph3/8. Compare next1000
   samples and mixed Scope to another independently loaded final file.
7. On stopped/drained subject reject a never-created file in the owned test
   directory; snapshot/Clock handle/sample1000 unchanged. Enqueue SetBpm199,
   reset, drain explicit silence, collect the original ID as CancelledByReset,
   then empty second collection. Global stays137, sample1000.
8. Finally consume a separate metadata-bearing saved rack into App. Its actual
   typed load notifications contain4 modules/9 cables/4 labels/one summary;
   summary counts match, parameter/label provenance PatchLoad, no fabricated
   removal messages. Source inspection of patch.rs:534–856 establishes its
   synchronous summary-last producer control flow. Runtime counts do not claim
   a shared cross-type timestamp or a serialized event bus.

Observed default/no-graph ordinary trace:82 rows, equal SHA256
`c9f365651430895ddfa131f639344859ea463e70baa482ceb30da6e3210d46f5`.
Retired cleanup at1200 is NoteOff67/CC123/CC120; restarted-old10000 cleanup is
CC123/CC120; narrow reset1000 is NoteOff62/CC123/CC120. These are separately
recorded, never folded into fresh-lifetime ordinary traces. All equality and
identity assertions pass; this experiment needed no product fix.

The same fresh artifacts compile/run the required Vec<CommandOutcome> external
consumer; forged CommandId(42) now fails E0423. Native doctests cover the other
public privacy/extensibility cases. Scope/trace comparisons here test one actual
representative seeded configuration, not all catalogs, platforms or hardware.

## Retention, limits and gate handoff

All27 current child criteria are accounted for. Original acceptances/dates remain
in their own records; current workspace tests and composed execution provide
affected integration evidence. No CC attestation was promoted by copying a verdict.
No endpoint, controller, hardware timing, new coverage campaign, benchmark or soak
was run. Existing RT ignores/advisories and restricted failures remain disclosed.
Configuration reconstruction does not restore RNG cursor, oscillator phase or an
exact performance checkpoint. Those limits are part of the approved contract.

Closing report's18-row inherited walk identifies original v0 amendments and
future representation/event deferrals. The old PatchUnloaded re-entry and the
strength of cross-type-order evidence need CDC's explicit disposition; no new
owner or acceptance is invented here. Runtime reproduction is not sufficient
authority for CRC to declare these historical obligations gone. All four arc
rows and six original project rows stay open; P-07 retains prior CDC acceptance.

Durable supporting artifacts: none. Canonical arc/child reports and ledgers are
the record; passing temporary logs are not a new artifact tree. What worked:
one real composed owner plus an uninterrupted saving control and fresh seed
reconstruction tied the individual feature receipts together without a new
runtime architecture. CDC receives the closing report, this record and ledger,
independently verifies the same candidate and returns through Duncan. Any required
changes need the next directive; no next arc/first slice is opened by CRC.

Planning close/relay check,2026-10-14: exact ten planning paths, all five child
ledgers/27 done, four arc rows open, six original project rows open/P-07 done,
eighteen inherited rows and unchanged issued prompts/CC report verified. New
arc links resolve; source/main pins remain clean/unchanged; no staged unrelated
files or prefilled CDC verdict. Planning diff check passed.
