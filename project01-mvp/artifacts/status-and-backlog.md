# Reconstructed status and backlog — 2026-09-17

Baseline: implementation `main` at `5cad90a6b8904a3ae615bcbd9ed3e277bd94d95a`. This is a best-effort reading of
the root README, legacy catalog, inventory 0014, Git history and small source
cross-checks. No Rust builds, tests, timing measurements or hardware demonstrations
were run during this documentation migration. Historical commit reports are
attested provenance, not freshly reproduced behavioral results.

## Implementation evidence and corrections

| Area | Evidence | Assessment |
| --- | --- | --- |
| RT | `f99f9d6`, `4abcafc`, `1a139ed`, `325d6eb`, `178fd9b`; later priority/timing fixes | Five RT phases have commits; current timing/hardware acceptance is unknown |
| Core | `74fff83`, `ec6bce9`, `a398802`, `8b2d2f5`, `4d8a473`, `f0a2f95`; follow-up `4fd37bd`, `f109eb7`, `205b252` | ECS, persistence and bridge implementation exists; historical plan details can be stale |
| Umbrella/API | `2583d40`, `c3756a7`, `d63c780`, `8e9652c`; `crates/oxurack/src/lib.rs` | Construction, install, patch load, graph, step and into_app exist; full proposed operating surface does not |
| Events/literate | `229b616`, `229eceb`, `a894e0a`, `ce10c39` | Substrate and patch-format changes landed; runtime label commands remain unimplemented in the inventory |
| Graph | `9a0dda4`, `fc0e000`, `5c49283`, then `7a6fcdb`, `d760049` | Getter follow-up supplies `GraphView::parameters_of`, completing the query previously called `knobs_of` |
| Turing Machine | `c0155c3`, `355962f`, `7158cb4`; crate Cargo.toml and source | Standalone engine exists; crate has no core/Bevy dependency and no OxurackModule implementation was found |
| MIDI | `4f920af`; `crates/oxurack-midi/src/lib.rs` | Shared MidiMessage/MidiWire types, not the MIDI-out rack module claimed by inventory 0014 |
| Command dispatch | `crates/oxurack-core/src/event.rs` | LoadPatch, SavePatch, AddCable, RemoveCable still return NotImplemented; direct load API is a separate working path |
| Live tools | `crates/oxurack/src/` contains lib.rs; workspace crate list | No dedicated CLI/REPL implementation found in this bounded reconnaissance |

The README's broad early-development status remains reasonable, but its component
progress is dated. Inventory 0014 predates the getter follow-up and overstates
the ready-to-use rack module catalog. Keep both unchanged as historical sources.

## Inventory 0014 disposition

Every original inventory category is retained below. `Historical implementation`
does not mean acceptance; `candidate/deferred` records a re-entry trigger rather
than silently deleting scope. Detailed slice plans will be written when work is selected.

