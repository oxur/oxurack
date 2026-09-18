# CC implementation assignment — make installed modules execute

Project01 MVP / Arc09 executable rack / Slice01 module behavior installation.
First execution of this slice, preserved replacement `cc-prompt-iteration01.md`
for the held, unexecuted `cc-prompt.md`. 2026-09-17. CC implements and returns
to Sofie/CRC through Duncan; CDC owns architecture. Three contributors and
Expedited Mode apply. Source baseline: clean `main` at
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`; planning assignment baseline
is the commit introducing this file. Verify both states before editing. The
replacement filename consumes no executed refinement pass (0 of 5 so far).

The outcome is a rack whose installed module systems really run for every
instance loaded from RON, exactly once per type per rack. A same-type reinstall
is inert; a different Rust type claiming an installed kind returns a typed error
before changing the rack. Preserve the old chaining API. Repair the eleven
known strict-rustdoc diagnostics in the three approved doc-only files. Execute
I-01–I-06; all six remain open until CRC review. This is implementation work.

## Required reading and intake before dependent edits

Roots: planning = `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`;
source = the isolated implementation checkout created from the pinned main commit;
skills = `/Users/oubiwann/.codex/skills`. Paths below are relative to those
roots, except the Bevy registry path. The planning files are at the planning
commit introducing this assignment; source sections are at the source pin.
Resolve a changed or inaccessible path with CRC before dependent work.

| Order | Exact path / state | Scope to load | Why and when |
| --- | --- | --- | --- |
| 1 | planning `project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration01.md` | Required-full | Active assignment, before all work. |
| 2 | planning `AGENTS.md`; `project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/slice-plan.md`; same directory `ledger.md` | Required-full, each | Local authority, exact source/evidence allowlists, six I rows and checks. |
| 3 | planning `project01-mvp/cdc-directive04.md` and `project01-mvp/cdc-directive03.md` | Required-full, each | Current relay/authoring amendment and approved behavior/ownership; initial prompt remains held. |
| 4 | planning `project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/artifacts/execution-contract.md` | Required-section: entire §1, plus §5 Q-01 paragraph and opening authority paragraph | Binding API and failure policy; §2–4 are future designs, not this slice. |
| 5 | planning `project01-mvp/arc09-executable-rack/arc-plan.md` | Required-full | Slice sequencing and inherited arc boundary. |
| 6 | planning `project01-mvp/project-plan.md` | Required-section: “Purpose and definition of done”, “Current roadmap — v3.0” through “Quality dispositions”, and “Prompt-authoring amendment — v3.1” | Milestone priority, retained acceptance and current handoff. Earlier sections are historical unless explicitly referenced. |
| 7 | skills `collaboration-framework/knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md` | Required-section: “Required reading and CC intake” in full | Load completely and perform its intake/readback before implementation. |
| 8 | skills `collaboration-framework/knowledge/project-management/guides/README.md` | Required-section: “Expedited Mode” | Exact commit, relay and gate behavior. |
| 9 | skills `rust-guidelines/SKILL.md`; `rust-guidelines/guides/02-api-design.md`; `rust-guidelines/guides/03-error-handling.md`; `rust-guidelines/guides/04-ownership-borrowing.md`; `rust-guidelines/guides/06-traits.md`; `rust-guidelines/guides/11-anti-patterns.md`; `rust-guidelines/guides/13-documentation.md` | Required-section: SKILL “Document Selection Guide” and “Strength Indicators”; API-23; EH-01, EH-04, EH-14, EH-18, EH-29; OB-02, OB-12, OB-13; TR-09; AP-09, AP-25; DC-05, DC-06, DC-12 | Apply the named API/error/borrow/doc rules before coding; read more if a new concern arises. |
| 10 | skills `testing/guides/01-testing-discipline.md` and `testing/guides/03-validation-gates.md` | Required-full, each | Behavioral oracles, failure triage and repository-native gates. |
| 11 | source `crates/oxurack-core/src/module.rs` (`OxurackModule`, `ModuleRegistry`); `crates/oxurack-core/src/error.rs` (`CoreError`); `crates/oxurack-core/src/parameter.rs` (`ParameterRegistry`); `crates/oxurack-core/src/patch.rs` (`Patch`, `ModuleConfig`, `apply_patch_to_world`) | Required-section: complete named definitions/functions and their imports/nearby docs | Recheck real trait, registry, spawner, accessor and RON path before edits. |
| 12 | source `crates/oxurack/src/lib.rs` (`Oxurack`, `new`, `install`, `load_patch`, `into_app`, `Default` and existing tests); `crates/oxurack-core/src/lib.rs` (`CorePluginFixed`/`build_core_plugin`); `crates/oxurack-core/src/tick.rs` (`TickPhase`) | Required-section: complete named definitions/functions and test helpers | Recheck constructor, fixed schedule, world-borrow and existing compatibility path. |
| 13 | local Cargo registry `bevy_ecs-0.18.1/src/schedule/set.rs` (§`ScheduleLabel`, `InternedScheduleLabel`) and `bevy_app-0.18.1/src/app.rs` (`App::add_systems`) | Required-section: named API definitions; locate the registry root from Cargo if its hashed index directory differs | Verify schedule label and add-system calls against locked Bevy 0.18.1 before using them. |
| 14 | planning `project01-mvp/artifacts/cdc-review03-rustdoc.log`; source `crates/oxurack-mod-turingmachine/src/length.rs`, `crates/oxurack-mod-turingmachine/src/write_knob.rs`, `crates/oxurack-graph/src/lib.rs` | Required-section: all eleven diagnostics and corresponding comments | Repair exact existing docs, before strict documentation checks. |
| 15 | planning `project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/cc-prompt.md` | Reference-only, historical | Preserved predecessor, not an instruction to execute or combine with this replacement. |

Read all required extents completely in bounded output chunks; recover truncation
contiguously. File existence, hashes, grep hits, summaries and another context's
reading do not establish intake. In the allowed `artifacts/test-evidence.md`,
record each item, revision/state, loaded extent and any gap; mark conditional
items triggered or not applicable. Before dependent edits, write a short
source-cited contract readback linking the binding API, no-mutation conflict,
hook-panic boundary, phase-specific execution, preserved functionality, tests
and exclusions to the sections/rows above. Name the easily missed rule that a
hook counter alone does not prove installed systems run. If material sources
conflict or are unavailable, resolve through CRC before affected edits; after
coherent preflight, proceed without another approval request. On a resumed or
compacted context, revalidate state and reload contracts no longer reliably
present. Intake is CC attestation; CRC checks the actual implementation too.

## Inspected source and applied engineering guidance

At the pinned source, `Oxurack::new` constructs the only rack struct literal,
adds `CorePlugin::fixed()` and optionally `RackGraphPlugin`, sets `Time<Fixed>`
to 1 ms and initializes `SimulationRunning`; `Default` calls `new`. `install`
currently calls `ModuleRegistry::register::<M>()` then registers setters/getters
in `ParameterRegistry`; neither operation checks collisions and both can replace
existing entries. `load_patch` reads RON, validates against the registry and
calls its stored spawner. `into_app` exposes the built `App`; existing tests
directly run `FixedUpdate` and show unrelated `Update` does not propagate.
`CorePluginFixed` configures `TickPhase::Produce -> Propagate -> Consume` in
`FixedUpdate`. `ModuleKind` is `Hash + Eq`; `OxurackModule` is `'static` and its
spawner/setter/getter functions are already type-erased. `CoreError` is an
existing `#[non_exhaustive]` `thiserror` enum. Core already depends on
`bevy_app`; no manifest edit is needed. The `crates/oxurack/tests/` directory
and RON fixture do not yet exist; create the two allowlisted files there.
Locked Bevy 0.18.1 defines `InternedScheduleLabel` in `schedule/set.rs`,
supports `ScheduleLabel::intern()` and `App::add_systems(schedule, systems)`.

