# Slice02 CC closing report — deterministic simulation samples

Date: 2026-09-17. Current assignment:
`arc09-executable-rack/slice02-simulation-samples/cc-prompt.md`.

Status is **CC proposed-done**. CRC must independently reproduce the six
ledger rows and inspect the source and planning artifacts. This report does
not accept the slice, close the ledger, amend arc09, or release slice03.

## Source and planning provenance

The implementation is on branch
`codex/oxurack-module-behavior-installation` in
`.worktrees/codex-oxurack-module-behavior-installation`. The final source
revision is `8a73d33ad17cfad849db34d10b672fa4d894a79a`:

- `1a22066d96cdf87effbf4df3160813f94a54b53e` — Implement deterministic simulation samples.
- `8a73d33ad17cfad849db34d10b672fa4d894a79a` — Fix sample pipeline Clippy complexity.

The source worktree was clean for the final validation pass. The planning
worktree was at `bc641325f2d48c56516cfe7b3a202e4f5e19c539` at intake. The
planning evidence commit is recorded after this report is committed; this
report will receive a follow-up metadata-only commit with that hash.

The source diff stayed within the exact allowlist:

- `crates/oxurack-core/src/tick.rs`
- `crates/oxurack-core/src/lib.rs`
- `crates/oxurack-core/src/error.rs`
- `crates/oxurack-core/src/module.rs`
- `crates/oxurack/src/lib.rs`
- `crates/oxurack/tests/module_installation.rs`
- `crates/oxurack/tests/sample_pipeline.rs`
- `crates/oxurack/tests/fixtures/sample-pipeline.ron`

## Delivered behavior

`SampleTime` is a private-state, 1 ms core resource with zero-based
`last_index()` and `completed_samples()` accessors. A running sample publishes
one index and completed-count pair to all five named phases, then finalizes
after Observe. Checked increment preserves the no-silent-wrap invariant.

`TickPhase` now runs as the strict chain
`Produce -> Propagate -> MergeInputs -> Consume -> Observe`. The sample-begin
gate latches `SimulationRunning` before Produce, and the same latch gates all
five phases and finalization. Port merge is in MergeInputs, so Consume sees
current-sample merged inputs while stateful outputs remain delayed to the next
sample. Module documentation defines named phase membership as the condition
for receiving this lifecycle guarantee.

The core plugins preserve raw-core compatibility: bare CorePlugin and
CorePluginFixed default to running only when the resource is absent and retain
an explicitly inserted false value. Oxurack constructs stopped, with the
shared core running resource set to false. Start and stop pause or resume the
named pipeline without resetting sample index or module state.

The umbrella exposes `SimulationDriver::{Live, Manual}` and the requested
inherent lifecycle and sample APIs. Manual advancement runs exactly the
requested number of 1 ms fixed samples, aligns Bevy virtual/fixed/generic
time during direct FixedUpdate execution, and returns the exact
`ManualSampleRequiresManualDriver` error in Live mode without mutation.
Manual mode, stop/start, and Live re-entry discard pending fixed overstep;
Live re-entry rebases real time and only unpauses when the rack is running.
`step()` retains its app-update return type and cannot create manual samples.

Propagation and fan-in ordering use stable module instance and port names.
The existing ModuleId seed/value behavior was left unchanged. The real RON
fixture installs a producer, stateful relay and sink, with a disconnected
sample-and-hold input.

## Sample traces

The public sample fixture produced this exact observation for manual samples
0, 1 and 2 and for controlled live groups `1+1+1` and `1+2`. Every row also
observed a generic time delta of 1 ms; the disconnected hold remained 0.25.

| index | producer out | relay input | relay out | sink input | sink out | completed after Observe |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 1 | 1 | 0 | 0 | 0 | 1 |
| 1 | 2 | 2 | 1 | 1 | 0 | 2 |
| 2 | 3 | 3 | 2 | 2 | 1 | 3 |

Inside every row, all phase markers saw the same pair:

