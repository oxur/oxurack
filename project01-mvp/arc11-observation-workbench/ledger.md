# Arc11 — composition ledger

Opened 2026-10-11 before implementation. Three planned slices; four rows open.
CRC assembled the candidate at `72fe6b5`: three accepted children/14 rows.
CDC independently verifies arc composition; all four parent rows remain open.
See [closing report](closing-report.md) and [CRC verification](crc-verification.md);
CDC records its independent verdict in cdc-verification.md, not yet produced.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| W-01 | Installed Clock/LFO/TM/MIDI path provides repeatable modulation | Run seeded example; compare defined phase/modulation behavior and emitted MIDI across fresh sessions | correctness-grade | P-07; Arc10 successor | open | CRC proposed-done at `72fe6b5`: fresh Clock/LFO/TM/MIDI runs, 10,000 affine observations, 80 exact Clock pulses, repeat traces/view/CSV and independent modulation oracle/control; crc-verification.md | Pending independent CDC acceptance; Same sample owner; no cross-platform sine bitwise claim |
| W-02 | Bounded capture observes real signals and MIDI without changing playback | Known waveform/edge fixture; overflow window/count; compare MIDI with capture disabled/enabled | correctness-grade | Operator Scope workbench | open | CRC proposed-done at `72fe6b5`: capture off/on stdout equal; CSV MIDI equals trace; bounded 3/overwrite 10067, empty and actual lifecycle/copy tests pass; crc-verification.md | Pending independent CDC acceptance; No second MIDI drain; stopped state preserved |
| W-03 | Workbench makes a selected patch's behavior inspectable | Run documented terminal view/export; identify sample, signal, bounds and event ordering using a known fixture | serious | P-07 | open | CRC proposed-done at `72fe6b5`: documented terminal/CSV command, 40,070 mixed records, 40,071 CSV lines, pinned hashes, selected latest/tails/empty behavior; crc-verification.md | Pending independent CDC acceptance; Minimal useful view; full interface remains Arc07 |
| W-04 | Three child slices and retained obligations reconcile | Walk actual child verdicts; reproduce integrated example; check successor owners and applicable gates | serious | Framework composition | open | CRC proposed-done: three children/14 rows done; fresh affected/native composition checks and justified `22500fa`/date retention; successor/project obligations walked in closing-report.md | Pending independent CDC acceptance; No implicit project or release acceptance |
