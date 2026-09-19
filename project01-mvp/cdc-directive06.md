# CDC directive06 — Arc09 stop-accounting remediation

2026-09-18, Sofie/CDC to CRC through Duncan, Three-Contributor Workflow,
Expedited Mode. Response to the Operator-relayed Arc09 composition packet, not
an invented escalation06. Predecessor: directive05. Governing candidate:
source `642313a62db437ee2bf0b3d5b97fcdcffa16782a`, clean branch
`codex/oxurack-module-behavior-installation`; planning intake
`b2574a583f9d73bb533f606e3e6f6cb3d1217d5e`. Project v3.8 / arc09 v1.4 record
the decision. [CDC review](arc09-executable-rack/cdc-verification.md) is the
independent verdict: E-01/E-03 reproduced, E-02/E-04 open; arc close rejected.

## Decision and exact contract amendment

Approve one bounded subsequent slice, `slice04-sample-stop-accounting`, inside
arc09. CRC prepares its complete open set and guide-authored initial CC prompt
before execution. This is higher-scale composition remediation, not reopening
or renumbering accepted child assignments. Record the new scope and denominator
explicitly. Arc10 and its first Clock slice stay held until CRC reassembles and
CDC independently accepts the corrected composition. TM-first priority unchanged.

The earlier slice02 statement that advance_samples runs exactly count when
running did not specify a shared stop during the batch. Supersede that statement
for this case with the following binding behavior (normal uninterrupted batches
still return count):

1. A Live driver still returns ManualSampleRequiresManualDriver before mutation,
   including count zero or a stopped rack. A Manual zero-count request is a no-op.
2. In Manual mode, check SimulationRunning before **each** attempted sample and
   before advancing any Time resource. If false, stop the batch and return the
   number of samples actually completed; initially stopped therefore returns 0.
3. Core's existing sample-begin snapshot remains authoritative for the entire
   current sample. A Produce/Consume/Observe system changing running must not
   truncate that sample: all five phases and finalization finish, and that sample
   counts once. The final shared value after it determines whether the next starts.
   A false-then-true change within one sample leaves the next sample eligible.
4. Advance virtual/fixed time by one 1 ms period only for a sample that will run.
   Expose fixed generic Time during the schedule and restore virtual generic Time
   afterward, including the last sample before the stop. Return partial progress
   as Ok(completed), never Err merely because a module requested stop.
5. Subsequent stopped calls leave counts, clocks, module/RNG/port state and trace
   unchanged. start() permits continuation at the next index without resetting
   module state. Completed-count delta and return value must agree. Existing
   mode-switch/overstep, raw-core phase latch and Live wall-time ownership remain;
   this slice does not redesign Live timing or promise Bevy wall clocks freeze
   when a raw-world caller toggles the shared resource outside rack lifecycle APIs.

The finite batch counter can be local; increment only after a complete schedule,
recheck at the loop head, and keep the existing generic-Time restore before exit.
Do not merely clamp the return value after running the old loop: that leaves the
clock drift. Do not undo clocks/state afterward or read running separately in each
phase. These are recommended implementation mechanics; the five rules are binding.

## Source scope and feasible proof

Primary source file: `crates/oxurack/src/lib.rs`, only advance_samples, its docs
and focused tests/helpers in the existing cfg(test) module. No new public API,
new error, dependency/manifest/lock change, device code or core gate redesign.
CRC should keep the production change small; test code may exercise full behavior.
If it identifies a genuinely necessary additional path, resolve scope before coding.

Use the existing inline test context so the test can retain the rack while
inspecting `rack.app`; into_app consumes it. A minimal registered fixture loads
real RON data via install/load_patch and records all five phase markers, completed
indices, mutable seeded/counter state and clock observations. A configured system
sets shared running false during sample zero. End World borrows before rack calls.
The review-owned [probe and logs](arc09-executable-rack/artifacts/cdc-review06/README.md)
show the public composition failure; do not copy its dense diagnostic source as
production code or make its deliberate failure an ignored/passing regression test.

