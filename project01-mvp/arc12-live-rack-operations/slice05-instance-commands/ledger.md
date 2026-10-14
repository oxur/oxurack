# Instance commands — acceptance ledger

Opened2026-10-13 before implementation. Six IC rows, all done. CRC accepts
iteration01 at27ca00f on2026-10-14; Slice05 closed after one correction1/5.
IC-01/02/03/05 retain their CRC2026-10-13 acceptance at7404fe2 with checked
applicability; IC-04/06 freshly accepted at27ca00f. See crc-verification.md.
No current CC assignment; Arc12 independent composition/CDC gate remains open.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| IC-01 | In-place removal retires exact owned hierarchy and incident topology in order, preserving survivors and failure state | Actual A→B→C/B→B/parallel removal; index/World/graph and ordered events; direct graph incoming/outgoing/self/name-map regressions; survivor/control/Scope and admission/error boundaries | correctness-grade | CDC12 D1/D3/D4; Arc05 runtime membership | done | attested: `cargo test --locked -p oxurack-core module_removal_orders_all_incident_cables`; `cargo test --locked -p oxurack --test instance_commands`; lifecycle removal/nested-owner tests; reproduced: CRC2026-10-13 at7404fe2, crc-verification.md | CRC accepted behavior at7404fe2; dated evidence retained only after delta review. Parent gates remain open. |
| IC-02 | Owned references and typed handles reject wrong type, cross-rack and stale lifetime/instance use before mutation | Real Clock/LFO/TM bindings; same-kind different Rust type; removed/reused/replaced/restarted/unloaded and failed-load/survivor cases; bound ports/knobs and dotted/tuple names | correctness-grade | CDC12 D3; Arc05 handles | done | attested: `cargo test --locked -p oxurack --test instance_commands` (14 tests); reproduced: CRC2026-10-13 at7404fe2, crc-verification.md | CRC accepted behavior at7404fe2; dated evidence retained only after delta review. Parent gates remain open. |
| IC-03 | Every synchronous command uses actual behavior/provenance and global BPM persists independently of Clock | All eight variants; typed events/graph without tick; real save/load, Panic/reset; global133/Clock120, invalid BPM and low-level unsupported cases | correctness-grade | CDC12 D3 commands/global tempo | done | attested: `cargo test --locked -p oxurack commands::tests` (4 tests); `cargo test --locked -p oxurack --example rack-session` (2 tests); `instance_commands` (14 tests); reproduced: CRC2026-10-13 at7404fe2, crc-verification.md | CRC accepted behavior at7404fe2; dated evidence retained only after delta review. Parent gates remain open. |
| IC-04 | Explicit FIFO queue preserves bounded outstanding slots, failures, identity and exactly-once reset cancellation results |64 accepted/65th rejected; checked ID exhaustion; execution-time path/name resolution; App swap retains tail; direct/queued reset, cancelled capacity, reset failure and one-time collection | correctness-grade | CDC12 D1/D3 queued load/save | done | reproduced: CRC2026-10-14 at27ca00f, crc-verification.md Iteration01 independent review; prior CC attestation remains in closing-report.md | Exact public API/privacy, bounded queue and immediate failed-reset state proof pass. |
| IC-05 | Selected sink removal appends exact bounded cleanup before releasing reservation, preserving other sinks and first fault | Actual sample300 literal NoteOff62/CC123/CC120; zero-sink drain, channel reuse/accounting/storage and other dirty sink/peak regressions | correctness-grade | CDC12 D2/D3; single MIDI owner | done | attested: `cargo test --locked -p oxurack-mod-midiout` (10 tests); selected-silence/channel-reuse and first-fault tests; live session trace; reproduced: CRC2026-10-13 at7404fe2, crc-verification.md | CRC accepted behavior at7404fe2; dated evidence retained only after delta review. Parent gates remain open. |
| IC-06 | Runnable instance/command session composes with persistence, Scope and native feature gates; inherited scope remains accounted for | rack-session real handles/removal/FIFO/failure/save plus fresh/default-no-graph traces; native core/MIDI-out/graph/facade/workspace/lint/Clippy/docs/builds; six-row/Arc05 and successor walk | serious | CDC12 D4; directive08; Arc12 composition | done | reproduced: CRC2026-10-14 at27ca00f, crc-verification.md Iteration01 independent review; prior CC attestation remains in closing-report.md | Collision-safe runner, exact final/reference cleanup, API docs and current recipes/gates pass. |

## CRC initial review — 2026-10-13

Six opening rows accounted for, four done/two open; no deferral/no-op or scope
amendment. CC return7404fe2/a3f350e remains attested in closing-report.md;
CRC independently accepts IC-01/02/03/05 at that source/date. Native gates, exact
host retry and normal musical recipes pass; restricted failures preserved.
Initial cc-prompt.md remains unchanged. Iteration01 repairs only the unresolved
contract/recipe/documentation/evidence gaps; no slice or Arc12 acceptance.

## CRC iteration01 final disposition — 2026-10-14

All six IC rows done; four prior acceptances retained with byte/delta rationale,
two remaining rows independently accepted at27ca00f. One correction completed1/5.
No criterion change or new unresolved Slice05 finding. Source/main unchanged by
CRC; initial failed review and both issued prompts remain preserved. Arc12 now
has five accepted children/27 rows; composition/CDC acceptance stays separate.
