# CRC carryover verification — slice01-notifications

2026-09-17. Sofie/CRC reviewed the original slice plan, its two-row ledger,
CDC's proposed closing report, the 24-row restart register and the current
source baseline at `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
The Operator authorized carried-over retirement. Verdict: **close this historical
slice as carried over**, with S-01 and S-02 both **deferred**, neither done.

| Row | Reason and exact re-entry |
| --- | --- |
| S-01 | Notification types and existing load/parameter emission have source and passing workspace-test evidence, but the historical A-everywhere emission and ordered mixed-event consumer contract were not independently reproduced. Re-enter only through a CDC-assigned successor that reproduces the original behavior against current source; the historical plan remains the acceptance input. |
| S-02 | Arc03 retains removal/unload emission sites, command notification consistency and cross-type ordering. Re-enter with a CDC-assigned arc03 slice that inventories every historical emission site and demonstrates ordering against an actual consumer. |

The owning domain and both original criteria remain in this ledger and in the
restart register's R-02 accounting. This is an administrative close, not
independent behavioral acceptance. The original plan and candidate report
remain historical evidence; project and arc capability gates remain open.
