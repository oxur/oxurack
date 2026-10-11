# CC implementation — terminal Scope workbench

Initial pass, 2026-10-11, Sofie/CRC. Three contributors, Expedited Mode.
Execute `arc11-observation-workbench/slice03-workbench-integration/cc-prompt.md`
and return implementation/evidence to CRC through Duncan. Deliver the third
Arc11 increment: inspect selected signals and generated/sent MIDI in the terminal,
with capture bounds and repeat/export instructions. This is product work.

## Baseline, authority and complete intake

S = `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch `codex/oxurack-module-behavior-installation`, clean accepted Scope source
`6c56e8460b693c285b8052633165e66129318604`.
P = `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp` on planning;
Scope acceptance commit `819cecd`, plus this issued open set. K =
`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`.
Resolve equivalent installed roots on another host. Verify heads/diffs and the
assignment in sibling slice-plan.md; reconcile material drift before coding.

Binding: sibling plan and terminal/CLI/failure semantics below detail Arc11's
approved third increment under directive09. Local discretion: private helper
names, test organization and mechanics that preserve these observable results.
Changes to ownership, scope, public API or accepted capture/runner behavior need
CRC/CDC resolution through Duncan before affected work. No merge/push/release.

Read required extents completely, recovering truncation. Record paths/state,
loaded extents and conditional applicability in this slice's closing-report.md
before dependent edits; give a concise source-cited contract readback covering
retained-window semantics, streams, enabling/preflight, failure ordering and
fresh-run versus clear. Receipts/greps/hashes are not comprehension. Proceed on
coherent preflight under existing authority; reload lost contracts on recovery.

| Order | Paths | Required extent and purpose |
| --- | --- | --- |
| 1 | This prompt, sibling slice-plan.md and ledger.md | Required-full, current four-row assignment |
| 2 | P/cdc-directive09.md; P/arc11-observation-workbench/arc-plan.md | Required-full, terminal boundary/ownership and CDC composition gate |
| 3 | P/cdc-directive08.md | Required-section: “Prompt quality for all remaining work” and “Checks and correction policy from Slice03 onward” |
| 4 | S/CLAUDE.md; S/crates/oxurack/Cargo.toml | Required-full, conventions/dependencies; historical architecture links reference-only here |
| 5 | S/crates/oxurack/examples/turing-machine-midi.rs | Required-full, actual CLI/prepare_scope/finish_scope/runners/tests to modify |
| 6 | S/crates/oxurack/src/scope.rs | Required-section: ScopeSelector/MidiDelivery/ScopeRecord/ScopeCapture and its public impl; immutable view API and CSV meaning. Configure/observer tests reference-only, protected |
| 7 | S/crates/oxurack/src/lib.rs; crates/oxurack-core/src/tick.rs (both under S) | Required-section: scope methods, start/stop/sample_time/drain_midi/advance_samples/into_app; SampleTime definition/impl. Lifetime and logical sample convention |
| 8 | S/crates/oxurack/examples/turing_machine_midi/playback.rs | Required-section: MidiSender, Session definition/new/send_performance/finish, PlaybackFailure, play/drain_generated/run_samples; tests fast_rack, Recorder, template_event/event, and three capture tests. Actual send/cleanup path to exercise, not edit |
| 9 | S/crates/oxurack/examples/turing_machine_midi/oracle.rs; examples/patches/turing-machine-lfo.ron; examples/patches/turing-machine-midi.ron (last two under S/crates/oxurack) | Required-full, independent seeded MIDI oracle and protected fixtures |
| 10 | S/crates/oxurack/tests/scope.rs | Required-section: patch/rack/run helpers and capacity_clear_resume_and_mixed_midi_fifo_are_bounded; reuse literal/lifecycle pattern without editing it |
| 11 | S/README.md | Required-section: musical MIDI example and Scope capture commands; integrate usable recipes |
| 12 | K/rust-guidelines/SKILL.md; K/testing/SKILL.md | Required-full, separately active skills; preserve project tooling/dependencies |
| 13 | K/rust-guidelines/guides/11-anti-patterns.md AP-33; guides/03-error-handling.md EH-01; guides/14-cli-tools/02-argument-parsing.md CLI-05; 04-output-and-ux.md CLI-21/CLI-27 (all relative to rust-guidelines) | Required-section: fallible reporting, clap and human/machine stream discipline |
| 14 | K/collaboration-framework/knowledge/testing/guides/03-validation-gates.md | Required-full, native proportionate proof and honest gate limits |

Conditional: read any directly governing API not listed before using it. Old
full prompt/checker/archive packets are reference-only. No structured dataset
query manifest applies. This bounded packet targets two example modules plus
exact API/guide sections, leaving implementation/recovery headroom.

## Inspected source and applied Rust guidance

At6c56e84, CLI prepare_scope enables only csv and validates/configures before
File::create; run_trace/run_live invoke it before start/native connection.
finish_scope exports after stop and preserves primary errors. ScopeCapture
exposes capacity/overwritten/selectors/records/write_csv; records returns only
`impl Iterator`, so no assumed ExactSize/DoubleEnded API. SampleTime has
completed_samples/last_index. playback::play stops, captures terminal drain,
cleans up Session and flushes trace; private renderer must read afterward.
Session and MidiSender are pub(crate), usable from the parent example tests.
Existing oracle/fast-rack tests supply genuine module/runner coverage. No new
library accessor, dependency, public API or capture-system change is needed.

| Read guide / strength | Decision and reason | Concrete proof |
| --- | --- | --- |
| CLI-05 MUST | Extend current clap derive and named ArgGroup; reject orphan flags at parse time | Enabled combinations/invalid flags through Cli::try_parse_from and binary |
| CLI-27 MUST | Keep stdout MIDI data byte-identical; optional inspection/status to stderr, CSV to selected file | Same fixture on/off comparisons and separate captured streams |
| CLI-21 SHOULD | Plain human view alongside existing machine CSV; explicit opt-in, no automatic TTY formatting | Deterministic Write output and identical CSV with/without view |
| EH-01 MUST; AP-33 AVOID | Writer errors are recoverable; attempt every requested report and keep all failures, primary first | Inject write/flush failures and primary failure together |

No logging/TUI/CSV/async crate imports from skill defaults. Plain text after stop
fits the approved small workbench and avoids overhead during samples. Existing
Box<dyn Error> application convention stays; no new library error variants.

## Terminal semantics and shape — binding

Add private `examples/turing_machine_midi/workbench.rs`, included with #[path]
from the example. Recommended fallible rendering entry point (illustrative,
not compiled drop-in; signature/behavior binding):

```rust
pub(super) fn write_view(
    writer: &mut impl std::io::Write,
    patch: &std::path::Path,
    time: oxurack::SampleTime,
    capture: &oxurack::ScopeCapture,
    rows: usize,
) -> std::io::Result<()>;
```

rows is already validated1..=1000. Render after stop/cleanup, with no changes to
capture/time, no drain, advancement, native endpoint access or per-sample I/O.
Two tails contain at most rows **each**, separate from latest-per-selector lines.
All displayed counts/latest/tails refer to retained records only; overflow can
leave partial samples/unpaired generated/sent records. Never infer lost history
or endpoint/audio acknowledgement. Compute sample span as minimum/maximum sample
among retained signal/MIDI records; insertion order is preserved in both tails.
Completed samples comes from time.completed_samples, not max retained sample+1.

Use this exact line grammar (example values illustrate a synthetic capture,
not a pinned real fixture result); headings and key names are binding:

```text
Scope workbench
patch="fixture.ron"
completed_samples=4 capacity=20 retained=6 overwritten=0
retained_sample_span=0..=3
retained_counts signal=4 generated=1 sent=1
latest signals (selector order; retained window):
module="lfo_main" port="bipolar_out" sample=3 kind=bipolar value=-0.625
recent signals (last 2 retained; insertion order):
2 module="lfo_main" port="bipolar_out" kind=bipolar value=-0.75
3 module="lfo_main" port="bipolar_out" kind=bipolar value=-0.625
recent MIDI (last 2 retained; insertion order):
1 generated 90 3C 5A
1 sent 90 3C 5A
```

Heading N uses requested rows, even if fewer exist. For an empty capture span is
`none`; counts are0; an unobserved selector line is
`module="..." port="..." sample=none value=unobserved`. No selectors prints
`(none selected)` after latest heading. Empty tails each print `(none retained)`.
Path and module/port labels use Rust Debug escaping (quoted strings, escaped
controls) so labels cannot inject lines/terminal controls. Display each Value
variant's scalar via Rust Display: float/bipolar/gate/raw, true/false for Gate;
legitimate NaN/infinity are displayed, not rewritten. MIDI bytes use uppercase
02X. Current enums are non-exhaustive outside the library: unsupported future
record/delivery/value variants return a diagnostic io::Error from rendering;
never silently mislabel them. Current Scope never captures Midi as a Signal.
Renderer does not own flush; finalization flushes the selected writer.

## CLI and output lifecycle — binding

Add `--scope-view` bool and `--scope-rows N` Option<usize> with a parser accepting
only1..=1000; default12 when view enabled. rows requires scope_view, not csv.
Capture is enabled by scope_csv OR scope_view. Keep repeated scope_port exact
last-dot parser and capacity positive/default100000; change their requires to
that output group. All Scope flags conflict with list_outputs. CSV+view allowed;
view-only no file is opened; no selectors still means MIDI-only capture.
No flags preserves current disabled behavior; existing CSV-only commands stay
valid and retain current summary/schema. No new output endpoint is implied.
Recommended clap wiring, illustrative (use current field identifiers):

```rust
#[command(group(ArgGroup::new("scope-output")
    .args(["scope_csv", "scope_view"]).multiple(true)))]
