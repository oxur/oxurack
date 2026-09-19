# Slice03 CRC verification

Date: 2026-09-18. Reviewer: Sofie/CRC, independent of CC.
Assignment: `arc09-executable-rack/slice03-seeded-spawn-context/cc-prompt.md`.

## Candidate and review scope

I reviewed clean source commit
`642313a62db437ee2bf0b3d5b97fcdcffa16782a` and CC planning evidence
`698d2d0` with metadata follow-up `cc35c4b955cb04a3e1ec841745f42d0969be43b2`.
The source diff from accepted predecessor `5c1ce2c` changes eleven allowlisted
files: core seed/context/patch exports, the umbrella export, core manifest and
Cargo-required lockfile cleanup, and bounded core/umbrella tests and RON
fixtures. TM, graph, RT/device, scheduler, earlier slices and other manifests
are unchanged. Both worktrees were clean before review.

I inspected the complete production diff, both fixtures, the 588 lines of new
integration tests, CC's root closing report, test evidence, validation matrix
and retained raw logs. The Cargo.lock change removes the now-unreferenced direct
`ahash` chain; it does not upgrade a dependency.

## Independent reproduction

These independent commands exited zero on the clean candidate:

- `cargo test --locked -p oxurack-core`: 414 unit, 5 full-cycle integration,
  9 seeded-context integration and 2 doctests.
- `cargo test --locked -p oxurack` and its `--no-default-features` variant:
  all umbrella, installation, sample-pipeline and seeded-context suites passed.
- `cargo test --locked -p oxurack-core --test seeded_spawn_context -- --nocapture`:
  all 9 tests passed, including two separately launched child processes.
- `cargo test --locked --workspace --all-features` and
  `--no-default-features`, with host MIDI access: both passed; oxurack-rt
  reported 150 passed and 3 pre-existing ignored tests.
- `cargo clippy --locked --workspace --all-features --all-targets -- -D warnings`,
  `cargo fmt --all -- --check`, strict all-features workspace rustdoc, and
  strict no-default umbrella rustdoc passed. The existing future-incompatibility
  notice for `block 0.1.6` remains informational.
- Host `make check` passed its dependency, lint, format, build and workspace
  test gates. `git diff --check 5c1ce2c..642313a` passed.
- The inherited planning verifier passed with 23 imported documents, 24 exact
  successor rows and 340 local links. It remains structural evidence tied to
  its historical slice01 assignment, not semantic slice03 acceptance.

I also recalculated all six seed vectors independently from the specified byte
stream without calling production Rust. The results match the planning
reference and implementation: `a854da736b8befab`, `21819f1bfbd4fcea`,
`21819e1bfbd4fb37`, `5365fb0ec2a0228b`, `fda42c6e50ff0d2a`, and
`199408757f0d299d`.

## Row-by-row disposition

| Row | Independent disposition |
| --- | --- |
| T-01 | **Done, reproduced.** The exact domain, zero separator, little-endian master and UTF-8 byte length/name stream yield all six fixed FNV-1a values. Direct `ahash` use and dependency are gone; `derive_module_rng` seeds locked `SmallRng` from the result. |
| T-02 | **Done, reproduced.** ModuleId retains `derive_seed(0,name)` and the fixed `vco_1` value. Slice02 LastWins/name-order, phase and driver tests pass after numeric IDs change. |
| T-03 | **Done, reproduced.** The private-field context exposes the four named read-only values before state construction. Registered hooks use it; the default trait method calls legacy spawn once; public registration layout and direct `insert` fallback remain usable, and replacement clears the private hook. |
| T-04 | **Done, reproduced.** Real core and umbrella RON loads deliver seed/name/ID/parameters before state construction. Reordering preserves per-name state; changed seed/name changes it; a typed failure followed by an independent load receives only the latter patch context. Existing non-atomic partial-world semantics are unchanged. |
| T-05 | **Done, reproduced.** Two actual `current_exe` child processes each emit the six-line hard-coded sorted trace. Both equal the committed oracle derived from hard-coded plan seeds; changed seed/name differ, reorder is stable, stop preserves state, resume continues and a fresh load restarts. |
| T-06 | **Done, reproduced.** The eleven-file source diff is within the literal allowlist. Package, feature-matrix, Clippy, format, docs, Make, diff and planning checks pass, with CC's restricted MIDI/DNS attempts and host retries retained separately. |

Opening, CC return and CRC close denominators are each six; all rows are
accepted `done`. No ignored or weakened new test was introduced.

## Bubble-up to arc09

Slice03 delivers the third assigned arc piece: deterministic seed derivation
and pre-construction per-instance context with fresh-process replay. Together
with accepted slice01 installation and slice02 sample semantics, arc09 is now
eligible for its separate composition review. The implementation revealed no
new source dependency, scope gap or sequencing change. The silent-drop diff is
complete: TM/MIDI belongs to arc10; Clock/LFO/Scope follows in arc11; live
snapshot, coverage, physical MIDI/timing, benchmarks, soak and musical
acceptance remain with their named successors and gates.

No arc-plan structural amendment is required. CRC must assemble the formal
arc close; CDC must independently review that assembly before arc09 closes or
arc10 opens.

## What worked

The independent byte vectors, hard-coded RNG trace, two child processes and
changed-input/reordered controls make process-local hashing, OS seeding and
entity-order derivation observably wrong. The private parallel dispatch map
preserves the public legacy registration surface while allowing construction
context to arrive at the only point where seeded state can be built correctly.