Applied guidance, not an unbounded requirement list:

| Read source, strength | Decision and rationale | Proof |
| --- | --- | --- |
| Rust TR-09 SHOULD; `06-traits.md` | Default behavior hook extends the existing behavior trait without forcing legacy modules to change. | Existing module impls compile; legacy fixture loads. |
| Rust OB-02 MUST, OB-12/13 SHOULD; `04-ownership-borrowing.md` | End temporary `World`/resource borrows before passing `&mut App` to the hook; keep the map on `Oxurack`. | Hook can add a real system; clippy/build pass without cloned app or unsafe workaround. |
| Rust EH-01/EH-04 MUST, EH-18 CONSIDER, EH-29 CONSIDER; `03-error-handling.md` | Recoverable kind conflict uses matchable `CoreError`; existing `install` retains documented programmer-error panic for compatibility. Do not catch hook panics or claim rollback. | Exact conflict variant, no mutation, panic-counterpart test/docs. |
| Rust DC-05/06/12 MUST; `13-documentation.md` | Document `try_install` errors, `install`/hook panic boundary and only resolvable links. | Both strict doc commands pass, with no lint allowance. |
| Testing discipline/validation gates, section “Test Behavior And Contracts” / “Gate Selection” | Assert loaded-instance behavior and original state after conflict, not just hook calls or a green wrapper command. | Two-instance trace, negative cases and full matrix below. |

