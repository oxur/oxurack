# Slice04 — sample stop accounting ledger

Opened 2026-09-18 before CC at clean source `642313a`; six rows. Authority and
exact expected behavior are in [slice-plan.md](slice-plan.md) and
[directive06](../../cdc-directive06.md). CC may propose completion; only CRC may
assign done after independent reproduction.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| M-01 | A shared stop during a manual batch finishes the latched sample, returns exact completed progress and advances clocks only for completed samples | Separate Produce/Consume/Observe stop at sample 0 of request 3: return/count 1, index 0, five markers once, state once, Fixed/Virtual +1 ms and generic Time restored; stop at index 1 of larger request yields all deltas 2 | correctness-grade | directive06 rules 2–4; CDC F-01 | open | — | Old implementation and a return-only clamp must fail |
| M-02 | Stopped calls are full no-ops and restart resumes state at the next index | Repeat stopped calls and compare every count/clock/state/RNG-port/trace observation; start plus one reaches index 1/count 2 and equals an uninterrupted two-sample reference including context-seeded state | correctness-grade | directive06 rule 5 | open | — | No reconstruction or seed reset on start |
| M-03 | Existing driver, zero, phase-latch, switching and live/manual behavior survives; final shared state controls continuation | Prove uninterrupted requested count, manual zero no-op, Live exact error before mutation even zero/stopped, raw-core full-sample latch, mode-switch/grouped-live traces, and false-then-true within one sample permits the next | correctness-grade | directive06 rules 1 and 3; slice02 | open | — | No Live timing redesign |
| M-04 | Public documentation states up-to-count partial-success semantics and shared-stop interaction | Inspect rustdoc for exact result/error/mutation behavior and examples; strict docs pass; no new accessor/error/feature | serious | directive06 M-04 | open | — | Preserve inherent API signature |
| M-05 | Focused, package and full repository validation pass without masking environment limits | Default/no-default focused regressions, core/umbrella and workspace matrices, all-targets Clippy, format, strict docs, native make, separate outdated, diff checks; preserve failed attempts and host retries | serious | arc09 quality contract; directive06 M-05 | open | — | Coverage/hardware gates retain owners |
| M-06 | Exact source/planning provenance, current assignment, evidence and four-slice arc accounting support independent review | Inspect committed source/planning hashes, explicit file scope, reports/logs, checker baseline plus missing-row/stale-assignment/status controls, links and bubble-up; denominator 4 slices/24 rows | serious | directive06 M-06 | open | — | Inherited slice01 checker is not current proof |
