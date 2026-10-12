# CC closing report — live configuration persistence

Status: implementation in progress; LP-01–LP-05 remain open pending independent CRC review.

## Intake and pre-edit readback

Assignment intake completed before source edits. Planning worktree `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning` is on `planning` at `d039c629ddae6b67c9a4d9040b61e76c41ee0b3a`, clean at intake. Source worktree `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation` is on `codex/oxurack-module-behavior-installation` at `72fe6b52197b7a138afc3d5d7ac970ddfdf66981`, clean at intake. Main is not the implementation target. The live assignment matches its sibling plan and five-row open ledger.

Required reads and extents:

- Assignment prompt (full, 363 lines), sibling slice plan (full), and ledger (full).
- Planning `cdc-directive10.md` and Arc12 `arc-plan.md` (full); `project-plan.md` from Current direction through the policy paragraphs before Design handoff history; `cdc-directive08.md` sections Assessment and decisions, Prompt quality for all remaining work, and Checks and correction policy from Slice03 onward.
- Facade `crates/oxurack/src/lib.rs`: production through the end of the module before `cfg(test)`, live parameter getter/setup tests, and load replacement test.
- Core `patch.rs`: Patch/RackConfig/ModuleConfig/CableConfig, validation and helpers, serialization/file APIs, `PatchHandle`, `apply_patch_to_world` and its helpers; `module.rs` production module/registration/registry definitions; `parameter.rs` production registry definitions; `event.rs` parameter event/source and command/set dispatch; `label.rs` label/tag/RackMeta definitions; `cable.rs` Cable and transform; `tick.rs` propagation ordering and merge; `error.rs` CoreError/PatchError; core `lib.rs` exports/plugin registration; `port.rs` endpoint identity fields.
- LFO fixture `crates/oxurack/examples/patches/turing-machine-lfo.ron` (full) and `crates/oxurack/Cargo.toml` (full). `tests/lfo.rs` loaded-modulation oracle and 10,000-sample fresh replay tests plus trace/setup helpers; `examples/turing-machine-midi.rs` `prepare_rack`/`run_trace`, and `examples/turing_machine_midi/workbench.rs` per-sample clear/continue behavior; README LFO, Scope, per-sample drain, and fresh-process descriptions.
- Rust skill 2.0: API-02/API-23, EH-01, AP-42/43/44 from `guides/02-api-design.md`, `03-error-handling.md`, and `11-anti-patterns.md`.

Conditional module-adapter reading was not applicable at intake; it became required after the LP-04 replay failure and was completed before considering any adjustment. No structured-data queries were required. Historical Arc02/03–05/11 material remains reference-only; the current prompt binds this assignment.

### Contract readback

The rack must expose synchronous parameter read/write and owned snapshot/save APIs. Reads and snapshots use the active `PatchHandle` and the actual registered getters; setters use the existing validation and event-dispatch path, emitting one `DirectApi` notification on success and none on error. Current module configuration, complete registered parameter schemas, current labels, current in-rack cables, and the loaded header identity are serialized. Extraction must reject incomplete or unsupported selected ECS state instead of substituting defaults. Saving is allowed between samples while the rack runs and must preserve sample time, pending MIDI, Scope state, and engine continuation. File errors remain fallible and honest.

The saved artifact is intended to be a seeded configuration: it should replay its edited parameters in fresh racks from the seed, without capturing live LFO phase, Turing Machine register/RNG state, or current engine continuation. Saving original overrides or comparing fresh replay to a midstream continuation would implement the wrong feature. The required example edits the already-running fixture rack, saves it without advancing during save, then hands the saved patch to the existing runner. This readback authorized implementation; the conditional module getter and cable-allocation conflicts surfaced only under the required replay oracles.

## Implementation and evidence

### Source result

Source commit: `a3633bc0d0f468e933cc0f1463fb637a623b33d1` on `codex/oxurack-module-behavior-installation`. It changes only the eight authorized source paths: `README.md`, facade `lib.rs`, new `persistence.rs`, new `tests/persistence.rs`, new `examples/save-live-patch.rs`, and core `event.rs`, `error.rs`, and `lib.rs`. No fixture, dependency, lockfile, module adapter, playback runtime, or patch schema changed. Source worktree is clean after the commit.

The prompt's conditional module-adapter read was triggered by LP-04. I inspected the Turing Machine setter/getter and modulation reconciliation in `crates/oxurack-mod-turingmachine/src/rack.rs` around lines 648–652, 1080–1087, and 1180–1183. No adapter behavior was changed because that path is outside the authorized source list and the prompt requires CRC/CDC disposition for material contract changes.

The facade now exposes synchronous `parameter`, `set_parameter`, `snapshot_patch`, and `save_patch`; successful loads retain version/seed/BPM identity; setter notification dispatch is shared with command handling; snapshots enumerate registered schemas/getters from active handle entities and gather current labels, cable topology/order, and RackMeta; and save extracts before writing. Tests cover DirectApi notification/error behavior, LFO phase continuity, metadata and direct registered-setter changes, pending MIDI/Scope/continuation, active-handle membership, empty patch, missing getter, disabled/cross-boundary cables, destination preservation, failed-load identity retention, and a LastWins tie-order discrepancy. The new example edits the live fixture and the README describes the configuration/checkpoint boundary.

### Five-row evidence walk

