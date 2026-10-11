# CC iteration01 — enforce LFO input invariants and prove both schedules

2026-10-11, Sofie/CRC. Three contributors, Expedited Mode. Execute this
implementation pass and return to CRC through Duncan.
Project-relative assignment:
`arc11-observation-workbench/slice01-lfo/cc-prompt-iteration01.md`.
Predecessor: `arc11-observation-workbench/slice01-lfo/cc-prompt.md`.
The initial prompt stays issued and unchanged. This pass addresses **L-02**;
L-01/L-03/L-04/L-05 are already CRC reproduced, with affected evidence refreshed
as below. Slice01 stays open and Scope/Slice02 stays held until CRC acceptance.

## State, scope and reading

S = `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`.
Source baseline `a9653866f6ba4e4f238137ad24c2c5a8720d3f83`, branch
`codex/oxurack-module-behavior-installation`, clean at issue.
P = `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp`.
Planning branch `planning`; CC return inspected at
`353766a9f63b4139bba5a46dc93f11a8b4269147`. The subsequent CRC review/issue
commit governs this pass. Check live heads/diffs and the explicit current
assignment in `slice-plan.md`; reconcile material drift before dependent edits.
K = `/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`.
Resolve installed equivalents on another host.

Only these source paths are authorized:

- `crates/oxurack-mod-lfo/src/rack.rs` — strict private input decoding; inline
  adapter tests are allowed if useful.
- `crates/oxurack/tests/lfo.rs` — actual installed schedule/invariant regressions.

No engine, core, Clock, TM, runner, fixture, Cargo or public API changes are
needed or authorized. Preserve existing tests. Local test/helper organization
is CC's choice inside those two paths. No new fault framework, dependencies,
validator, coverage target or endpoint witness. No merge/push/release.

Complete the named reads, recover truncated output, then record a brief
source-cited readback in the existing closing report before implementation.
Do not make a new intake artifact or request another approval when coherent.

| Read | Scope and purpose |
| --- | --- |
| This prompt; sibling slice-plan.md, ledger.md, crc-verification.md | Required-full: assignment, all five rows and reproduced F-01/F-02 |
| P/cdc-directive09.md; P/arc11-observation-workbench/arc-plan.md | Required-full: delegated authority and held next slice |
| S/CLAUDE.md | Required-full: conventions and skill routing |
| Initial cc-prompt.md | Required-section: “Source-grounded design and engineering choices”, “Recommended API and implementation sequence”, “Tests and checks”, and “Scope, discretion and return”; inherit binding behavior, not old baseline/current assignment |
| S/crates/oxurack-mod-lfo/src/rack.rs; S/crates/oxurack/tests/lfo.rs | Required-full: actual systems, input helpers, existing oracles and lifecycle tests |
| S/crates/oxurack-core/src/lib.rs | Required-section: CorePlugin and CorePluginFixed implementations; actual supplied-schedule setup |
| S/crates/oxurack-core/src/port.rs | Required-section: Port/CurrentValue and spawn_port_on_module; typed initialization |
| S/crates/oxurack-core/src/tick.rs | Required-section: TickPhase, SampleTime, SimulationRunning and sample running latch/finalization; core owns gating |
| S/crates/oxurack-mod-lfo/src/lfo.rs | Required-section: next_sample/CV sanitization and sync state; numeric nonfinite policy stays intact |
| K/rust-guidelines/SKILL.md; K/testing/SKILL.md | Required-full: active domain/testing routes |
| K/rust-guidelines/guides/11-anti-patterns.md | Required-section: AP-24, AP-25, AP-33; invariant failure versus recoverable configuration and silent defaults |
| K/collaboration-framework/knowledge/testing/guides/03-validation-gates.md | Required-full: affected native proof and honest attempt retention |

Historical reports/checkers are reference-only; no archaeology campaign.
Read additional directly governing definitions if needed, not unrelated modules.

## Findings and engineering route

