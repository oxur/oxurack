# Turing Machine first — dependency and milestone design

CDC/Sofie, 2026-09-17. Source inspected at
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. Source inspection only; no behavioral
reproduction in this planning pass. Operator decision: first integrate the existing
standalone Turing Machine into the Bevy rack and generate MIDI output; then add
Clock, LFO and Scope for a testing workbench. Retain the rest of the roadmap.

## Milestones and architectural intent

**M1: a RON-loaded, seeded Turing Machine rack plays a chosen MIDI endpoint.**
The adapter uses the existing musical engine, with a clear simulation/musical-clock
contract, a real MIDI-out module/path, observable errors, and reliable note cleanup.
An offline capture sink proves the exact stream before physical output is claimed.
Manual synthetic pulse injection in tests is useful evidence but is not the user
milestone. The live runner needs a real clock authority and explicit start/stop.

**M2: Clock + LFO + Scope make that rack inspectable and controllable.**
Clock and LFO supply known signals; an in-patch Scope records what downstream
modules actually receive. Start with bounded deterministic capture and readable
inspection/export, then the smallest usable display/control workbench. A full
four-pane TUI, general REPL, or GUI is not implied by the user's request. Select
presentation with Duncan when capture semantics work. Scope is an explicitly
requested diagnostic addition; the original eleven musical modules remain tracked.

Keep the simulation engine headless, MIDI device ownership at the RT boundary,
and the standalone TM library usable without Bevy or a MIDI device. Prefer an
opt-in rack adapter surface; choose feature versus wrapper crate at the first
implementation design gate with actual dependency/feature tests. Do not duplicate
the musical algorithm or move device I/O into a module's tick function.

## Dependency evidence and necessary contracts

Paths below are implementation-repository-relative at the pinned source revision.
They identify observed boundaries; proposed fixes remain planning decisions.

| ID / owner | Observed source boundary | Requirement before milestone acceptance |
| --- | --- | --- |
| TM-01 / core + API (02/05) | `crates/oxurack-core/src/module.rs:137` OxurackModule has schemas/spawn/setters/getters; `crates/oxurack/src/lib.rs:81` install only registers these | A module installs behavior as well as metadata. Define once-per-type system registration, repeated installation and many-instance behavior, schedule ownership and feature-off builds. A loaded inert entity is not an adapter. |
| TM-02 / core + RT + API (01/02/05) | `crates/oxurack/src/lib.rs:59` fixed 1 ms setup; `:160` step may run zero or many fixed updates; `crates/oxurack-core/src/bridge.rs:116` translates 24-PPQN RT events into TickNow | Separate simulation samples, musical pulses and app/render updates. Pick one live musical authority for M1, convert PPQN/division explicitly, and provide deterministic exact-tick test driving. No double advance, dropped burst ticks or one TM step per 1 kHz sample. Later rack Clock must use the same contract rather than becoming a second competing authority. |
| TM-03 / core + modules (02/06) | `crates/oxurack-core/src/lib.rs:184` chains Produce/Propagate/Consume; `crates/oxurack-core/src/tick.rs` implements propagation then input merge; engine.tick advances on each call | Define when a clock edge reaches TM, when new outputs become visible, and when MIDI-out/Scope consume them. A sample-and-hold high gate must not retrigger each sample. Same-sample versus one-sample latency must be explicit and tested with impulses, multiple modules and creation-order changes; topological cable order alone does not execute a whole module chain. |
| TM-04 / core + TM (02/06) | `crates/oxurack-core/src/patch.rs:546` spawner receives world/name/parameters, not master_seed; master_seed exists on Patch. `crates/oxurack-core/src/rng.rs:21` derives seeds. `crates/oxurack-mod-turingmachine/src/engine.rs:62` accepts explicit seed | Carry patch seed into module construction before initial random state, using stable instance identity. Prove equal output in separate processes, order independence and distinct instance seeds. Existing same-process RNG tests alone do not prove cross-process stability. Define reset/reload semantics: engine.reset clears register/dividers/count but does not reconstruct seeded RNG state. Restart from seed and transport stop are distinct operations. |
| TM-05 / TM (06) | `engine.rs:125` onward supplies setters; `outputs.rs:8` StepOutputs includes optional note/velocity, gate, auxiliary gates/divisions, CC and raw state; core/TM have separate Scale types | Specify port/parameter names, defaults/ranges, absent-note representation, getters and conversion of scale/root/range. Preserve standalone behavior; distinguish strict parameter validation from legacy clamping. Expose every intended engine capability or explicitly account for a later adapter extension; do not let the minimal audible path silently replace the engine's full scope. |
| TM-06 / core + MIDI-out (02/06) | `crates/oxurack-core/src/value.rs:21` is a scalar CurrentValue vocabulary; `crates/oxurack-core/src/bridge.rs:46` is a command buffer | Define ordered MIDI event delivery, not repeated sampling of a retained Value::Midi. Same-step note-off/note-on, retriggers, silence, channel/velocity/CC and more than one pending event must survive. Prefer typed signal ports with MIDI-out owning note lifecycle when sufficient; otherwise design an explicit bounded event path. Resolve this before implementing an adapter around a lossy scalar assumption. |
| TM-07 / RT + API (01/05) | `crates/oxurack-core/src/bridge.rs:155` flush drains a Vec; full RT queue drops commands and emits OutputQueueFull; absent bridge does not flush. Core Panic/SetBpm are successful no-ops (`event.rs:340`) | Own Runtime lifetime, bridge insertion, device/channel selection, start/stop and note-off cleanup. Bound the no-device/no-consumer path. Test saturation, disconnect, errors and shutdown; dropped note-offs cannot be dismissed as harmless. A minimal implemented stop/panic path is an M1 dependency even if the full ergonomic transport API waits. Honor real thread ownership; no unsafe Sync workaround. |
| TM-08 / persistence + API (02/05) | `crates/oxurack/src/lib.rs:126` direct RON load already applies cables; queued mutations remain stubs | M1 can use a static validated patch and small runner. Require bad parameter/port errors and no partial audible startup. Determine failed/repeated-load behavior before promising reload. Live world saving, arbitrary module-state checkpoints and runtime cable editing are not prerequisites for initial playback; remain tracked later. |
| WB-01 / Clock (06) | No clock module crate; old science-rig draft specified pulse width and 1 kHz simulation | After M1, provide named BPM/division/reset semantics and explicit Gate-versus-Float routing, edge behavior and pulse width limits. Test exact edge counts at named interval boundaries and tempo changes. If M1 lacks any usable clock source, pull only the necessary Clock capability forward rather than postpone audible TM. |
| WB-02 / LFO (06) | No LFO crate; core has Float/Bipolar and parameter getters | Phase-continuous frequency changes, finite/range-checked rate/amplitude/offset, reset, and selected sample-rate limits. Clocked versus free-running behavior must be named. LFO output can be scoped first; modulation of TM requires an explicit CV input/parameter mapping, not an assumed cable-to-knob feature. |
| WB-03 / Scope + interface (06/07) | No Scope crate; RackGraph reports topology/parameters, not sample history | Bounded tick/time-stamped buffer captured after the chosen consume boundary; distinguish held samples, trigger edges and MIDI events. Define overflow/drop counters, pause/reset and observation overhead. Preserve narrow pulses when reducing data for display; rendering must not drive simulation. Begin with single-channel scalar capture plus MIDI event log, then expand only for a concrete consumer. |

