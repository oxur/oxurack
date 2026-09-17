# Historical slice — standalone Turing Machine

Status: historical implementation; current acceptance unverified.

## Goal and scope

Preserve the standalone MIDI-domain Turing Machine engine design and its lineage
(`c0155c3`, `355962f`, `7158cb4`). The original design is in
[artifacts](artifacts/0001-turing-machine-to-midi-module-design-doc.md).

The Bevy/OxurackModule adapter is still remaining arc06 work. Do not count the
standalone engine as a completed rack adapter or silently expand this historical
slice into that future assignment. No current implementation assignment is issued.

## Verification and exit

On resumption, check the historical behavior against current crate tests and
examples, recording actual results in [ledger.md](ledger.md). The existing engine
must remain usable standalone; rack integration needs its own selected plan.
Artifacts belong in this slice's `artifacts/`. No independent review is claimed.

## Version History

- v1.0 — 2026-09-17: Added a minimal historical wrapper around the unchanged design;
  distinguished the implemented standalone engine from the remaining rack adapter.

## Current restart disposition — v2.0, 2026-09-17

The preceding implementation instructions are preserved historical text, not a
current CC assignment. Proposed close: carried over, pending CRC verification.
See [closing-report.md](closing-report.md) for both row dispositions. Remaining
reconciliation is assigned to [arc08-validation-and-release/slice01-restart-baseline](../../arc08-validation-and-release/slice01-restart-baseline/slice-plan.md), R-02;
this is a transfer of unresolved work, not behavioral acceptance or reimplementation.
No historical prompt has been reissued. No iteration count is inferred or reset.

### Version History — restart amendment

- v2.0 — 2026-09-17: Operator-authorized carryover candidate; arc08 restart
  baseline receives reconciliation because migration supplied no current review.
