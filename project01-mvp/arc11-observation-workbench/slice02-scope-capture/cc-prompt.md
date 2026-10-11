# CC implementation — bounded Scope capture in the musical rack

Initial pass, 2026-10-11, Sofie/CRC. Three contributors, Expedited Mode.
Execute `arc11-observation-workbench/slice02-scope-capture/cc-prompt.md` and
return to CRC through Duncan. This is implementation work released by directive09
after LFO acceptance, not another planning-only assignment. Deliver a usable CSV
capture through the existing example; Slice03 supplies the terminal view later.

## Baseline, authority and intake

S = `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`.
Branch `codex/oxurack-module-behavior-installation`, accepted/clean predecessor
`c4b517c89f51d44c0eb1bec4a7b891984d3e0a92`.
P = `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp` on
planning branch; LFO closure commit `acb97eb`, plus this issued open-set commit.
K = `/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`.
Resolve installed equivalents on another host. Verify heads/diffs and explicit
current assignment in slice-plan.md; reconcile material drift before coding.

Binding behavior is sibling slice-plan.md, particularly capture/lifecycle,
MIDI ownership and CSV sections, within Arc11's approved later-slice design.
The API/record shapes below resolve local details under CRC's delegated authority.
Private helpers and ordinary test organization are local discretion. Any required
change to phase ownership, drain ownership, source scope or accepted behavior
goes through Duncan before dependent work. No merge/push/release.

Read these completely within their stated bounds, recovering truncated output.
Before dependent edits, record loaded extents/current source and a concise
source-cited contract readback in the existing closing-report.md intake section.
Cover selection/atomicity, capacity unit, Observe latch, MIDI borrow ownership,
failure/export and compatibility. Hashes/headings are not full reading. Once
coherent, proceed under existing authority without another approval.

| Order | Exact paths | Required scope / purpose |
| --- | --- | --- |
| 1 | This prompt; sibling slice-plan.md and ledger.md | Required-full: current assignment, binding local design, all five rows |
| 2 | P/cdc-directive09.md; P/arc11-observation-workbench/arc-plan.md | Required-full: authority, observation/drain constraints and later terminal boundary |
| 3 | P/cdc-directive08.md | Required-section: “Prompt quality for all remaining work”, “Checks and correction policy from Slice03 onward” |
| 4 | S/CLAUDE.md; S/crates/oxurack/Cargo.toml | Required-full: conventions, already available dependencies and feature boundaries |
| 5 | S/crates/oxurack/src/lib.rs | Required-section: imports/re-exports, Oxurack/new, load_patch, start/stop/sample_time/drain_midi/advance_samples/into_app; existing ownership and clocks |
| 6 | S/crates/oxurack-core/src/patch.rs, port.rs, value.rs, lib.rs, tick.rs | Required-section: PatchHandle; Port/CurrentValue/spawn_port_on_module; Value/ValueKind and their impls; CorePlugin/Fixed/build_core_plugin; TickPhase/SampleTime/SimulationRunning and begin/finalize/gate functions. Tests unrelated to reused APIs are reference-only |
| 7 | S/crates/oxurack-mod-midiout/src/lib.rs | Required-section: MidiOutputEvent, MidiOutputBuffer public impl and consumed-event production; event identity and order |
| 8 | S/crates/oxurack/examples/turing-machine-midi.rs; examples/turing_machine_midi/playback.rs (second relative to S/crates/oxurack) | Required-full: actual CLI, both runners, Session send/cleanup and existing fake-sender tests; preserve all failure paths |
| 9 | S/crates/oxurack/tests/lfo.rs; examples/turing_machine_midi/oracle.rs; examples/patches/turing-machine-lfo.ron; examples/patches/turing-machine-midi.ron (last three relative to S/crates/oxurack) | Required-full: loaded fixtures, existing oracle, exact port lookup and temporary-patch pattern; do not change these files |
| 10 | K/rust-guidelines/SKILL.md; K/testing/SKILL.md | Required-full: active domain routes; no new runtime/dependency defaults |
| 11 | K/rust-guidelines/guides/11-anti-patterns.md AP-24/AP-25/AP-33; guides/03-error-handling.md EH-01; guides/04-ownership-borrowing.md OB-02; guides/08-performance.md PF-23; guides/13-documentation.md DC-01 | Required-section: invariant/error separation, exclusive drain borrow, bounded deque and public docs; paths relative to rust-guidelines |
| 12 | K/rust-guidelines/guides/14-cli-tools/02-argument-parsing.md CLI-05; 04-output-and-ux.md CLI-27 | Required-section: existing clap derive and trace/diagnostic stream separation |
| 13 | K/collaboration-framework/knowledge/testing/guides/03-validation-gates.md | Required-full: proportionate native proof, honest failures/ignores |