// scope_port / scope_capacity: requires = "scope-output"
// scope_view: conflicts_with = "list_outputs"
// scope_rows: requires = "scope_view", conflicts_with = "list_outputs"
```

prepare_scope must configure on either output while stopped, opening requested
CSV before samples/native connection. Keep existing load/selection/error order.
Do not weaken selector/capacity/running checks to fit a helper. Both run_trace and
run_live call a common post-stop finalizer. Playback failure does not skip view;
CSV failure does not skip view and view failure does not suppress CSV. Attempt
write and flush for each requested output, collecting both where applicable.
Return nonzero for any reporting failure; diagnostics order primary, CSV write/
flush, view write/flush. Disabled outputs introduce no failures. Existing Scope
summary may remain; do not write the view before live Session cleanup completes.

Recommended generic finalizer allows real failure injection without a special
filesystem device. Keep it private; adapt ordinary helper names locally:

```rust
fn finalize_scope<C: Write, V: Write>(
    cli: &Cli, rack: &Oxurack, csv: Option<C>, view: &mut V,
    primary: Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    // If csv: attempt capture.write_csv + flush, record each error.
    // If view: call workbench::write_view + flush, record each error.
    // Collect first; do not use ? to skip another requested report.
    // If primary Err, preserve it first; append report errors in specified order.
    // If no errors return Ok. Disabled configuration returns primary unchanged.
}
```

Actual wrapper supplies BufWriter<File> and locked stderr. Tests supply Vec<u8>
or injected failing Write; no publicly exposed writer hook. Keep stdout trace
and playback module unchanged. A shared formatting helper may map scalar kind/
value within workbench; avoid moving accepted CSV code or changing library API.

## Implementation sequence and viable tests

1. New workbench module: one retained scan computes counts/span and selector
   latest values; forward tail passes use known count, not reverse iteration or
   whole capture cloning. Extra memory O(selectors), bounded display rows; time
   O(retained records+selectors), after play. A feasible tail idiom:

```rust
let skip = signal_count.saturating_sub(rows);
for record in capture.records()
    .filter(|r| matches!(r, ScopeRecord::Signal { .. })).skip(skip) {
    // Fallible formatted write; immutable borrow, copied scalar only.
}
```

2. Existing CLI: add opt-in/group/parsers; configure OR output, retain file
   preflight, replace finalization with requested CSV/view attempts. Wire both
   runners after stop and actual live cleanup, keep old trace paths unchanged.
3. Inline tests in new module/parent example; no private library constructor
   or public world accessor needed. prepare_rack loads fixtures but stays stopped.
   For exact signals use a temporary accepted LFO RON modified to saw_up62.5Hz,
   offset0, select bipolar_out, configure/start/advance/drain/stop. Samples0..3
   give -1,-.875,-.75,-.625. For synthetic event display, obtain an actual drained
   event, copy it and edit its public sample/status/data1/data2 (as playback tests
   already do), then record Generated/Sent explicitly with the public facade.
   No dependency on a MidiWire re-export (umbrella has none). Snapshot records,
   counters and sample_time before render to assert immutable after.
4. README: commands below plus field/overflow/retained-window explanation, scalar
   versus MIDI event distinction, software-send meaning and reset/new-run recipe.
   No new standalone runbook/evidence tool necessary.

| Row | Real operation and oracle | Rejects / limits |
| --- | --- | --- |
| B-01 | Loaded saw_up4 samples, tail2: latest sample3=-.625; tail2/3=-.75/-.625. Exact labels/kinds, caller selector order, span0..=3/count4. Add known copied90 3C 5A Generated/Sent sample1, assert exact MIDI lines and counts | Misindexed data, unordered labels, held-MIDI invention; synthetic display events isolate formatting, not proof of sends |
| B-01 | Capacity3 signal-only5 samples keeps2/3/4, overwritten2; latest4=-.5, tail1 sample4; separately mixed record FIFO and MIDI-only/zero-sample/escaped labels | Invented evicted history, independently sized capture queues, multiline/control injection, empty crash |
| B-02 | Parse view-only, csv-only, combined, repeat selectors; orphan port/capacity/rows, rows0/1001, list conflicts. Built unknown selector/bad CSV path fail before stdout/play | Unusable combinations or late validation; do not open endpoint to test usage |
| B-03 | Generic finalizer with Vec/failing Write: primary+CSV write/flush+view write/flush failures retains primary first and attempts both; CSV success survives view error and view success survives CSV error; primary-only failure still exports/displays | Short-circuit reporting/silent errors/success masking; injected writers adequate without /dev/full |
| B-03 | Actual playback::play on accelerated original fixture200 with fake MidiSender; fail second performance send, then render/export. Generated includes attempted event; Sent only successful prefix; cleanup attempts precede finalizer and excluded from capture. Cancellation after first send returns stopped partial view, no further samples/drain | Formatting mock alone cannot prove runner integration. Reuse oracle.expected_trace and fast_rack setup; helpers are private to playback tests, reproduce small setup in parent tests, don't edit playback |
| B-03 | Render twice before/after clear_scope (no advancement), then resume and render; accepted scope lifecycle test plus snapshots | Rendering/reset confusion. clear removes records/overwrites only, resumed absolute time/phase continues; fresh process resets seeded run |
| B-04 | Actual README view/csv/no-view commands and pinned hashes; fresh process repeat output; default/no-default and native gates | Product runnable with actual patch; no hardware timing/audio or arc acceptance inferred |

For actual runner test, implement local MidiSender over shared recorded bytes
and optional cancellation/fail index; Session::new/play accessible from parent.
Stop helpers that already start before configure_scope, then start again. Finalizer
receives only stopped rack after play returns; snapshot before and after render.
Use successful prefix bytes from independent original oracle, not a second view
call. Protected production playback/Scope tests remain existing regression proof.
Do not change their test visibility or library API merely to reuse private helpers.

## Native gates and runnable commands

From S, record exits and denominators, failed/blocked/ignored attempts separately:

```bash
cargo test --locked -p oxurack --example turing-machine-midi
cargo test --locked -p oxurack --no-default-features --example turing-machine-midi
cargo test --locked -p oxurack --test scope --test lfo --test midi_output
cargo test --locked -p oxurack --no-default-features --test scope
make test
make lint
cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' make docs
cargo build --locked -p oxurack --example turing-machine-midi
git diff --check
```

Keep restricted CoreMIDI failures with unchanged host retry; three existing RT
ignores and block0.1.6 advisory disclosed. No new ignore/suppression. No make
check/freshness/coverage/benchmark/checker campaign. No endpoint selected/opened.

Document/use built command (stdout remains MIDI trace; stderr shows view):

```bash
./target/debug/examples/turing-machine-midi \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --samples 10000 \
  --scope-view --scope-rows 12 --scope-capacity 100000 \
  --scope-port clock_main.pulse --scope-port lfo_main.bipolar_out \
  --scope-port tm_main.write_modulation --scope-port tm_main.note \
  > /tmp/oxurack-workbench.trace