```text
sample 0: Produce/Propagate/MergeInputs/Consume/Observe -> (0, 0)
sample 1: Produce/Propagate/MergeInputs/Consume/Observe -> (1, 1)
sample 2: Produce/Propagate/MergeInputs/Consume/Observe -> (2, 2)
```

The manual/live test also proves that stopped racks do not change sample
state, manual `step()` creates no sample, zero-count manual advancement is a
no-op, and a wrong-driver manual request returns the exact error without
mutation. The LastWins test gives the same value for source spawn orders
`[a,b]` and `[b,a]` with an unrelated entity present.

## Proposed six-row walk

This is the CC's proposed completion walk. Each row remains open for CRC
reproduction.

| ID | Proposed CC result | Evidence |
| --- | --- | --- |
| S-01 | proposed-done | `sample_pipeline` manual indices 0/1/2, completed counts 1/2/3, controlled live groups, and `SampleTime` API |
| S-02 | proposed-done | producer/relay/sink RON fixture, exact delayed values, disconnected hold, and five phase markers |
| S-03 | proposed-done | core default/explicit-false tests, stopped Oxurack construction, start/stop negatives, and latched phase gate |
| S-04 | proposed-done | driver mismatch error, manual `step()` negative, transition/overstep tests, and live grouping equivalence |
| S-05 | proposed-done | name-stable LastWins test under spawn permutation and unrelated entity, plus named propagation tie-break |
| S-06 | proposed-done | bounded source diff, default/no-default tests, workspace gates, docs, Make, outdated, diff and planning logs |

CRC disposition, ledger status changes, and slice advancement are pending.

## Validation

The complete command matrix is in
`artifacts/validation-results.tsv`. Every raw log includes cwd, HEAD, branch,
source state, toolchain, exact command, actual exit code and command output.

The final clean source revision passed focused core, umbrella, module
installation, sample pipeline, all-targets Clippy with `-D warnings`, format,
strict workspace rustdoc, strict no-default umbrella rustdoc, host `make
check`, host dependency audit, `git diff --check`, and the structural planning
verifier. The workspace host retries passed both all-features and
no-default-features matrices; each reported 150 oxurack-rt tests passed and 3
pre-existing timing tests ignored.

The restricted all-features and no-default workspace runs each exited 101
after four MIDI initialization tests failed in the restricted environment;
the same commands passed with host access. Restricted `make check` exited 2
after the same MIDI failures; its host retry passed. Restricted
`cargo outdated --root-deps-only --ignore-external-rel` exited 1 because
`index.crates.io` DNS was unavailable; the host retry passed. The preflight
`cargo-binstall --version` command exited 2 because the installed parser treats
`--version` as a package-version argument; no installation was attempted.
These results are preserved rather than collapsed into a pass-only summary.

## Artifact inventory

- `artifacts/test-evidence.md` — bounded intake/readback and source/plan provenance.
- `artifacts/validation-results.tsv` — command matrix, exits, revisions, states and log references.
- `artifacts/logs/01-preflight-cargo-binstall.log` through `22-planning-verifier-clean-source.log` — raw validation records.
- `artifacts/closing-report.md` — this proposed-done report.

## Bubble-up to arc09

The slice delivers the arc09 executable rack capability specified for this
slice: deterministic 1 ms samples, shared phase visibility, stateful
sample-delayed propagation, explicit stopped/start/stop lifecycle, exclusive
manual/live drivers, name-stable observable ordering, raw-core compatibility,
and a public RON-backed producer/relay/sink proof.

The implementation exposed one test-only Clippy type-complexity issue in the
new sample fixture; it was corrected in the scoped follow-up commit. No new
design gap or source-scope expansion was found. The remaining restricted
failures are environment conditions in inherited MIDI and registry access.

The following remain deliberately deferred by the approved slice boundary:
musical Clock, TM adapter, seed handoff, MIDI-out/device access, live
snapshot/save, full transport, physical timing promises, and the inherited
95% coverage target owned by Q-02/Q-03. No arc-plan amendment is proposed.
Arc09/slice03 remains held until CRC closes this slice.
