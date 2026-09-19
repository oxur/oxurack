# Slice04 CRC verification

Date: 2026-09-18. Reviewer: Sofie/CRC, independent of CC.
Assignment: `arc09-executable-rack/slice04-sample-stop-accounting/cc-prompt.md`.

## Candidate and scope review

I reviewed clean source commit
`37deeace90e7fe2dff633ada9d58ef9051577c3d` and CC planning evidence commit
`7efed01357a9332b5b00556daefcf8a0c1464d3a`. The accepted source predecessor is
`642313a62db437ee2bf0b3d5b97fcdcffa16782a`; the issued planning assignment is
`0cce066066c71de6b3c46fd29e19ce98800d7b40`. Both worktrees were clean before
review.

The source diff changes exactly `crates/oxurack/src/lib.rs`. Production scope is
limited to `advance_samples` and its rustdoc. The remainder is the focused private
test harness. There is no API signature, error, dependency, manifest, lockfile,
core, graph, TM, RT, MIDI or device change. I inspected the complete source diff,
CC closing report, evidence matrix and retained restricted/host logs.

The production loop retains Live-error precedence and the manual zero fast path,
checks shared running before each attempted sample and clock advance, runs the
existing complete latched schedule, restores generic Virtual time, increments a
local completed counter afterward and returns it. This is the directive06 route;
it does not clamp after execution, undo state or introduce per-phase gating.

## Independent reproduction

The following commands exited zero on the clean candidate:

- Focused `cargo test --locked -p oxurack stop_accounting -- --nocapture` and
  its `--no-default-features` variant: 9/9 tests passed in each configuration.
- `cargo test --locked -p oxurack` and its no-default variant: 24/22 umbrella
  unit tests plus installation, sample-pipeline and seeded-context integrations
  passed.
- `cargo test --locked -p oxurack-core`: 414 core unit tests, 5 full-cycle tests,
  9 seeded-context tests and 2 doctests passed, including the raw phase latch.
- Host `cargo test --locked --workspace --all-features` and
  `--no-default-features`: both complete matrices passed; oxurack-rt reported
  150 passed and 3 pre-existing ignored tests in each relevant matrix.
- All-features/all-targets Clippy with `-D warnings`, formatting, strict workspace
  docs and strict no-default umbrella docs passed. The existing `block 0.1.6`
  future-incompatibility notice remains informational.
- Host `make check` passed dependency, lint, format, build and test gates. The
  separate `cargo outdated --root-deps-only --ignore-external-rel` observation
  completed without changing dependencies.
- `git diff --check 642313a..37deeace` and the one-path source-scope comparison
  passed.
- The Slice04 planning checker passed after closure and rejected its missing-row,
  stale-assignment and invalid-row-status controls. CC's raw logs preserve the
  pre-close checker runs, restricted four-MIDI-test failures, DNS failure and
  exact successful host retries.

The focused cases independently reproduced these exact observations: stops in
Produce, Consume or Observe during sample zero return/completed count 1, retain
index 0, emit all five markers once and advance Fixed/Virtual by 1 ms; a stop at
index 1 returns/counts/advances 2. Generic time equals restored Virtual time.
Repeated stopped calls preserve the complete observation. Starting and advancing
one matches a separately constructed uninterrupted two-sample seeded reference.
False in Produce followed by true in Observe permits all three requested samples.

## Row-by-row disposition

| Row | Independent disposition |
| --- | --- |
| M-01 | **Done, reproduced.** Produce/Consume/Observe stop cases finish one complete five-phase sample and return/count/advance exactly one; the index-one case produces two full phase sets and 2 ms. The predecessor and return-only clamp are rejected by the time/count oracle. |
| M-02 | **Done, reproduced.** Repeated stopped requests return zero and preserve counts, all time observations, owned seed/state, output port and trace. Start plus one reaches index 1/count 2 and equals the independent uninterrupted reference. |
| M-03 | **Done, reproduced.** Uninterrupted count, manual zero, exact Live error before mutation for normal/zero/stopped requests, raw-core latch, switching/overstep and manual/grouped-live suites pass. False then true follows final shared state and continues. |
| M-04 | **Done, reproduced.** Rustdoc states up-to-count complete progress, latched current sample, shared-stop next-sample boundary, zero/stopped no-op and Live misuse error. Strict docs pass; the public signature is unchanged. |
| M-05 | **Done, reproduced.** Focused, package, core, both host workspace matrices, Clippy, format, strict docs, native Make, separate outdated and diff checks pass. Restricted failures/host retries and the three ignored RT tests remain explicit. |
| M-06 | **Done, reproduced.** Source and planning hashes, one-file source fence, CC artifacts, assignment, negative-control checker, four-slice/24-row accounting and successor gates reconcile. |

Opening, CC return and CRC close denominators are each six; all rows are now
accepted `done`. This was the initial assignment and used 0 of 5 corrective passes.

## Bubble-up and arc re-entry

Slice04 repairs the sole failed interaction from CDC review06 without changing
the accepted installation, phase, seed or live-driver design. All four arc09
slices and 24 child rows are now CRC-accepted on their respective evidence, and
the integrated source candidate is `37deeace`.

CRC has refreshed the arc closing report, operational verification and composition
ledger for this candidate. E-01–E-04 remain at the independent CDC gate; this
slice close does not close arc09 or authorize arc10. TM/MIDI remains arc10,
Clock/LFO/Scope remains arc11, and coverage, physical MIDI/timing, benchmark,
soak and musical acceptance gates retain their owners.
