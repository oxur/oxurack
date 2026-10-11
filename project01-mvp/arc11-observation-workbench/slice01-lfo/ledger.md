# Arc11 Slice01 LFO — ledger

Opened 2026-10-11 before implementation; five rows. Current assignment is
`cc-prompt.md`; governing behavior is `slice-plan.md`. CC has supplied an
attested proposed-done report at `closing-report.md`; all rows remain open until
CRC independently reproduces and accepts them. No Arc11 or next-slice status is
advanced here.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| L-01 | Standalone LFO implements five waveforms, rate/offset/width, CV and rising sync | Literal quarter-phase/rate/sync sequences and range/finite checks | correctness-grade | Catalog LFO; slice contract | open | `closing-report.md` §Validation, focused crate tests; CC attested | CRC reproduction pending; no Bevy in default normal dependencies |
| L-02 | Installed LFO publishes in the right sample with independent state | Real registered adapter, Clock sync, two instances, stop/resume and reinstall | correctness-grade | Workbench composition | open | `closing-report.md` §Validation, `tests/lfo.rs`; CC attested | CRC reproduction pending; input consumed n applies next Produce; output reaches TM same sample |
| L-03 | All four parameters are correctly registered and validated | Registered setters/getters; invalid spawn and edit preserve state; valid edit preserves phase | correctness-grade | Module API | open | `closing-report.md` §Validation, `tests/lfo.rs`; CC attested | CRC reproduction pending; String waveform, three Floats; no new Choice variant |
| L-04 | Executable seeded patch actually modulates TM and emits repeatable MIDI | Loaded accelerated square-wave independent oracle plus fresh example traces and README commands | correctness-grade | Operator playable workbench | open | `closing-report.md` §Validation, fixture, oracle, example; CC attested | CRC reproduction pending; original fixture retained; endpoint audition optional |
| L-05 | Applicable native gates, source scope and honest return hold | Focused feature tests, make test/lint, affected all-target Clippy, strict docs, diff/return review | serious | Directive08; framework2.6.0 | open | `closing-report.md` §Validation and source commit `a965386`; CC attested | CRC reproduction pending; restricted CoreMIDI failure and successful host retry preserved; no dependency-freshness/coverage campaign |
