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