```

Expect completed10000, retained40070, overwritten0, signals40000/generated70/
sent0. Show latest named ports and recent sample-indexed signals/MIDI. Add
`--scope-csv /tmp/oxurack-workbench.csv` for40071 CSV lines, same stdout and same
view body. Compare CSV with accepted Scope-only command byte-for-byte. Repeat
fresh processes with same patch/path/options: both trace and view equal. New
fixture70-row stdout SHA256 stays
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`;
original fixture2000 retains13 rows/hash
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.

Also demonstrate capacity3 with only lfo_main.bipolar_out:10000 samples retains3,
overwritten10067 (10000 signals+70Generated), signal tail9997/9998/9999, generated
retained0, completed10000; stdout still70 rows. Zero samples gives empty view
with unobserved selected ports/none tails and header-only CSV. Optional live
recipe adds an exact currently selected output name; Duncan may audition but
no new witness is mandatory. View emits after finite completion or Ctrl-C cleanup,
not continuously. Fresh-process rerun resets seed/phase/time; stop/start and
clear_scope do not. Show a small API usage snippet with clear_scope on an
already configured stopped rack and explicit continuation; no reset command.

## Authorized paths, commits and return

Exactly three source paths:

- `crates/oxurack/examples/turing-machine-midi.rs`
- `crates/oxurack/examples/turing_machine_midi/workbench.rs` (new, includes tests)
- `README.md`

