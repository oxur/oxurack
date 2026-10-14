# Arc07 Slice01 — CC closing report

Date: 2026-10-14. Assignment: [`cc-prompt.md`](cc-prompt.md), initial pass,
correction 0/5. CC proposes LC-01–LC-05 done; CRC disposition is pending.

## Delivered

Added the paused-at-start `rack-live` terminal example with a single synchronous
rack/output owner, bounded stdin reader, typed live controls, Scope inspection
and CSV export, patch save, MIDI trace/native output selection, and orderly
pause/reset/exit handling. Added the documented read-only `Oxurack::is_running`
observation and an interactive README recipe. Existing finite examples remain
covered by their original tests.

## Evidence

The ledger records the pre-edit contract/readback, LC-row results, all commands,
the initial restricted CoreMIDI failures and the successful host retry. Both
default and no-default actual subprocess smokes passed while stdin remained open;
they exercised playback, edits, malformed-line recovery, pause stability,
save/export, stdout routing, and clean quit/EOF. Fake sender tests cover native
delivery without a physical endpoint operation.

Validation passed: both oxurack all-target test modes, both example build modes,
`make lint`, strict all-feature Clippy, warning-free rustdoc, formatting check,
and full `make test` host retry. The no-default test build retains one existing
unused-mut warning in `crates/oxurack/src/cables.rs:231`. The restricted initial
`make test` run's four CoreMIDI initialization failures are preserved in the
ledger; the exact host retry passed. Three existing runtime tests remain
ignored. No product defect or unaddressed validation failure was found.

## Return and disposition

Correction count is 0/5. Source and planning artifacts are committed separately;
their commit hashes and final worktree state are reported to the Operator in the
handoff. This report does not claim CRC acceptance. Duncan: please route this
Slice01 return and its two commits to CRC for independent review before any
successor slice is issued.

## Iteration01 return — correction 1/5

Date: 2026-10-14. Assignment: [`cc-prompt-iteration01.md`](cc-prompt-iteration01.md).
This is a corrective proposed-done return; LC-01–LC-05 remain open until CRC
independently reviews the new evidence. The initial report and CRC disposition
above remain preserved.

The frontend now configures bounded Scope for every loaded rack, reports sorted
live status/help, schedules one due sample fairly across command work, traces rack
MIDI in both output modes, and retains primary plus cleanup/reader failures. The
new interaction evidence covers native/trace-only parity and Generated/Sent
records, dirty-voice cleanup, real pause/resume against a public control, live
Scope/save continuation, and a 2,000-sample fresh replay against an equivalent
independently loaded configuration and an unchanged negative control.

Both default and no-default CLI subprocess sessions kept stdin open while sample
time advanced. They exercised edits, malformed recovery, status/help, Scope,
save/export, reset/pause, quiet stdout routing, and clean exit. EOF exited cleanly;
a dirty-voice SIGINT with stdin still open exited0 and traced NoteOff/CC123/CC120.
Pinned finite traces reproduced: MIDI2000, 13 rows,
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`; LFO10000,
70 rows, `988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.

Final `oxurack` all-target tests passed with default and no-default features;
both example builds, `make lint`, strict Clippy, warning-free rustdoc, and diff
checks passed. Restricted `make test` first reproduced the four recorded
CoreMIDI initialization failures; its exact host-access retry passed the full
workspace and doctests with three existing runtime ignores. The no-default build
retains an existing unused-mut warning in `crates/oxurack/src/cables.rs:231`.
No hardware endpoint was opened and no product defect was reproduced. The ledger
preserves the compile/test/lint corrections as well as both restricted and host
test receipts.

The source and planning changes were committed separately using exact authorized
paths. Source commit: `e582aed6e87c234799921cc66a5dd55214f76afd`. The planning
commit hash and clean worktree verification are reported in the handoff. CRC
review is pending; no acceptance, closure, or successor advancement is claimed.
