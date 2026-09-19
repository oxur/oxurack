# Slice02 Turing Machine adapter — ledger

Opened 2026-09-19 before CC; six rows. All start open. CC proposes; CRC accepts.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| A-01 | Existing standalone engine gains an opt-in rack adapter without default dependency or behavior regression | Default/rack feature trees and tests; seeded construction uses context; standalone reference unchanged | correctness-grade | Arc10 design §2; T-02 | open | — | No OS RNG on patch path |
| A-02 | Every engine setting/action has a strict framework mapping and actual-state introspection | Full schema/setter/getter/action matrix; arbitrary Scale conversion; invalid/no-mutation controls | correctness-grade | directive07 inventory gate | open | — | No silent audible-subset reduction |
| A-03 | Every StepOutputs field publishes as one coherent delayed scalar snapshot | Compare all 28 output ports and validity/step gates to independent standalone trace | correctness-grade | Arc10 design §2; T-01 | open | — | Produce holds between steps |
| A-04 | Edge, conflict, modulation and lifecycle semantics are exact | Wide pulses, reset/move/clock priority, nonaccumulating modulation, stop/resume/pending/reinstall tests | correctness-grade | Arc09 execution contract; Clock prerequisite | open | — | At most one advance per sample |
| A-05 | Seeded adapter replay is fresh-process and order stable | Exact child-process traces; changed seed/name; reordered modules/unrelated entity; standalone comparison | correctness-grade | T-02; seeded spawn context | open | — | Dependency-state reproducibility only |
| A-06 | Source scope, docs and full gates support independent review | Intake/provenance, allowlist, package/workspace/lint/fmt/docs/Make/outdated/checker results and bubble-up | serious | Framework quality contract | open | — | Physical MIDI and Arc10 composition remain open |
