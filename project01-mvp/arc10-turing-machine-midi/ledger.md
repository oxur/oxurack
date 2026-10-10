# Arc10 — composition ledger

Opened 2026-09-18 before implementation. Four planned slices as amended 2026-10-10; no accepted arc rows.
CRC assembles; CDC independently verifies; endpoint witness requires Operator.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| T-01 | Static installed Clock→TM→MIDI-out composes with documented sample latency | Load real seeded RON; observe pulse edges, coherent next-sample snapshots and exact MIDI order across multiple instances | correctness-grade | M1; Arc09 successor | open | — | At most one step per sample |
| T-02 | Fresh processes reproduce MIDI and preserve standalone TM capability | Compare the executable fixture byte/sample trace in two fresh processes; reuse unaffected Slice02 mapping/seed evidence and check affected feature builds | correctness-grade | P-02/P-07; catalog carryover | open | — | Locked implementation scope |
| T-03 | Output storage and the direct playback lifecycle are bounded and cleanup-aware | Exercise unread output, retrigger, silence/invalid note, partial send failure, stop/interrupt cleanup and explicit new-session recovery; inspect no stale replay | correctness-grade | RT/API carryover | open | — | No queue-atomicity or hard real-time claim; design v1.2 |
| T-04 | Selected software or hardware MIDI endpoint plays the integrated fixture | Named endpoint/config/patch/seed, explicit play/stop, Operator witness and offline-to-device comparison | correctness-grade | M1 Operator acceptance | open | — | Operator witness; availability deferral allowed under design section 5, never a simulated pass |
| T-05 | Four child slices, applicable quality gates and inherited obligations reconcile | Walk actual slice ledgers/reports and integrated source; directive08 gates; successor comparison including new deferrals | serious | Framework composition | open | — | No implicit closure of historical/project rows |

## Amendment — 2026-10-10

Operator-authorized recalibration during Slice02 review, directive08: T-01's
behavior remains; T-02 avoids redundant seed/mapping re-proofs; T-03 replaces
unissued async admission/recovery machinery with the direct runner contract;
T-04 permits an actual software endpoint and an explicit availability deferral;
T-05 counts four slices (was five). All five rows remain open. Physical timing
stays project P-03/Arc08; the old protocol is preserved in Git at `32e6934`.
