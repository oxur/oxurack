# Slice04 test evidence

## Intake and contract readback

Pre-edit intake was performed against the pinned source predecessor
`642313a62db437ee2bf0b3d5b97fcdcffa16782a` on branch
`codex/oxurack-module-behavior-installation`; that source worktree was clean.
The planning worktree was clean at `0cce066066c71de6b3c46fd29e19ce98800d7b40`
on branch `planning`; the required planning baseline
`6a1bc1340ae6eecb8ed606eb6c1aa8323f7a7119` is an ancestor and the current HEAD
contains the issued Slice04 prompt. No unrelated changes were absorbed.

The only allowed source path is `crates/oxurack/src/lib.rs`. Planning outputs
belong under this slice's `artifacts/` directory plus its root
`closing-report.md`. Core, graph, module, RT, MIDI, device, manifest, lockfile,
prior-slice, parent-plan and checker changes are outside the assignment.

The five binding rules read back as follows:

1. Live rejects every manual request with the exact typed error before mutation;
   manual zero is a no-op.
2. Manual mode checks shared running state at each loop head before either clock
   advances, returning the actual completed count when stopped.
3. Core's existing sample latch finishes all five phases and finalization after
   a mid-sample stop; only the final shared value controls the next sample.
4. Each runnable sample advances Virtual and Fixed by exactly 1 ms, exposes Fixed
   generic `Time` during the schedule, restores Virtual generic `Time`, and
   treats a stop as partial success.
5. Stopped calls are full no-ops; `start()` resumes the next sample and preserves
   module, seeded, port, clock and trace state.

The easily missed constraint is that returning the corrected count after running
the old loop is insufficient: skipped samples must not advance either clock,
and the already latched sample must still emit all five phases before the
counter is incremented. The tests therefore observe return/count/time/phase,
mutable deterministic state, seed-derived state, port snapshots and resume
equivalence through the real RON patch-loading path.

Required full reads were loaded before editing: the 290-line prompt, 108-line
slice plan, 15-line ledger, 205-line checker, 46-line planning `AGENTS.md`,
139-line directive06, 98-line CDC review, 50-line CDC review README, both
8-line diagnostic logs, the 537-line current status, the 115-line arc report,
and the 85-line CRC assembly. The source audit loaded the full 1123-line
umbrella library, 715-line public sample-pipeline integration test and
1577-line core tick file in bounded chunks. The exact pre-edit source audit and
review-table projections are preserved in `logs/00-source-audit.log`,
`logs/00-review-probe-results.log`, and `logs/00-review-checks.log`.

Concrete guidance applied before implementation:

- prompt authoring guidance requires transition-aware partial-result oracles;
  the focused harness distinguishes a completed latched sample from a skipped
  next sample;
- Rust EH-01 keeps expected early batch termination as `Ok(completed)` and
  retains only the existing Live misuse error;
- API-24 preserves the inherent `advance_samples` signature;
- OB-02 and OB-12 scope each mutable world borrow inside the iteration and end
  test borrows before rack lifecycle calls;
- CG-L-01 requires the local Clippy developer loop, with no manifest change;
- testing discipline requires exact contract-derived observations, an
  independent uninterrupted reference, negative controls for the predecessor
  and no sleeps or tolerance-based timing assertions.

The CDC review projections were read without suppressing failures: both
`default/mid-stop` and `no-default/mid-stop` exited 101 with returned 3,
completed 1 and 3 ms clocks; normal process/live/reorder/changed-seed controls
exited 0. The review check table retained restricted workspace exits 101,
host retries 0, and the separate successful package/lint/format/docs/make/
outdated checks. These are diagnostic and historical inputs; the dense probe is
not copied into production or retained as an ignored test.

## Source reconnaissance

At baseline, `advance_samples` validated Manual once, returned early for zero or
stopped, then ran the requested loop while advancing both clocks, swapping
generic `Time` to Fixed and back to Virtual, and returned the requested count.
`start`, `stop`, `set_driver_mode` and `is_running` own the shared lifecycle
state. Core configures `begin_sample_system` before Produce, chains Produce →
Propagate → MergeInputs → Consume → Observe under the latched `SampleGate`, and
finalizes after Observe. The public sample-pipeline tests already cover normal
manual/live, zero, stopped, resume, overstep discard, mode switching and grouped
live behavior; Slice04 adds the missing mid-batch and state/time accounting.