Scope/facade/core/module algorithms, playback/oracle, prior tests, fixtures,
Cargo manifests/lock are protected. If a real requirement cannot fit these
paths/contracts, report the concrete issue to CRC through Duncan before affected
changes. No inferred expansion. Stage only new module, commit explicit paths:

```bash
git add -- crates/oxurack/examples/turing_machine_midi/workbench.rs
git commit --only -m 'feat(example): add terminal Scope workbench inspection' -- \
  crates/oxurack/examples/turing-machine-midi.rs \
  crates/oxurack/examples/turing_machine_midi/workbench.rs README.md
```

Separate planning commit exactly sibling closing-report.md and ledger.md:

```bash
git add -- project01-mvp/arc11-observation-workbench/slice03-workbench-integration/closing-report.md
git commit --only -m 'docs(plan): return terminal workbench evidence' -- \
  project01-mvp/arc11-observation-workbench/slice03-workbench-integration/closing-report.md \
  project01-mvp/arc11-observation-workbench/slice03-workbench-integration/ledger.md
```

No broad staging. Record actual path inventories, exact source/planning hashes,
assignment/intake/readback, four-row walk, behavior/native results, self-review,
limits, scope-as-specified/delivered and Bubble-up to the arc. Supporting artifacts
none expected; necessary durable outputs go in sibling artifacts/ with named
commit paths, not another evidence layer. Leave four B rows open with CC proposed-
done/attested evidence; CRC owns independent acceptance, plan changes and arc
assembly. No self-referential metadata commit required. Preserve issued prompt.
