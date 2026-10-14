# Live controls — acceptance ledger

Opened 2026-10-14 before implementation. All five rows open. CC supplies proposed
results; CRC independently accepts. Authority: slice plan and cc-prompt.md.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LC-01 | Commands change actual live parameters and reject invalid input without corrupting the session | Real getter/setter/save values; malformed→valid input and typed validation; command replies/provenance | correctness-grade | Operator live controls | open | — | Scalar/bool/string; scale via patch file |
| LC-02 | Waiting input and command processing coexist with continuous single-owner playback | Real 200+1000-sample subject/control/unchanged comparison; bounded fairness; paused 0; no wall-time catch-up | correctness-grade | Existing direct runner | open | — | Public read-only is_running, no shadow flag |
| LC-03 | Pause/reset/shutdown preserve semantics and release output ownership honestly | Exact cleanup and sample state; EOF/quit/Ctrl-C/read/send/write failure paths; Generated versus Sent; blocked-reader exit | correctness-grade | Arc12 lifetime/MIDI contract | open | — | No hidden second drain or stuck join |
| LC-04 | Scope inspection/export and save expose current state without interrupting simulation continuity | Full retained records, export collision/error, save/fresh configuration replay and continuing control | correctness-grade | Arc11/12 composition | open | — | No performance checkpoint claim |
| LC-05 | Runnable terminal recipe, both feature modes and affected native gates pass | Actual idle-open stdin/play/edit/pause/save/quit subprocess; fake native output; default/no-graph/native/lint/docs and old example regressions | serious | Playable-first policy | open | — | No hardware/coverage/soak expansion |

## Execution notes

CC records actual required-reading coverage/readback and concise command results
here, linking the closing report on return. No prefilled completion claims.
