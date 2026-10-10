# CC implementation — installed MIDI-out and runnable TM trace

## Assignment and authority

Implement project01-mvp / arc10-turing-machine-midi / slice03-midi-output,
initial pass. Prompt: `arc10-turing-machine-midi/slice03-midi-output/cc-prompt.md`.
Three contributors, Expedited Mode; return to Sofie/CRC through Duncan.
Source `S=/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch `codex/oxurack-module-behavior-installation`, accepted clean predecessor
`b2ffb16d342d9a6cc800bab2713813d9646fb505`. Planning
`P=/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp`,
predecessor `95bf441`; record actual issue HEAD. Preserve unrelated dirt; resolve
unexpected source drift before dependent edits rather than resetting it.

**Binding authority:** directive08 and Arc10 design v1.2 sections 1–3/5; this
prompt details those decisions. Slice02 is accepted. Deliver the real installed
sink, bounded facade drain and a headless example that loads seeded RON and
prints MIDI bytes. Production changes in the named components are authorized.
Endpoint I/O/pacing/cleanup is Slice04. No async service, new driver/clock,
RT rewrite, signal kind, persistence, UI or validation framework here.

## Required reading and intake

Installed roots: `F=/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills/collaboration-framework`;
`R=/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills/rust-guidelines`.
If paths differ on CC's host, resolve the installed equivalents and record
versions/paths before using them. Author applied framework 2.6.0/Rust 2.0.0.

| Order | Exact input at issue / accepted source | Required scope / purpose |
| --- | --- | --- |
| 1 | This prompt, `slice-plan.md`, `ledger.md` | Full; assignment, five rows, authorized behavior and boundaries. |
| 2 | `P/cdc-directive08.md`; `P/arc10-turing-machine-midi/design.md` | Directive full; design §§1–3/5 full, §4 reference-only future playback. |
| 3 | `P/project-plan.md` Current direction v3.17 including current handoff history; Arc10 `arc-plan.md` Capability, Slice sequence, Composition and Delivery sections | Named sections only; future policy and successor authority. Historical appended plans/directives/packets/logs reference-only. |
| 4 | `S/crates/oxurack/src/lib.rs` beginning through `impl Default`; `S/crates/oxurack-core/src/lib.rs::build_core_plugin`; `tick.rs` TickPhase through sample_is_running; `error.rs` CoreError/TickError definitions | Sections with definitions; real install/load/manual/Live/stop ownership, pre-Produce latch and typed errors. |
| 5 | `core/src/module.rs` spawn helpers, schema, OxurackModule trait; `port.rs::spawn_port_on_module` and CurrentValue definitions; `patch.rs::validate_patch` and `apply_patch_to_world`; `parameter.rs` registry and ParameterValue definitions | Full named symbols and definitions; wiring/static config and existing partial-load behavior. Prefix `core` means `S/crates/oxurack-core`. |
| 6 | `S/crates/oxurack-mod-turingmachine/src/rack.rs` schema, spawn/register, produce/consume/publication functions; Clock rack schema/register; `S/crates/oxurack-midi/src/wire.rs` MidiWire and constructors/to_bytes; umbrella `tests/turing_machine.rs` fixture/new_rack/run_patch/configured_engine/standalone-match functions plus fixture RON | Named sections and necessary types; actual published ports and reusable real test patterns. Other predecessor matrices reference-only. |
| 7 | `S/Cargo.toml`, `S/crates/oxurack/Cargo.toml`, Clock module Cargo.toml, `S/CLAUDE.md`; `S/Makefile` test/lint/docs and check dependency definitions | Manifests/CLAUDE full, Make sections only; dependency/example and instruction integration. Query Cargo.lock for resolved Bevy/clap packages after edits; do not read whole lockfile as prose. |
| 8 | `R/SKILL.md`, `R/guides/11-anti-patterns.md`; `R/guides/02-api-design.md` API-23/API-39, `03-error-handling.md` EH-01, `05-type-design.md` TD-05, `13-documentation.md` DC-05; `R/guides/14-cli-tools/README.md` and `02-argument-parsing.md` CLI-05 | Entrypoint/anti-patterns and CLI README full; other complete named sections and necessary definitions. Unprefixed chapter paths are relative to their immediately named guide directory. Apply these decisions, not a default async runtime. |
| 9 | `F/knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md` Required reading and CC intake and §5; `F/knowledge/testing/guides/03-validation-gates.md` | Named authoring sections; testing guide full. Real behavior/proportionate proof; no inherited freshness/report-validator campaign. |

Before dependent edits, put a concise intake in this slice's `closing-report.md`:
source/planning identity/cleanliness, loaded ranges and recovered truncation,
versions and source-cited readback. Reading/query coverage are distinct; hashes,
grep headings or old receipts are not complete reads. Explain delayed TM
publication, level-valued step per sample, lexical MIDI order, drain before next
sample, admission before the running latch, and partial error accounting.
Reload lost binding context after recovery. Resolve conflicts/missing inputs;
otherwise proceed under existing authority without another approval request.

## Source-grounded decisions and Rust guidance

At `b2ffb16`, `try_install` calls a module hook once with FixedUpdate; the loader
passes only supplied parameters and seeds before construction. TM publishes all
ports in Produce, propagation/merge precede sink Consume, and `step` is a
one-sample level. MidiWire constructors **mask** bytes; validation must precede
them. `advance_samples` returns Result<u64, CoreError> and owns both Bevy clocks.
Core begins/latches before Produce and finalizes after Observe. Public wrappers
have no mutable World accessor. `into_app(self)` consumes the wrapper.

| Verified guidance | Applied choice / proof |
| --- | --- |
| AP-23 AVOID | Ordinary owned Vec, <=16 static sinks, synchronous drain; no atomics/queue service or low-jitter claim. |
| API-23 MUST; EH-01 MUST; AP-25/AP-33 AVOID | Concrete TickError variants, explicit fault inspection and completed counts; no masked input, panic for user config or swallowed runner errors. |
| API-39 MUST | Decode i64 before narrowing; strict config, enabled MIDI data and duplicate routes/channels before mutation. |
| TD-05 MUST | Private config, active-note state and buffer internals; plain Copy event record is the explicit public-data exception. |
| DC-05 MUST | Document every fallible API's errors, partial completion and terminal recovery; it is not a generic examples rule. |
| CLI-05 MUST | Thin clap-derive example, stdout for trace, stderr/nonzero for failures; no custom argv parser. |

**Binding public shape** (new signatures; sketches below are not compiled):

```rust
// oxurack-mod-midiout, re-exported by oxurack:
pub struct MidiOutModule; // KIND = "midi_out"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiOutputEvent { pub sample: u64, pub message: oxurack_midi::MidiWire }
// Existing advance signature retained:
impl Oxurack {
    pub fn advance_samples(&mut self, count: u64) -> Result<u64, CoreError>;
    pub fn drain_midi(&mut self) -> std::vec::Drain<'_, MidiOutputEvent>;
    pub fn midi_output_error(&self) -> Option<&oxurack_core::TickError>;
}
```

Keep a public ECS `MidiOutputBuffer` resource with private fields and narrow
drain/fault/admission methods needed by the facade/module hook. Initialize it in
the facade constructor and idempotently in the hook, so empty drains work without
a loaded sink. Bevy 0.18.1 `Mut::into_inner` yields a World-lifetime mutable
reference; `resource_mut::<MidiOutputBuffer>().into_inner().drain()` can return
the Vec Drain without a cloned registry or public World accessor. The borrow
prevents advancing until the iterator is consumed/dropped. Document that dropping
a partially consumed Drain discards its remaining events; the runner consumes all.

### Sink schema, configuration and transitions

Seven input ports, all MergePolicy::Reject, no outputs: `step`, `note_valid`,
`velocity_valid`, `gate` use Gate; `note`, `velocity`, `noise_cc` use Raw.
Four static Int parameters: `channel=0`, `noise_cc=-1`, `note_cc=-1`,
`velocity_cc=-1`. -1 disables a route; enabled numbers are 0..=119. Channel is
0..=15. Reject every other variant, signed/extreme/out-of-range Int, unknown key,
duplicate enabled route and duplicate channel across sinks. Unknown keys win
lexically before decoding; apply config in table order and attribute errors to
the actual instance/parameter. Use existing InvalidParameterValue/UnknownParameter
for config. Validate before creating a module/port or reserving channel ownership.
Supply registered getters for all four actual settings, no runtime setters.
Static channel/CC edits are outside M1; do not pretend missing setters are mutable.

Cache port entities and own one `Option<u8>` active note per sink. Consume the
published step **as a per-sample event**, not a rising-edge detector: consecutive
true samples are two events. If step=false, emit nothing and retain the active
note; ignore unused scalar values. For step=true, read/validate coherent inputs
before mutating that sink. Validate step as Gate even on idle samples; on an
active step validate both validity flags and gate as Gate too. A valid note/velocity,
or enabled noise CC value, must be Raw <=127; report invalid values instead of
allowing MidiWire masking. note_valid=false ignores raw note and releases the old
note. velocity_valid=false ignores raw velocity, uses 64 for a new note and skips
velocity CC. With a valid note, false gate or velocity zero is silent (off only
for the voice). CC emission still follows its own validity on a silent step:
noise always valid, note/velocity only when their validity is true.

Binding order: lexical sink instance name (not entity/ModuleId/spawn order),
then old note-off velocity0, then new note-on if note_valid && gate && velocity>0,
then enabled noise/note/velocity CC in that order. Same-pitch steps retrigger.
Validate the current sink before taking its active note. Store messages via
MidiWire::note_off/note_on/cc, sample = SampleTime::last_index(). No physical I/O.

Illustrative private transition after validation (exact behavior is binding):

```rust
if !input.step { return Ok(()); }
let input = validate_used_values(input, config)?;
if let Some(old) = state.active_note.take() {
    emit(MidiWire::note_off(config.channel, old, 0));
}
if input.note_valid && input.gate && input.velocity > 0 {
    emit(MidiWire::note_on(config.channel, input.note, input.velocity));
    state.active_note = Some(input.note);
}
// Emit configured noise, note, velocity CC routes in that order,
// applying the corresponding validity rule, even on a silent step.
```

### Finite storage, failure and sample accounting

Recommend one reusable Vec with logical capacity `5 * static_sink_count`, checked
arithmetic and fallible reservation at spawn/load time. At most 16 sinks because
channels must be distinct. Reserve before creating the new sink; never resize
for event backlog or silently overwrite/drop output. Keep lexical ordering work
bounded to the configured population; private persistent scratch or a small
per-sample collection is local discretion. No allocation-free/RT performance
claim is required. Guard every append against logical capacity. A step uses at
most five messages per sink (off, on, three CCs).

Add typed variants to the existing non-exhaustive `TickError` and derive Clone
for fault retention (existing variants contain only String/usize):

```rust
OutputBackpressure { pending_messages: usize, completed_samples: u64 },
InvalidMidiOutput { module: String, port: String, reason: String,
                    completed_samples: u64 },
