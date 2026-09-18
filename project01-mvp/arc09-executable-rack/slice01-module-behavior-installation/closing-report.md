# Slice01 CC proposed-done report

Status: proposed-done for CRC reproduction; I-01 through I-06 remain open.
This report records execution of the active replacement prompt
`arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration01.md`.
The predecessor `cc-prompt.md` remained held and unexecuted; executed refinement
count remains 0 of 5.

## Exact worktrees and commits

- Source worktree: `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`
- Source branch: `codex/oxurack-module-behavior-installation`
- Source baseline: `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`
- Source implementation commit: `d43078ea9c71bba10035699c9914bef3891f1d83` (`Make installed modules execute in the rack`)
- Planning worktree: `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`
- Planning state at intake: `4e50a3a0171b44344f19e4bbf1b8307dafec741c`
- Planning evidence commit: `c53989d7f74aa33653a64077fbdf2594f9bd5400`

Validation commands ran against the source worktree at the baseline revision
with the implementation changes present and unstaged. The exact resulting tree
is source commit `d43078e`; the results table records the revision actually
reported by the commands. The source worktree is clean after that commit.

## Delivered scope

The implementation adds the default `OxurackModule::register_systems` hook,
fallible `Oxurack::try_install`, and typed
`CoreError::ModuleKindConflict`. Installation keeps private rack-local
`ModuleKind -> TypeId` ownership, rejects a different type before registry,
accessor, schedule, or resource mutation, invokes a new kind's hook once on
`FixedUpdate.intern()`, and records the owner only after the hook returns.
Same-type installation is idempotent. The existing `install` method delegates
to the fallible path and retains chaining with a documented programmer-error
panic on collision. Hook panics remain uncaught programmer failures, and the
process-local `TypeId` map is not serialized or used in seeds/events.

The fixture test loads two instances through the public RON path, runs the
actual installed system on direct `FixedUpdate`, proves `Update` does not run
it, checks the `(0, 0) -> (2, 3) -> (4, 6) -> (8, 9)` state trace, exercises
parameter accessors after reinstall, checks same-kind conflict preservation,
independent racks, the legacy default hook, the convenience panic, and the
graph-enabled/disabled builds. A hook counter is supplemental; the assertions
inspect each loaded entity's state.

The only additional source changes are the three prompt-authorized,
doc-comment-only repairs in the Turing-machine and graph crates required by
strict rustdoc. No manifest, lockfile, dependency, scheduler, device, MIDI,
seed, timing, or future-slice source changed.

Exact source paths in `d43078ea9c71bba10035699c9914bef3891f1d83`:

- `crates/oxurack-core/src/module.rs`
- `crates/oxurack-core/src/error.rs`
- `crates/oxurack/src/lib.rs`
- `crates/oxurack/tests/module_installation.rs`
- `crates/oxurack/tests/fixtures/behavior-installation.ron`
- `crates/oxurack-mod-turingmachine/src/length.rs` (doc comments only)
- `crates/oxurack-mod-turingmachine/src/write_knob.rs` (doc comments only)
- `crates/oxurack-graph/src/lib.rs` (doc comments only)

## I-01 through I-06 walk

| ID | CC evidence | Proposed status |
| --- | --- | --- |
| I-01 | Focused default and no-default tests pass. The real installed system updates both loaded instances on `FixedUpdate`; two racks each invoke their hook once; the legacy no-op module loads. | Open pending CRC reproduction. |
| I-02 | Same-type reinstall preserves loaded state and accessors. The distinct same-kind type returns `CoreError::ModuleKindConflict` before mutation; original metadata, spawner, setter/getter and behavior remain observable. | Open pending CRC reproduction. |
| I-03 | The public fixture test observes no `Update` change, then exact per-instance fixed-schedule changes; graph module count is checked when the graph feature is enabled. Both feature matrices pass under host access. | Open pending CRC reproduction. |
| I-04 | Strict workspace rustdoc and strict no-default umbrella rustdoc pass. Hook, fallible install, conflict, and legacy panic docs are present; the three legacy files contain only bounded comment repairs. | Open pending CRC reproduction. |
| I-05 | Focused/package/workspace tests, format, strict rustdoc, targeted clippy, host `make check`, and host dependency audit pass. Restricted MIDI runs are retained as failures. Full all-features clippy remains exit 101 on pre-existing diagnostics in out-of-allowlist `oxurack-graph/src/lib.rs` and `oxurack-core/src/bridge.rs`; the planning verifier remains exit 1 on its expected stale `Assignment mismatch`. | Open; CRC/CDC must disposition the retained gate results. |
| I-06 | This report, the intake readback, validation TSV, individual raw logs, explicit source commit, and explicit planning evidence are preserved. Source and planning scopes remain separate, and no ledger row is closed. | Open pending CRC review. |

