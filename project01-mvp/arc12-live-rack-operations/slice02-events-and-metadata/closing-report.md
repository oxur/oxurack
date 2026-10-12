# Slice02 — live metadata and events

## CC intake and contract readback

**Disposition:** implementation in progress; no slice rows are closed or accepted by this report.

**Issued prompt:** `cc-prompt.md`, SHA-256 `d8cdefd5cf8dd993191c5e38521a556656fc65d35f7f971abcab73984cc5dffb`.

**Role and mode:** Three-Contributor Expedited Mode. CC implements and records evidence; CRC review and CDC disposition remain separate. No push, merge, release, or direct CDC communication is authorized.

**Worktree pins at intake:**

| Role | Path | Branch | HEAD | Intake state |
|---|---|---|---|---|
| S | `.worktrees/codex-oxurack-module-behavior-installation` | `codex/oxurack-module-behavior-installation` | `fabfa1cb4beab5fd79f05b88d4bbec34208a1cac` | clean |
| W | `.worktrees/planning` | `planning` | `6653332918530f3d18c640699635927ea1ae7aa9` | clean |
| main | repository worktree | `main` | `a7ce6e61536ca96b6aeb2227ee06e430ed8df162` | protected; unchanged |

The source contract is a synchronous live metadata API, with one shared core write helper and explicit command dispatch. It resolves rack metadata from `RackMeta`, modules and cables only through the active `PatchHandle`, and ports by exact `Port.name` under the selected module. Cable occurrence is zero-based over matching endpoint entities, ordered by present `CableOrder` before absent, then ordinal, then `Entity`. Validation precedes resolution after the no-patch check; all fallible work precedes mutation.

Supported values are text for display name/description, validated tag sets for tags, and ordered note lists for rack notes only. Empty entity text/tag sets remove their component while a successful clear still emits one event carrying the explicit accepted value and actual prior raw value. Rack reads always return the stored value, including empty defaults. Each successful write emits exactly one `LabelChanged` with the correct target identity and provenance; reads and rejected operations emit none. The API does not advance samples, drain MIDI, reset module/transport/Scope state, or alter topology, cable order, or propagation caches. Existing graph queries read labels fresh; existing snapshot/save fields carry labels into fresh replay.

Patch-load event emission remains append-only and retains its existing per-type checks. Existing evidence does not establish a single global cross-type event timeline, so this implementation will not claim that stronger proof. Removal/unload emissions remain Slice04 work. Arc03 inherited rows and their historical child rows remain separate, open obligations; this report will account for the delivered sub-obligations and remaining owners without changing their ledgers.

**Read set and authority:** The full issued 473-line `cc-prompt.md` and checksum; planning `AGENTS.md`; current Project01 status (2026-10-12) and plan (v3.34); Arc12 plan (v1.3) and ledger; Slice02 plan/ledger; Slice01 CRC-accepted verification; Directive08 and Directive11; the Arc03 arc/child ledgers, child plan, CDC carryover disposition, and `artifacts/cdc-successors03.tsv`. Source inspection at the pinned HEAD covered the full core `label.rs`, `error.rs`, `event.rs` API/dispatch sections, core `lib.rs`, patch handle/load and label-emission sections, port/cable/tick-order definitions, facade `lib.rs` runtime/load/snapshot sections, persistence production and loaded-rack/event/pending-state tests, Scope record/accessor behavior, graph label query builders, existing persistence tests, `save-live-patch`, package example discovery configuration, and README runtime/Scope instructions. Rust/posture, project-management, work-verification, and testing instructions were applied. No binding mismatch or required out-of-scope change was found.

**Authorized source paths:** `crates/oxurack-core/src/label.rs`, `event.rs`, `error.rs`, `lib.rs`; `crates/oxurack/src/lib.rs`, `labels.rs`, `tests/labels.rs`, `examples/annotate-live-patch.rs`; and `README.md`. Planning edits are limited to this report and the assigned Slice02 ledger if required by the prompt. No dependency, fixture, engine, persistence, graph, propagation, or lifecycle source changes are authorized.

## M-01–M-05 evidence and closeout

### M-01 — target resolution and raw semantics

CC implemented `LabelTarget::{Rack, Module, Cable}` and raw `label_value`/`set_label_value` helpers in core, with facade forwarding methods. Both require an active `PatchHandle`; module entities must still contain the expected `Module` instance identity. Cable selectors resolve source module then source port before target module/port, match exact child `Port.name`, filter cables by resolved endpoint entities, and sort `(order_absent, ordinal_or_zero, Entity)`. Typed errors distinguish unknown modules, exact unknown ports, missing cable occurrences, and invalid kind/payload combinations.

