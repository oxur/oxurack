# Arc10 — composition ledger

Opened 2026-09-18 before implementation. Five planned slices; no accepted rows.
CRC assembles; CDC independently verifies; endpoint witness requires Operator.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| T-01 | Static installed Clock→TM→MIDI-out composes with documented sample latency | Load real seeded RON; observe pulse edges, coherent next-sample snapshots and exact MIDI order across multiple instances | correctness-grade | M1; Arc09 successor | open | — | At most one step per sample |
| T-02 | Fresh processes reproduce MIDI and preserve standalone TM capability | Compare exact byte/sample trace twice; changed seed/name and reordered spawn controls; full engine mapping/feature audit | correctness-grade | P-02/P-07; catalog carryover | open | — | Locked implementation scope |
| T-03 | Output admission and session lifecycle are bounded and cleanup-aware | Force full/absent/stalled sink, same-note retrigger, silence, disconnect, stop/panic/recovery/shutdown; inspect no stale replay | correctness-grade | RT/API carryover | open | — | Atomic admission is not atomic wire delivery |
| T-04 | Selected physical endpoint plays the integrated fixture | Named endpoint/config/patch/seed, explicit play/stop, Operator witness and offline-to-device comparison | correctness-grade | M1 Operator acceptance | open | — | Hardware gate; not final P99 timing |
| T-05 | Five child slices, quality gates and inherited obligations reconcile | Walk actual slice ledgers/reports and integrated source; full gates; exact successor comparison | serious | Framework composition | open | — | No implicit closure of historical/project rows |
