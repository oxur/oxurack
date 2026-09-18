# Slice02 CC closing report — iteration01 evidence repair

Date: 2026-09-17. Current assignment:
`arc09-executable-rack/slice02-simulation-samples/cc-prompt-iteration01.md`.

Status is **CC proposed-done for iteration01**. The earlier
[`artifacts/closing-report.md`](artifacts/closing-report.md) is preserved as prior-attempt evidence. CRC must
independently reproduce the six ledger rows and inspect the repaired source and
planning artifacts. This report does not accept the slice, close the ledger,
amend arc09, or release slice03.

## Source and planning provenance

The implementation is on branch
`codex/oxurack-module-behavior-installation` in
`.worktrees/codex-oxurack-module-behavior-installation`.

- `1a22066d96cdf87effbf4df3160813f94a54b53e` — Implement deterministic simulation samples.
- `8a73d33ad17cfad849db34d10b672fa4d894a79a` — Fix sample pipeline Clippy complexity.
- `5c1ce2ccb5e0ba36ac9fd2605a047e0ff45928b2` — Repair iteration01 transition, fixed-plugin, and ordering evidence.

The source worktree was clean at iteration01 intake and is clean at the final
source revision. The planning worktree was at
`881b834a83aaed1ed7663fa393074574b543dd6e` at intake. The iteration01
planning evidence commit is `733327f`; this metadata-only follow-up records
the final planning provenance.

The source diff stayed within the iteration01 allowlist and contains only test
changes:

- `crates/oxurack-core/src/tick.rs`
- `crates/oxurack-core/src/lib.rs`
- `crates/oxurack/src/lib.rs`
- `crates/oxurack/tests/sample_pipeline.rs`

## Iteration01 evidence repair

This return addresses the CRC partial-review findings without changing
production behavior or widening the source allowlist. The new checks prove
that zero-elapsed Live re-entry does not replay discarded time, stopped Live
updates do not run module phases, and the first fresh 1 ms update advances one
contiguous sample. The fixed core compatibility test explicitly preinserts
`SimulationRunning(false)` before `CorePlugin::fixed()` and proves no phase
runs until the caller enables it.

The ordering fixtures choose valid lexical/ModuleId oppositions at test
runtime. They assert the opposition before driving the real propagation and
topological paths, then assert the lexical-last source/name wins under
permuted entity creation. The value that a rejected numeric-ID order would
select is explicitly identified by the test oracle. `derive_seed` and ModuleId
production code remain unchanged.

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
[`artifacts/validation-results.tsv`](artifacts/validation-results.tsv). Every
raw log includes cwd, HEAD, branch, source state, toolchain, exact command,
actual exit code and command output. The current-slice checker is
[`artifacts/verify-slice02-planning.py`](artifacts/verify-slice02-planning.py).

The final clean source revision passed focused core, umbrella, module
installation, sample pipeline, all-targets Clippy with `-D warnings`, format,
strict workspace rustdoc, strict no-default umbrella rustdoc, host `make
check`, host dependency audit, `git diff --check`, and the structural planning
structural checker. The workspace host retries passed both all-features and
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
The inherited slice01 verifier remains structural-only evidence, not
current-slice acceptance evidence. The current checker ran one predicate
against the valid tree and deliberate negative copies: `valid_exit=0`,
`missing_close_exit=1`, and `stale_assignment_exit=1`. It reports structure
only and cannot establish behavior, CRC independence, or slice acceptance.

## Artifact inventory

- [`artifacts/test-evidence.md`](artifacts/test-evidence.md) — bounded intake/readback, proof delta, and final gate evidence.
- [`artifacts/validation-results.tsv`](artifacts/validation-results.tsv) — command matrix, exits, revisions, states and log references.
- [`artifacts/verify-slice02-planning.py`](artifacts/verify-slice02-planning.py) — current-slice structural predicate and negative controls.
- `artifacts/logs/01-preflight-cargo-binstall.log` through `artifacts/logs/53-current-slice-checker-final.log` — raw validation records, including all earlier attempts and the iteration01 repair gates.
- [`artifacts/closing-report.md`](artifacts/closing-report.md) — preserved prior-attempt report.

## Bubble-up to arc09

The slice delivers the arc09 executable rack capability specified for this
slice: deterministic 1 ms samples, shared phase visibility, stateful
sample-delayed propagation, explicit stopped/start/stop lifecycle, exclusive
manual/live drivers, name-stable observable ordering, raw-core compatibility,
and a public RON-backed producer/relay/sink proof.

The iteration01 return delivers the requested proof repair. It exposed and
corrected no production defect, required no architecture or scope amendment,
and added no seed, device, transport, or future-slice behavior. The previous
test-only Clippy type-complexity issue remains corrected in the inherited
source history. The remaining restricted failures are environment conditions
in inherited MIDI and registry access.

The following remain deliberately deferred by the approved slice boundary:
musical Clock, TM adapter, seed handoff, MIDI-out/device access, live
snapshot/save, full transport, physical timing promises, and the inherited
95% coverage target owned by Q-02/Q-03. No arc-plan amendment is proposed.
Arc09/slice03 remains held until CRC closes this slice.