## Validation and limitations

`artifacts/validation-results.tsv` records every required command with its
working directory, actual source revision, toolchain, exit code, result, and
raw log path. The key gate facts are:

- Focused module-installation tests passed in default and no-default builds;
  core and umbrella package tests passed.
- Both workspace feature matrices passed with host access. Restricted runs
  failed only because four existing `oxurack-rt` MIDI initialization tests
  could not initialize MIDI; each failure and the successful host retry remain
  recorded. Three pre-existing physical/timing/priority tests stayed ignored;
  no ignored tests were run.
- Strict workspace and no-default umbrella rustdoc passed. Format passed.
- Full all-features clippy returned 101 for the two existing diagnostics named
  above. Targeted clippy for the changed core and umbrella surfaces passed.
  No out-of-scope lint allowance or repair was added.
- Restricted `cargo outdated` returned 1 on crates.io DNS; the host retry
  returned 0 and listed available upgrades without changing dependencies.
- Restricted `make check` stopped at the same MIDI initialization tests; host
  `make check` passed dependency, lint, format, build, and test stages.
- `git diff --check` passed. The preserved planning verifier returned 1 with
  `Assignment mismatch` because it still asserts the held predecessor
  `cc-prompt.md`; the raw result is retained as required and is not promoted
  to a pass.

No coverage target, sample-timing claim, seed claim, or acceptance claim is
made here. CRC must reproduce the exact source commit and inspect the planning
evidence before any row or slice status changes.

## Evidence inventory and handoff

- `artifacts/test-evidence.md` — bounded intake and contract readback recorded
  before dependent source edits, including the explicit actual-state oracle.
- `artifacts/validation-results.tsv` — command, cwd, revision, toolchain, exit,
  result, and log index.
- `artifacts/logs/00-environment.txt` through
  `artifacts/logs/18-planning-verifier.txt` — individual raw command outputs,
  including restricted failures and host retries.
- `slice-plan.md` — assignment disposition only; design, ledger rows, and
  verifier source remain unchanged.
- Source commit `d43078ea9c71bba10035699c9914bef3891f1d83` — implementation
  candidate for CRC reproduction.

The next review remains CRC reproduction of I-01 through I-06. Arc09 slice02
continues to own sample/phase/manual-live semantics, and slice03 continues to
own seed-context behavior; this implementation does not advance either
successor or alter their contracts.

## Iteration02 corrective report

Status: proposed-done for CRC review of the first corrective pass; the ledger
remains CC/CRC-owned and no row is closed by this report. Active assignment:
`arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration02.md`.
Predecessor: iteration01 at `d43078e`; the held initial `cc-prompt.md` remains
unexecuted. This execution consumes corrective pass 1 of 5.

### Exact states and source scope

- Source worktree: `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`
- Source branch: `codex/oxurack-module-behavior-installation`
- Source baseline: `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`
- Iteration02 source commit: `6a31c0f99241eccb0dcf0f23e9f64ac9dec2f94a` (`Prove live module reinstall and clear gates`)
- Source state after commit: clean
- Planning worktree: `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`
- Planning assignment state at intake: `c12ab9d7f19daf5c7510e5697fb64129ddf5bbbb`
- Planning evidence commit: `0f4561b06708f36287abd519d5bf0d24a3e7bcde`

The corrective source diff contains exactly four paths:

- `crates/oxurack/src/lib.rs`: existing unit-test module only; live
  after-edit reinstall fixture and temporal oracle.
- `crates/oxurack/tests/module_installation.rs`: one test rename clarifying
  its pre-edit `into_app` boundary.
- `crates/oxurack-graph/src/lib.rs`: `find_port` test-helper let-chain only.
- `crates/oxurack-core/src/bridge.rs`: queue test `std::mem::take` only.

No production behavior, public API, scheduler, seed, MIDI, device, dependency,
manifest, lockfile, coverage target, or future-slice source changed.

### I-01 through I-06 proposed walk

