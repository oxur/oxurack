# Slice01 LFO — CRC review

**Current verdict (2026-10-11): accepted and closed at
`c4b517c89f51d44c0eb1bec4a7b891984d3e0a92`; all five rows done.**

2026-10-11, Sofie/CRC; Three-Contributor Workflow, Expedited Mode.
Assignment reviewed: `cc-prompt.md`; CC return: `closing-report.md` in planning
`353766a9f63b4139bba5a46dc93f11a8b4269147`.
Source reviewed: `a9653866f6ba4e4f238137ad24c2c5a8720d3f83`, based on accepted
Arc10 `65522f116a12a9ac96d35cfe25ed9c184f5bddcb`. Both worktrees were clean.
Main remains `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.

## Initial review verdict and authority — historical

Four rows are independently reproduced and accepted; **L-02 remains open**.
The musical modulation path runs and native gates pass. One binding invariant
failure needs repair, and the explicitly requested supplied-schedule proof needs
completion. Issue `cc-prompt-iteration01.md`, preserving CDC's initial prompt
and CC's initial report. Slice01 and Arc11 remain open; Slice02 stays held.
This restores the existing contract, without a design change or new acceptance
gate. Directive09 is acknowledged; its subsequent Scope assignment follows LFO
acceptance. No source implementation repair was performed by CRC.

## Independent checks at a965386

| Command / method | Observed result |
| --- | --- |
| `cargo test --locked -p oxurack-mod-lfo` | 4 unit tests + 1 doc test pass |
| `cargo test --locked -p oxurack-mod-lfo --features rack` | 4 unit tests + 1 doc test pass |
| `cargo test --locked -p oxurack --test lfo` | 5 pass |
| `cargo test --locked -p oxurack --no-default-features --test lfo` | 5 pass |
| `cargo test --locked -p oxurack --test midi_output --example turing-machine-midi` | 2 integration + 11 example tests pass |
| `cargo tree --locked -p oxurack-mod-lfo --edges normal` | Standalone has thiserror; no Bevy/core normal dependency |
| `make test`, host access | Workspace passes; RT 150 pass, 3 pre-existing ignores |
| `make lint` | Pass, including format check |
| `cargo clippy --locked -p oxurack-mod-lfo -p oxurack --all-features --all-targets -- -D warnings` | Pass |
| `RUSTDOCFLAGS='-D warnings' make docs` | Pass |
| `cargo build --locked -p oxurack --example turing-machine-midi` | Pass |
| `git diff --check` | Pass |
| Two fresh built-example processes, new patch, `--samples 10000`, then `cmp` | Both 70 rows; equal |
| Fresh built-example process, original patch, `--samples 2000` | 13 rows; pinned compatibility hash matches |

Commands ran in the source worktree. Trace commands used
`./target/debug/examples/turing-machine-midi --patch
crates/oxurack/examples/patches/turing-machine-lfo.ron --samples 10000`, and
the original `turing-machine-midi.ron` for 2000. New trace SHA-256:
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
Original trace SHA-256:
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.
No endpoint was opened for these checks. The existing block0.1.6 advisory and
three RT ignores are not newly passing evidence. CC's restricted four CoreMIDI
failures and unchanged host retry remain preserved in its report; CRC reproduced
the host gate, not another restricted failure. Temporary traces/logs are routine
verification outputs, not durable supporting artifacts.

## Findings

**F-01 — wrong input value types silently become neutral controls (L-02).**
`crates/oxurack-mod-lfo/src/rack.rs:246–265` maps every non-Float frequency
input to 0 and every non-Gate sync input to false. The binding slice-plan rack
section instead classifies missing/wrong typed internal port state as an
invariant bug. Core `spawn_port_on_module` initializes each input with its typed
default, so an unconnected port does not require this fallback. The public
engine's Float NaN/infinity-to-neutral policy is a separate, legitimate policy
and must remain intact.

CRC reproduced the failure through the actual adapter, not a copied helper:
create `App` with `CorePlugin` (Update) or `CorePlugin::fixed()` (FixedUpdate),
call `LfoModule::register_systems` with that interned label, spawn one LFO,
locate the input by `Port.name`, overwrite `CurrentValue`, set
`SimulationRunning(true)`, and run that schedule under `catch_unwind`.
The probe used a temporary Cargo example in the source worktree and removed it
after execution; the worktree was clean afterward.

| Supplied schedule | Corrupted input | Panic observed | Required |
| --- | --- | --- | --- |
| Update | freq_in = Gate(true) | false | invariant failure |
| Update | sync_in = Float(0.5) | false | invariant failure |
| FixedUpdate | freq_in = Gate(true) | false | invariant failure |
| FixedUpdate | sync_in = Float(0.5) | false | invariant failure |

**F-02 — direct supplied Update proof absent (L-02).** Initial prompt's tests
table explicitly requests Update and FixedUpdate using actual CorePlugin.
Existing LFO integration tests import/run FixedUpdate and facade installation
also uses FixedUpdate. Source wiring appears correct; this is missing requested
proof, not a demonstrated valid-Update production defect. Add a small shared
schedule test with literal phase outputs and core stop/resume gating.

## Row walk and scope

Opening/report/review denominator: **five**; four done, one open; no deferrals
or no-ops.

| ID | CRC status | Independently reproduced evidence / qualification |
| --- | --- | --- |
| L-01 | done | Engine source and default/rack tests: literal waveform orientation/phase, width, CV clamp/nonfinite/hold, edge sync and validation; optional dependencies remain optional |
| L-02 | open | Existing Clock/two-instance latency, stop/resume/reinstall tests pass; F-01 violates the internal input contract and F-02 leaves requested Update proof incomplete |
| L-03 | done | Registered four getters/setters, valid edits preserve phase, invalid edits preserve continuation, failed spawn adds no entities; source validates before mutation |
| L-04 | done | Loaded accelerated fixture agrees with independent seeded TM/literal MIDI oracle and differs with modulation cable removed; fresh processes and original trace hash reproduce |
| L-05 | done | Eleven-path source scope inspected, native gates above reproduced, honest restricted/host/ignored receipts retained; no freshness/coverage campaign or validator |

The source commit's 11 paths match CC's explicit inventory, including the
authorized shared test oracle. Original fixture, TM production and output
selection behavior remain unchanged; Cargo.lock adds only local LFO package
and dependency edges. No slice-owned supporting artifacts were produced; the
implementation, tests, fixture and README are in source. CC's silent-drop claim
is qualified by F-02. No additional architectural dependency emerged.

What worked: the literal accelerated modulation oracle and cable-removed control
prove a real musical effect, and the original fixture hash bounds compatibility.
The correction does not invalidate engine-only L-01 claims. Adapter/integration
checks are affected and must be refreshed; retain earlier evidence only after
inspecting the corrected delta and recording applicability at its original
revision/date. Passing gates alone do not close F-01 or F-02.


## Iteration01 independent acceptance — 2026-10-11

Reviewed `cc-prompt-iteration01.md` and CC's dated return in planning
`0e8de65408cc79af355bcb090a2163c9b61a25ee`. Source/worktree clean at
`c4b517c89f51d44c0eb1bec4a7b891984d3e0a92`. Source delta is exactly the two
assigned paths: rack.rs replaces two fallback arms with descriptive invariant
panics; tests/lfo.rs adds actual-module supplied-schedule regression evidence.
No engine, configuration, dependencies, scheduler, fixture or runner changes.
CC's readback correctly distinguishes internal variant corruption from numeric
nonfinite CV sanitization. Initial return and both issued prompts are preserved.

CRC freshly ran:

- `cargo test --locked -p oxurack-mod-lfo`: 4 unit + 1 doc pass.
- Same with `--features rack`: 4 unit + 1 doc pass.
- `cargo test --locked -p oxurack --test lfo`: 12 pass.
- Same with `--no-default-features`: 12 pass.
- `cargo test --locked -p oxurack --test midi_output --example turing-machine-midi`:
  2 integration + 11 example tests pass.
- Host `make test`: workspace/doc tests pass; RT 150 pass, 3 existing ignores.
- `make lint`, affected two-package/all-feature/all-target strict Clippy,
  `RUSTDOCFLAGS='-D warnings' make docs`, example build and diff check: pass.
- Two fresh built-example processes, new fixture10000: equal70 rows/hash
  `988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
