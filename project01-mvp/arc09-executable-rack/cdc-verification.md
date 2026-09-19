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

## CDC repaired-candidate acceptance — review07, 2026-09-18

**Accepted for Arc09 closure. E-01–E-04 are independently reproduced/done on
`37deeace90e7fe2dff633ada9d58ef9051577c3d`.** The rejection above remains the
historical verdict for `642313a`; it is superseded only for the repaired candidate.
Planning intake/CRC assembly: `f6c778e` (full hash in
[provenance](artifacts/cdc-review07/provenance.txt)). Source remained clean and
unchanged throughout CDC's review. CRC's operational and CDC's independent
records now concern the same source. This is arc acceptance, not release or
project acceptance.

### Reproduction and scope

I inspected the one-file Slice04 diff and complete focused test harness against
directive06. Production changes are confined to advance_samples and its docs:
Live error still precedes mutation; each iteration checks running before advancing
time, finishes the existing latched schedule, restores generic Virtual time,
then increments completed. The loop returns actual progress, without rollback,
return-only clamping, new APIs or per-phase running checks. Nine stop-accounting
tests pass in both modes, including all three stop phases, later stop, no-op,
resume, false-then-true and error precedence.

CDC rebuilt the original independent, connected two-instance probe against the
repaired source and ran 12 processes. The former failure now reports request 3,
returned 1, completed 1/index 0, Fixed 1 ms/Virtual 1 ms in both graph modes. The runner
checks both count and clocks, so a return-only repair fails. Ten normal trace
controls still match their independent oracle across processes, grouped Live and
spawn reorderings, with changed seed differing. This is the same reviewer-owned
probe that rejected the predecessor, not a replacement assertion tailored to
CC's implementation.

All 13 independently rerun gates pass: focused default/no-default, core,
umbrella default/no-default, both full host workspace matrices, all-targets
Clippy, formatting, two strict docs, native make check and separate outdated.
Host access was requested up front for known MIDI/network requirements; prior
restricted failures remain preserved. Workspace RT reports 150 passed/3 existing
ignored, with the pre-existing block 0.1.6 future-incompatibility notice. No
hardware playback, timing, coverage, benchmark, soak or musical claim follows.
Source scope and diff checks pass. Slice04's current structural checker passes
its baseline and rejects missing-row, stale-assignment and invalid-status cases.

[Evidence index](artifacts/cdc-review07/README.md) links the reproducible fixture,
commands/exits and raw logs. The four-child walk confirms 24 done rows; child
acceptance records were preserved at their original candidates. F-02's correction
is present in CRC's closing-report addendum: Slice01 one corrective pass,
Slice02 one proof repair, Slice03/04 zero. Neither finding remains open.

### Final four-row walk

| Row | Final disposition and independent basis |
| --- | --- |
| E-01 | done / reproduced: full installation suites plus connected two-instance probe preserve once-per-type execution, reinstall state and kind-conflict behavior. |
| E-02 | done / reproduced: exact manual/grouped-Live phase traces and merge latency pass; original failure now returns/counts/advances only completed samples, with complete current phase and resume semantics. |
| E-03 | done / reproduced: seeded constructor/package fresh-process suites and connected reviewer probe retain same-seed/order invariants and changed-seed controls. |
| E-04 | done / reconciled and reproduced: 4/4 accepted slices, 24/24 child rows, corrected refinement counts, bounded one-file repair, full gates and retained successor obligations. |

### Bubble-up and what worked

Arc09 delivers the executable, deterministic sample/seed substrate promised by
the roadmap. Composition exposed one missing stop-during-batch interaction;
the explicit amended contract, phase-aware partial-result oracles and CRC's
single bounded Slice04 resolved it without another corrective pass. Carry that
interaction-based prompt practice into Clock rate edits and later MIDI cleanup.

No original capability or quality criterion is dropped. Historical carried-over
rows remain deferred where they also require live persistence/RT/catalog work.
P-01–P-07 stay open. The dependency gate for the existing Arc10 roadmap is now
satisfied: [directive07](../cdc-directive07.md) opens Arc10 and issues its first
musical Clock slice. TM/MIDI remains first milestone, LFO/Scope workbench second.