## Execution order and acceptance examples

1. **Arc08 slice01 baseline and reconciliation (open now).** Reproduce the existing
   gates and check the dependency observations above. Failures get bounded repair
   work, not a bulk modernization prerequisite. CRC returns evidence to CDC.
2. **Arc02/05 execution contract (next design/implementation boundary).** CDC
   selects the smallest slice(s) covering module behavior registration, deterministic
   stepping, seed handoff and phase/edge semantics. Requires isolated tests with
   two real module behaviors; installing/spawning dummy metadata is insufficient.
3. **Arc06 TM rack adapter.** Load a seeded TM through registry/patch, exercise
   parameter getters/setters, pulse it through the actual schedule, compare
   adapter output with the standalone engine for a fixed event sequence; negative
   controls detect double triggering, wrong seed and stale outputs.
4. **Arc06 MIDI-out plus Arc01/05 live boundary.** Preserve event ordering through
   an in-memory sink, then RT queues; pair note-on/off and exercise stop/overflow.
   Minimal endpoint runner and explicit play/stop produce audible output on the
   Operator's chosen device. A mock sink is not hardware acceptance. Full external
   slave synchronization/jitter claims remain their separate inherited gates.
5. **Arc06 Clock, LFO, Scope; Arc07 minimal workbench.** Scope shows a known clock
   pulse train and LFO waveform at simulation time, then TM control signals alongside
   captured MIDI. Change a parameter while observing; playback and capture remain
   stable when display refresh differs from simulation frequency.

Steps 2–5 are roadmap entries, not issued CC assignments or fake future slice
folders. CDC details the first concrete prerequisite slice after baseline results;
CRC advances routine slices within the approved design. Cross-arc contracts come
back to CDC. Carryover retirement does not remove these capability owners; reopen
an existing arc with a recorded successor slice or assign a new arc before execution.

## Retained later work and re-entry

Runtime cable editing/typed handles/full REPL/controller bindings: arcs05/07, after
static playback and inspection expose actual interaction needs. Live save/unload
and mid-stream module-state checkpointing: arcs02/05, when changing/reresuming a
live patch is selected; distinguish reproducible start from seamless continuation.
Other catalog modules and MIDI-in deferral: arc06, original roadmap still applies.
Chord/state design and Lykn: candidates, not prerequisites for TM playback.
Hardware jitter, fresh-process stream equivalence, coverage target, large-rack
benchmarks, soak and musical acceptance: arc08/project, unchanged. Scope is newly
selected diagnostic scope; retain the old broad-MVP percentage as historical only,
not a measured completion estimate for these new milestones.

The ignored science-rig memo informed pulse/capture questions. Its dormant-RT
assumption and four-pane UI are not adopted wholesale: M1 explicitly requires MIDI.
This tracked map preserves the relevant decisions without relying on that ignored
file surviving. Exact API signatures, adapter packaging, event representation and
workbench presentation remain design gates before their implementation slices.