## Implementation summary

The source repair keeps the driver check first, retains the manual zero-count
fast path, introduces a local `completed` counter, checks `is_running()` at the
top of every attempted iteration, preserves the existing clock/schedule/time
sequence, increments only after generic Virtual time is restored, and returns
`completed`. The rustdoc now documents upper-bound requests, complete-sample
counting, shared-stop behavior and the sole Live misuse error.

Focused private tests in the existing umbrella `cfg(test)` module construct a
real temporary RON patch, install a context-aware deterministic module, retain
the rack wrapper, and record phase/index/state/time observations. They cover
Produce/Consume/Observe stops, index-one partial progress, repeated stopped
no-ops, start/resume against an independently constructed uninterrupted
reference, false-then-true continuation, Live precedence, manual zero,
uninterrupted count, raw-core compatibility, mode switching and grouped-Live
regression coverage.

## Evidence status

This is CC implementation evidence and remains proposed-done pending CRC
independent reproduction. The validation matrix and raw logs preserve every
attempt, restricted failure, host retry, warning and ignored test. No coverage,
hardware timing, physical MIDI, soak, benchmark or musical acceptance claim is
made.

## Committed implementation and oracle results

The source-only implementation commit is
`37deeace90e7fe2dff633ada9d58ef9051577c3d` on
`codex/oxurack-module-behavior-installation`. It changes only
`crates/oxurack/src/lib.rs`. The production method checks the shared running
resource before each clock advance, scopes the mutable world borrow around one
scheduled sample, restores generic Virtual time before incrementing the local
completed counter, and returns that counter. The same file contains the private
real-RON probe and focused tests.

The focused default and no-default commands each ran all 9 new
`stop_accounting` tests successfully. Produce, Consume and Observe stops each
returned 1 for a three-sample request, completed sample index 0 once, emitted
the five phase markers, advanced state once and advanced Fixed/Virtual by 1 ms
with generic Time restored to Virtual. The sample-one case returned 2 with ten
ordered phase markers and 2 ms clocks. Repeated stopped requests returned 0
without changing counts, clocks, state, output port or trace. Restart plus one
sample matched an independently constructed uninterrupted two-sample reference;
false at Produce followed by true at Observe allowed the next sample. The Live
tests retained the exact typed error before mutation for counts 3, 0 and a
stopped rack. Logs: `logs/05-focused-default.log` and
`logs/06-focused-no-default.log`.

The package and core suites passed in both feature configurations. Existing
umbrella tests provide mode-switch, overstep, manual/live, grouped-live and
zero/stopped coverage; the core suite retains its raw phase-latch test. The
workspace host retries passed all suites. The restricted workspace attempts
failed only the four MIDI initialization/port tests (`146 passed, 4 failed, 3
ignored`) in each matrix; their raw logs remain beside the successful host
retries. `make check` has the same restricted failure and passed on host retry.
The restricted `cargo outdated` attempt failed on crates.io DNS resolution and
passed on host retry. The strict gates passed, with the existing future-Rust
warning for `block v0.1.6` retained in the doc, Clippy and Make logs.

No corrective refinement was needed: `0 of 5`. No production source path beyond
the allowlist was changed, and no ledger, slice plan, parent plan, status or
Arc10 artifact was updated. The implementation disposition is proposed-done;
CRC must independently reproduce the six ledger rows before any row or slice
is marked done.

The canonical planning checker passed from the planning worktree and rejected
all three requested negative controls; see `logs/17-checker-planning-root.log`.
The literal repository-root invocation is retained in
`logs/16-checker-main-root.log` as a layout failure because the checker derives
its root from the current Git checkout and the main checkout does not expose
the planning worktree's `project01-mvp` root. This does not replace the
canonical planning-worktree result.
