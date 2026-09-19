# CDC review06 evidence

Source `642313a62db437ee2bf0b3d5b97fcdcffa16782a`, planning intake
`b2574a583f9d73bb533f606e3e6f6cb3d1217d5e`, 2026-09-18. This is reviewer-owned
composition evidence, not product changes or a substitute for CRC's records.

- checks.json enumerates commands, source/cwd, exits and logs. Restricted full
  workspace tests fail four MIDI initialization tests; host repeats pass.
  Core/umbrella/no-default packages, full Clippy, format, strict docs, host
  make check and separate outdated all pass. No dependency upgrade occurred.
- probe-results.json records both feature modes: ten successful trace controls
  and two reproduced mid-batch-stop failures. Failure exit 101 is a real failing
  assertion, not counted as implementation success.
- probe/ holds Cargo.toml/lock, final source, the earlier pre-stop-probe source,
  real RON inputs and independent expected.txt. build_probe.py derives that
  six-line oracle directly from the specified FNV bytes, then a simple local
  recurrence; it does not call production derive_seed. The final source adds
  the stop-during-Consume case. This probe is a synthetic module, not TM/RNG
  equivalence; the existing package suites independently exercise SmallRng replay.
- run_probe.py builds and invokes the final probe with/without graph. It checks
  package name/version inclusion against the source lock; only the probe itself
  is added. The first build attempt without the source lock failed on yanked
  spin 0.10.0; seeding the external lock from the source lock resolved it without
  an upgrade. Earlier build logs include an unused probe import; it was removed
  for the final builds. All attempts remain here.
- Normal controls: two separate manual processes, grouped live updates, reordered
  module spawning, changed seed. Manual runs also reinstall between sample groups
  and stop/resume without losing seeded state. The trace includes sample index,
  name, published output, merged input and next state of two connected instances.
- mid-stop: a registered Consume system sets SimulationRunning(false) in sample
  zero of advance_samples(3). Both feature modes print returned=3, completed=1,
  last_index=Some(0), fixed_elapsed=3ms, virtual_elapsed=3ms, then fail the
  returned-versus-completed assertion. The core correctly latches the current
  sample; the umbrella batch loop does not recheck before advancing the next one.
- planning-before.log is inherited structural evidence only. It is tied to
  slice01's preserved current-assignment metadata, not a semantic arc validator.

Reproduction: restore probe/ in a temporary directory; point its local path
 dependencies at the exact source checkout, retaining the source lock versions.
Use `cargo run --offline --locked --manifest-path <probe>/Cargo.toml -- mid-stop
<probe>/probe.ron`, then add `--no-default-features` before `--`. Exit 101 and
the actual values above reproduce the defect. Other modes are manual/live;
run_probe.py records the exact local commands from this review. Script roots
are explicit local provenance and must be adjusted when replaying elsewhere.
No device playback, coverage, timing target, soak or Operator acceptance is claimed.

Staging check: the raw Cargo/make logs retain three blank-at-EOF notices.
`git diff --cached --check` passes for non-log artifacts; with only that
whitespace category disabled, the entire staged packet passes. Raw logs were
kept byte-for-byte rather than normalized to hide this distinction.