No predecessor checker/archive or catalog archaeology is required. Conditional:
read any newly encountered directly governing API before using it. This reading
packet is a few relevant source files plus scoped guides, not the full history;
implementation and recovery must fit the same slice context.

## Source reconnaissance and applied guidance

At c4b517c, facade new installs CorePlugin::fixed and owns App; load_patch stores
PatchHandle.modules; actual port lookup uses Port plus ChildOf.parent (as in
tests/lfo.rs). Core Observe inherits a sample-start latch; finalization happens
after Observe, so last_index is the current zero-based index. The offline loop
and playback run_samples each drain after advance_samples(1). drain_midi returns
a Vec Drain borrowing the rack; Session::send_performance reports success only
after sender.send. Session.finish owns cleanup, outside rack performance events.
MidiOutputEvent is Copy, with sample and MidiWire (to_bytes). All required APIs
are verified in local pinned source; no guessed new Bevy/plugin interface.

| Applied source / strength | Decision here | Implementation / proof |
| --- | --- | --- |
| Rust EH-01 MUST; AP-24/AP-25/AP-33 AVOID | User selector/settings errors are typed Results; post-validation missing/wrong typed entities are bugs; scalar nonfinite observations are not silently rewritten | Transactional configure; diagnostic observer invariant; negative config and actual value tests |
| Rust OB-02 MUST | Finish the exclusive drain borrow before calling a facade capture method | Collect the already bounded per-sample MIDI batch, then copy/call sender; actual runner tests |
| Rust PF-23 SHOULD | VecDeque fits oldest-drop/newest-append; stable owned selector table avoids per-record label cloning | One logical record limit, exact overflow window/count |
| Rust DC-01 MUST; SKILL.md public Debug guidance | Document public types/methods, capacity unit, delivery meaning, errors and lifecycle | Strict rustdoc; a usable README/example |
| Rust CLI-05/CLI-27 MUST | Extend existing clap derive; file gets CSV, stdout retains bytes, stderr gets bounds | Parse conflicts and actual binary trace compatibility |

Use existing std and Bevy dependencies. A small typed ScopeError with manual
Display/Error needs no new thiserror dependency in the umbrella crate; this is
a bounded exception to the skill's default error-derive recommendation.
No concurrency primitive, unsafe, worker, CSV crate or performance benchmark.

## Types and API — binding local design

Add `crates/oxurack/src/scope.rs`, private module re-exported by lib.rs.
Representative public shapes below are **illustrative, not compiled drop-in
code**; names/observable behavior and signatures are the assignment contract.
Provide rustdoc and Debug; Clone/PartialEq on selectors and copyable records
where supported (Value is PartialEq, not Eq).