Rust AP-09/AP-25 (AVOID) and API-23 (MUST) reinforce the same boundary; they
do not override CDC's explicit legacy `install` convenience contract. No new
runtime, dependency, broad refactor or generic core installer is implied by
these guides.

## Binding contract and recommended implementation sequence

**Binding from execution-contract §1.** Add exactly this default trait method:

```rust
fn register_systems(
    app: &mut bevy_app::App,
    schedule: bevy_ecs::schedule::InternedScheduleLabel,
) {}
```

Add `Oxurack::try_install<M: OxurackModule>(&mut self) -> Result<&mut Self,
CoreError>` and `CoreError::ModuleKindConflict { kind: String }`. Successful
installations are tracked **per rack** by `ModuleKind` and `TypeId::of::<M>()`.
TypeId is process-local identity only: never serialize it, derive seeds from it,
or use it for event ordering. A new kind registers the existing metadata,
spawner, setters and getters, invokes `M::register_systems(&mut self.app,
FixedUpdate.intern())` exactly once, then records success. Same type/kind returns
unchanged. Different type/same kind returns the exact variant with `kind` equal
to `M::KIND` before registry, accessor, schedule or resource mutation. Error
Display must name the kind. No silent replacement and no per-spawn hook.

Keep `install<M>(&mut self) -> &mut Self` for chaining; delegate to `try_install`
and panic with a clear programmer-error message on collision. Dynamic module
selection uses `try_install`; patch-file loading must never invoke this panic
path. A user hook panic is a programmer failure and may leave partial
registration; no catch-unwind or arbitrary rollback guarantee. Raw
`ModuleRegistry::register` remains metadata-only. `step`, timing, seeds,
`SimulationRunning`, phase ordering and existing patch-load semantics stay
unchanged. No device/RT ownership is added here.

Recommended ordered recipe (private helper names are local discretion):

1. In `crates/oxurack-core/src/module.rs`, add the documented no-op default
   hook beside `spawn`, explaining schedule, once-per-kind use and no per-instance
   effects. Reuse the existing Bevy dependencies; do not alter the raw registry.
2. In `crates/oxurack-core/src/error.rs`, add the documented named conflict
   variant to `CoreError` with a Display message containing the kind.
