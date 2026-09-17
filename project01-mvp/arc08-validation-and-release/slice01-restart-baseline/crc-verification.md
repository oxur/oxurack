# CRC verification — restart baseline, iteration01

2026-09-17. Reviewer: Sofie/CRC, Operator-selected GPT-5.6 Sol context.
CC packet: `df6ec0a6906c2b16c049db8f267145861b623c41`.
Implementation source: `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
Verdict: **accept the six-row evidence baseline**. This verifies observation and
accounting, not product behavior, release readiness, or historical acceptance.

| Row | CRC disposition and independent evidence |
| --- | --- |
| R-01 | done. Reproduced `make check`, locked build, all-feature clippy, format, three workspace test variants and all named feature probes. The three workspace variants initially failed in the restricted environment at the same four MIDI initialization tests (146 passed, 4 failed, 3 ignored); normal tool escalation gave all three exit 0. This preserves both outcomes and identifies MIDI access as the environmental distinction. Checked the 53-row validation table against the raw-log inventory (44 pass, 7 fail, 2 blocked), tool/source pin and lockfile. |
| R-02 | done for accounting only. Ran `verify-planning.py` and independently compared all 24 register criteria with the six historical slice and six arc ledgers: exact 12+12 row set, domain owners, re-entry and no completion upgrade. Historical behavioral reconciliation remains deferred in its source rows. |
| R-03 | done for source observation only. Rechecked the TM engine constructor/tick, metadata-only rack installation and module-spawner contract, unused patch master seed, bridge queue, command stubs and no-ops, and absence of rack Clock/LFO/Scope modules. The eleven TM/WB observations are appropriately bounded; no adapter or MIDI output demonstration is inferred. |
| R-04 | done for failure/obligation accounting. Independently reproduced the two rustdoc unresolved links. Coverage runs succeed with MIDI access: a clean `cargo llvm-cov --locked --workspace --all-features --lib --summary-only` reported 13,560 lines, 886 missed, **93.47% line coverage**, below the inherited 95% target. The tool still warned that nine functions have mismatched profile data after `cargo llvm-cov clean --workspace`; interpret the percentage with that limitation. Three RT tests remain ignored; hardware timing, benchmarks, 24-hour soak and Operator musical use remain unrun. |
| R-05 | done. `verify-planning.py` passed for 23 imported documents, 24 rows and current local links. `git diff --check` passed. CC's commit changed only the 68 files in this restart slice; implementation source and lockfile remained unchanged. |
| R-06 | done. The CC report walks six rows, names the assignment and pin, inventories four principal artifacts and 61 logs, and bubbles TM/workbench dependencies to the owning arcs without making design decisions. The additional CRC coverage observation belongs in the continuing Arc08/P-05 quality backlog. |

The independent rerun used `CARGO_NET_OFFLINE=true` where appropriate. The
restricted MIDI-test failures and CC's coverage failures are retained as raw
observations rather than overwritten by the elevated reruns. The clean coverage
rerun passed 12, 433, 65, 59, 115 and 150 library tests in the six reported
groups, with 3 ignored; it did not run hardware, examples, benches or a DAW.
`cargo outdated --root-deps-only --ignore-external-rel` also ran successfully and
reported available updates; no dependency changes were made.

Reviewer checks: `python3 artifacts/verify-planning.py`, exact source-ledger
comparison, raw TSV/log inventory, source-anchor searches, `git diff --check`,
locked build/lint/test probes, documentation and both native/direct coverage
commands. First coverage runs accumulated mismatched profiles; clearing only
generated llvm-cov workspace artifacts and rerunning did **not** remove the nine
function warnings. The result is sufficient to retain the below-target finding,
but a future quality slice must investigate the warning before final coverage
acceptance. No structural arc design was changed by this review.

Open product gates: broken docs links; below-target coverage and profile warning;
three ignored RT tests; named physical MIDI/DAW timing; 10/50/100-module resource
benchmarks; 24-hour soak; Operator musical acceptance; and the TM-in-rack/MIDI
and Clock/LFO/Scope implementation gaps. Arc08 and project P-05 own validation;
CDC assigns cross-arc implementation successors under the selected priority.
