# Terminal workbench — ledger

Opened 2026-10-11 before implementation; four rows open. Current assignment
cc-prompt.md, binding sibling plan/prompt. CC proposed-done is attestation;
CRC independently verifies. Parent composition and CDC gate remain open.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| B-01 | Terminal view links patch, retained scalar snapshots and MIDI events with honest bounds/sample labels | Literal loaded sequences, selector order, independent generated/sent bytes, mixed overwrite/empty/tail/escaping cases | correctness-grade | Arc11 usable inspection | open | Pending | Immutable retained-window view, no evicted-data claims or extra drain |
| B-02 | View/CSV CLI composes with existing offline/live selection and validates before play | Enabled combinations, orphan/zero/oversized/list errors; file/selector preflight; stdout and CSV compatibility | correctness-grade | Arc11 integration; accepted Scope contract | open | Pending | At least one Scope output enables capture; no-view legacy unchanged |
| B-03 | Post-stop reporting preserves primary failures and lifecycle without changing performance | Actual runner with fake sender, cancellation/failure; injected CSV/view Write/flush errors; immutable before/after capture/time | correctness-grade | Sole drain and failure ownership | open | Pending | Attempt all requested reports after cleanup; Sent is software send success |
| B-04 | README supplies runnable view/export/new-run recipes and affected native checks pass | Built fixture commands, exact hashes/row counts/overwrites, repeat run, default/no-default tests and native gates | serious | Directive09 third executable increment | open | Pending | No physical endpoint required; Arc11 closure still requires CDC composition |
