# CC initial assignment — deterministic musical Clock

## Assignment and authority

Execute implementation for project01-mvp / arc10-turing-machine-midi /
slice01-musical-clock. Exact assignment:
`arc10-turing-machine-midi/slice01-musical-clock/cc-prompt.md`.
CDC authored this first slice; CRC independently reviews your return through
Duncan. Three contributors, Expedited Mode; initial pass, zero corrective passes.
Arc09 is independently accepted at `37deeace90e7fe2dff633ada9d58ef9051577c3d`.
Use source `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`
on `codex/oxurack-module-behavior-installation`; planning is the adjacent
`planning` worktree, branch planning. Main remains untouched. Check HEAD, worktree
status and this assignment against slice-plan.md before edits. If source differs,
report drift to CRC before dependent work; do not silently repin the baseline.
The planning issue is the commit containing directive07 and this initial prompt.

Deliver a standalone 1 ms Clock and optional installed Gate source, using the
existing sample pipeline. All six C rows are assigned. No TM, RT, MIDI sink,
core change, global SetBpm, UI or additional transport authority in this slice.

## Required reading and intake

Paths below use P=/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp,
S=the source checkout above, F=/Users/oubiwann/.codex/skills/collaboration-framework,
R=/Users/oubiwann/.codex/skills/rust-guidelines. Expand these aliases explicitly.
Current issue documents mean this prompt's planning commit; source means 37deeace.
Installed guidance is the installed version at execution: record its version and
resolve conflicts with CDC rather than changing the contract. A file outside
these shared local roots must be supplied before dependent work if unavailable.

| Order | Path / source state | Required extent | Purpose |
| --- | --- | --- | --- |
| 1 | P/arc10-turing-machine-midi/slice01-musical-clock/{cc-prompt.md,slice-plan.md,ledger.md} | Required-full, each of three files | Current scope, exact algorithm/lifecycle/API and six criteria |
| 2 | P/cdc-directive07.md; P/arc10-turing-machine-midi/arc-plan.md | Required-full | Authority, current arc and subsequent-slice boundary |
| 3 | P/arc10-turing-machine-midi/design.md | Required-section: introduction and section 1; sections 2–5 reference-only for this assignment | Sample/tempo ownership; future implementation excluded |
| 4 | P/project-plan.md | Required-section: Current roadmap table and latest CDC Arc09 acceptance / Arc10 opening amendment | Priority and retained gates; earlier assignment history is provenance |
| 5 | F/SKILL.md; F/knowledge/project-management/guides/README.md | Required-full, follow relevant execution/Expedited routes | Workflow and exact commit/return rules |
| 6 | F/knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md | Required-section: Required reading and CC intake, through Review evidence at the right strength | Record real reading and a source-cited readback before editing |
| 7 | R/guides/02-api-design.md; R/guides/05-type-design.md; R/guides/13-documentation.md | Required-section: API-23 and API-39; TD-05; DC-05, complete sections | Boundary validation, private invariants and explicit errors |
| 8 | F/knowledge/testing/guides/01-testing-discipline.md | Required-full | Meaningful behavior and complete gates |
| 9 | S/crates/oxurack-core/src/module.rs | Required-section: OxurackModule trait, ModuleSpawnContext and spawn_module_entity definitions | Exact hook/spawn/parameter API |
| 10 | S/crates/oxurack-core/src/{parameter.rs,error.rs,port.rs} | Required-section: ParameterValue/Schema/Setter/Getter/Registry definitions and impl; CoreError; spawn_port_on_module and CurrentValue | Error and ownership integration |
| 11 | S/crates/oxurack-core/src/tick.rs; S/crates/oxurack-core/src/patch.rs | Required-section: TickPhase/SampleTime/running gate; apply_patch_to_world function in full | Current sample and aggregate spawn behavior |
| 12 | S/crates/oxurack/src/lib.rs | Required-section: production Oxurack impl in full | Actual install/load/driver/stop API and consuming into_app |
| 13 | S/crates/oxurack/tests/sample_pipeline.rs | Required-section: imports/types; SampleProducer/SampleRelay/SampleSink implementations; fixture_path through live_reentry_has_no_zero_elapsed_catch_up_sample (complete definitions) | Working fixture and controlled-live construction; no invented APIs |
| 14 | S/Cargo.toml; S/crates/oxurack/Cargo.toml; S/crates/oxurack-mod-turingmachine/Cargo.toml | Required-full | Workspace wildcard and versions |
| 14a | S/Makefile | Required-section: build, test, lint, format, common-checks, check, check-deps and ensure-binstall targets in full | Native validation and dependency-tool prerequisites |
| 15 | S/Cargo.lock | Required-data: extract all package name/version/source triples before/after and full added/changed entries | No dependency upgrade; Clock package and new edges only |
| 16 | R/SKILL.md and relevant additional guide chapters | Conditional before work requiring further Rust guidance; entrypoint required on first Rust-skill activation | Follow skill routing without changing repository toolchain |

