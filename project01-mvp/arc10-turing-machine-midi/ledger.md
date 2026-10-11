# Arc10 — composition ledger

Opened 2026-09-18 before implementation. Four planned slices as amended 2026-10-10; all four children/22 rows accepted. No CDC-accepted arc rows.
CRC assembles; CDC independently verifies; endpoint witness requires Operator.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| T-01 | Static installed Clock→TM→MIDI-out composes with documented sample latency | Load real seeded RON; observe pulse edges, coherent next-sample snapshots and exact MIDI order across multiple instances | correctness-grade | M1; Arc09 successor | open | [CRC assembly](crc-verification.md); [closure candidate](closing-report.md) | At most one step per sample |
| T-02 | Fresh processes reproduce MIDI and preserve standalone TM capability | Compare the executable fixture byte/sample trace in two fresh processes; reuse unaffected Slice02 mapping/seed evidence and check affected feature builds | correctness-grade | P-02/P-07; catalog carryover | open | [CRC assembly](crc-verification.md); [closure candidate](closing-report.md) | Locked implementation scope |
| T-03 | Output storage and the direct playback lifecycle are bounded and cleanup-aware | Exercise unread output, retrigger, silence/invalid note, partial send failure, stop/interrupt cleanup and explicit new-session recovery; inspect no stale replay | correctness-grade | RT/API carryover | open | [CRC assembly](crc-verification.md); [closure candidate](closing-report.md) | No queue-atomicity or hard real-time claim; design v1.2 |
| T-04 | Selected software or hardware MIDI endpoint plays the integrated fixture | Named endpoint/config/patch/seed, explicit play/stop, Operator witness and offline-to-device comparison | correctness-grade | M1 Operator acceptance | open | [CRC assembly](crc-verification.md); [closure candidate](closing-report.md) | Operator witness; availability deferral allowed under design section 5, never a simulated pass |
| T-05 | Four child slices, applicable quality gates and inherited obligations reconcile | Walk actual slice ledgers/reports and integrated source; directive08 gates; successor comparison including new deferrals | serious | Framework composition | open | [CRC assembly](crc-verification.md); [closure candidate](closing-report.md) | No implicit closure of historical/project rows |

## Amendment — 2026-10-10

Operator-authorized recalibration during Slice02 review, directive08: T-01's
behavior remains; T-02 avoids redundant seed/mapping re-proofs; T-03 replaces
unissued async admission/recovery machinery with the direct runner contract;
T-04 permits an actual software endpoint and an explicit availability deferral;
T-05 counts four slices (was five). All five rows remain open. Physical timing
stays project P-03/Arc08; the old protocol is preserved in Git at `32e6934`.

## Slice02 acceptance / Slice03 issue — 2026-10-10

Clock and TM adapter child slices are independently accepted at `a19cfd1` and
`b2ffb16`. Slice03 installed MIDI-out/trace is now issued with five M rows open.
This supplies partial T-01/T-02 support only; sink/byte composition, note cleanup
and endpoint witness remain unproved. T-01–T-05 remain open. Four-slice denominator
and directive08 witness/release policy are unchanged.

## Slice03 acceptance — 2026-10-10

CRC accepted five M rows at `2942c59`; [independent record](slice03-midi-output/crc-verification.md) reproduces installed trace, literal ordering, bounded admission and native gates. T-01/T-02 now have actual sink/trace support, T-03 has buffer/transition support; direct playback/cleanup and endpoint witness remain Slice04 work. All five composition rows remain open pending assembly and CDC review.

## Slice04 issue — 2026-10-10

[Direct playback/cleanup assignment](slice04-output-runtime/cc-prompt.md) issued at accepted `2942c59` with five O rows open, including actual endpoint witness. No composition row is closed by issue; unavailable O-05 must return as an explicit gap for CDC T-04 disposition.

## CRC Arc10 assembly — 2026-10-11

Four accepted children,22 done child rows, and an actual MIDI Bus1 musical witness
are assembled at source `65522f1`. CRC reproduces current composition, including
a loaded two-sink RON against lexical/spawn order and fresh-process replay, and
reconciles unchanged child proof/native gates and successors. T-01–T-05 remain
open for CDC's independent composition/design verdict, with CRC recommending
done for all five. No new deferral/no-op and no availability gap. Required next
record: CDC-authored `cdc-verification.md`; no arc close or Arc11 issue is claimed
by these two CRC-authored assembly records.