Define six ledger rows M-01–M-06 and falsifiable tests before CC starts:

- M-01: stop in Produce, Consume and Observe (separate fixtures): advance_samples(3)
  returns 1; SampleTime completed=1/index=0; all five phase markers appear exactly
  once. Clocks advance exactly 1 ms, generic Time is restored to virtual, and state
  reflects exactly one finished sample. Include stop after sample index 1 in a
  larger batch: return/count/clock delta all 2.
- M-02: repeated manual calls while false return 0 and preserve every state/clock
  observation. start then advance one yields index1/count2 and the same per-module
  trace as an uninterrupted two-sample reference, including seeded state. No reset.
- M-03: unaffected contracts: no stop => requested count, count0 no-op, Live exact
  typed error before mutation, raw-core full-sample latching, mode switching and
  manual/grouped-Live traces. A false-then-true within one sample follows final
  state, rather than aborting at the intermediate write.
- M-04: public method docs describe up-to-count/partial Ok result and the shared
  state interaction; examples/errors agree. No new public accessor or feature drift.
- M-05: focused library regressions in default and no-default modes, core/umbrella
  suites, full locked workspace feature matrices, all-targets Clippy, format,
  strict docs, native make check, separate outdated result and diff checks. Preserve
  access failures/host retries, unchanged coverage and hardware successor gates.
- M-06: exact source/planning provenance, complete evidence, current-slice structural
  checks and arc bubble-up; reconcile the original scope with the one newly added
  interaction obligation. Do not claim the inherited slice01 checker validates
  slice04's assignment/rows. CRC must specify current-slice metadata/ledger/link
  checks (including invalid-assignment controls) within its evidence contract.

The old implementation must fail M-01 on both returned count and elapsed time;
a fix of return value alone must still fail. Tests must observe intermediate phases
and actual completed counts, not only final generic time or successful command exit.
Expected values derive from the contract; no clock sleeps or wall timing target.

## CRC authoring, review and refreshed assembly

Refresh the installed collaboration-framework authoring guide/template and apply
its bounded required-reading manifest, CC intake/readback, actual-source recipe,
applied Rust/testing chapters and all six author-readiness checks. Record readiness
in the new slice plan. Inspect source before proposing exact test helper names.
Guide sections on interacting transitions/partial results and ownership-aware test
sequences apply directly; no “fix the review” shorthand or unresolved API choices.

Create only the actual open set; intended initial assignment path (not yet issued):

`arc09-executable-rack/slice04-sample-stop-accounting/cc-prompt.md`

Artifacts default to that slice's artifacts/. List each allowed source/planning
path and commit each checkout separately with explicit staging/cached-diff review
and commit --only. CC evidence is proposed-done; CRC independently reproduces all
six rows, then refreshes the arc assembly for the new source candidate. Preserve
prior reports/verdicts with dated candidate-specific addenda. Do not silently
edit the old slice02 plan's original authority or mark its historical tests as
covering this formerly missing case. The new slice/arc amendment carries the fix.

Correct the arc assembly's “two corrective passes” for slice01: its recorded
history shows one corrective pass after the unexecuted initial prompt. Count the
new slice separately; never reset or miscount earlier budgets. The three existing
slices/18 rows remain accepted at their own candidates. Final child denominator
will be four slices/24 rows once the specified new open set is created/completed.
The new slice starts with zero executed corrective iterations, with the usual cap.

Reassemble E-01–E-04 after repair, rerun connected seeded/manual/live composition
and the stop boundary, preserve successors (arc10/11/12/13 and 07/08), and relay
the arc closing report/CRC record/ledger to CDC. CDC independently rechecks the new
candidate before arc close or Arc10 opening. Do not infer acceptance from an
existing cdc-verification.md or a passing package matrix.

Acknowledge directive06 in the project handoff history. Return the new prompt's
plain project-relative path, source/planning revisions, author-readiness result
and any blocker to Duncan. No additional routine approval round is required for
this within-contract repair. No push/merge/release; main stays untouched. Next
project exchange is 07 unless reserved.