- **LP-01 — partially proved:** facade error-class test passes for no patch, unknown module/parameter, and rejected type with unchanged prior value. Inline test proves a successful equal-value DirectApi set emits exactly one event with `previous == new`, failures emit none, and the rack remains running. A separate 200-sample test changes LFO frequency and observes `sin(0.2*pi)` at sample 201, with sample time unchanged by the edit. Core's existing command event tests remain in the passing core suite. CRC still needs to reproduce.
- **LP-02 — partial; acceptance mismatch found:** inline tests prove schema-complete getter extraction, live facade and direct registered-setter values, sorted module names, current module/cable labels and transform, sorted tag sets, RackMeta notes order, unchanged loaded header, independent Clock BPM, active PatchHandle membership, empty-patch support, getterless failure, disabled-cable rejection, and destination preservation. A real parallel-cable LastWins test shows the in-rack current Entity tie order can be extracted, but a fresh load allocates those tied cables in the opposite order: the original input is `.25`, and fresh replay becomes `.75`. Thus the prompt's assertion that saving ascending current Entity order preserves the same tie through sequential fresh cable spawn is false for this source/runtime combination. No load implementation path was authorized for correction.
- **LP-03 — proposed done:** inline control-rack test leaves the sample count, Scope record/overwrite counts, and one unread MIDI event unchanged across snapshot; it then drains once and compares subsequent per-sample MIDI continuation through sample 1000. Separate tests prove unsupported extraction leaves a sentinel destination untouched, I/O failure is returned, and a failed parse after a successful load retains its prior loaded identity. CRC reproduction remains required.
- **LP-04 — open:** the exact required fixture case advances 200 samples, sets LFO frequency to 1 Hz and Turing Machine write probability to `.25`, saves, and compares two fresh saved-file racks with an independently edited reference and the original. Saved-file replay is deterministic across its two racks and differs from the original, but it does not equal the `.25` reference. Source inspection triggered by this failure found that `crates/oxurack-mod-turingmachine/src/rack.rs` registers `get_write_probability` from `state.engine.write_probability()` (around lines 1180–1183), while `reconcile_write_probability` applies current LFO modulation to the engine value (around lines 648–652). After setting the base to `.25` at sample 200, the snapshot getter returns the effective modulated value (observed serialized value `0.5426204`), so the fresh configuration cannot equal the prompt's `.25` reference. This adapter is outside the authorized source list. Small correction proposal for CRC/CDC: either make the registered getter expose the configured base independently of the modulated engine output (requires adapter scope authorization), or revise the LP-04 expected configuration to the documented getter value. The current binding simultaneously requires actual getters and a `.25` base-value replay.
- **LP-05 — example/docs demonstrated; gate suite incomplete:** `save-live-patch` ran successfully and printed `lfo_main.frequency_hz: Float(0.5) -> 1.0`, saved path, and `completed_samples=300`. The existing runner then loaded that saved file in a fresh process, completed 10,000 samples, and reported 10,000 Scope signal rows plus 56 generated MIDI events. It emitted a nonempty trace. The focused default/no-default test commands and `make test` stop at the LP-04 replay assertion, so the complete LP-05 gate is not established.

### Commands and outcomes

- `cargo test --locked -p oxurack-core` — exit 0: 414 unit, 5 integration, 9 seeded-spawn, and 2 doctests passed.
- Focused `cargo test --locked -p oxurack --lib persistence::tests` — exit 0: all 8 persistence unit tests passed.
- Required default package command (`--lib --test persistence --test scope --test lfo --test midi_output --example save-live-patch --example turing-machine-midi`) — exit 101 at `live_configuration_saves_and_replays_in_fresh_racks`; failure is the LP-04 MIDI trace comparison above. The default command was rerun after the source commit and has the same result.
- Required `--no-default-features` package command — exit 101 at the same LP-04 trace comparison. The no-default command was rerun after the source commit and has the same result.
- `make test` — exit 2 / underlying Cargo test exit 101 at the same LP-04 replay assertion; workspace sequence stops there.
- `make lint` — exit 0; Clippy and formatting passed. `cargo clippy --locked -p oxurack-core -p oxurack --all-features --all-targets -- -D warnings` — exit 0.
- `RUSTDOCFLAGS='-D warnings' make docs` — exit 0.
- Both required `cargo run --locked` commands — exit 0. Save example completed at sample 300; existing runner produced a 10,000-sample fresh-process capture.
- `cargo fmt --all -- --check` and `git diff --check` — exit 0.

The existing `block 0.1.6` future-incompatibility advisory remains visible; no warnings were suppressed and no ignored tests were added. The complete required run remains failed on a contract defect, not an endpoint, host-permission, or environmental restriction.

### Self-review and handoff

The implementation preserves owned snapshots, fallible typed errors, setter provenance, and synchronous sample ownership. It does not cache parameter values or infer seed/tempo from engine state. The two discrepancies above were not hidden by weakening the required replay oracle or changing an out-of-scope adapter/loader. Both require CRC/CDC disposition before LP-02/LP-04 can be proposed done. All five ledger rows remain open; this report and the ledger are CC evidence, not independent acceptance.

Planning worktree commit will contain only this `closing-report.md` and sibling `ledger.md`. There are no durable artifacts beyond the committed source changes and these canonical records. No push, merge, release, or downstream slice work was performed.