```rust
pub struct ScopeSelector { pub module: String, pub port: String }
pub enum MidiDelivery { Generated, Sent }
pub enum ScopeRecord {
    Signal { sample: u64, selection: usize, value: Value },
    Midi { event: MidiOutputEvent, delivery: MidiDelivery },
}
// Resource; storage and resolved entity bindings private:
pub struct ScopeCapture { /* capacity, overwritten, selectors, bindings, records */ }
impl ScopeCapture {
    pub fn capacity(&self) -> usize;
    pub fn overwritten(&self) -> u64;
    pub fn selectors(&self) -> &[ScopeSelector];
    pub fn records(&self) -> impl Iterator<Item = &ScopeRecord>;
    pub fn write_csv(&self, writer: &mut impl std::io::Write) -> std::io::Result<()>;
}
impl Oxurack {
    pub fn configure_scope(&mut self, selectors: &[ScopeSelector], capacity: usize)
        -> Result<&mut Self, ScopeError>;
    pub fn scope(&self) -> Option<&ScopeCapture>;
    pub fn clear_scope(&mut self) -> &mut Self;
    pub fn disable_scope(&mut self) -> &mut Self;
    pub fn record_scope_midi(&mut self, event: MidiOutputEvent, delivery: MidiDelivery);
}
```

The public error variants and payloads are binding (derive Debug; manual
Display/std::error::Error; Allocation exposes its source). Use non-exhaustive
public enums for error/record/delivery surfaces so callers allow future variants:

```rust
pub enum ScopeError {
    Running,
    ZeroCapacity,
    NoPatch,
    DuplicateSelector { module: String, port: String },
    UnknownModule { module: String },
    UnknownPort { module: String, port: String },
    UnsupportedKind { module: String, port: String, kind: ValueKind },
    InvalidPortState { module: String, port: String },
    Allocation(std::collections::TryReserveError),
}
```

InvalidPortState covers missing/wrong typed CurrentValue at configuration; later
corruption is an invariant bug. No CoreError changes. Disabled clear/disable/record
are documented no-ops. Empty selectors allow MIDI-only capture; zero capacity
is rejected. Allocation errors occur before replacing the old capture.

## Ordered implementation recipe

1. **scope.rs: records, queue, validation, CSV.** Build a candidate capture locally
   from the borrowed selector list and actual World. Copy module Entity from
   PatchHandle.modules before querying child ports (end the resource borrow).
   Resolve exact child/name, declared kind and CurrentValue kind. Only allow
   Float/Bipolar/Gate/Raw. Check duplicates in caller order. Store owned labels
   once and copy entity/kind into bindings. Only after all checks insert the
   resource. An invalid candidate must not clear old records or counters.
   Private push pops_front and increments overwritten when len==capacity before
   push_back. Count every overwritten record, including MIDI; no hidden reserve
   outside the logical bound (record length, not allocator bytes). Use VecDeque::new
   then try_reserve_exact(capacity), mapping failure to ScopeError::Allocation
   before resource insertion. No unchecked allocation from the user capacity and
   no hard RT latency guarantee. Simple checked counter increment is sufficient.
2. **lib.rs: facade ownership and one observer.** Re-export types; add Scope
   methods delegating into the private module. In new, add observe_scope once
   to FixedUpdate/TickPhase::Observe. Use an optional Resource so capture-disabled
   racks remain inert; configuring replaces only that resource. After successful
   load_patch, remove old capture to invalidate entity bindings; on failure,
   preserve existing capture. No changes to advancement, MIDI admission or clocks.
3. **scope.rs: real Observe system.** Read ports via Query<&CurrentValue> and
   Res<SampleTime>, copy each value in selector order; never read/drain MIDI.
   Do not add a raw SimulationRunning check: a stop during Consume still completes
   this latched sample's Observe. Borrow fields in a short scope before mutating
   the queue. Illustrative shape:

