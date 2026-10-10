# Slice03 MIDI output — ledger

Opened 2026-10-10 before CC; five rows. All open; CC proposes, CRC accepts.
Current assignment: `arc10-turing-machine-midi/slice03-midi-output/cc-prompt.md`.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| M-01 | A real installed MIDI-out consumes the coherent published TM step | Schema/config/error tests and installed Clock→TM→sink fixture through facade load/advance | correctness-grade | Arc10 design §3; T-01 | open | — | Seven scalar inputs, four static parameters, registered getters; no standalone MIDI wrapper bypass. |
| M-02 | MIDI note/CC order and validation are exact | Literal bytes for retrigger/idle/silence/default velocity and CC validity/order; opposite spawn/name order on two distinct channels | correctness-grade | Arc10 design §3; T-03 | open | — | No masking invalid MIDI data; lexical sink order then off/on/noise-note-velocity CC. |
| M-03 | Finite output storage cannot overwrite unread messages or miscount samples | Facade partial-advance and stop tests; real pre-latch admission in Update and FixedUpdate; drain/error inspection | correctness-grade | design §3; Arc09 completed-sample contract | open | — | Preserve prefix, module state and SampleTime when next sample is denied; errors report completed count. |
| M-04 | The runnable seeded RON example has an independently checked, fresh-process trace | Installed integration trace against separately driven engine/literal MIDI rule; two actual example processes compared | correctness-grade | design §5; T-02 | open | — | Sample/bytes streamed; usable musical tempo; no hardware witness claimed. |
| M-05 | Source/docs and proportionate gates support independent review and successor use | Component diff, intake/readback, command outcomes, docs, Rust decisions and closing-report bubble-up | serious | directive08; framework authoring/verification | open | — | Five-row walk; failed/blocked/ignored checks retained. No report validator or release-scale campaign. |