F-01 is reproduced at rack.rs:246–265: `float_value` silently returns 0 for
Gate and `gate_value` silently returns false for Float. Both schedules accept
the corrupted internal state. The plan already requires internal type failures
to be invariant bugs. Port construction already supplies Float(0)/Gate(false)
for unconnected inputs; no generic None fallback is required.

Use the current query/entity ownership. Replace only the wrong-variant arms
with diagnostic panics; keep missing-component `expect`s. Illustrative code
below is source-grounded guidance, **not a compiled patch**:

```rust
fn float_value(values: &Query<&CurrentValue>, entity: Entity) -> f32 {
    match values
        .get(entity)
        .expect("LfoState input must have a CurrentValue")
        .0
    {
        Value::Float(value) => value,
        actual => panic!("LFO freq_in must carry Value::Float; got {actual:?}"),
    }
}
// gate_value has the same shape:
// Value::Gate(value) => value,
// actual => panic!("LFO sync_in must carry Value::Gate; got {actual:?}"),
```

Applied Rust guidance: AP-24 uses panic for a broken construction invariant;
AP-25 keeps recoverable spawn/setter configuration errors as existing Results;
AP-33 prevents incompatible values from disappearing into ordinary controls.
Do not reject a correctly typed Float NaN/infinity here: standalone next_sample
still maps numeric nonfinite CV to neutral. Do not reset phase, alter retained
controls, add a second running gate, or change valid-input scheduling.

F-02 is a proof gap, not a claimed schedule implementation bug. The initial
tests contract explicitly asks for Update and FixedUpdate; current integration
tests only exercise FixedUpdate. Build one small shared actual-module setup
using the public hook, rather than copying the scheduler or engine:

```rust
use bevy_app::{App, FixedUpdate, Update};
use bevy_ecs::schedule::{InternedScheduleLabel, ScheduleLabel};
use oxurack_core::{CorePlugin, OxurackModule, SimulationRunning};
use oxurack_mod_lfo::LfoModule;

fn installed_app(fixed: bool) -> (App, InternedScheduleLabel) {
    let mut app = App::new();
    let schedule = if fixed {
        app.add_plugins(CorePlugin::fixed());
        FixedUpdate.intern()
    } else {
        app.add_plugins(CorePlugin);
        Update.intern()
    };
    LfoModule::register_systems(&mut app, schedule);
    // Spawn with actual ParameterValue entries and find its four port entities.
    // Existing lfo.rs integration helpers may be reused or narrowly extended.
    (app, schedule)
}
// app.world_mut().resource_mut::<SimulationRunning>().0 = true;
// app.world_mut().run_schedule(schedule);
```

These APIs were used in CRC's compiled actual-module corruption probe. The
full test fragment above is illustrative and must be compiled by CC. Keep
direct adapter installation distinct from facade `try_install`, which always
selects FixedUpdate. Using App::update would obscure which schedule is tested;
run the supplied schedule directly and let CorePlugin own all phases/gating.

## Discriminating regression evidence

Add small tests through actual scheduled Consume/Produce, not only direct
helper calls. Sharing setup across the two schedules is sufficient.

1. For each supplied schedule, spawn one LFO and identify its inputs by actual
   Port.name. Separately corrupt freq_in to Gate(true) and sync_in to Float(0.5).
   Set SimulationRunning true and run that schedule. Each must fail with the
   relevant invariant diagnostic. `#[should_panic(expected = ...)]` or checked
   `catch_unwind` are fine; isolate Apps so one failure cannot satisfy another.
   Demonstrate these regressions fail against a965386's neutral fallback before
   applying the fix; a passing panic test must not be a missing port/setup panic.
2. For each schedule, use saw_up, frequency_hz62.5, offset0, width.5, no cables.
   First run stopped: SampleTime completed0 and typed output defaults unchanged.
   Run four samples after start. Literal bipolar sequence is
   `[-1.0, -0.875, -0.75, -0.625]`; unipolar is
   `[0.0, 0.0625, 0.125, 0.1875]`; completed count is4. Stop and run once:
   outputs/count stay unchanged. Resume once: bipolar-.5, out.25, count5.
   This rejects wrong schedule registration, duplicate execution, output-after-
   increment, or adapter advancement while core is stopped. Expected values
   come from 1/16-cycle increments, not calling Lfo to predict itself.
