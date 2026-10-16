# CC return — Arc07 Slice02 session observation

Date: 2026-10-15. Initial implementation return to CRC. Status is **open**;
SO-01 through SO-05 are not independently accepted. No Slice02, Arc07 or project
closure is claimed.

## Changes

Core producer staging now preserves owned chronology and stable cable IDs while
keeping typed Bevy messages. The facade owns a bounded journal, checked IDs,
independent cursors, lifecycle receipts, file origin and current cable/tag
queries. The rack-live owner supports editing, labels, tags, lifecycle commands
and bounded watch output on stderr while retaining the same synchronous MIDI
session. The exact two-module fixture, observation tests, README and a user test
runbook were added. Load/restart/unload cleanup is delivered after a successful
transaction and before Scope is configured for the new owner, so failed loads
preserve the current owner.

## Row disposition requested

All five ledger rows remain open. The ledger records concrete proposed evidence
and the residual checks CRC should apply. Most notably, the controlled fresh
musical comparison and some detailed retirement/removal and watch-failure
interactions from the prompt were not completed as Slice02 evidence. CRC should
not infer row acceptance from green aggregate gates or the CLI smoke runs.

## Validation

The ledger records the final command results. Restricted `make test` reproduced
146 passed, 4 CoreMIDI initialization failures, 3 ignored; the unchanged approved
host retry passed. Default and no-graph CLI subprocess runs exited successfully,
each with 20 MIDI trace rows on stdout and diagnostics/watch on stderr. No
endpoint was opened. No-graph first attempt produced zero samples because pause
arrived before the first due sample; the longer rerun produced 20 trace rows. A
fake-sender owner test covers cleanup delivery through the same session across
restart and unload. Existing future incompatibility and no-default unused-mut
warnings remain visible.

## Intake and process note

The contract readback was recorded before dependent edits, and the specific Rust
guideline sections applied are listed in the ledger. At closeout, CC found that
complete independent read extents for every entry in the prompt's required-read
manifest were not all recorded. CRC should decide whether that requires a
corrective intake iteration. This does not close or waive any criterion.

## Artifacts and provenance

- Product/test artifacts: source `crates/oxurack/tests/fixtures/session-observation.ron`,
  `crates/oxurack/tests/observations.rs`, and
  `docs/user-testing-rack-session.md`.
- Planning artifacts: this report, [ledger.md](ledger.md), and
  [slice-plan.md](slice-plan.md). Scratch logs remain under `/tmp` only.
- Source branch/base: `codex/oxurack-module-behavior-installation` /
  `e582aed6e87c234799921cc66a5dd55214f76afd`.
- Planning branch/base: `planning` /
  `fd3129fb22e014fd4759ae52f2dea5d72dbfeb3a`.
- Source commit: pending. Planning commit: pending.

Scope stayed within the prompt allowlist. No graph implementation, Scope or
playback source, note schema, terminal editor, external endpoint or future slice
was changed. CRC owns independent review and row disposition.