OutputCapacity { reason: String, completed_samples: u64 },
```

Wrap with existing CoreError::Tick, preserving the facade's result type. Capacity
arithmetic/reservation failure is explicit at construction; invalid used port
data is a latched runtime fault. Buffer faults are terminal for this static
session: expose them through midi_output_error, stop new samples and preserve
queued output for caller inspection/discard. Drain does not clear faults or
restart playback; recovery is a fresh rack. Ordinary stop without a fault still
preserves the existing module/pending/held state and can resume after draining.
Retain the first latched fault; a later unread-output check must not replace
an invalid-data fault with backpressure and conceal the original cause.

**Admission must precede the core latch.** Label the existing begin-sample system
with a new public unit SystemSet `SampleStart` in core tick.rs/lib.rs. Add the
sink buffer admission guard `.before(SampleStart)` in the passed schedule.
This is an ordering label on the existing boundary, not a sixth phase or new
scheduler. Guard runs outside latched phase run_if: when running with unread
events or an existing fault, latch a fault and set SimulationRunning=false
before begin_sample. Produce/other phases therefore do not start and SampleTime
does not advance. Do not inspect running locally inside normal sink Consume to
truncate an already executing sample. A failure discovered in Consume stops
the next sample but allows the latched sample to finalize.

The facade must perform the same buffer check **before either Bevy clock is
advanced**, at each manual loop iteration, after its running check. This
conservatively requires draining any nonempty output before requesting another
sample, even if that next sample might be idle; document this single-slot rule.
On denial, stop and return CoreError::Tick(OutputBackpressure) with this call's
completed count; queue, module state, SampleTime and Bevy clocks are unchanged
by the denied sample. After running a sample, derive completed progress from
SampleTime's before/after completed_samples, then inspect any newly latched
fault and return it with that progress. This catches faults inside the sample
without calling a partially completed request wholly successful. Pure shared
stop still returns Ok(completed); live-driver refusal, zero-count and ordinary
stopped-request behavior remain unchanged. A newly detected Consume error has
completed_samples including the finalized failing sample; raw buffer faults use
0 until the facade attaches the request-relative count. Document that distinction.

For Live/into_app callers, the pre-latch guard preserves unread output and blocks
the next logical sample; faults are inspectable in the buffer/facade. No promise
to stop wall-clock time in an externally owned App. On invalid data in a later
lexical sink, preserve earlier sinks' ordered output prefix and stop; no atomic
rollback of a whole sample is claimed. The failing sink remains unmutated and
later sinks emit nothing. Slice04 will track actually sent notes separately.

## Implementation recipe

1. Create `crates/oxurack-mod-midiout/{Cargo.toml,README.md,src/lib.rs}` as a
   framework-only module crate, edition2024/version0.1.0/project license and
   repository metadata. Normal dependencies: oxurack-core, oxurack-midi and
   Bevy app/ECS using existing version/features. Reuse thiserror only if needed.
   Workspace `crates/*` discovers it. Add schemas, strict static decode,
   private state/ports and buffer, registered getters and the transition above.
   Private source modules/tests inside this new crate are allowed.
2. Add only the SampleStart label/export/membership and typed TickError variants
   to core `tick.rs`, `lib.rs`, `error.rs`. Retain all existing phase/latch/stop
   semantics. Register the new hook once: buffer init, pre-latch guard and one
   Consume system processing all sinks in lexical order.
3. In umbrella Cargo/lib, add a normal path dependency on the sink crate,
   re-export MidiOutModule/MidiOutputEvent (and TickError for named errors),
   implement the borrowed drain/fault methods and per-sample admission/error
   accounting. Uninstalled/no-sink racks retain prior behavior. Keep accepted
   Clock/TM as dev dependencies for examples/tests. Add clap4 with derive as a
   dev dependency for the example; update only needed lockfile resolution, no
   unrelated package upgrades. Verify the selected dependency's actual API.
4. Add `crates/oxurack/examples/turing-machine-midi.rs` and
   `examples/patches/turing-machine-midi.ron`. The musical fixture uses seed42,
   Patch.bpm120 and explicit Clock bpm120/ppqn4/pulse_width_ms1; one TM length8,
   write_probability0.75, major/pentatonic supported Scale and range48..72;
   one sink channel0/routes disabled. Supply actual RON metadata fields as in
   the existing fixture. Connect Clock pulse→TM clock, then TM step/note/
   note_valid/velocity/velocity_valid/gate/noise_cc to same-named sink inputs.
   Default `--samples 2000`; require `--patch PATH`; allow `--samples 0` to
   load/validate and produce no rows. Use clap::Parser, a fallible run function
   and ExitCode. Stream every drained event as
   `sample|STATUS_HEX DATA1_HEX DATA2_HEX` (uppercase two-digit bytes), no stdout
   headers/logs. Handle read/load/advance/write errors via stderr and nonzero
   exit; on write failure stop the rack. No accumulated performance Vec or
   standalone MidiTuringMachine loop. The headless loop is:

   ```rust
   // Illustrative, not compiled. All operations end before the next borrow.
   rack.try_install::<ClockModule>()?;
   rack.try_install::<TuringMachineModule>()?;
   rack.try_install::<MidiOutModule>()?;
   rack.load_patch(&cli.patch)?;
   rack.set_driver_mode(SimulationDriver::Manual).start();
   for _ in 0..cli.samples {
       if rack.advance_samples(1)? == 0 { break; }
       for event in rack.drain_midi() {
           let [a, b, c] = event.message.to_bytes();
           writeln!(out, "{}|{a:02X} {b:02X} {c:02X}", event.sample)?;
       }
   }
   rack.stop();
   out.flush()?;
   ```

   Ensure the real run function stops on any error too; an `?` before the final
   stop must not bypass its explicit stop path. Drop is not MIDI cleanup evidence.
5. Add `crates/oxurack/tests/midi_output.rs`, private module/core/facade tests
   where World/state access is needed, README run instructions and docs. Update
   CLAUDE.md's One-Contributor paragraph to current roles/Expedited Mode and
   stale assets/ai skill routes to resolvable installed skills; correct its docs
   target to target/doc/oxurack. Future feature gates use directive08, not the
   aggregate check-deps target. Preserve history/issued planning packets.

## Behavioral proof and gates

Use existing Cargo tests and the actual example; no copied probe crate, new
parser/validator, mutation suite or fixed test-count/report quota.

| Row / setup and action | Expected observation / mistake rejected |
| --- | --- |
| M-01/02 config and installed synthetic producer | Exact seven-port/four-default schema and four getters; valid channel0/15, disabled routes and CC0/119; invalid channel16/negative/extreme, CC120/-2, wrong variants, duplicate routes/channel, unknown keys fail with context before that sink spawns. Distinct-channel two sinks succeed. Static getters are actual config. |
| M-02 channel0, active60, next step note60/velocity100 | `[80,3C,00]`, `[90,3C,64]`; following idle none; silent/invalid-note step only `[80,3C,00]`. Same-pitch and consecutive true steps each retrigger; reject edge-only step handling. Separate tests exercise invalid-velocity fallback64 and zero-velocity silence. |
| M-02 routes noise74/note75/velocity76, note60/velocity100/noise23 | New-note bytes then `[B0,4A,17]`, `[B0,4B,3C]`, `[B0,4C,64]`; validity suppresses only its matching CC. Gatefalse releases voice but valid routes still emit. Raw0/127 are accepted; Raw128 for used values fails before wire masking or that sink's note mutation; idle/invalidity suppress unused-data checks as specified. |
| M-02 two installed sinks spawned z_sink(channel0) before a_sink(channel1), same known inputs | a_sink's `91` note-on precedes z_sink's `90`; on retrigger complete a off/on/CC sequence precedes z's. Real cables/Consume and reversed insertion keep exact order. |
| M-03 facade advance several samples without draining, producer emits at sample0 | Error OutputBackpressure.completed_samples=1, original bytes remain; completed time is1, denied sample changes no module/clock/observer state. Draining before each request allows all requested samples. No sinks/idle output permits grouped advance. |
| M-03 actual pre-latch guard in Update and FixedUpdate | First sample emits; next raw schedule run blocks with SampleTime/state/output unchanged and running=false. Reinstall once after progress preserves state/buffer and adds no duplicate execution. Ordinary stop holds pending output; drain/resume preserves execution; terminal fault does not reset on drain/start. |
| M-03 invalid used value discovered in later sink | Earlier lexical prefix retained, failing sink state unchanged, later sinks omitted; failing sample finalizes once, facade error count includes it, next sample prevented. Pure mid-sample shared stop still completes all phases and returns Ok(completed). |
| M-04 real Clock→TM→sink RON | Compare sample+literal bytes to a separately seeded/configured standalone TuringMachine plus independently assembled MIDI transition expectations. Use fast Clock 1000/30/1 in a cloned test patch for 24 samples, sink sees delayed TM steps1,3,…; do not derive expected messages using the sink's helper. Default/no-default facade modes agree. |
| M-04 actual example processes | Musical fixture at120 BPM emits nonempty valid ordered trace over2000 samples; two fresh commands give byte-identical stdout. Missing/invalid patch exits nonzero; samples0 emits none. Proves executable composition/replay, not endpoint playback or timing. |
| M-05 component/source/evidence inspection | Five-row close and below gates; relevant standalone isolation remains; no unrelated upgrades or changes to accepted engine/adapter behavior. |

Hex literals above are MIDI bytes, not decimal Rust literals; encode as 0x80
etc in tests. Inline/core/facade tests may use existing private World access;
public integration uses facade drain before advancing, never reuses an owner
after into_app. Retain existing seed/control tests rather than rebuilding them.
Count completed samples independently from events (idle samples can emit none).

Run from S, record exact source state/exits/results in closing-report.md:

```bash
cargo test --locked -p oxurack-mod-midiout
cargo test --locked -p oxurack --test midi_output
cargo test --locked -p oxurack --no-default-features --test midi_output
cargo test --locked -p oxurack-mod-clock
cargo test --locked -p oxurack-mod-turingmachine
cargo tree --locked -p oxurack-mod-turingmachine --edges normal
make test
make lint
cargo clippy --locked -p oxurack-mod-midiout -p oxurack-core -p oxurack --all-features --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' make docs
git diff --check
```

Build the example once, then execute its built binary twice for fresh-process
comparison, e.g. `cargo build --locked -p oxurack --example turing-machine-midi`
and `target/debug/examples/turing-machine-midi --patch
crates/oxurack/examples/patches/turing-machine-midi.ron --samples 2000` twice
to distinct temporary outputs; `cmp` must succeed and output must be nonempty.
Record command/result and a few rows, not a tracked full performance dump.
Use the same binary for missing/invalid patch and samples0 checks. Record the
exact newly resolved lockfile/dependency state for checks after edits.

Known four restricted RT MIDI initialization failures and three ignored timing
tests remain explicit beside successful host retry of make test. Do not weaken
tests or convert restricted failures to passes. Genuine new gate/source failures
stay open. `make check`, check-deps/cargo outdated, coverage, benchmarks and soak
are not this slice's gates under directive08. Old Slice02 planning checker is
not a Slice03 validator; no replacement is required. Self-review five criteria,
source diff, links/commands and bubble-up directly.

## Component scope, commits and return

Authorized components: the new midiout crate; core `src/{tick,lib,error}.rs`
and directly related core tests; umbrella `Cargo.toml`, `src/lib.rs`, new MIDI
integration test and named example/fixture; Cargo.lock; root README.md/CLAUDE.md.
Private helpers/tests adjacent within those components are allowed. Preserve
unrelated work and accepted Clock/TM production, RT/bridge/wire, existing
fixtures and issued prompts. No lint suppression, ignored regression, scheduler
replacement, merge, push, release, tool installer or hardware claim.
Changed public behavior, major scope or acceptance goes to CRC/CDC through Duncan;
ordinary helper/import/regression adjustments within this contract do not need
an escalation. If the proposed SampleStart integration cannot preserve the actual
latch contract, report the source mismatch before adopting a different driver.

Allowed planning return paths within this slice: closing-report.md, ledger.md
proposed-done notes, artifacts/test-evidence.md if useful, and optional named
artifacts/logs/* for failures/retries. Do not change the issued prompt/plan,
parent plans/ledgers or prior review history. Intake and command/results may live
in the existing report; no new artifact format/checker needed.

Commit source first, evidence second. Inspect/report the exact changed paths
and use explicit `git add -- <actual files>` and `git commit --only -- <same
files>` lists, not globs or git add .; stage only those files. Keep source and
planning worktrees clean without discarding unrelated changes. Return exact
prompt/source/evidence hashes, path list, five-row proposed disposition with
test/oracle evidence, actual gate outcomes/ignored/failure/retry counts, intake,
applied Rust decisions, deviations/limitations and artifact inventory. Include
`## Bubble-up to arc`: capability delivered, newly discovered dependencies and
specified-vs-delivered scope. CRC independently accepts; Arc10 and Slice04 stay
unadvanced by CC's proposed-done report.