Record actual loaded extents/revisions, recovered truncations, structured query
coverage (separate from prose reading), conditional decisions and a concise
source-cited contract readback in artifacts/intake.md before implementation.
Summaries/grep/hash receipts are not full reading. Bound outputs and recover
omitted portions. Explain the interaction that a high Gate during a rate edit
must include one low sample before the next rising edge; also explain why the
module's bpm does not come from Patch.bpm. Resolve missing/conflicting required
material before affected edits, then proceed without another routine approval.

## Source-grounded implementation route

The binding standalone API, validation order, numeric formula, edits and adapter
contract are in slice-plan.md sections Binding numeric and public API contract,
Binding edits and lifecycle, and Rack adapter and parameter boundary. Read them
fully; this spine supplements them and cannot replace their exact values.

1. Add `crates/oxurack-mod-clock/Cargo.toml`, src/lib.rs, src/clock.rs and src/rack.rs.
   Root workspace already includes crates/*, so do not edit root Cargo.toml.
   Match neighboring package metadata. Default features=[]; rack enables optional
   oxurack-core (path ../oxurack-core, version0.1.0), bevy_app/bevy_ecs (0.18,
   default-features=false, std). thiserror2 is already locked. No other new library.
   lib.rs exports Clock/ClockConfig/ClockError; export ClockModule only with rack.
2. Implement canonical immutable Config validation before mutable state. Unit
   Clock stores bounded integer phase, high_remaining, fresh and restart_pending.
   Use the plan's D/R accumulator; no wall clock, growing elapsed counter,
   floating modulo, repeated rounded interval or sample-thread allocation.
   All bounds are checked at Config creation, not on the hot path. No sleeps,
   threads, locks or Drop work. Public API shapes and error variants are binding;
   private helper names/layout are local discretion.
3. Implement ClockModule using actual core trait helpers. Decode the complete
   unordered override map into a candidate before spawning anything; schema
   defaults and getters agree with canonical config. Core's patch loader calls
   spawn with overrides and then emits ParameterChanged messages; it does NOT
   apply your setters to initialize the engine. Do not assume loader magic.
   Register one Produce system querying all private Clock states and port handles
   on the supplied schedule. Write Value::Gate to CurrentValue. Reinstall relies
   on rack idempotence; Clock must not clear module-owned state in its hook.
4. Parameter setters copy the existing config, parse the strict variant/range,
   validate the complete replacement, then commit. Invalid input leaves all
   state, pending restart and output unchanged. Unknown key/type/overlap/missing
   entity outcomes follow the plan; no unwrap on user input or partial spawning.
   Keep parameter registry borrowing scoped; follow resource_scope or copy the
   trait's setter pointer into a local before mutably borrowing World.
5. Add umbrella dev-dependency oxurack-mod-clock with rack enabled and its real
   integration test/fixture. The new crate has no normal umbrella dependency;
   avoid a runtime cycle or default Bevy dependency. Tests inside rack.rs may
   directly inspect private state and use App+CorePlugin for parameter edits.
   Integration tests use install/load_patch/start/advance_samples/stop and end
   with into_app to inspect traces; do not reuse the consumed wrapper. Install
   a fixture consumer module/hook to record Consume/Observe without public World
   access. Test setters with App/World in rack.rs, where access is available;
   do not add an umbrella accessor or expose private state to rescue a test.
6. Add crate README and crate/item rustdoc describing millisecond units, 0.001
   BPM resolution, first pulse, low-gap rule, restart/edit/no-op semantics,
   standalone/rack features and explicit per-module tempo. Include executable
   standalone doctest and real RON example. Patch metadata tempo must not be
   presented as a working global control.

## Applied engineering guidance

| Read guide | Applied decision and proof |
| --- | --- |
| Rust API-39 (MUST) | Validate all override values and cross-field invariant before spawn/commit; invalid spawn has no new entities, invalid setter preserves subsequent trace |
| Rust TD-05 (MUST) | Private ClockConfig and phase state prevent callers bypassing low-gap/finite bounds; setters use validated replacement |
| Rust API-23 (MUST) | Concrete ClockError plus existing CoreError boundary variants; tests match variants/instance/parameter, not fragile reason prose |
| Rust DC-05 (MUST) | try_new and fallible adapter operations document precise errors and mutation boundaries; strict rustdoc + doctest |
| Testing discipline, Test Behavior And Contracts | Exact fractional/edited/paused traces and real RON cable path reject plausible alternative algorithms |

These are author-applied decisions, not a request to invent a design after
reading guidelines. Sketches in the plan are algorithms, not compiled drop-ins.

## Behavioral oracles — derive expectations independently

- C-01 defaults: high at 0..9, low 10..499, high 500..509, low to999, high
  1000..1009. At 120 BPM/4 PPQN/width3 rising samples are 0,125,250,375,500.
  At 137 BPM/4 PPQN/width3 the first eight rising samples are
  **0,110,219,329,438,548,657,767**. Assert high and low samples, not just counts.
  The fractional list is ceil(k*60000/(137*4)); repeated ceil/floor periods fails.
  Boundary 1000 BPM/30 PPQN/width1 gives 0,2,4,6; width2 is PulseOverlap.
  NaN/infinities/nonpositive/out-of-range BPM, PPQN0/97, width0/1001 fail exact
  enum categories. Canonical 120.0004 becomes120.000; 120.0006 becomes120.001.
- C-02: generate default samples0,1,2 (high), change bpm to240, then n3 must
  be low and n4..13 high; n14 low, next rising254. Equal canonical config at n3
  produces the original continuation (no low separator). Several edits before
  execution produce one separator; changing back to original still has a pending
  separator; equal setter after a real change cannot cancel it. Invalid edit
  while restart pending must preserve it. Compare continued traces to a cloned
  control state in private tests or independently replayed input sequence.
- C-03: actual RON loads two Clock instances with different rates and two fixture
  consumers connected by Gate cables. Observe exact named pulse/input samples
  and Consume rising-edge counts (initial previous=false), with one fixture's
  long high pulse producing just one edge. Drive >1100 samples manually in groups
  and through controlled Live `TimeUpdateStrategy::FixedTimesteps`; compare full
  named traces, reversed spawn ordering and unrelated entity allocation.
  Install twice before load; reinstall after progress; stop while high, make
  repeated requests (including zero), resume: continuation must equal uninterrupted
  reference after removing stopped requests. Keep Oxurack until all wrapper
  actions finish, then inspect App. Also use App + CorePlugin (Update) and call the hook with Update.intern(),
  then execute Update to prove it respects that label; the umbrella fixture
  separately proves FixedUpdate. These are the two existing CorePlugin APIs;
  no custom-schedule builder or new core API is needed.
- C-04: read each getter after spawn and valid edit; vary map insertion order.
  Use aggregate overrides bpm=1000,ppqn=30,width=1: valid together despite the
  default width being invalid at that rate. Prove invalid aggregate spawn creates
  no Clock entity/port; unknown keys sorted, wrong variants reject, Float NaN
  tested through direct trait spawn/setter (RON loader may reject it earlier).
  Setter overlap reports the edited parameter and leaves phase/state/output
  intact. Missing entity returns UnknownModule. A RON patch bpm90 with omitted
  Clock bpm must still run at120; explicit Clock90 must run at90. This rejects
  accidental metadata authority. Whole-patch rollback is not asserted.
- C-05: default Clock normal dependency tree excludes Bevy/core; rack mode
  compiles and runs. Graph on/off umbrella tests, old install/sample/seed/stop
  regressions and full workspace gates pass. Preserve pre-existing ignored tests
  and future-incompatibility notice; do not add ignores or lint suppression.
- C-06: all opening/closing six rows represented; source diff equals explicit
  fence; ledger evidence points to actual commands/tests. The provided artifacts/verify-planning.py checker
  verifies assignment/rows/files/links and rejects in-memory missing C-06, wrong
  assignment and invalid row-status controls using the same predicate. You may extend the supplied checker within its allowed path as evidence grows;
  retain the initial author checks at this issue commit and record changed
  properties. Prior Arc09 checkers are historical scoped evidence, not Clock validation.

## Validation and evidence

From source checkout, log each command, exit, toolchain, feature and source revision
under this slice's artifacts/logs/ and enumerate artifacts/results.tsv:

```sh
cargo test --locked -p oxurack-mod-clock
cargo test --locked -p oxurack-mod-clock --features rack
cargo tree --locked -p oxurack-mod-clock --edges normal
cargo test --locked -p oxurack
cargo test --locked -p oxurack --no-default-features
cargo test --locked -p oxurack-core
cargo test --locked --workspace --all-features
cargo test --locked --workspace --no-default-features
cargo clippy --locked --workspace --all-features --all-targets -- -D warnings
cargo fmt --all -- --check
RUSTDOCFLAGS='-Dwarnings' cargo doc --locked --workspace --all-features --no-deps
RUSTDOCFLAGS='-Dwarnings' cargo doc --locked -p oxurack --no-default-features --no-deps
make check
cargo outdated --root-deps-only --ignore-external-rel
git diff --check
```

Adding the workspace package requires refreshing Cargo.lock once before locked
checks; use existing versions/offline resolution, review all package/version
triples and do not upgrade transitive dependencies. Include strict standalone
Clock docs too. Host access is needed for existing MIDI-init and registry gates;
retain restricted failures and exact host retry if encountered. Arc09 review07
passed these existing gates; three ignored RT tests and block0.1.6 notice remain.
No new hardware/coverage claim from software gates. Run current planning checker
from planning, both baseline and negative cases. Record every blocked/unrun
required check; do not weaken commands or call shell-wrapper exit success proof.

## Explicit scope and return

Allowed source files only:

- Cargo.lock (new package/edges only)
- crates/oxurack-mod-clock/Cargo.toml
- crates/oxurack-mod-clock/README.md
- crates/oxurack-mod-clock/src/lib.rs
- crates/oxurack-mod-clock/src/clock.rs
- crates/oxurack-mod-clock/src/rack.rs
- crates/oxurack/Cargo.toml (Clock dev-dependency only)
- crates/oxurack/tests/clock.rs
- crates/oxurack/tests/fixtures/clock.ron

Allowed planning writes only this slice's ledger.md evidence/status,
closing-report.md, artifacts/intake.md, artifacts/results.tsv,
artifacts/test-evidence.md, artifacts/verify-planning.py and artifacts/logs/ files
enumerated in the return. Preserve issued prompt and slice plan. CRC owns close
and parent/status updates. Do not copy source targets/binaries into evidence.

After self-review and gates, inspect explicit changed paths and cached diff;
stage exact files (enumerate individual logs, no git add .), then commit --only
those exact paths separately in source and planning. CC may commit before CRC
acceptance. Do not merge, push, publish or touch main. Report source and planning
hashes, exact assignment, six-row walk, intake/readback link, command results
(including failures), dependency diff, artifacts, deviations and bubble-up.
Your result remains proposed-done until CRC reproduces it. A necessary extra
source path, different API/timing/lifecycle contract or failed gate needs CRC/CDC
resolution through Duncan before scope expands; private helper choices do not.
