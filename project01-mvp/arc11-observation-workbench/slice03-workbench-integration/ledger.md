# Terminal workbench — ledger

Opened 2026-10-11 before implementation; four rows open. Current assignment
cc-prompt.md, binding sibling plan/prompt. CC proposed-done is attestation;
CRC independently verifies. Parent composition and CDC gate remain open.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| B-01 | Terminal view links patch, retained scalar snapshots and MIDI events with honest bounds/sample labels | Literal loaded sequences, selector order, independent generated/sent bytes, mixed overwrite/empty/tail/escaping cases | correctness-grade | Arc11 usable inspection | open | CC proposed-done at `f0b045330c8bb74fb3267ba8ef0a515dfaf1012e`; saw-up 0–3, exact MIDI bytes, capacity/empty/escape and immutable snapshot tests | CRC independently verifies retained-window semantics and output literals |
| B-02 | View/CSV CLI composes with existing offline/live selection and validates before play | Enabled combinations, orphan/zero/oversized/list errors; file/selector preflight; stdout and CSV compatibility | correctness-grade | Arc11 integration; accepted Scope contract | open | CC proposed-done at `f0b045330c8bb74fb3267ba8ef0a515dfaf1012e`; parser/preflight tests; stdout and accepted Slice02 CSV byte equality | CRC independently verifies CLI contract and compatibility evidence |
| B-03 | Post-stop reporting preserves primary failures and lifecycle without changing performance | Actual runner with fake sender, cancellation/failure; injected CSV/view Write/flush errors; immutable before/after capture/time | correctness-grade | Sole drain and failure ownership | open | CC proposed-done at `f0b045330c8bb74fb3267ba8ef0a515dfaf1012e`; injected ordered errors and real send-failure finalization after cleanup | CRC independently reviews finalizer order and runner integration |
| B-04 | README supplies runnable view/export/new-run recipes and affected native checks pass | Built fixture commands, exact hashes/row counts/overwrites, repeat run, default/no-default tests and native gates | serious | Directive09 third executable increment | open | CC proposed-done at `f0b045330c8bb74fb3267ba8ef0a515dfaf1012e`; README recipes, pinned hashes, repeat/bounds/empty and all gates recorded in closing report | CRC reviews runnable evidence and documentation; no endpoint witness required |
