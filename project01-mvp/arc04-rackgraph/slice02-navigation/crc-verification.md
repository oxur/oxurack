# CRC carryover verification — slice02-navigation

2026-09-17. Sofie/CRC reviewed the original slice plan, its two-row ledger,
CDC's proposed closing report, the 24-row restart register and the current
source baseline at `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
The Operator authorized carried-over retirement. Verdict: **close this historical
slice as carried over**, with S-01 and S-02 both **deferred**, neither done.

| Row | Reason and exact re-entry |
| --- | --- |
| S-01 | Graph navigation/query code and the graph feature are present; current workspace passes, but this does not independently establish every historical navigation and freshness requirement. Re-enter only through a CDC-assigned successor that reproduces the original behavior against current source; the historical plan remains the acceptance input. |
| S-02 | Arc04 retains full navigation-query semantics and graph freshness across runtime topology and parameter edits. Re-enter with a CDC-assigned arc04 slice using representative loaded and live-mutated patches in both feature modes. |

The owning domain and both original criteria remain in this ledger and in the
restart register's R-02 accounting. This is an administrative close, not
independent behavioral acceptance. The original plan and candidate report
remain historical evidence; project and arc capability gates remain open.
