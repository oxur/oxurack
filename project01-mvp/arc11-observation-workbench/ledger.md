# Arc11 — composition ledger

Opened 2026-10-11 before implementation. Three planned slices; four rows open.
CRC assembles; CDC independently verifies arc composition. No work accepted yet.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| W-01 | Installed Clock/LFO/TM/MIDI path provides repeatable modulation | Run seeded example; compare defined phase/modulation behavior and emitted MIDI across fresh sessions | correctness-grade | P-07; Arc10 successor | open | — | Same sample owner; no cross-platform sine bitwise claim |
| W-02 | Bounded capture observes real signals and MIDI without changing playback | Known waveform/edge fixture; overflow window/count; compare MIDI with capture disabled/enabled | correctness-grade | Operator Scope workbench | open | — | No second MIDI drain; stopped state preserved |
| W-03 | Workbench makes a selected patch's behavior inspectable | Run documented terminal view/export; identify sample, signal, bounds and event ordering using a known fixture | serious | P-07 | open | — | Minimal useful view; full interface remains Arc07 |
| W-04 | Three child slices and retained obligations reconcile | Walk actual child verdicts; reproduce integrated example; check successor owners and applicable gates | serious | Framework composition | open | — | No implicit project or release acceptance |