| Original item | Owner | Current disposition / next action or re-entry condition |
| --- | --- | --- |
| A1 bucket-getter; D1 knobs_of | 02, 04 | Historical implementation `7a6fcdb` + `d760049`; query named parameters_of; validate on restart |
| A2 runtime label mutation | 03; exposed by 05 | Remaining SetDisplayName/Description/Tags/Note operations |
| A3 cable runtime operations | 05 | Remaining; explicit NotImplemented arms; candidate first new slice |
| A4 transport API | 05, consuming 01 | Remaining ECS/rack start/stop/pause/resume; RT primitives already exist |
| A5 reset/All-Notes-Off | 05, consuming 01/06 | Remaining public emergency reset semantics and queue handling |
| A6 PatchUnloaded | 05; notification owned by 03 | Add with real unload lifecycle; not a completed event |
| B1 handles | 05 | Remaining typed module/port/knob handles |
| B2 per-module enums | 05 contract; 06 implementations | Remaining port and knob vocabularies |
| B3 domain value enums | 05, 06 | Remaining conversions to ParameterValue |
| B4 Style C/D surfaces | 05 | Remaining tuple and string-path patch/set operations |
| C baseline catalog | 06 | Eleven baseline modules including adapted Turing Machine and MIDI-out; see explicit list in project plan |
| C MIDI-in | 06, post-MVP | Deferred by catalog to v1.1; revisit after MIDI-out and input use case |
| C chord module | 06, candidate | Not an inherited v1 requirement; scope decision and a concrete stateful-module use case required |
| D2 CableKind/modulation cables | 02/04/05, candidate | Revisit when port-to-knob modulation semantics are selected |
| D3 stronger cross-type ordering test | 03 | Open test-strength question; inspect existing graph tests before deciding whether coverage subsumes it |
| E1 ParameterSetFailed | 03, candidate | Revisit when audit/UI consumers need explicit failure notifications |
| E2 audit-log subscriber | 03, candidate | Retained small demonstrator; select when notification diagnostics are prioritized |
| E3 serde on notifications | 03, deferred | Revisit with a concrete cross-process consumer |
| E4 dependency-graph command ordering | 03, deferred | Revisit when concurrent producers or replay require it |
| F1 port-instance labels | 03, deferred | Revisit with a consumer requiring overrides beyond schema labels |
| F2 tag validation extensions | 03, candidate | Decide limits/normalization when observed input needs exceed v0 rules |
| F3 hierarchical tags | 03, deferred | Revisit if flat tags fail a concrete use case |
| F4 note IDs and CRUD events | 03, deferred | Revisit with per-note runtime editing |
| F5 tag search/filter | 04/07 | Retained interface follow-up; detail with REPL query needs |
| F6 acronym persistence | 03/07 | Retained user-configuration follow-up |
| G1 PersistableModuleState | 02, candidate | Requires Operator selection and concrete stateful module, likely chord |
| G2 arbitrary state in patch files | 02, candidate | Pair with G1; decide schema/compatibility before implementation |
| H1 Lykn Rust embedding | Project, externally gated | Verify actual external API availability when integration is selected; old Liaozhai dependency is historical, not checked current |
| H2 Lykn module behavior | 06, deferred extension | Requires H1, state persistence and Operator scope decision |
| I BatchGenerator | 02/06, deferred | Revisit only when a concrete generator/profiling case needs it |
| I delta hot reload | 05/07, post-v1 | Revisit for a live-performance reload use case |
| I DebugInspector | 07 | REPL watch/inspection belongs to existing v1 roadmap; graphical/TUI versions deferred |
| I port aliases | 05, post-v1 | Flat names suffice until a demonstrated need |
| I Raw(u16) usage audit | 08 | Retain before release; decide removal only after usage review |
| I plugin hosting | Project, post-v1 | Revisit by explicit future project selection |
| I MIDI 2.0 | Project, post-v1 | Revisit by explicit future project selection |
| I distributed/network racks | Project, post-v1 | Revisit by explicit future project selection |

Additional source-observed gap: command-queued LoadPatch/SavePatch dispatch belongs
to arc05 even though inventory A lists only cable operations. This does not imply
that all persistence is absent; direct file loading and core serialization exist.

## Original build roadmap coverage

| Catalog phase | Owning arcs / remaining interpretation |
| --- | --- |
| 0 skeleton | 01, 02, 05; integration acceptance still needs a current baseline |
| 1 first real module | 06; standalone TM exists, Bevy rack adapter remains |
| 2 MIDI output | 01 + 06 for rack MIDI-out; 08 for audible/jitter acceptance |
| 3 patch persistence | 02 core format; 05 runtime API; 07 CLI; 08 determinism across fresh runs |
| 4 slave clock | 01 primitives; 05 public transport; 07 CLI; 08 Logic Pro validation |
| 5 essential generators | 06: Noise, LFO, Sequencer, Euclidean, Clock |
| 6 transformers | 06: Quantizer, Range, Sample-and-Hold, Arpeggiator |
| 7 REPL | 07: commands, completion, watch, nanoKONTROL2/configuration |
| 8 polish | 08: docs/examples, actionable errors, performance, soak, optional release/blog communication |

The old week labels are preserved in the artifact, with no present scheduling meaning.
