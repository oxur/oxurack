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
