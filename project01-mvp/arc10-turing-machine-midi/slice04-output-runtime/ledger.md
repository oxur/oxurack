# Slice04 direct output — ledger

Opened 2026-10-10 before implementation; five rows, all open.
Current assignment: `arc10-turing-machine-midi/slice04-output-runtime/cc-prompt.md`.
CRC accepted O-01–O-04 at source `65522f1` on 2026-10-10; O-05 remains open.
Five-row denominator retained; Slice04 is not closed. See [CRC review](crc-verification.md).

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| O-01 | Same executable lists and explicitly selects an endpoint before playback, retaining offline use | Clap controls, exact-name/duplicate/index tests, actual invalid-patch/impossible-index exits; preparation/connect order inspection | correctness-grade | design§4; T-04 | done | crc-verification.md §Verdict and five-row walk; reproduced | Host listing returned MIDI Bus 1 and nanoKEY2 CTRL. No substring/fallback; no advance/send before successful load/open. |
| O-02 | Single-owner runner sends installed rack messages in order with finite monotonic pacing | Actual loaded fast fixture through recording sender and independent seeded byte oracle; deadline on-time/late values | correctness-grade | design§4; T-01/T-03 | done | crc-verification.md §Verdict and five-row walk; reproduced | 24 samples matched oracle; runner stops and drains; no catch-up burst or new logical clock. |
| O-03 | Stop/interrupt/error discard unsent performance and attempt complete, idempotent cleanup | Failed-off/failed-on successful-send ledger, actual runner cancellation/failure, literal cleanup order and multiple failure aggregation; fresh session | correctness-grade | design§4; T-03 | done | crc-verification.md §Verdict and five-row walk; reproduced | Preserve primary + cleanup errors; no silence/timeout/SIGKILL promise. |
| O-04 | Executable compatibility, docs and proportionate native gates hold at exact source | Fresh original 13-row trace/CLI negatives, example and integration feature checks, make test/lint, all-target Clippy, strict docs/diff | serious | directive08; T-02/T-05 | done | crc-verification.md §Verdict and five-row walk; reproduced | Initial sandbox CoreMIDI init failures and successful host retry are recorded; three existing RT ignores retained. |
| O-05 | Duncan witnesses integrated fixture on a named real software or hardware MIDI endpoint | Actual command/seed42/endpoint/routing and musical/stop observation, performance trace compared with offline equivalent | correctness-grade | design§5; T-04 Operator gate | open | closing-report.md §Five-row proposed disposition | Inventory: MIDI Bus 1, nanoKEY2 CTRL. No endpoint selected/connected; Duncan's musical/routing/stop observation remains open. |
