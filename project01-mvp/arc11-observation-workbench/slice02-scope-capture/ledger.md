# Arc11 Slice02 Scope capture — ledger

Opened 2026-10-11 before implementation; five rows open. Current assignment
`cc-prompt.md`; binding behavior in sibling slice-plan.md. CC attestation remains
proposed-done until CRC reproduces; no parent/successor acceptance is implied.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S-01 | Selected Float/Bipolar/Gate/Raw values are copied at Observe with correct sample indices | Actual loaded module/cable path, literal LFO/Clock/Raw sequences, same-sample propagated input, core stop latch | correctness-grade | Arc11 Observe contract | open | — | No port mutation, second clock or raw running recheck |
| S-02 | Mixed record storage stays bounded and lifecycle is explicit | Exact oldest-overwrite window/count; stop/resume, clear, reconfigure, disable and successful patch reload | correctness-grade | Arc11 bounded observation | open | — | Capacity in records; partial sample window allowed; no duplicate observer |
| S-03 | Capture configuration validates before mutation in both feature modes | Missing patch/module/port, duplicate/Midi selector, zero capacity/running rejects preserve prior capture | correctness-grade | Arc11 pre-play validation | open | — | PatchHandle/child ports, no graph/label fallback |
| S-04 | Actual MIDI copies retain ordered generation/delivery evidence without altering playback | Capture-enabled/disabled loaded trace comparison; actual runner fake-sender success/failure/cancel and cleanup | correctness-grade | Arc11 sole-drain ownership | open | — | Generated versus successful send; held MIDI rejected, cleanup excluded |
| S-05 | Public API, CSV example and applicable native gates deliver usable export | CSV schema/escaping/error/empty output; documented built CLI run and original/LFO hashes; tests/lint/docs/build | serious | Directive09 runnable capture | open | — | No endpoint, coverage/freshness campaign or validator prerequisite |