- Original fixture2000: 13 rows/hash
  `98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.

L-02's four wrong-type tests pass only on their matching diagnostic; both direct
Update/FixedUpdate runs give the literal bipolar/unipolar sequence, typed initial
values, stopped count0, running count4, stable pause, continuation count5. Typed
Float(NaN) remains neutral and finite through both schedules. The earlier CRC
actual-module a965386 probe above establishes the baseline failure; CC's new
four red tests are attested, not represented as another fresh CRC baseline run.
F-01 and F-02 are resolved. No production repair was authored by CRC.

All five rows are accounted for: L-01 engine-only claims and standalone normal
 dependency boundary retain a965386 evidence because their implementation,
 dependencies and inputs are unchanged; engine tests additionally reran now.
L-02 is newly reproduced/done; L-03 parameter/continuation and L-04 loaded
modulation/control/trace checks freshly reproduce; L-05 native gates and strict
 two-file scope freshly reproduce. No deferrals, no-ops or silent drops. CC's
restricted CoreMIDI failures and final host retry stay in the closing report;
CRC independently reproduced the host gate. Existing block0.1.6 advisory and
three RT ignores remain baseline limitations. No endpoint audition or new
release acceptance is claimed. Routine temporary logs/traces are not durable
slice artifacts; supporting-artifact inventory remains none.

## Final bubble-up check

The slice delivers Arc11's installed five-waveform LFO and actual seeded TM
modulation through the existing runner. The correction restores the existing
invariant policy; no new dependency, scope or sequencing change emerged.
Arc-plan needs routine child status/source updates, with its approved capture
ownership/design intact. All five child rows are closed; Arc11's four composition
rows and the project's seven rows stay open. CRC now authors Slice02 bounded
Scope capture at accepted c4b517c under directive09. What worked remains the
literal/control musical oracle and the direct supplied-schedule regression.