| ID | Iteration02 evidence | Proposed disposition |
| --- | --- | --- |
| I-01 | Clean default and no-default integration tests pass after the corrective commit. The existing two-rack, legacy-hook and once-per-kind behavior remains intact. | Candidate regression reproduced; CRC review remains required. |
| I-02 | New unit test keeps the rack alive after two fixed runs to `(4,6)`, edits alpha to 4 through `ParameterRegistry`, verifies `(4,3)`, reinstalls the same type, verifies state/getters and one hook invocation remain unchanged, then reaches `(8,9)` with exactly one system execution per tick. Existing conflict and before-edit integration coverage remains. | Proposed close pending CRC reproduction. |
| I-03 | Clean default/no-default integration tests and both clean workspace feature matrices pass under host access; the public RON fixture still proves fixed-schedule behavior and graph-on/off coverage. | Candidate regression reproduced; CRC review remains required. |
| I-04 | Clean strict workspace and no-default rustdoc both pass; no legacy documentation files changed in iteration02. | Candidate regression reproduced; CRC review remains required. |
| I-05 | Full all-targets Clippy passes after the two authorized test-only repairs. The assignment-aware full planning checker passes. Restricted workspace and `make check` attempts retain MIDI/timing failures, while host retries pass; outdated restricted DNS and host success are both retained. | Proposed close pending CRC disposition of retained environment failures. |
| I-06 | Exactly ten historical nonexistent cwd cells were corrected, all old rows and raw logs remain, explicit dirty/clean source states were added, clean candidate rows point to `6a31c0f`, and this report contains the required bubble-up. | Proposed close pending CRC inspection of provenance and scope. |

### Validation and retained limitations

`artifacts/validation-results.tsv` has 62 command rows: the original 20 are
preserved with corrected cwd metadata and explicit historical dirty state; 22
new dirty-candidate rows cover the live test, unchanged gates, checker and
fixture negatives; 20 clean-candidate rows cover focused/full regressions and
gate reruns. Each row names its exact raw log.

The dirty and clean candidate focused tests, package tests, full all-targets
Clippy, formatting and strict rustdoc pass. Both workspace matrices pass with
host access and retain three ignored pre-existing real-time/timing/priority
tests; no ignored tests were run. Restricted workspace tests fail four MIDI
initialization cases. Restricted clean `make check` additionally reports one
`timing::tests::test_monotonic_clock_advances` failure; its host retry passes.
Restricted dependency audits fail on crates.io DNS, and host retries complete
with available updates listed but no dependency changes. The full checker and
all six temporary assignment fixtures pass. These results do not claim hardware,
sample-time, seed, coverage-target, or independent acceptance completion.

### Bubble-up to arc09

The delivered capability is a candidate-complete module-installation
prerequisite: installed behavior remains once-per-kind and rack-local, live
state survives same-type reinstall after a parameter edit, and the formerly
blocked all-targets lint and assignment-aware structural checks now run green.
The new evidence also makes the historical dirty-tree provenance explicit.

Newly confirmed gaps are environmental: restricted execution cannot initialize
MIDI and can expose a timing flake, while restricted dependency inspection
cannot resolve crates.io. Host retries pass, but CRC must decide whether those
environment-qualified results satisfy the existing gate contract. The stale
checker assertion is repaired without weakening its denominator, import,
successor, closure-record or local-link checks.

Scope was delivered as specified for this corrective pass: the four named
source paths, the after-edit unit oracle, the two test-only lint repairs, the
metadata-driven checker, six temporary checker cases, provenance correction,
explicit logs and report. Deferred work remains CRC independent reproduction,
ledger closure, arc09/slice02 sample/phase/manual-live semantics, slice03 seed
context, Q-02/Q-03 coverage work, and all hardware/MIDI acceptance. No successor
slice is advanced by this packet.

### Iteration02 artifact inventory

- `artifacts/test-evidence.md`: fresh intake/readback and provenance basis.
- `artifacts/iteration02-verifier-fixtures.md`: exact temporary fixture script,
  inputs and outputs.
- `artifacts/validation-results.tsv`: preserved historical rows plus dirty and
  clean candidate rows with source state.
- `artifacts/logs/00-environment.txt` through `18-planning-verifier.txt`:
  iteration01 raw evidence preserved unchanged.
- `artifacts/logs/19-live-reinstall-default.txt` through
  `60-planning-verifier-clean-source.txt`: iteration02 raw attempts, retries
  and clean candidate gates.
- Source commit `6a31c0f99241eccb0dcf0f23e9f64ac9dec2f94a`: four-path corrective
  candidate for CRC reproduction.