3. In `crates/oxurack/src/lib.rs`, add a private
   `HashMap<ModuleKind, TypeId>` field to `Oxurack`; initialize it in `new`.
   `Default` already delegates, and `into_app` consumes the rack without
   needing to export bookkeeping. Check all constructor/caller sites.
4. Implement `try_install` by checking the map before *any* mutable registry
   access. Same TypeId returns `Ok(self)`; a different TypeId returns the
   conflict. For a new kind, reuse the current metadata/spawner and
   setter/getter loops. Put temporary `World`/resource borrows in a bounded
   scope so they end before calling the hook with `&mut App`. After the hook
   returns, insert the kind/TypeId in the map and return `Ok(self)`.
5. Make `install` delegate to `try_install`, preserving chaining and documenting
   `# Panics`; document `try_install` `# Errors`, and the hook's panic/partial
   registration boundary. Do not replace this with a global singleton, a
   per-instance system, a silent kind overwrite, or a transaction claim.
6. Add the real RON fixture and integration tests below. Run focused tests
   before broad gates; fix mistakes inside the eight-path allowlist only.
7. Change **only doc comments** at the three legacy doc paths using the exact
   diagnostics below; run strict rustdoc after new API docs are in place.

Representative *uncompiled* borrow/order sketch; the binding properties are
precheck, existing wiring, borrow end, one hook call and post-hook record:

```text
kind = ModuleKind::from(M::KIND); identity = TypeId::of::<M>()
match self.installed.get(&kind):
    Some(identity)       -> Ok(self)
    Some(other_identity) -> Err(CoreError::ModuleKindConflict { kind: M::KIND.into() })
    None                 -> continue
{ world = self.app.world_mut()
  world.resource_mut::<ModuleRegistry>().register::<M>()
  world.resource_mut::<ParameterRegistry>() registers M setters and getters }
M::register_systems(&mut self.app, FixedUpdate.intern())
self.installed.insert(kind, identity)
Ok(self)
```

Do not copy the sketch as compiled Rust. CC may choose private helper names and
test-module internals, but must preserve the exact public signatures, ordering,
failure boundary and observations. A needed public API, scheduler, feature or
acceptance change is unresolved design work: stop affected edits and ask CRC to
route it to CDC through Duncan.

## Behavioral fixture and falsifiable oracles

Use the allowlisted `crates/oxurack/tests/fixtures/behavior-installation.ron` as
a static `Patch` with version `1.0`, seed 42, BPM 120, no cables, and two
`behavior_counter` instances named `alpha` and `beta`. Their `increment`
`ParameterValue::Int` overrides are 2 and 3. Optional labels/tags/rack data can
stay empty. Validate the fixture by parsing through the existing
`load_patch_from_file` and loading it through public `Oxurack::load_patch`.
The integration test `module_installation.rs` may use
`spawn_module_entity`, a private `StepState { increment: i64, total: i64 }`
component, `PatchHandle.modules` and `ParameterRegistry` APIs. The module's
spawner reads the override and initializes total to 0; its registered Produce
system queries **all** matching instances and adds each instance's increment
once per direct `FixedUpdate`. A hook-count resource is supplemental.
Use `app.add_systems(Update, || {})` if necessary to ensure an unrelated Update
schedule exists without replacing the graph plugin's schedule; then direct
`World::run_schedule(Update/FixedUpdate)` after `rack.into_app()`. Do not use
`rack.step()` to claim exact fixed runs because it may execute zero or more.

