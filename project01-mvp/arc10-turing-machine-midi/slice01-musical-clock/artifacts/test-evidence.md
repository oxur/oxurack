# Slice01 test evidence

Source implementation commit: `3009b1efb8685e68d9fde9d8ca2bec4c40a0c979`
on `codex/oxurack-module-behavior-installation`.

The standalone package is `oxurack-mod-clock`. Its default feature set has no
rack dependencies; the optional `rack` feature supplies the ECS adapter. The
implementation stores canonical milli-BPM, advances a bounded `u64` phase
accumulator once per sample, starts a fresh epoch high, holds the exact pulse
width, and inserts a low separator after a changed configuration. The unit
oracles cover defaults, 137 BPM/4 PPQN fractional rises, the 1000 BPM/30 PPQN
boundary, validation order, canonicalization, equal-set no-ops, coalesced
edits, and pending-restart behavior.

The rack adapter exposes one `pulse` Gate output and registers one Produce
system on the supplied schedule. Its focused tests cover deterministic Update
registration, lexical unknown-parameter errors, strict aggregate validation
before entity creation, valid 1000/30/1 construction, setter commit ordering,
getter ownership, and non-finite rejection.

The umbrella fixture uses real RON cables and Consume/Observe consumers. It
records named sample traces for two different rates, the omitted-bpm default,
and an explicit 90 BPM module while patch metadata is 90 BPM. It advances
1,105 manual samples and observes rising indices `[0, 500, 1000]` at 120 BPM,
`[0, 250, 500, 750, 1000]` at 240 BPM, and `[0, 667]` at 90 BPM. It also
covers exact pulse low gaps, stop/zero-count behavior, restart after stop,
idempotent reinstall before and after progress, reversed module ordering and
an unrelated unconnected entity.

The package and umbrella locked tests, both feature modes, full Clippy,
formatting, rustdoc, locked dependency tree, `make check`, and `git diff
--check` are recorded in `artifacts/logs/`. The restricted workspace runs
failed only in pre-existing `oxurack-rt` host MIDI/clock tests; the exact
workspace commands passed with host access. Restricted `cargo outdated` was
blocked by crates.io DNS and passed on its host retry. These are retained as
separate logs rather than collapsed into a pass.

The planning checker passed its positive case and rejected missing-row,
stale-assignment and invalid-status negative controls. This packet is CC
proposed-done evidence only; independent CRC reproduction is still required.

## Iteration01 final corrective evidence

The corrective implementation is recorded in source commits
`2e4bfac` and `a19cfd1`. The cached source review and final status records
show exactly three changed paths: `crates/oxurack-mod-clock/src/clock.rs`,
`crates/oxurack-mod-clock/src/rack.rs`, and `crates/oxurack/tests/clock.rs`.
Cargo manifests, `Cargo.lock`, the README and
`crates/oxurack/tests/fixtures/clock.ron` remain byte-identical.

`rack.rs` now performs an unknown-name pass before any value decoding, using
the lexical first unknown key. It then decodes known fields in fixed order and
validates the complete candidate through `ClockConfig::try_new`; spawn errors
map BPM, PPQN, pulse-width and overlap failures to their responsible
parameters. Numeric range checks were removed from the adapter decoder, so
the standalone configuration remains the single numeric validation oracle.
Invalid spawns leave Module and Port counts unchanged. Setter tests cover all
three valid paths and wrong-value variants, getters read committed Clock
state, missing/non-Clock entities return `UnknownModule`, and invalid edits
preserve the private Clock snapshot and held `CurrentValue` across the pending
low separator and following rise.

`clock.rs` now asserts the 120 BPM / 4 PPQN / 3 ms fractional schedule with
rises `[0, 125, 250, 375, 500]` and complete high/low windows. The rejection
matrix includes NaN, positive and negative infinity, BPM range, PPQN range,
pulse-width range and overlap. The edit oracle covers high samples 0--2, the
pending low sample 3, high samples 4--13, low samples 14--253 and the next
rise at 254, alongside no-op, coalescing and return-to-original behavior.

The umbrella Clock test now uses reusable manual and controlled-live runners.
`GateTrace` and `ClockRun` compare complete named sample vectors and
`SampleTime`, using an independently constructed expected trace for slow,
fast, default, explicit-90-BPM and disconnected consumers. It covers
repeated installation before load, manual `[1105]` and grouped
`[1,2,7,95,1000]` execution, live single and grouped execution, stop while
high with zero/nonzero stopped requests, pause/resume, reinstall after
progress, and the reversed fixture. The live harness raises only the test
virtual-time cap needed to exercise the requested 1000-sample group.

The final locked package, adapter, umbrella, core, workspace, tree, Clippy,
format, rustdoc, Make, freshness and diff results are listed in the appended
`results.tsv` rows and raw logs `63`--`81`. Restricted workspace and Make
runs retain the pre-existing `oxurack-rt` MIDI initialization failures;
host-access replays pass. Restricted `cargo outdated` retains its crates.io
DNS failure; the host-access replay passes. The initial Clippy attempt and
its retained failure are `52-cargo-clippy-iteration01.log`; the corrected
final pass is `71-cargo-clippy-final.log`.

The current planning checker result is retained in
`artifacts/logs/82-planning-checker-iteration01-final.log`; its positive
baseline and missing-row, stale-assignment and invalid-status negative
controls pass. All six rows remain proposed-done pending independent CRC
reproduction. No CDC escalation, ledger closure or Slice02 advancement is
claimed.