Evidence: core tests `cable_occurrence_uses_active_ports_then_order_and_entity_tiebreak` and `module_resolution_uses_only_the_active_handle_and_rejects_stale_entities`; public tests `public_label_reads_writes_and_clear_semantics`, `no_patch_and_unknown_active_target_errors_are_typed`, and `failed_value_validation_does_not_change_raw_state`. These cover ordinal opposing Entity order, Entity tie-break, absent order, unrelated lower order, disabled cable selection, exact source-before-target failure, active membership/stale IDs, empty loaded rack, typed wrong payloads, Tag constructor validation/sorting/deduplication, notes order/duplicates/empty strings, raw empty component reads, component clearing, graph formatter fallback, and rack defaults. Both required focused feature-mode test commands pass.

### M-02 — events and provenance

The shared core setter validates and resolves before reading prior raw state, copies event identity, writes one metadata field, then emits exactly one `LabelChanged`. Equal writes and repeated clears emit; rack previous values include `Some(empty)`, entity component absence remains `None`. Core `SetLabel` uses `Command`; facade `set_label` uses `DirectApi`. The explicit dispatcher documentation now states that messages have no queue consumer; no reactor or implicit execution path was added.

Evidence: `labels::tests::direct_and_command_events_have_exact_fields_and_call_order` asserts None→Lead, equal Lead, clear and second clear, rack event identity/previous, direct and command provenance, and a consumer sequence at each completed call boundary: direct label, explicit parameter command, explicit label command. `cable_occurrence_uses_active_ports_then_order_and_entity_tiebreak` asserts selected cable entity and exact `clock_main.pulse -> tm_main.clock` identity. Invalid payload rejection preserves raw metadata and emits no label event. No cross-type `MessageId` ordering is claimed.

### M-03 — graph and persistence

Entity labels use existing `DisplayName`, `Description`, and `Tags` components; rack values use existing `RackMeta`. Empty entity text/tag writes remove components; rack values store empty data. Graph code was unchanged and already reads labels at query time. Existing snapshot/save code was unchanged and already reads/writes these fields.

Evidence: default-feature public test checks immediate graph fallback and cable/module labels; public round-trip test sets rack/module/cable fields, checks loaded identity header, compares complete `Patch` snapshots before/after save and fresh installed-rack load, and verifies raw notes/tags/cable values. The requested public API/save/reload command passes with and without default features. No patch schema field, dependency, persistence, or graph source changed.

### M-04 — running-rack preservation

Evidence: `labels::tests::metadata_calls_preserve_running_engines_scope_and_pending_midi` runs subject/control instances of the actual LFO/Turing fixture, configures identical Scope captures, starts both and advances/drains 200 samples. The subject performs raw read, set/equal/clear, rejected notes edit, and save; sample time, driver, running flag, full Scope records, and overwrite count remain unchanged. Both continue in lockstep to 252 completed samples while the event at index 251 remains pending. Admission still reports one pending message after an intervening Rack write, rejected entity Notes write, read, and save. Each runner drains that same event exactly once, then is empty on a second drain. Full retained Scope records and per-sample MIDI/sample-time continuation match through sample 1000. Metadata is intentionally different; the musical continuation is equal.

### M-05 — example, native gates, and inherited rows

The new `annotate-live-patch` follows the existing live-save example lifecycle, touches only labels, extends Notes by reading/appending/replacing the full list, saves, advances to 300 and stops on success/error. README documents target/error, clear, tag, note, feature, and invocation behavior.

The issued shell recipe was run against the built executable. The example printed fixed raw annotations and `completed_samples=300`; the saved RON was inspected and contains rack name/tags/notes, module display name, and cable description. Repeated fresh 10,000-sample runs and the original LFO fixture compare byte-for-byte; `annotated-1.trace` has 70 rows and SHA-256 `988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`. Scope-view stdout also compares equal. The original MIDI fixture remains 13 rows with SHA-256 `98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`. Missing input and unwritable destination both exited 1 with concrete I/O diagnosis. Trace receipts are in transient `/tmp/oxurack-labels-final.6zcnxa`; no durable artifact was added under D/artifacts.

