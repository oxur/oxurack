# CC assignment — an LFO that modulates the playable Turing Machine rack

Initial pass, 2026-10-11. Execute
`arc11-observation-workbench/slice01-lfo/cc-prompt.md`; return to CRC through
Duncan. Author: Sofie/CDC. Three contributors, Expedited Mode. Arc10 is accepted;
this is implementation work, not a request for another planning packet.

## Outcome, baseline and authority

Deliver a five-waveform standalone/rack LFO and a new seeded RON patch that
modulates TM write probability through the existing MIDI example. The user
should be able to run and hear it immediately using their existing endpoint.

Source worktree:
`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`
Branch: `codex/oxurack-module-behavior-installation`
Accepted predecessor: `65522f116a12a9ac96d35cfe25ed9c184f5bddcb`, clean at authoring.
Planning worktree: `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`.
New planning inputs are the commit containing directive09 and this open set.
Verify these inputs/current assignment; reconcile material source drift before
editing. Source and planning commits stay separate; no merge/push/release.

## Required reading and preflight

In this table P means the absolute planning worktree above plus `/project01-mvp`;
S means the source worktree above; K means
`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`.
Resolve equivalent installed skill paths on another host. Planning state is the
issued packet; source state is65522f1. Use complete bounded reads, recovering any
truncation, then a brief source-cited contract readback before dependent edits.
Record loaded paths/sections and source state in the existing closing report,
not a new intake tool. Once coherent, proceed without a separate approval.

| Order | Path | Scope / purpose |
| --- | --- | --- |
| 1 | This prompt, sibling `slice-plan.md`, sibling `ledger.md` | Required-full: current assignment and five acceptance rows |
| 2 | P/cdc-directive09.md; P/arc11-observation-workbench/arc-plan.md | Required-full: authorization, current slice and later capture boundaries |
| 3 | P/cdc-directive08.md | Required-section: “Prompt quality for all remaining work”, “Checks and correction policy from Slice03 onward”; implementation/proof policy |
| 4 | P/project-plan.md | Required-section: “Current direction”, through current design handoff/history before “Purpose and definition of done”; historical appended packets reference-only |
| 5 | S/CLAUDE.md; S/crates/oxurack-mod-clock/Cargo.toml, src/lib.rs | Required-full: source conventions and standalone/rack feature pattern |
| 6 | S/crates/oxurack-mod-clock/src/rack.rs | Required-section: implementation before `#[cfg(test)]`; adapter, strict decode, setters/getters and scheduling pattern. Tests reference-only except relevant fixtures you reuse. |
| 7 | S/crates/oxurack-core/src/parameter.rs | Required-section: ParameterValue, ParameterSchema, setter/getter aliases and ParameterRegistry impl; actual registration APIs |
| 8 | S/crates/oxurack-core/src/tick.rs | Required-section: TickPhase, SampleTime, sample start/finalization and running gate definitions; timing authority |
| 9 | S/crates/oxurack-mod-turingmachine/src/rack.rs | Required-section: produce_turing_machine_outputs, consume_turing_machine_inputs, reconcile_write_probability and the write_modulation/clock/step port schemas; same-sample LFO effect and delayed TM outputs |
| 10 | S/crates/oxurack/examples/turing-machine-midi.rs; examples/patches/turing-machine-midi.ron; tests/midi_output.rs; examples/turing_machine_midi/oracle.rs (last three relative to S/crates/oxurack) | Required-full: executable integration, existing fixture and independent oracle conventions |
| 11 | K/rust-guidelines/SKILL.md | Required-full: domain routing; use actual installed source, not remembered pattern IDs |
| 12 | K/rust-guidelines/guides/11-anti-patterns.md AP-08, AP-23, AP-25, AP-33; guides/03-error-handling.md EH-01; guides/05-type-design.md TD-04, TD-05, TD-12; guides/13-documentation.md DC-01 | Required-section: numerical loop, validated state, errors, public API/docs; guides paths relative to K/rust-guidelines |
| 13 | K/collaboration-framework/knowledge/testing/guides/03-validation-gates.md | Required-full: proportionate proof; repository examples in the skill are not Oxurack commands |