```rust
fn observe_scope(
    capture: Option<ResMut<ScopeCapture>>,
    values: Query<&CurrentValue>,
    time: Res<SampleTime>,
) {
    let Some(mut capture) = capture else { return; };
    let sample = time.last_index().expect("Observe requires a begun sample");
    for selection in 0..capture.bindings.len() {
        let (entity, kind) = {
            let binding = &capture.bindings[selection];
            (binding.entity, binding.kind)
        };
        let value = values.get(entity).expect("Scope selected port disappeared").0;
        assert_eq!(value.kind(), kind, "Scope selected port changed value type");
        capture.push(ScopeRecord::Signal { sample, selection, value });
    }
}
```

   CorePlugin supplies set ordering/gating; no copied scheduler. All referenced
   Bevy/core types are available at the baseline. The sketch itself must compile
   in CC's implementation with the actual imports/resource/private fields.
4. **Existing offline/live runners: copy sole-owner drain.** End its borrow
   before calling record_scope_midi. A private playback helper may centralize
   draining plus Generated copies for run_trace, run_samples and terminal discard.
   With capture off these API calls are no-ops. Collect only the current existing
   bounded output batch, not all session events. Illustrative borrow/flow:

```rust
let events: Vec<_> = rack.drain_midi().collect();
for &event in &events {
    rack.record_scope_midi(event, MidiDelivery::Generated);
}
for event in events {
    if cancellation.load(Ordering::SeqCst) { break; }
    session.send_performance(event)?; // retain existing ownership/cleanup logic
    rack.record_scope_midi(event, MidiDelivery::Sent);
    // Existing stdout trace write and cancellation checks stay in their order.
}
```

   Offline drains copy Generated but never Sent. Retain original event indices
   and buffer order. A full drained batch may have Generated-only suffixes when
   cancelled/send fails; document this. Finish's rack-buffer discard also uses
   the Generated copy helper. Session.finish cleanup sends remain excluded.
   No change to drain_midi's public signature/automatic side effects.
5. **CLI and export integration.** Add scope_csv Option<PathBuf>, scope_port
   Vec<ScopeSelector> and scope_capacity Option<usize>, using clap derive.
   A custom positive usize parser avoids assuming a version-specific range API.
   Custom selector parser splits at last dot and rejects empty parts. Explicit
   scope_port/scope_capacity require scope_csv; all scope flags conflict with
   list_outputs. Scope CSV alone is valid MIDI-only capture. When enabled, use
   capacity.unwrap_or(100000), configure after patch load and before start or
   native connection, then open the chosen file before playback. Use BufWriter.
   After playback/stop/cleanup, export and flush even on an already returned
   playback failure. Preserve the primary failure; if CSV also fails, include
   both in the returned diagnostic with the primary first. If only CSV fails,
   return nonzero. Do not turn a failed playback into export success. File-open
   failure prevents playback. End summary to stderr gives capacity, retained,
   overwritten. No capture I/O during Observe or per-sample sends.
6. **CSV implementation and README.** Exact header/schema are in slice-plan.
   Escape any textual field containing comma/quote/CR/LF by quoting it and
   doubling quotes. Export immutable retained insertion order; header even when
   empty. Caller owns flush and its error; write_csv propagates writer failures.
   MIDI bytes use uppercase02X; scalar Display, Gate true/false. Document record
   capacity/partial windows, label meaning, clear/reload and optional live use.
   Do not add the terminal view early.

## Real behavioral checks and expectations

Use integration test `crates/oxurack/tests/scope.rs` for public facade behavior;
inline scope.rs tests can cover private deque/CSV and direct App lifecycle where
needed. Extend existing example/playback tests inside their current files.
No separate replay/checker framework or target test count.

