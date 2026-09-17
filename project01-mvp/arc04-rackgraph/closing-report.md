# Arc carryover assembly — RackGraph

2026-09-17. Sofie/CRC assembled this packet against the original arc plan,
two-row ledger, restart R-02 register and source pin
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
Status: **proposed carried-over close, pending independent CDC verification**.

Actual children (3): slice01-foundation (closed-carried-over, two deferred S rows); slice02-navigation (closed-carried-over, two deferred S rows); slice03-diagnostics (closed-carried-over, two deferred S rows).

| Row | Proposed status | Reason, owner and re-entry |
| --- | --- | --- |
| A-04-01 | deferred | Graph queries, including later `parameters_of`, exist; freshness under future runtime edits and all edge cases remain unaccepted. The RackGraph arc retains this capability. Assign feature-on/off, query, getter and post-mutation freshness validation; coordinate mutation contracts with arc05. |
| A-04-02 | deferred | Walked all 3 actual child ledgers and their CRC records; no child behavior was promoted to accepted. The RackGraph arc retains child and historical deferrals. CDC independently checks this assembly and assigns successor work before final arc close. |

Two of two arc rows are accounted for: 0 done, 2 deferred, 0 no-op. Graph queries, including later `parameters_of`, exist; freshness under future runtime edits and all edge cases remain unaccepted.
The original arc criterion and child obligations remain in their ledgers and
the 24-row register. Assign feature-on/off, query, getter and post-mutation freshness validation; coordinate mutation contracts with arc05. Historical implementation is evidence of code,
not independent composition or musical acceptance. No source or dependency
change, scope deletion, target reduction, publication or release is authorized.
CRC's assembly check is in `crc-verification.md`; only CDC can perform the
second, independent arc review and finalize status.
