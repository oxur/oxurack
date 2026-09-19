# Arc09 CDC independent composition review

Date: 2026-09-18. Sofie/CDC, original design context, independent of CC's
implementation and CRC's arc assembly. Source candidate:
`642313a62db437ee2bf0b3d5b97fcdcffa16782a`. Planning intake:
`b2574a583f9d73bb533f606e3e6f6cb3d1217d5e`; CRC assembly `2a0411b` plus its
provenance follow-up. Source remained clean and unchanged. Inputs: actual arc
plan, closing report, CRC record, four-row ledger, all three six-row child
ledgers/CRC dispositions, source implementations/tests and successor routing.

## Verdict

**Not accepted for closure. E-01/E-03 reproduced; E-02/E-04 remain open.**
The normal installed/seeded/sample composition works. A reproduced stop-during-
manual-batch defect violates the intended sample count/time consistency and
exposes an underspecified partial-return case in the earlier slice contract.
[Directive06](../cdc-directive06.md) resolves that case and assigns a bounded
fourth remediation slice. Arc10 remains unopened; a later clean candidate needs
CRC's refreshed assembly and CDC's independent composition verdict.

## Independent reproduction

[Evidence index](artifacts/cdc-review06/README.md), [command results](artifacts/cdc-review06/checks.json),
[probe results](artifacts/cdc-review06/probe-results.json) and raw logs preserve all
attempts. On the pinned source, CDC reran core/umbrella/default/no-default suites,
both full workspace feature matrices, all-targets Clippy, formatting, both strict
doc configurations, native make check and separate outdated. Restricted workspace
runs each failed the four known MIDI-init cases; host repeats passed (150 RT tests,
three pre-existing ignored). Other named gates passed. The inherited structural
check passed 23 preserved imports, 24 deferred successors, 365 local links; this
validates its own properties, not the current arc's semantics. No dependency
upgrade, physical MIDI/timing, coverage, benchmark, soak or music session acceptance.

I read the production changes in registration, context dispatch, patch loading,
seed derivation, phase gating and manual/live ownership. Existing tests reproduce
same-type/no-mutation collision behavior, live edited state, five-phase indexed
traces, lexical ordering and two actual SmallRng replay child processes. The six
FNV constants and review-probe expected trace were derived independently from the
specified byte stream. The seed helper/ModuleId migration and unused ahash-chain
removal are within slice03's approved compatibility scope.

The additional review-owned public-rack probe installs one context-aware module
type twice, loads two seeded instances with a real cable, publishes in Produce,
updates state from merged inputs in Consume, and records in Observe. It checks
reinstall between manual samples, stopped no-op and resume. Two fresh processes
and grouped live driving match the six-line independent oracle; reordered spawn
preserves it and changed seed differs. These controls pass with graph on and off.
This is stronger composition evidence than merely concatenating child reports.

## Findings and row walk

| Row | Disposition / strength | Basis |
| --- | --- | --- |
| E-01 | done / reproduced on 642313a | Existing installation/live-state tests plus the two-instance connected seeded probe establish once-only installed behavior, retained state and conflict preservation on the final source. |
| E-02 | open / reproduced failure | F-01 below: a mid-sample shared stop gates future phases correctly, but the outer manual batch over-reports completed samples and advances clocks for skipped samples. Normal driver equivalence is reproduced but does not cover this transition. |
| E-03 | done / reproduced on 642313a | Package tests run actual seeded child processes; independently derived vectors and the connected public-rack probe agree across processes, ordering changes, manual/live groups and changed-seed controls. No TM/hardware claim. |
| E-04 | open / partially reconciled | Existing 3 slices / 18 rows are CRC-accepted and their gates reproduce, but F-01 requires planned remediation and another integrated candidate. The prior assembly's slice01 refinement count also needs correction. |

**F-01 — correctness-grade, `crates/oxurack/src/lib.rs`, advance_samples, lines
288–313.** Running is checked once before the loop. A module setting the shared
SimulationRunning to false in the first sample's Consume phase causes the core
to complete that latched sample and skip subsequent sample phases, but the outer
loop still increments Time<Virtual>/Time<Fixed> three times and returns count=3.
Actual output in both feature modes:

```text
requested=3 returned=3 completed=1 last_index=Some(0)
fixed_elapsed=3ms virtual_elapsed=3ms
```

The required result after the clarified contract is return 1, completed count 1,
last index 0, clocks advanced only 1 ms, followed by a true no-op while stopped.
The current sample must still finish all five phases; early exit inside it would
be a different defect. Evidence: probe-final-default-mid-stop.log and
probe-final-no-default-mid-stop.log under the evidence index. Product source was
not patched by CDC; the probe is diagnostic only.

**F-02 — administrative discrepancy.** The assembly says slice01 delivered
“after two corrective passes”; its actual assignment history records the held
initial prompt, iteration01 as first execution and iteration02 as corrective
pass 1/5. Correct the assembly via an explicit addendum while preserving its
original claim. This does not invalidate independently accepted child behavior.

## Composition and project bubble-up

The prior 3/3 child completion is accurate for the planned implementation work;
it is insufficient to close the higher-scale gate with F-01 present. Add
slice04-sample-stop-accounting for the newly exposed interaction between shared
running state and the manual batch driver. Preserve accepted child records at
their candidates, maintain the original three capabilities and all inherited
successors, and amend the partial-return contract explicitly in the new plan.
No broader redesign or quality waiver follows. After repair, expected arc child
accounting is 4 slices / 24 rows if CRC uses the six specified remediation rows;
the new directory/open set must be authored before execution, not assumed present.

All claims here are bounded by their actual commands/source. E-01/E-03 are pinned
to this candidate and must be checked for regressions when the remedial candidate
returns. Arc10/TM-MIDI then the observation workbench remains the priority order.