| Row | Setup and actual operation | Expected / plausible defect rejected |
| --- | --- | --- |
| S-01 | Load a temporary patch using real Clock1000BPM/PPQN30/width1 and LFO saw_up62.5Hz/offset0/width.5. Select clock.pulse, LFO out/bipolar_out and propagated tm.write_modulation with the existing Affine(.5,0) cable. Run four samples, draining normally | At indices0..3, Clock Gate true,false,true,false; LFO bipolar -1,-.875,-.75,-.625 and Float0,.0625,.125,.1875; TM bipolar input -.5,-.4375,-.375,-.3125. Selector order within each sample, same Observe index, no extra delay. Values derived from formula/cable, not by calling Scope itself |
| S-01 | A tiny test module publishes Raw(17) in Produce through actual loaded registry; select its Raw port | Scope copies Raw(17), preserving kind/value; no coercion. Include this in an existing short run; no new production module |
| S-01/S-02 | Configure, convert into_app, add a Consume system that clears SimulationRunning during sample0; run actual FixedUpdate then again | First latched sample has observations at0 and completes1; second stopped run adds none. No second running check may drop sample0 |
| S-02 | One selected saw_up port, capacity3, five running samples | Only indices2,3,4 remain, bipolar -.75,-.625,-.5; overwritten2. Stop/advance adds nothing; clear resets records/count only; resumed absolute sample5 gives -.375 |
| S-02 | Mixed queue capacity3: captured signal0, copy generated event0, copy sent event0, then signal1 | Three retained records are generated0/sent0/signal1, overwritten1; FIFO is across kinds, not separate hidden queues |
| S-02 | Valid reconfigure, disable, successful reload, failed reload | Reconfigure empties queue and has only one observer; disable yields None and does not affect module/time; successful reload yields None until explicit reconfigure; failed reload preserves capture. Do not claim module rollback beyond existing load_patch contract |
| S-03 | Configure a valid capture with records, then missing module/port, duplicate selector, Midi selector, zero/unrepresentable usize::MAX capacity, running and no-patch cases | Typed errors before playback; prior selector/records/count unchanged on rejected replacement. For no-patch use fresh rack; wrong/missing CurrentValue preflight may use inline access. Run default and no-default to reject graph dependence |
| S-04 | Two fresh loaded accepted LFO fixtures, capture off/on with enough capacity; runner drains once/sample and copies | Exact generated sample/bytes equal, nonempty, same clocks. Captured Generated events equal drained output in order. No Sent offline. A held Midi selector is rejected rather than emitting phantom messages |
| S-04 | Extend actual playback::play with its existing fake sender and independent seeded oracle; compare capture off/on and force a later send failure/cancellation | Successful performance sends and stdout agree with baseline; Generated full drained batch captured, Sent only the successful prefix. Failure never marks failed/suffix events Sent. Cleanup still runs and is excluded from captured performance. Test via actual runner, not a direct record-only mock |
| S-05 | CSV from short literal capture, escaped selector labels (comma/quote/newline) and a failing Write; empty capture | Exact nine-column header, typed scalar/hex MIDI fields and FIFO; correct escaping, io::Error propagated, empty header only. Inline tests may construct records to isolate writer behavior |
| S-05 | CLI parse and built binary, no flags versus capture; malformed selector/zero/orphan flags/list conflict/bad file path | Invalids nonzero before playback; capture-enabled stdout equals disabled; separate CSV is populated and stderr bounds honest. No hardware endpoint necessary |

For the short public Scope test, existing tests/lfo.rs provides the temporary RON
serialization and real installed Clock/LFO/TM/MIDI helper pattern. Its helpers
are private to that test target: reproduce a small setup in scope.rs integration
tests, do not invent a public world accessor or edit the protected LFO tests.
For Consume-stop and corrupted preflight, inline tests can access crate internals
or consume into_app after configuration; do not reuse the consumed facade.
Use actual SampleTime/ScopeCapture resources in that App. The existing
install_rack/fast_rack helpers call start: stop their returned rack before
configure_scope, then start after successful configuration. This is still
before their first advanced sample, so it preserves the intended fresh oracle.
Do not relax the running-configuration rejection to make a helper fit.

Run from S:

