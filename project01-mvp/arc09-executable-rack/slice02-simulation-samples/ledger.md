# Slice02 — simulation samples ledger

Opened 2026-09-17 before CC at source `6a31c0f`; six rows, all open. CC's
initial proposed-done packet at `8a73d33` received a
[partial CRC review](crc-verification.md): S-01/S-02 behavior reproduced,
S-03–S-06 require iteration01 evidence repair, and no row is yet accepted.
CRC must reproduce every row and check the bubble-up before this slice closes.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S-01 | A running rack has an authoritative 1 ms, zero-based sample index shared by manual and live driving | Start from `None`/0; inspect indices 0,1,2 and completed counts 1,2,3 under exact manual steps and controlled live fixed-time groups; no sleep | correctness-grade | CDC execution contract §2 | open | — | `step()` remains app update and may run 0+ fixed samples |
| S-02 | Produce/Propagate/MergeInputs/Consume/Observe execute in that order with current merged inputs visible before Consume and stateful outputs delayed one sample per stage | Public RON producer→relay→sink fixture; assert each sample-indexed output/input/state/Observe trace and an independent phase marker order; reject same-sample topological shortcut | correctness-grade | CDC execution contract §2 | open | — | Zero-contribution input retains prior value |
| S-03 | Shared running state gates every named module phase, preserves raw core compatibility and rack stopped default | Bare CorePlugin Update and fixed variants run by default; preinserted false gates; Oxurack starts false; stopped manual/live steps do not change sample/state/trace; start resumes without reset | correctness-grade | CDC execution contract §2 | open | — | Existing modules must join a named phase; arbitrary unassigned systems are outside that guarantee |
| S-04 | Manual/live drivers are exclusive and switching discards pending wall/fixed overstep | Wrong-driver manual call returns exact error with no mutation; manual `step()` yields no samples; Live→Manual→Live and stop→start cases do not replay overstep; controlled 1+1+1 versus 1+2 live grouping matches manual trace | correctness-grade | CDC execution contract §2 | open | — | Bevy `Time<Fixed>` and generic time align during manual schedule |
| S-05 | Observable fan-in and ordering follow stable instance/port names rather than entity allocation or current ModuleId numeric order | LastWins fan-in and independent modules with permuted spawn order/extra unrelated entity yield identical named traces; core topological tie-break remains deterministic without claiming same-sample execution | serious | CDC execution contract §2 | open | — | No change to seed derivation or ModuleId value; slice03 owns those |
| S-06 | Implementation scope, docs, regressions and complete validation support independent review | Exact source/planning diffs and commits; default/no-default, package/workspace, full Clippy, format, strict rustdoc, make check, separate outdated, verifier; retain failed/retried raw logs and bubble-up | serious | Arc09 quality contract | open | — | Existing 95% coverage, physical MIDI and timing gates remain explicit successors |