| Rows / case | Setup and action | Exact expected observable | Bad implementation rejected |
| --- | --- | --- | --- |
| I-01/I-03 two instances | Install counter once; public RON load; inspect `StepState` through `PatchHandle`; run direct `FixedUpdate` once. | Initial `(alpha,beta)` totals `(0,0)` and increments `(2,3)`; after one fixed run totals `(2,3)`; hook count 1. | Metadata-only install, hook per spawn, system only updates one entity. |
| I-03 wrong schedule | Run direct `Update`, then another direct `FixedUpdate`. | Totals remain `(2,3)` after Update, then become `(4,6)`; no extra hook call. | Hardcoded Update or wrong set/schedule registration. |
| I-02 reinstall/state | Use original registered setter to change `alpha` increment to 4 after totals `(4,6)`; read getter, reinstall same type; run one fixed schedule. | Setter/getter reports `Int(4)`; totals remain `(4,6)` immediately after reinstall, hook count stays 1; next fixed run yields `(8,9)`. | Duplicate systems, reset resources/state or replaced accessor. |
| I-02 collision | In a separate rack install original counter, then a distinct Rust type with the *same kind* but different display metadata, spawner marker, setters/getters and system. Call `try_install` before load. | `Err(CoreError::ModuleKindConflict { kind })` with kind exactly `behavior_counter`; original metadata and accessor/spawner functions remain; conflicting hook count 0. Loading the static fixture still creates original `StepState` (no conflicting marker), and one FixedUpdate yields `(2,3)`; original setter/getter still work. | Partial replacement, mutation-before-conflict, silent success or wrong error. |
| I-01 independent racks | Construct two racks and install the same Rust type in each; load fixture independently and run one fixed schedule per rack. | Each rack reports hook count 1 and its own totals `(2,3)`; neither rack's install affects the other. | Global installation map/singleton. |
| I-01 legacy compatibility | Define a small legacy `OxurackModule` omitting the new hook; use `install` chaining and a minimal RON load. | It compiles and loads; reinstalling it is harmless. | Breaking old trait impls or new hook required per instance. |
| I-02 convenience panic | In a separate rack call old `install` with a conflicting kind; catch only in test to assert documented programmer-error behavior. | Clear panic naming kind; dynamic `try_install` returns the typed error. | Silent overwrite or user-input path panic. |
| I-03 graph matrix | Run the same integration test with default graph and `--no-default-features`; graph-only assertions behind `cfg(feature = "graph")`. | Identical `(2,3)`, `(4,6)`, `(8,9)` traces in both. With graph on, loaded module count is 2; off builds without graph API. | Feature-dependent hook, accidental graph dependency. |
| I-04 docs | Run both strict doc commands after comment-only repairs. | Exit 0; all eleven diagnosed links resolved without lint suppression. | Partial doc repair or behavior edit in legacy files. |
| I-05/I-06 gates/evidence | Run complete matrix and review exact diff/commits and six-row report. | Every runnable required check recorded and passing; any blocked/failed check is preserved and dispositioned. | Green-wrapper inference, missing logs or silent file/scope drift. |

For setter testing, use the existing `ParameterRegistry::set_parameter` and
`get_parameter` signatures; Bevy `World::resource_scope` can temporarily expose
the registry alongside `&mut World`, as `Oxurack::load_patch` does for its module
registry. Ensure instance state inspection is by the loaded handle's named
entities, not just an aggregate hook counter. Do not change production APIs
solely to make the test easier. No patch-load rollback claim is made by this
slice; the conflict no-mutation test covers the pre-install collision boundary.

## Exact documentation repair scope

CDC's pinned strict-rustdoc log has **3 unresolved links and 8 redundant
explicit targets**. In TM `length.rs:67`, qualify the `NonZeroUsize` link as
`std::num::NonZeroUsize` (or an equally resolvable target). In TM
`write_knob.rs:36`, link the getter as `Self::probability` or render the field
as plain code. In graph `lib.rs:311`, render the dependent umbrella
`Oxurack::graph` reference as prose/code, since graph cannot link to its
dependent umbrella crate. Remove only the redundant explicit target parts of
the four imported notification links at lines 6–9 and the same four at
855–859, preserving the useful links and prose. Do not add lint allowances,
delete useful documentation, change behavior in these files or add a graph
dependency on the umbrella crate. Document new hook/fallible/convenience APIs
with the failure and panic boundaries above.