```bash
cargo test --locked -p oxurack --test scope
cargo test --locked -p oxurack --no-default-features --test scope
cargo test --locked -p oxurack --lib --example turing-machine-midi
cargo test --locked -p oxurack --no-default-features --lib --example turing-machine-midi
cargo test --locked -p oxurack --test lfo --test midi_output
make test
make lint
cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' make docs
cargo build --locked -p oxurack --example turing-machine-midi
git diff --check
```

Use existing built binary twice for new fixture10000; original2000 once. Expected
new70-row hash `988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`;
original13-row hash `98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.
Run the documented capture command and compare stdout with the same disabled
command. For this four-selector10000 demo, capacity100000 retains40000 signal
rows plus70 Generated events, overwritten0; CSV has40071 lines including header.
Actual source sample convention remains0-based; no assumed platform-independent
sine hash beyond this pinned build.

```bash
./target/debug/examples/turing-machine-midi \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron --samples 10000 \
  --scope-csv /tmp/oxurack-scope.csv --scope-capacity 100000 \
  --scope-port clock_main.pulse --scope-port lfo_main.bipolar_out \
  --scope-port tm_main.write_modulation --scope-port tm_main.note \
  > /tmp/oxurack-scope-midi.trace
```

Document optional live command by adding the same accepted output-name/index
flag; automated checks do not choose/open an endpoint. Known restricted CoreMIDI
failures get an unchanged host retry, both recorded. Retain three RT ignores and
block0.1.6 advisory. No new ignore/suppression, coverage/freshness/benchmark or
planning-checker campaign. Record actual commands/results, including unrun or
blocked checks; passing native gates do not replace behavioral evidence.

## Authorized files, commits and return

Source scope is exactly these six paths:

- `crates/oxurack/src/scope.rs` (new)
- `crates/oxurack/src/lib.rs`
- `crates/oxurack/tests/scope.rs` (new)
- `crates/oxurack/examples/turing-machine-midi.rs`
- `crates/oxurack/examples/turing_machine_midi/playback.rs`
- `README.md`

Core/RT/MIDI module algorithms, Clock/LFO/TM, original/new fixtures, existing
LFO/oracle files, Cargo manifests/lock and terminal interface are protected.
If a real dependency requires another path or a binding contract change, stop
that portion and report the specific need through Duncan; no inferred expansion.

Commit source with explicit paths (stage only the two new paths as needed):

```bash
git add -- crates/oxurack/src/scope.rs crates/oxurack/tests/scope.rs
git commit --only -m 'feat(scope): capture bounded signals and MIDI with CSV export' -- \
  crates/oxurack/src/scope.rs crates/oxurack/src/lib.rs \
  crates/oxurack/tests/scope.rs crates/oxurack/examples/turing-machine-midi.rs \
  crates/oxurack/examples/turing_machine_midi/playback.rs README.md
```

No broad staging/-a. Inspect actual committed path list and both tree states.
Separately commit planning return with exactly:

```bash
git commit --only -m 'docs(plan): return Scope capture implementation evidence' -- \
  project01-mvp/arc11-observation-workbench/slice02-scope-capture/closing-report.md \
  project01-mvp/arc11-observation-workbench/slice02-scope-capture/ledger.md
```

Stage the new closing-report.md explicitly first. Report exact source/planning
hashes through Duncan; no self-referential metadata follow-up required. Preserve
issued prompt and plan; CRC owns assignment changes, acceptance and advancement.
Leave all S rows open with CC proposed-done/attested evidence. The report includes
assignment/hash, intake/readback, full five-row walk, changes/commands/attempts,
self-review, source/planning path inventories, scope-as-delivered versus arc
assignment and any actual dependency/design findings. Supporting artifacts:
none for routine passing results; concrete durable supporting output, if needed,
belongs in this slice's artifacts/ and must be named explicitly in its commit.
Do not create an extra tool/reporting layer. Arc11/project remain open; Slice03
waits for CRC's independent acceptance of this runnable capture increment.
