# Arc11 Slice01 LFO — ledger

Opened 2026-10-11 before implementation. Five rows, all open; CC proposes,
CRC independently accepts. Governing contract: slice-plan.md and cc-prompt.md.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| L-01 | Standalone LFO implements five waveforms, rate/offset/width, CV and rising sync | Literal quarter-phase/rate/sync sequences and range/finite checks | correctness-grade | Catalog LFO; slice contract | open | — | No Bevy in default normal dependencies |
| L-02 | Installed LFO publishes in the right sample with independent state | Real registered adapter, Clock sync, two instances, stop/resume and reinstall | correctness-grade | Workbench composition | open | — | Input consumed n applies next Produce; output reaches TM same sample |
| L-03 | All four parameters are correctly registered and validated | Registered setters/getters; invalid spawn and edit preserve state; valid edit preserves phase | correctness-grade | Module API | open | — | String waveform, three Floats; no new Choice variant |
| L-04 | Executable seeded patch actually modulates TM and emits repeatable MIDI | Loaded accelerated square-wave independent oracle plus fresh example traces and README commands | correctness-grade | Operator playable workbench | open | — | Retain original fixture; endpoint audition optional |
| L-05 | Applicable native gates, source scope and honest return hold | Focused feature tests, make test/lint, affected all-target Clippy, strict docs, diff/return review | serious | Directive08; framework2.6.0 | open | — | No dependency-freshness/coverage campaign |
