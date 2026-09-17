# arc06-module-catalog — ledger

Opened 2026-09-17 during retrospective migration. These are current reconciliation
and composition obligations, not reconstructed historical acceptance records.
All rows start open. One-Contributor checks will be recorded as attested; any
required independent or Operator acceptance must be attributed when it occurs.
Implementation status and commit pointers are in the project backlog.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| A-06-01 | Baseline musical modules work standalone and in a seeded rack | Per-module tests and composed TuringMachine-to-MIDI-out then representative multi-module patches | correctness-grade | Project roadmap; historical plans | deferred | `cdc-verification.md`; `crc-verification.md`; reproduced transfer accounting | Capability/reconciliation not accepted; successor IDs and re-entry in CDC record and project cdc-successors03.tsv |
| A-06-02 | Child work and deferred findings are accounted for at arc composition | Walk current child ledgers and project backlog; compare delivered scope with arc capability | serious | Framework composition discipline | deferred | `cdc-verification.md`; `crc-verification.md`; reproduced transfer accounting | Capability/reconciliation not accepted; successor IDs and re-entry in CDC record and project cdc-successors03.tsv |

## Restart disposition — 2026-09-17

Both rows remain open pending CRC assembly and CDC independent arc review.
Candidate disposition is `deferred` for both; evidence is insufficient for a
`done` claim. The [restart register](../arc08-validation-and-release/slice01-restart-baseline/artifacts/carryover-register.tsv)
retains each row, original capability owner, receiving R-02 and re-entry condition.
Source criteria are preserved. CRC must account for children before writing the
arc close; CDC verifies the carryover rather than signing off its own proposal.

## CRC assembly — 2026-09-17

CRC assembled and self-checked the two-row proposed close in
`closing-report.md` and `crc-verification.md`. Actual children: 1.
Both A rows remain **open** until independent CDC review. Their proposed
disposition is deferred, retaining this arc as owner and its original criteria.

## CDC final disposition — 2026-09-17

Both rows are deferred; arc closed-carried-over under the Operator's retirement
authorization. [CDC verification](cdc-verification.md) checks CRC's actual packet.
The preceding open/pending statements are preserved historical states, superseded
by this disposition. Original criteria remain binding in the named successors.
