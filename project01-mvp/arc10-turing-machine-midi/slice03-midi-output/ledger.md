# Slice03 MIDI output — ledger

Opened 2026-10-10 before CC; five rows. All open; CC proposes, CRC accepts.
Current assignment: `arc10-turing-machine-midi/slice03-midi-output/cc-prompt.md`.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| M-01 | A real installed MIDI-out consumes the coherent published TM step | Schema/config/error tests and installed Clock→TM→sink fixture through facade load/advance | correctness-grade | Arc10 design §3; T-01 | open | `closing-report.md` M-01 | CC proposes done; CRC review pending. Seven scalar inputs/four static getters and actual cable integration. |
| M-02 | MIDI note/CC order and validation are exact | Literal bytes for retrigger/idle/silence/default velocity and CC validity/order; opposite spawn/name order on two distinct channels | correctness-grade | Arc10 design §3; T-03 | open | `closing-report.md` M-02 | CC proposes done; CRC review pending. Covers lexical order, Raw boundaries, invalid masking and per-sample step. |
| M-03 | Finite output storage cannot overwrite unread messages or miscount samples | Facade partial-advance and stop tests; real pre-latch admission in Update and FixedUpdate; drain/error inspection | correctness-grade | design §3; Arc09 completed-sample contract | open | `closing-report.md` M-03 | CC proposes done; CRC review pending. Denied samples preserve clocks/state; Consume faults include the finalized sample. |
| M-04 | The runnable seeded RON example has an independently checked, fresh-process trace | Installed integration trace against separately driven engine/literal MIDI rule; two actual example processes compared | correctness-grade | design §5; T-02 | open | `closing-report.md` M-04 | CC proposes done; CRC review pending. Two fresh outputs compare equal; no hardware witness claimed. |
| M-05 | Source/docs and proportionate gates support independent review and successor use | Component diff, intake/readback, command outcomes, docs, Rust decisions and closing-report bubble-up | serious | directive08; framework authoring/verification | open | `closing-report.md` M-05 and validation receipts | CC proposes done; CRC review pending. Restricted failures, host retry and ignores retained. |
