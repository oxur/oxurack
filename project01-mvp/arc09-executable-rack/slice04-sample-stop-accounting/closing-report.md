# Slice04 closing report — sample stop accounting

Date: 2026-09-18. Workflow: CC implementation return in Expedited Mode.
Disposition: **proposed-done, pending CRC independent reproduction**.

## Provenance and scope

The source predecessor was the clean
`642313a62db437ee2bf0b3d5b97fcdcffa16782a` on
`codex/oxurack-module-behavior-installation`. The planning baseline was
`6a1bc1340ae6eecb8ed606eb6c1aa8323f7a7119`; the planning worktree was at
`0cce066066c71de6b3c46fd29e19ce98800d7b40` before these evidence artifacts.
The source implementation commit is
`37deeace90e7fe2dff633ada9d58ef9051577c3d`.

The source change is exactly [crates/oxurack/src/lib.rs](../../../../codex-oxurack-module-behavior-installation/crates/oxurack/src/lib.rs).
The planning files produced for this return are this report,
[artifacts/test-evidence.md](artifacts/test-evidence.md),
[artifacts/validation-results.tsv](artifacts/validation-results.tsv), and the
raw logs in `artifacts/logs/00-source-audit.log`,
`00-review-probe-results.log`, `00-review-checks.log`,
`01-package-default.log`, `02-core-package-default.log`,
`03-package-no-default.log`, `04-core-no-default.log`,
`05-focused-default.log`, `06-focused-no-default.log`,
`07-workspace-all-features.log`,
`07-workspace-all-features.host-retry.log`,
`08-workspace-no-default.log`,
`08-workspace-no-default.host-retry.log`,
`09-clippy-all-features.log`, `10-fmt-check.log`,
`11-doc-workspace-all-features.log`, `12-doc-oxurack-no-default.log`,
`13-make-check.log`, `13-make-check.host-retry.log`,
`14-cargo-outdated.log`, `14-cargo-outdated.host-retry.log`, and
`15-source-diff.log`, `16-checker-main-root.log`,
`17-checker-planning-root.log`, `18-checker-final.log`, and
`19-checker-final-after-report.log`. No other source or planning path was
changed.

## Implementation

`Oxurack::advance_samples` still rejects every Live request first with
`CoreError::ManualSampleRequiresManualDriver`; this includes zero and stopped
requests and occurs before mutation. Manual zero remains a no-op. Manual batches
now check `SimulationRunning` at each loop head, advance Virtual and Fixed by
one millisecond only after that check, run the existing latched core schedule,
restore generic `Time` from Virtual, and increment a local completed counter
only after the sample finishes. A shared stop therefore finishes the current
sample and prevents the next one without a new error or clock catch-up.

The private harness uses a registered deterministic module loaded through a real
temporary RON patch. The module owns its context-derived instance seed, updates
wrapping state once per Produce phase, mirrors state to an output port, and
records all five phase/index markers. This makes return count, completed index,
Fixed/Virtual/generic time, fixed overstep, state, seed, port state, trace and
resume behavior independently inspectable.

## Ledger dispositions

- **M-01 — proposed satisfied.** Produce, Consume and Observe stops at sample
  zero of a three-sample request each return 1, complete index 0 once, emit all
  five phases, advance state once, and advance Fixed/Virtual by exactly 1 ms.
  A stop at sample index 1 returns 2 with ten ordered markers and 2 ms clocks.
  Generic `Time` restores to Virtual and Fixed overstep remains zero.
- **M-02 — proposed satisfied.** Repeated stopped calls return zero and preserve
  count, clocks, module state, seeded state, output port and trace. `start()`
  followed by one sample matches an independently constructed uninterrupted
  two-sample reference.
- **M-03 — proposed satisfied.** Uninterrupted manual and zero-count behavior,
  Live typed-error precedence for counts 3, 0 and a stopped rack, existing core
  phase latching, mode-switch/overstep/manual/live/grouped-live suites, and a
  false-at-Produce then true-at-Observe continuation all pass.
- **M-04 — proposed satisfied.** The inherent method’s rustdoc documents
  up-to-count complete-sample semantics, the shared-stop latch interaction,
  stopped/zero no-op behavior and the existing Live error. Strict rustdoc and
  the all-targets Clippy gate pass; no public API, error, feature or manifest
  was added.
- **M-05 — proposed satisfied.** Focused default/no-default, core/umbrella,
  workspace default/no-default, all-targets Clippy, formatting, strict docs,
  `make check`, dependency observation and source diff were run. Restricted
  workspace and Make attempts retain the four MIDI initialization failures and
  host retries; restricted dependency observation retains the crates.io DNS
  failure and host retry. The successful host retries and all other gates are
  recorded in `validation-results.tsv` with raw logs.
- **M-06 — proposed satisfied.** The source and planning predecessors,
  source-only commit, explicit file fence, evidence packet, exact assignment,
  4-slice/24-row denominator and current checker negative controls are recorded
  for CRC review. The planning artifact commit is the next planning commit
  after predecessor `0cce066066c71de6b3c46fd29e19ce98800d7b40`; its exact hash
  is part of the final handoff and repository history.

## Validation and limits

The focused commands report 9 passed tests in each feature configuration. The
default umbrella package reports 24 unit tests plus its integration suites; the
no-default package reports 22 plus its integration suites. The core package
reports 414 unit tests plus its integration suites in both configurations. The
full host workspace matrices pass. Existing future-Rust warning output for
`block v0.1.6`, ignored tests, restricted MIDI failures, DNS failure and host
retries are retained rather than hidden; see the TSV and logs for denominators.

No coverage percentage, hardware timing, physical MIDI, soak, benchmark or
musical acceptance claim is made. No corrective refinement was performed: **0
of 5**. The ledger remains open, CC remains proposed-done, CRC must independently
reproduce M-01–M-06, and Arc10 remains held.

The checker passes from the planning worktree and rejects all three negative
controls; its literal main-checkout invocation is retained as a layout failure
because the checker resolves paths from its current Git root.