Required validation on the final source state:

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo test --locked -p oxurack-core` | pass: 417 unit, 14 integration, 2 doctests |
| default focused `cargo test --locked -p oxurack --lib --test labels --test persistence --test scope --test lfo --test midi_output --example save-live-patch --example annotate-live-patch --example turing-machine-midi` | pass: facade 44, labels 4, persistence 2, Scope 6, LFO 12, MIDI output 2, example test harnesses pass |
| same focused facade command with `--no-default-features` | pass: facade 42, labels 4, persistence 2, Scope 6, LFO 12, MIDI output 2, example harnesses pass |
| restricted `make test` | expected environment failure: four `oxurack-rt` CoreMIDI initialization tests; RT 146 passed, 4 failed, 3 existing ignored; Cargo 101 / make 2 |
| host-access retry `make test` | pass: workspace suite green, RT 150 passed / 3 existing ignored |
| `make lint` | pass: Clippy and format check; existing `block 0.1.6` future-incompatibility advisory visible |
| `cargo clippy --locked -p oxurack-core -p oxurack --all-features --all-targets -- -D warnings` | pass; existing dependency advisory visible |
| `RUSTDOCFLAGS='-D warnings' make docs` | pass; docs generated |
| `cargo build --locked -p oxurack --example annotate-live-patch --example turing-machine-midi` | pass |
| `git diff --check` | pass |

### Bubble-up to Arc03

No approved behavior design was changed. The source assignment adds a synchronous live writer/read path using the existing metadata model and event; the only documentation correction clarifies explicit dispatch and the absence of a queue consumer. Runtime removal/unload, cross-type load-order proof, and independent reconciliation of the original Arc03 promises remain open. Existing patch-load checks assert per-type ordering/counts and loader sequencing; they do not provide a cross-type event timeline. Arc12 Slice04 owns future lifecycle/remove/unload notification sites. Arc12 composition with CRC/CDC tracking owns the remaining global ordering proof and complete legacy reconciliation. No Slice03 work was advanced.

The six historical Arc03 ledger rows remain deferred and unaccepted. The status below records this iteration's delivered sub-obligation and remaining owner without changing historical ledgers or criteria.

| Historical row | Delivered sub-obligation in this iteration | Still deferred / receiving owner |
|---|---|---|
| Arc03 `A-03-01` — commands and patch loads produce consistent notifications and persisted labels | Direct API and explicit `SetLabel` command emit exact provenance; existing patch-load label path remains intact; raw runtime metadata saves and reloads. See M-02/M-03 tests above. | Full consistency across all command/load event classes and cross-type global ordering were not independently accepted. Lifecycle removal/unload events: Arc12 Slice04. Arc composition and historical reconciliation: CDC design / CRC tracking in this Slice02 successor. |
| Arc03 `A-03-02` — child work and deferred findings are accounted for at arc composition | Both actual child ledgers and original scopes were read and matched against current source/tests; this table preserves each S row separately. | Full Arc03 composition remains a CDC/CRC task; the two child ledgers remain historical records and were not edited or accepted here. |
| Arc03 Slice01 `S-01` — historical specified behavior reconciled with current implementation | Current explicit dispatch, direct writes, event fields, failures, and live persistence are exercised by M-01–M-04 tests and native gates. | This is only the Slice02 runtime metadata subset. Reconciliation of all original notification behavior remains with CDC design / CRC tracking under Arc12 Slice02; original criterion remains binding. |
| Arc03 Slice01 `S-02` — remaining differences and deferrals have explicit Arc ownership | Runtime removal/unload and cross-type order-proof gaps are named above; successor register assigns this row to Arc12 Slice02, owner CDC design / CRC tracking. | Other notification-tier historical deferrals remain in their original registries/owners. This accounting does not convert the historical row to done. |
| Arc03 Slice02 `S-01` — historical specified behavior reconciled with current implementation | Existing patch label fields/load emissions and their persistence remain regression-covered; this iteration adds runtime read/edit/snapshot/reload evidence. | This is a bounded runtime metadata subset, not acceptance of the whole literate-patching slice. Remaining original specification reconciliation stays with CDC design / CRC tracking under Arc12 Slice02. |
| Arc03 Slice02 `S-02` — remaining differences and deferrals have explicit Arc ownership | Full-list rack Notes and runtime persistence are implemented; legacy deferred findings are named as retained work, and lifecycle/order gaps have receiving owners above. | Rich provenance, per-note identity/events, port labels, tag search, formatter configuration persistence, failure notifications and remaining original deferrals are not implemented by this slice. Their original registers remain authoritative; CDC design / CRC tracking retain this successor row. |

**Deviations and self-review:** A first round-trip test attempted to load over the fixture in the same rack and hit the existing duplicate MIDI sink-channel guard; the test now uses a fresh installed rack, as the contract requires. An initial rustfmt check showed formatting-only differences; the assigned Rust files were formatted and the required format gates pass. No source-scope widening or dependency change occurred. Manual review rechecked exact signatures, `ChildOf`/port resolution, source-before-target precedence, active-handle checks, raw previous values, event identity/source, release of borrows before mutation/event, and unchanged snapshot/graph/load APIs. Existing Bevy load tests do not prove a single global cross-type event timeline; the report and M-05 claim only their actual evidence.

**Commit and workspace closeout:** source changes were committed on `codex/oxurack-module-behavior-installation` as `20c60efa63c4fbded5c0036a3bcd10ff2c4ba4d9` using `git commit --only` with the nine authorized paths; source worktree is clean at that commit. The separate planning commit for this report and ledger is pending. All M-01–M-05 ledger rows remain **open** with CC proposed-done evidence only. CRC independent review and CDC disposition remain outstanding. No push, merge, release, ancestor closure, or Slice03 advancement occurred.