3. Use the same actual adapter to prove correctly typed Float(NaN) remains
   neutral. With saw_up and base0, injecting CV before the first run gives
   bipolar -1 on three samples and finite unipolar0, with no invariant panic.
   Typed default zero/false is already covered by case2. Existing engine tests
   retain infinity/clamp/held-edge semantics; no new exhaustive matrix needed.

Keep existing loaded-patch, Clock/two-instance, sync latency, parameter,
stop/resume/reinstall and cable-removed tests. No need to redesign their oracle.

## Validation and evidence retention

From S, run after correction:

```bash
cargo test --locked -p oxurack-mod-lfo
cargo test --locked -p oxurack-mod-lfo --features rack
cargo test --locked -p oxurack --test lfo
cargo test --locked -p oxurack --no-default-features --test lfo
cargo test --locked -p oxurack --test midi_output --example turing-machine-midi
make test
make lint
cargo clippy --locked -p oxurack-mod-lfo -p oxurack --all-features --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' make docs
cargo build --locked -p oxurack --example turing-machine-midi
git diff --check
```

Reproduce two fresh offline 10000-sample new-fixture processes and compare;
expected70 rows/hash `988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
Original2000 sample fixture remains13 rows/hash
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.
Use the built example commands in CRC verification; no output selection.
Keep restricted failures and exact unchanged host retries distinct if encountered.
Keep all three pre-existing RT ignores and block0.1.6 advisory visible. No new
ignore, lint suppression, weakened assertion or successful-only receipts.

CRC's engine-only L-01 and dependency/scope observations at a965386 can be
retained after confirming the two-file delta leaves them applicable. L-02 and
adapter-dependent L-03/L-04/native L-05 results get fresh affected checks above.
No repeated Cargo freshness, planning verifier, coverage or artifact campaign.
Routine command results in closing-report.md are sufficient.

## Commit and return

Source commit with explicit paths only, from S:

```bash
git diff --check
git commit --only -m 'fix(lfo): enforce input invariants and verify supplied schedules' -- \
  crates/oxurack-mod-lfo/src/rack.rs \
  crates/oxurack/tests/lfo.rs
```

Stage a new authorized path explicitly first if necessary; both currently exist.
Inspect the actual commit path list. Preserve unrelated changes; never use `-a`
or broad staging. Keep source and planning separate. If any implementation
requires an unlisted file or contract change, stop that portion and report the
specific dependency through Duncan; do not silently expand scope.

Update existing closing-report.md with a dated iteration01 section identifying
this prompt/hash, actual source commit, readings/readback, F-01/F-02 red/green
results, full five-row walk, command results and source path inventory. Preserve
the original initial-pass report as history. Update ledger.md's L-02 evidence
to CC-attested proposed-done but leave its status open for CRC. Preserve other
CRC done statuses and revision-qualified evidence; CC cannot accept the repair.
Artifact inventory: none unless concrete durable output is needed in sibling
artifacts; no new tool. Bubble up actual design/scope effects, or state none.

Planning commit from the planning worktree with exactly these paths:

```bash
git commit --only -m 'docs(plan): return LFO corrective iteration01 evidence' -- \
  project01-mvp/arc11-observation-workbench/slice01-lfo/closing-report.md \
  project01-mvp/arc11-observation-workbench/slice01-lfo/ledger.md
```

Return source/planning hashes and tree states through Duncan. No self-referential
metadata follow-up is required; the return supplies the evidence commit hash.
CRC owns verification, assignment disposition, closure and Scope advancement.

## Author readiness

CRC inspected a965386's private input helpers, typed port initialization,
CorePlugin schedules, existing integration tests and actual compiled corruption
probe. Binding failure policy is already decided; no new public design choice
is pending. Rust AP-24/AP-25/AP-33 applied above, two-file ownership and reading
budget bounded, illustrative snippets labeled, literal/negative controls supplied,
existing runner/oracle retained. All six author-readiness checks pass.
