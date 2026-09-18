# CRC verification — slice01 implementation, iteration01

2026-09-17. Sofie/CRC, independent reviewer in the Operator-selected
Three-Contributor Workflow and Expedited Mode. Candidate: CC's
`cc-prompt-iteration01.md` at source commit
`d43078ea9c71bba10035699c9914bef3891f1d83`; CC planning evidence
`c53989d7f74aa33653a64077fbdf2594f9bd5400`, with metadata corrections
through planning HEAD `2da485ebad1c685b847bcebf954921dd0aecedfa` at
review intake. Main remains `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
Source and planning worktrees were clean at review intake. This is CRC's own
review, not CC's proposed-done attestation or CDC's design approval.

## Verdict and row walk

**Partial verification; slice remains open.** The implementation follows the
approved API and install ordering, and the behavioral tests run in both
feature modes. Three of six criteria reproduce; I-02, I-05 and I-06 remain
open. Do not advance to arc09/slice02 or call this implementation accepted.

| Row | CRC result | Independent basis / outstanding work |
| --- | --- | --- |
| I-01 | Done, reproduced at `d43078e` | Existing and new module implementations compile. `Oxurack::try_install` checks its rack-local `ModuleKind -> TypeId` map and calls the hook only for a new kind. The loaded two-instance test checks actual fixed-schedule effects, and the separate-racks/legacy test passes in both feature modes. `HookCalls` counts system executions, not hook invocations; code inspection plus the exact per-instance trace supplies the once-only conclusion. |
| I-02 | Open | Code inspection and the conflict test support typed rejection before mutation, retained metadata/spawner/accessors and original system behavior. The named reinstall test calls `try_install` immediately after loading, **before** the two fixed runs and before changing `alpha` through the setter (`module_installation.rs:323` versus `:328–338`). It therefore does not test preservation of modified live state through reinstall as the prompt and row require. Move the reinstall after the `(4,6)` state and setter change, assert state/getter immediately, then run the next fixed step to `(8,9)`; retain the collision negative control. |
| I-03 | Done, reproduced at `d43078e` | The static RON fixture loads `alpha`/`beta` through `Oxurack::load_patch`; direct `Update` leaves `(0,0)`, two direct `FixedUpdate` runs produce `(2,3)` and `(4,6)`. Both focused feature-mode runs pass; graph-on module count is two. This proves the supplied fixed schedule, not later exact-time sample semantics. |
| I-04 | Done, reproduced at `d43078e` | Both strict rustdoc commands exit 0. Diff inspection confirms only comment changes in the three bounded legacy files: three unresolved links and eight redundant explicit targets were repaired without allowances. New hook/fallible/convenience docs describe errors and panic boundaries. |
| I-05 | Open | Both focused/package/workspace test matrices, format, strict docs, targeted clippy and host `make check` pass. Full required `cargo clippy --locked --workspace --all-features --all-targets -- -D warnings` exits 101 on `collapsible_if` in graph `lib.rs:1036` and `drain_collect` in core `bridge.rs:481`. Both exact constructs exist at source baseline `a7ce6e6`; graph is doc-only under the current allowlist and bridge is outside it. The planning verifier exits 1 on its stale initial-prompt assignment assertion. Restricted `make check` failed four MIDI initialization tests; host retry passed. The full gate and verifier still need a CDC-authorized repair/disposition; their failures are not waived by targeted green checks. |
| I-06 | Open | The source commit changes exactly the eight allowed paths and both worktrees were clean at intake. All 20 validation-table log targets exist, and CC retained failed/retried attempts. Ten `cwd` cells omit `/oxur/` and name a nonexistent checkout; every row's `source_revision` is the baseline HEAD although commands ran with an uncommitted implementation tree. The report explains the eventual commit, but the table needs an exact working-tree-state clarification. Its arc bubble-up should explicitly answer delivered capability, discoveries and silent-drop diff. Correct provenance without deleting the original failed attempts or raw logs. |

## Independent checks and scope review

At the clean source candidate, CRC ran the two focused `module_installation`
feature variants (5/5 each), `cargo test --locked -p oxurack-core`,
`cargo test --locked -p oxurack`, both locked workspace test matrices,
both strict rustdoc commands, and `cargo fmt --all -- --check`: all exited 0.
Workspace matrices included 150 passing `oxurack-rt` tests with three pre-existing
ignored tests; no ignored tests were run. Targeted clippy for core and the
no-default umbrella exited 0. The required all-targets workspace clippy exited
101 on the two baseline warnings described above. `git diff --check` passed;
the eight changed paths match the source allowlist, and the graph/TM changes
outside the integration work are comment-only.

CRC's restricted `make check` returned 2 on the same four MIDI initialization
tests recorded by CC; an approved host-access rerun returned 0. Restricted
`cargo outdated --root-deps-only --ignore-external-rel` returned 1 on crates.io
DNS; host-access rerun returned 0 and listed available upgrades without editing
dependencies. The planning verifier independently returned 1 with
`Assignment mismatch`: its line 70 still requires the held `cc-prompt.md` as
current. The source tests passed independently here, but a command's zero exit
does not turn that structural verifier or all-targets clippy green.

The production diff contains the approved default trait hook, typed error,
rack-local identity map, pre-mutation collision check, bounded world borrow,
one fixed-schedule hook call for a new kind, post-hook identity record, and
delegated chaining convenience. No new scheduler, seed/device/MIDI path,
manifest, dependency or runtime API appeared. The CC close report and artifact
inventory correspond to the actual planning evidence and preserve the initial
prompt and all six open-row origins.

## Bubble-up and next gate

The slice delivers a working prerequisite for arc09 but is not closed. Its
observed behavior does not require changing the arc09 slice sequence or the
sample/seed contracts. The required test-order correction and evidence repairs
are within the current slice; the two baseline clippy warnings and stale
planning verifier require a decision about exact source/checker scope before
the full gate can pass. These are routed in project-level
[`crc-escalation05.md`](../../crc-escalation05.md) through Duncan to CDC.
CRC will issue one preserved corrective CC prompt only after that directive,
then independently review the new candidate. No arc09/slice02 advancement,
source merge, release, or acceptance claim follows from this partial verdict.

## Iteration02 independent close

2026-09-17. Sofie/CRC reviewed the current
`cc-prompt-iteration02.md` assignment against clean source
`6a31c0f99241eccb0dcf0f23e9f64ac9dec2f94a` and CC planning commits
`0f4561b06708f36287abd519d5bf0d24a3e7bcde` and
`4fb9f2f755ab261634f93509d8a74ab19aabefc3`. The earlier partial verdict
above remains an accurate record for `d43078e`; it is superseded for the new
candidate. **All six I rows are now done, independently reproduced, and slice01
is closed.**

| Row | CRC disposition and independent basis |
| --- | --- |
| I-01 | Done. The prior once-per-type/rack review remains valid; the new source diff leaves production registration unchanged. Both current integration feature variants and host workspace matrices pass, including the independent-rack, legacy and conflict cases. |
| I-02 | Done. Source inspection confirms the new library test retains the rack, loads the two-instance RON fixture through `install`/`load_patch`, runs two fixed schedules to `(4,6)`, sets alpha's increment from 2 to 4 through `ParameterRegistry`, ends the world borrow, reinstalls, then checks unchanged totals/getters/hook count and reaches `(8,9)` after one more fixed run. The live test passes in both feature variants; the conflict test still checks the original registry/spawner/accessor behavior. |
| I-03 | Done. The old public fixture's `Update` versus `FixedUpdate` oracle and graph-on/off cases pass independently at the new source commit. The corrective diff does not change the scheduler. This is fixed-schedule installation proof, not sample-time or phase acceptance. |
| I-04 | Done. Both strict rustdoc variants pass independently. The corrective diff touches no legacy doc-only file or public API. |
| I-05 | Done, qualified by environment. Full workspace/all-feature/all-target Clippy with `-D warnings`, formatting, package tests, focused tests, both strict rustdoc commands and the complete planning verifier pass independently. Both restricted workspace matrices reproduce four MIDI initialization failures; host-access retries pass with 150 RT tests and three pre-existing ignored tests. Host `make check` and a separate `cargo outdated` exit zero. CC retained its restricted `make check` timing/MIDI and dependency-DNS failures alongside successful host retries. No ignored tests, dependency update or coverage waiver was introduced. The inherited 95% global coverage obligation stays open with Q-02/Q-03; hardware/MIDI timing acceptance is outside this slice. |
| I-06 | Done. The source corrective commit changes only the four directive05 paths, each within its test-only bound. The 62-row evidence table has all referenced raw logs and existing cwd paths. Comparing its first 20 rows with the prior table shows exactly ten cwd corrections; old commands, revisions, exits, results and log paths are unchanged. Every historical row explicitly says dirty/uncommitted and treats its eventual commit mapping as CC attestation/uncertain. Clean rows name `6a31c0f`. The six-row CC walk, artifact inventory and bubble-up are complete; both worktrees were clean at review intake. |

Independent command results at the clean source candidate: the new library test
passed with and without default features (1/1 each); integration tests passed
5/5 in each mode; `cargo test --locked -p oxurack-core` and `-p oxurack` passed.
Restricted full-workspace feature matrices each exited 101 at the same four
MIDI initialization cases (146 passed, three ignored in RT). Both host-access
workspace retries exited 0; full Clippy, format, both strict docs, host
`make check`, direct host dependency audit, source diff check and the planning
verifier exited 0. The verifier reported 23 preserved imported documents,
24 exact deferred successor rows, child denominator `0/0/2/3/0/1`, ledger
counts `7/4/6` and 297 valid local links. Re-running the six temporary
assignment-checker fixtures yielded the required five failures and one valid
alternate success. These are structural checks, not product acceptance.

### Bubble-up to arc09

Against arc09's first-slice row, the rack now installs executable module
behavior once per kind and per rack, preserves edited state through same-type
reinstall, rejects a conflicting kind before mutation, and passes the bounded
documentation and quality gates. The exact test-only Clippy repairs and
assignment-aware verifier required no change to the arc's three-slice sequence,
scope or sample/seed contracts. The restricted MIDI/timing and DNS failures are
environment-qualified and retained; host reproduction resolves this slice's
runnable gates without claiming hardware or timing acceptance. The full
specified slice scope is delivered. Exact-sample driving, phase/merge/observe
semantics and manual/live exclusivity remain slice02; seeded construction and
fresh-process replay remain slice03. Q-02/Q-03 and physical MIDI remain with
their named successors. The CC artifact inventory matches the actual slice
supporting files, and every issued prompt remains preserved. No arc-plan
amendment is required before opening slice02.
