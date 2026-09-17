# CDC directive03 — close carryovers and implement module installation

Issued 2026-09-17 by Sofie/CDC to CRC through Duncan. Response to
[crc-escalation03.md](crc-escalation03.md); predecessor
[cdc-directive02.md](cdc-directive02.md). Reviewed actual planning HEAD
`390d2aa2c3449f8a6e4bbaff3df6c64d821ce4ea`, resolved from Git (use this full SHA,
not the extra-character form in the relay text). Implementation remains
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. The decision state is the commit
introducing this directive, project v3.0, retired arc v3.0 and arc09/slice01 v1.0.

## Decisions on every requested item

| Request | Decision / reason |
| --- | --- |
| Six independent arc reviews | Accepted for carried-over retirement: arcs01–06 each have cdc-verification.md, both A rows deferred. Actual children 0/0/2/3/0/1 checked. No behavioral composition, hardware or musical-use acceptance. |
| Concrete successor ownership | All 24 inherited rows mapped in artifacts/cdc-successors03.tsv. New arc09 owns executable-rack prerequisites; arc10 TM/MIDI; arc11 observation workbench; arc12 remaining live operations; arc13 remaining catalog. Arc07 interaction and arc08 quality/system gates continue. Future IDs are roadmap reservations, not unissued implementation permission. |
| Installation and tick/phase contracts | Approve default module behavior hook, per-rack idempotent type/kind registration and pre-mutation collision error. Split installation, exact samples/phase semantics, and seed context into arc09 slices01/02/03. Only slice01 issued now. |
| Seed/reset | Explicit pre-construction spawn context and versioned fixed byte-level seed derivation; process-boundary tests mandatory. Stop, seeded restart and engine.reset have distinct semantics. Implement in slice03, not slice01. |
| Adapter and MIDI mapping | TM crate's opt-in rack feature wraps the standalone engine; full mapping inventory retained. Initial typed snapshot/step-trigger path feeds MIDI-out's ordered whole-message batches and capture sink; held scalar Midi is not an event stream. No algorithm duplication. Detail before arc10 assignments. |
| Clock and runtime ownership | Internal simulation-driven minimal Clock pulled into M1; explicit RT output-only service and single session owner. No competing RT clock, silent note-off drops or inferred hardware/jitter acceptance. External 24-PPQN sync remains a separate retained gate. |
| Workbench | Clock/LFO/Scope after M1, with bounded sample/event observation and independent display cadence. No full TUI/GUI or Chord requirement added. |
| Documentation failures | Q-01 bounded into slice01 to make the public API doc gate pass. CDC rerun found 11 link diagnostics across TM and graph, with raw log retained. Doc-only repairs authorized in three named files; no lint suppression. |
| Coverage/profile warnings | Q-02 arc08/slice02-coverage-provenance before Q-03 arc08/slice03-coverage-hardening; 95% unchanged. No arbitrary exclusions or dependency upgrades. 93.47% remains CRC's reproduced but limited observation, not accepted coverage. |

Full design: [execution-contract.md](arc09-executable-rack/slice01-module-behavior-installation/artifacts/execution-contract.md).
It makes choices for later work but authorizes no hidden scope inside slice01.
Operator authority is the explicit TM-first/workbench-second priority and prior
permission to close as carried over; no new release, device operation, scope
reduction or broader UI decision is inferred.

## CRC return assignment

1. Read the project/arc/slice plans and this directive against actual Git state;
   acknowledge directive03 in the project's Design handoff history with your
   context and revisions. Reconcile divergent state before execution.
2. Relay through Duncan the complete CC implementation assignment:
   `arc09-executable-rack/slice01-module-behavior-installation/cc-prompt.md`.
   All I-01–I-06 are open. CC uses an isolated implementation branch/worktree,
   keeps planning on planning, and commits each exact allowed file set.
3. Review source, tests and evidence independently. Require actual behavior after
   patch load, idempotence, conflict non-mutation and correct schedule, not just
   registration counters. Preserve negative/failed attempts and all known global
   gaps. Record crc-verification.md only when review occurs. No implementation
   repair by CRC may be self-accepted.
4. After slice01 acceptance, CRC may prepare slice02 and then slice03 within the
   approved arc09 design, preserving issued prompts. Escalate changed contracts,
   unresolved compatibility, cross-arc implications, evidence waivers or the
   five-refinement cap; next project exchange is 04 unless already reserved.
5. At arc09 close, assemble composition evidence for E-01–E-04 and relay to CDC.
   CDC independently checks the integrated fixtures and prepares arc10's first
   slice. Do not silently advance into an unissued new arc.

The inherited arc retirement decision is final as an administrative status;
original unmet criteria remain obligations of their successors. Do not rerun
history merely to manufacture done rows, reopen historical prompts, or edit
accepted baseline logs. [CDC review evidence](artifacts/cdc-review03.txt) records
exact checks and limits. Current planning uses the new slice's structural checker;
prior checkers remain frozen evidence of their original seven-slice shape.

Return to Duncan: acknowledgement, exact CC prompt/source/planning state, eventual
six-row reviewer verdict, doc-gate status, failures and proposed next slice path.
No push/merge/release. Explicit-path commits and immediate within-plan advancement
follow Expedited Mode; independent and Operator gates remain binding.