## Validation, evidence, commits and return

From the implementation checkout, run each separately and retain actual exit
codes, cwd, revision, toolchain, raw output and known access limits:

```text
cargo test --locked -p oxurack --test module_installation
cargo test --locked -p oxurack --no-default-features --test module_installation
cargo test --locked -p oxurack-core
cargo test --locked -p oxurack
cargo test --locked --workspace --all-features
cargo test --locked --workspace --no-default-features
cargo clippy --locked --workspace --all-features --all-targets -- -D warnings
cargo fmt --all -- --check
RUSTDOCFLAGS=-Dwarnings cargo doc --locked --workspace --all-features --no-deps
RUSTDOCFLAGS=-Dwarnings cargo doc --locked -p oxurack --no-default-features --no-deps
make check  # preflight cargo-binstall and cargo-outdated; never make deps
cargo outdated --root-deps-only --ignore-external-rel  # separately capture exit/output
git diff --check
python3 /Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/artifacts/verify-planning.py
```

The planning verifier has an absolute path so it works from the isolated source
checkout. Its current assignment assertion still pins the held initial prompt,
so an `Assignment mismatch` on the authorized iteration01 plan is an expected
structural-check failure; preserve the raw result, do not edit the historical
verifier in CC's implementation packet, and flag it for CRC/CDC disposition before
I-05 can close. Continue unaffected implementation and validation. `make check` is not a
substitute for direct locked probes. The prior restricted sandbox failed MIDI
initialization tests; preserve the failed attempt, use normal tool escalation
when available, and record both results. Never call an inaccessible/unrun test
passed. Do not run ignored physical/timing tests. Global coverage is known
below 95% with profile warnings; Q-02/Q-03 own that separate repair, and this
slice does not lower the target. New behavior still needs the tests above.
Unexpected failures keep I-05 and acceptance open until fixed within scope or
explicitly dispositioned by CRC/CDC under the existing gate.

Only these source paths may be changed; omit untouched paths at commit:

```text
crates/oxurack-core/src/module.rs
crates/oxurack-core/src/error.rs
crates/oxurack/src/lib.rs
crates/oxurack/tests/module_installation.rs
crates/oxurack/tests/fixtures/behavior-installation.ron
crates/oxurack-mod-turingmachine/src/length.rs
crates/oxurack-mod-turingmachine/src/write_knob.rs
crates/oxurack-graph/src/lib.rs
```

The last three paths permit doc-comment changes only. No manifest, lockfile,
new dependency, scheduler/seed/TM/MIDI/device or unrelated source changes.
Use an isolated `codex/` implementation branch/worktree from the pin; do not
edit main or merge planning ancestry. `artifacts/` in this slice is the
supporting evidence home. Create `artifacts/validation-results.tsv`,
`artifacts/test-evidence.md` (including intake/readback), and individually
named `artifacts/logs/` files. At return, write `closing-report.md` with this
assignment and predecessor, exact source/planning states, six-row walk,
commands including failed/unrun attempts, artifact inventory, source diff,
scope-as-specified versus delivered and bubble-up to arc09/slice02–03. Update
the slice plan's assignment disposition only; CC does not write a verifier or
close its own ledger rows.

Commit source changes first, then planning evidence, with **explicit individual
paths**. Inspect each checkout's cached diff and name/status; use `git add --`
with only the touched literal files and `git commit --only -m "..." --` with
those same exact paths. No blanket staging, push, merge or release. Return to
Duncan for CRC the prompt path, source branch/worktree/commit, planning commit,
per-I-row evidence, actual command results, limitations and deviations. A prior
report or reading receipt is not implementation acceptance. Any changed
public behavior, collision/panic contract, schedule, source allowlist or
acceptance gate needs CRC/CDC reconciliation before affected work continues.