No predecessor report/checker archive is required. Conditional: if changing a
playback helper, read that complete helper and affected tests first; its existing
ownership/cleanup behavior must remain. Read any additional directly governing
source definition needed by the implementation; report unresolved contradictions.

## Source-grounded design and engineering choices

Binding behavior is the slice plan's oscillator, rack timing and demonstration
sections. They resolve all four ports/parameters, ranges, waveform phase, CV,
sync, edits, pause/resume and publication. Key consequences:

- `ParameterValue` has String, not Choice. Use String waveform names; a `OnceLock`
  parameter-schema vector is suitable because String defaults are not const.
- Clock's adapter shows the actual supplied `InternedScheduleLabel`/TickPhase
  installation hook, private port entities and validate-before-mutate setters.
  Reuse that pattern, not its restart-on-edit policy: LFO edits preserve phase.
- TM Consume reads a Bipolar input and clamps base + modulation. Existing
  `CableTransform::Affine { factor, offset }` supplies modulation depth. There
  is no need for a new modulation bus or to change TM.
- `Oxurack::into_app` consumes the facade. For tests needing registry/world
  access, retain the returned App and operate it; do not subsequently use the
  moved rack. For end-to-end MIDI tests, keep the facade and drain each sample.
- Rust AP-23 (AVOID premature optimization) supports a bounded f64 phase and
  ordinary sin/formulas; AP-08 avoids per-sample allocation. TD-05 (MUST private
  state), TD-04 (SHOULD named records), TD-12 (MUST Debug) guide validated config
  and LfoSample. EH-01/AP-25 require Result for bad configuration; AP-33 forbids
  discarded failures. The explicit nonfinite signal-to-neutral rule is signal
  semantics, not swallowing a failed configuration Result. DC-01 requires useful
  public documentation, including ranges and the one-sample control latency.

## Recommended API and implementation sequence

Public names/signatures below are the recommended coherent shape. Preserve the
binding semantics; equivalent private organization is local discretion. A public
API change with material caller/behavior consequences returns to CRC/CDC first.
These are illustrative sketches, not compiled drop-in source.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Waveform { Sine, Triangle, SawUp, SawDown, Square }
// FromStr accepts exactly sine/tri/saw_up/saw_down/square; Display emits them.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LfoConfig { /* private waveform/frequency/offset/width */ }
impl LfoConfig {
    pub fn try_new(waveform: Waveform, frequency_hz: f32,
                   phase_offset: f32, pulse_width: f32) -> Result<Self, LfoError>;
    // Four value getters, plus Default as specified in the slice plan.
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LfoSample { pub out: f32, pub bipolar_out: f32 }
#[derive(Debug, Clone, PartialEq)]
pub struct Lfo { /* config, phase: f64, previous_sync: bool */ }
impl Lfo {
    pub fn new(config: LfoConfig) -> Self;
    pub fn config(&self) -> LfoConfig;
    pub fn set_config(&mut self, config: LfoConfig); // phase/history unchanged
    pub fn next_sample(&mut self, frequency_cv: f32, sync: bool) -> LfoSample;
}
// LfoError: typed invalid-frequency/offset/width and unknown-waveform errors,
// Debug + thiserror::Error + PartialEq; no panic for invalid public config.
```

1. Create `crates/oxurack-mod-lfo/{Cargo.toml,src/lib.rs,src/lfo.rs,src/rack.rs}`.
   Follow Clock's package metadata/edition/default=[] and optional `rack` deps
   (Bevy app/ECS0.18 and local core); use already-locked thiserror2. The workspace
   `crates/*` glob picks it up; no root member-list edit needed. Keep standalone
   state independent of Bevy, core, midir, wall clock and RNG. Public docs include
   a small executable standalone example and conditional adapter links.
2. Implement validation and pure oscillator first. Private phase starts0. Sync
   rising resets before evaluating, then each call increments once. Resolve
   waveform from the enum, convert bounded output to f32. Suggested core:

```rust
// Within next_sample; sketch, not tested source.
if sync && !self.previous_sync { self.phase = 0.0; }
self.previous_sync = sync;
let p = (self.phase + f64::from(self.config.phase_offset())).rem_euclid(1.0);
let b = match self.config.waveform() {
    Waveform::Sine => (std::f64::consts::TAU * p).sin(),
    Waveform::Triangle => 1.0 - 4.0 * (p - 0.5).abs(),
    Waveform::SawUp => 2.0 * p - 1.0,
    Waveform::SawDown => 1.0 - 2.0 * p,
    Waveform::Square => if p < f64::from(self.config.pulse_width()) { 1.0 } else { -1.0 },
}.clamp(-1.0, 1.0);
let cv = if frequency_cv.is_finite() { frequency_cv.clamp(0.0, 1.0) } else { 0.0 };
let hz = (f64::from(self.config.frequency_hz()) + 100.0 * f64::from(cv)).min(100.0);
self.phase = (self.phase + hz / 1000.0).rem_euclid(1.0);
LfoSample { out: ((b + 1.0) * 0.5) as f32, bipolar_out: b as f32 }
```

3. Adapter: `LfoModule` KIND="lfo", private component owns Lfo, retained CV/sync,
   and four port Entity handles. Validate unknown keys (lexically first) before
   known fields in waveform/frequency/offset/width order, before spawning entities.
   Numeric errors map to `CoreError::InvalidParameterValue` with the right field;
   unknown key uses UnknownParameter. Register all four setters and getters.
   Setter builds a complete candidate from current config, validates, then calls
   set_config; errors leave oscillator and retained input state untouched.
4. Register Produce and Consume once in the supplied schedule. Produce calls
   next_sample with retained controls and writes both CurrentValues. Consume only
   stores current freq_in/sync_in. Initial controls0/false. Core gates stopped
   samples; do not add a second running check inside each phase. No explicit
   ordering relative to TM within the same phase is needed: propagation separates
   output writing from input reading. Use typed port queries as in Clock/TM.
5. Add umbrella dev dependency with feature `rack`; install LfoModule in the
   example's `prepare_rack`. Add the new RON fixture with the exact cable/config
   in the plan; leave the original fixture literal unchanged. README documents
   offline and optional `--output-name "MIDI Bus 1"` usage; users still select
   their endpoint. Do not open an endpoint merely to satisfy automated tests.
6. Add small engine/rack tests and `crates/oxurack/tests/lfo.rs` for actual patch
   integration. Reuse temporary-patch and registry/observer conventions rather
   than build a new harness. Extend test-only helper code only as needed for a
   genuinely independent MIDI oracle; do not compute expectations by calling Lfo.

Registry test shape (existing API, illustrative):

```rust
app.world_mut().resource_scope::<ParameterRegistry, _>(|world, registry| {
    registry.set_parameter(world, entity, &"lfo".into(), "frequency_hz",
                           ParameterValue::Float(2.0))
}).unwrap();
```

For setup, install LfoModule on the rack first, then use into_app if the test
needs direct world access. Obtain the entity by Module.instance_name. Keep
private transition tests in the crate; observe actual published ports for
installation assertions. For reinstall/pause assertions, keep the facade through
advance/stop/start/try_install and convert into_app only after the final sample
for port inspection. Example: SawUp62.5Hz, three samples, reinstall, one sample
must publish -.625 at index3; an extra system execution or phase reset disagrees.
No test-only public accessor is required.

## Tests and checks

Use these discriminating checks; group cases naturally, with no target test count.

| Row | Setup/action | Expected result / mistake rejected |
| --- | --- | --- |
| L-01 | Each waveform at62.5Hz, zero offset; inspect samples0/4/8/12/16 | Plan's quarter-phase values, sample16 equals phase0; tolerance1e-6 for floats. Reject output-after-increment and wrong orientation. Width.25 square falls at sample4. |
| L-01 | Zero base plus CV.625; zero base/CV0; large/negative/nonfinite CV | First matches62.5Hz sequence; zero holds; finite clamps and NaN/inf neutral follow contract, all outputs finite/bounded. |
| L-01/L-02 | SawUp62.5Hz; sync false,false,true,true,false,true | Standalone bipolar -1,-.875,-1,-.875,-.75,-1. Installed input consumed at n resets at n+1, not n. Held high does not repeatedly reset. |
| L-02 | Actual installed Clock→LFO sync, two LFOs with different offsets; observer in Observe | Clock edge0 resets LFO at1; no further held-high resets; exact short sequence and independent offsets. Stop produces no phase advance; resume continues; reinstall leaves same sequence. Exercise supplied Update and FixedUpdate via existing CorePlugin patterns, not a duplicated scheduler. |
| L-03 | Registered four setters/getters; valid edits after advancement, then bad type/range/NaN/unknown waveform and unknown spawn key | Returned config agrees; successful edits preserve accumulated phase; failed edits leave config and continuation equal to untouched control; invalid spawn adds no entities. Table-driven cases are enough, not a combinatorial matrix. |
| L-04 | Real loaded accelerated fixture: Clock1000BPM/30PPQN/width1, LFO square62.5Hz/offset0/width.5, TM base.5, affine depth.5,24 samples | LFO bipolar +1 at samples0..7, -1 at8..15, +1 at16..23. At even TM tick samples use write probability1/0/1 accordingly; next odd sample emits that step. Separately drive seeded standalone TM and literal off/on logic, as existing oracle does, with this hand-derived probability schedule. Compare exact ordered sample/bytes, require nonempty output and contrast an unmodulated control so a missing cable cannot pass. |
| L-04 | New unchanged musical fixture, existing built example,10000 samples twice; original fixture2000 once | New traces equal/nonempty; original remains SHA256 `98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`. This is pinned-build reproducibility, not cross-platform sine equality. |

Author check: a temporary Rust program linked to the existing engine/core at
65522f1 exercised the hand-derived 24-sample modulation schedule: 8 MIDI rows
versus 10 for the unmodulated control, nonempty and different. It did not use an
LFO implementation (none exists yet), is not a delivered test tool, and does not
replace CC's real installed-path test. API/oscillator sketches above remain
illustrative, not compiled implementations.

Run from S after updating Cargo.lock for the new local package without unrelated
upgrades. Initial dependency resolution may need one unlocked build; report it,
then use locked checks. Record exact commands and outcomes:

```bash
cargo test --locked -p oxurack-mod-lfo
cargo test --locked -p oxurack-mod-lfo --features rack
cargo test --locked -p oxurack --test lfo
cargo test --locked -p oxurack --no-default-features --test lfo
cargo test --locked -p oxurack --test midi_output --example turing-machine-midi
cargo tree --locked -p oxurack-mod-lfo --edges normal
make test
make lint
cargo clippy --locked -p oxurack-mod-lfo -p oxurack --all-features --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' make docs
cargo build --locked -p oxurack --example turing-machine-midi
git diff --check
```

Use the built binary for the described trace comparisons. Normal standalone
LFO dependencies must exclude Bevy/core. Full workspace tests historically need
host CoreMIDI access; use available approval flow if needed, preserve actual
blocked/failed outcomes. Three pre-existing RT ignores and block0.1.6 advisory
remain explicit. No new ignored tests, suppression, coverage/freshness campaign
or structural checker. Native gates include example compatibility; further
testing is driven by findings, not speculative completeness.

## Scope, discretion and return

Authorized source components: new LFO crate; umbrella Cargo.toml, example loader,
new fixture, integration tests/test helpers; README and Cargo.lock wiring.
Planned explicit paths:
`crates/oxurack-mod-lfo/Cargo.toml`, its `src/lib.rs`, `src/lfo.rs`, `src/rack.rs`;
`crates/oxurack/Cargo.toml`; `crates/oxurack/examples/turing-machine-midi.rs`;
`crates/oxurack/examples/patches/turing-machine-lfo.ron`;
`crates/oxurack/tests/lfo.rs`; `README.md`; `Cargo.lock`.
Private helpers/tests inside those components may use additional files, reported
and explicitly named when staging/committing. No unrelated changes or blanket
source freeze. Core/Clock/TM/RT algorithm changes, new public scheduling/fault
machinery, Scope or a new GUI are outside this assignment; report an actual need
to CRC/CDC rather than implementing a competing design.

Commit implementation with explicit actual paths (`git commit --only`); never
stage everything. Separately commit this slice's closing-report.md and ledger.md
(and any necessary named artifact). Preserve this issued prompt and plan; CRC
owns their review/assignment updates. Return source/planning hashes, all five row
claims with evidence strength, command results including gaps, concise intake/
readback, changed paths, self-review, deviations and arc bubble-up. Expected
artifact inventory is only the existing records unless a real failure needs a
supporting log. CC remains proposed-done until CRC's independent review.
