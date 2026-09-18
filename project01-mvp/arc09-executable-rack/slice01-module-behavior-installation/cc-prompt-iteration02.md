# CC corrective assignment — prove live reinstall and clear unchanged gates

Project01 MVP / arc09 executable rack / slice01 module behavior installation.
2026-09-17. Execute this preserved iteration02 after the partially accepted
[`cc-prompt-iteration01.md`](cc-prompt-iteration01.md). Sofie/CRC reviews through
Duncan; CDC owns design. Three-Contributor Workflow, Expedited Mode. Use the
existing clean source worktree
`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`
on `codex/oxurack-module-behavior-installation` at
`d43078ea9c71bba10035699c9914bef3891f1d83`. Main is still `a7ce6e6`.
Planning intake is clean `dcab7931f8379380e0054c9b44b7b7d82b9be597`;
the assignment state is the planning commit that introduces this file. Verify
both worktrees and stop affected edits if either has drifted. The held initial
`cc-prompt.md` was never executed; executing this iteration consumes the first
corrective pass (1 of 5), not a reset of the slice.

**Outcome.** Close the three open rows without changing installed-module
production behavior: I-02 needs a genuine after-edit reinstall test; I-05 needs
the previously failing *full* all-targets Clippy and structural planning
verifier to pass; I-06 needs truthful command provenance and an explicit arc
bubble-up. I-01/I-03/I-04 were reproduced by CRC only for `d43078e`; preserve
and rerun their relevant behavior/docs on the new candidate. Directive05
authorizes the exact source/checker scope below. No API, scheduler, seed, MIDI,
device, coverage target, dependency or release change is authorized.

## Required reading and intake before dependent edits

Roots: planning = `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`;
source = the implementation worktree above; skills =
`/Users/oubiwann/.codex/skills`. Entries name exact paths relative to a root
and required extent. The planning content is at this assignment's commit;
source content is at `d43078e` until CC makes recorded changes. Read bounded
chunks completely and recover any truncated section contiguously.

| Order | Path / state | Required extent and purpose |
| --- | --- | --- |
| 1 | planning `project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration02.md`; `AGENTS.md`; `project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/slice-plan.md`; `project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/ledger.md` | Required-full each. Active identity, local scope, six-row state and handoff before source edits. |
| 2 | planning `project01-mvp/cdc-directive05.md`; `project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/crc-verification.md` | Required-full each. Approved corrections, candidate-specific partial verdict and exact review findings. |
| 3 | planning `project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/artifacts/execution-contract.md` | Required-section: opening authority paragraph and entire §1. Preserve the installed production contract; §2–4 are future work. |
| 4 | planning `project01-mvp/arc09-executable-rack/arc-plan.md`; `project01-mvp/project-plan.md` | Required-full arc plan; required-section project plan “Purpose and definition of done”, “Current roadmap — v3.0” through “Quality dispositions”, “CDC scope return — v3.4”, and “CRC corrective handoff — v3.5”. Preserve sequencing and inherited gates. |
| 5 | planning `project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration01.md` | Required-section: “Binding contract and recommended implementation sequence”, “Behavioral fixture and falsifiable oracles”, and “Validation, evidence, commits and return”. Carry forward the approved behavior and original gate matrix; its old assignment identity is historical. |
| 6 | planning `project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/closing-report.md`; same slice `artifacts/validation-results.tsv` and `artifacts/test-evidence.md` | Required-section: report “Exact worktrees and commits”, “I-01 through I-06 walk”, “Validation and limitations”, “Evidence inventory and handoff”; all TSV rows and test-evidence contract readback. Repair provenance without erasing attempts. |
| 7 | skills `collaboration-framework/knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md`; `collaboration-framework/knowledge/project-management/guides/README.md`; `collaboration-framework/knowledge/project-management/guides/03-planning-top-down.md` | Required-section: authoring guide “Required reading and CC intake” in full; README “Expedited Mode”; top-down guide “Issuing and executing an iteration”. Record fresh intake and preserve prompt/commit rules. |
| 8 | skills `rust-guidelines/SKILL.md`; `rust-guidelines/guides/15-cargo/07-lints-and-formatters.md`; `rust-guidelines/guides/04-ownership-borrowing.md`; `rust-guidelines/guides/11-anti-patterns.md` | Required-section: SKILL “Strength Indicators”; CG-L-01 (SHOULD), OB-19 (SHOULD), AP-01 (AVOID). Apply to full all-targets gate and the exact Vec test transfer; do not make library lint attributes. |
| 9 | skills `testing/guides/01-testing-discipline.md` and `testing/guides/03-validation-gates.md` | Required-full each. Real state transition, negative fixtures and separate gate evidence. |
| 10 | source `crates/oxurack/src/lib.rs`; `crates/oxurack/tests/module_installation.rs`; `crates/oxurack/tests/fixtures/behavior-installation.ron`; `crates/oxurack-core/src/module.rs`; `crates/oxurack-core/src/parameter.rs` | Required-section: `Oxurack` install/load/into_app/test-only world access, complete existing `#[cfg(test)] mod tests` helpers, integration fixture types and five tests, full RON fixture, `OxurackModule` hook/spawn and `ParameterRegistry` setter/getter signatures. Design the after-edit test against the actual ownership boundary. |
| 11 | source `crates/oxurack-graph/src/lib.rs`; `crates/oxurack-core/src/bridge.rs` | Required-section: `find_port` test helper around line 1036 and `test_midi_output_queue_push_and_drain` around line 481, including setup/assertions. Apply only the approved test-code edits. |
| 12 | planning `project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/artifacts/verify-planning.py` | Required-full. Preserve all structural assertions and link scan while replacing the obsolete assignment assertion. |
| 13 | planning `project01-mvp/cdc-directive04.md`; `project01-mvp/arc09-executable-rack/slice01-module-behavior-installation/cc-prompt.md`; `project01-mvp/crc-escalation05.md` | Reference-only history. Directive05 and this assignment govern the correction; consult these only to resolve a cited discrepancy. |

Append a concise fresh intake table and source-cited contract readback to the
existing `artifacts/test-evidence.md` before dependent edits. Record each
required item, revision, loaded extent, truncation recovery or gap; conditional
items explicitly if they arise. Readback must connect the after-edit oracle,
the `into_app(self)` ownership trap, unchanged full Clippy/checker contracts,
exact source scope, retained failed logs and future-slice exclusions. A file
existence/hash/heading check or another context's reading is not intake. Resolve
material conflicts through CRC before affected work; proceed after coherent
preflight without another approval request. Revalidate on a resumed context.

## Inspected baseline and applied guidance

At `d43078e`, `Oxurack::into_app(self)` consumes the rack. The integration
test calls `try_install` before conversion and before its parameter change;
moving that call below `into_app` cannot compile without a new API. The
existing `crates/oxurack/src/lib.rs` test module can use the private `rack.app`
or the existing `world()/world_mut()` test helpers while retaining `rack`.
It already imports `OxurackModule`, `ParameterSchema`, `ParameterValue` and
`PatchHandle`, has RON save/load tests and directly runs `FixedUpdate`; the
static two-instance RON fixture is in the umbrella crate's `tests/fixtures/`.
The existing integration counter provides the verified spawn/system/accessor
shape. The verifier currently checks 23 imports, 24 successors, historical
child counts, 7/4/6 ledger rows and local Markdown links, but its line-70
literal still requires the held initial prompt. The two Clippy diagnostics are
in test code at the exact locations named by directive05.

| Guide actually read | Applied decision and rationale | Proof |
| --- | --- | --- |
| Rust CG-L-01 SHOULD, `15-cargo/07-lints-and-formatters.md` | Keep `--all-targets` in the required command because it finds the two test-only warnings that default targets omit. | Full command exits 0; targeted/Make results remain separate evidence. |
| Rust OB-19 SHOULD, `04-ownership-borrowing.md` | Use `std::mem::take(&mut queue.commands)` in the bridge test to move the Vec while leaving a valid empty Vec. | Both command identities/order and source-empty assertions continue to pass. |
| Rust AP-01 AVOID, `11-anti-patterns.md` | Keep warnings-as-errors in the command; no `#![deny(warnings)]`, local allow, or new lint policy in production source. | Source diff limited to the two diagnosed test edits. |
| Testing discipline / validation gates | Use observed state after each transition and independent negative verifier inputs; do not infer the full gate from `make check`. | Exact `(0,0) → (4,6) → (8,9)` test, parser negatives and separate command results. |

## Binding corrective scope and ordered implementation route

1. **Clippy, graph test only.** In
   `crates/oxurack-graph/src/lib.rs`'s `find_port` helper, combine the existing
   `if let Some(port) = world.get::<Port>(child)` and inner name equality into
   the compiler-suggested let chain. Keep first-match return, child iteration
   and not-found panic identical. This is the sole non-comment exception to
   graph's earlier doc-only allowance; do not refactor graph production code.
2. **Clippy, bridge test only.** In
   `crates/oxurack-core/src/bridge.rs`'s
   `test_midi_output_queue_push_and_drain`, replace only the full
   `queue.commands.drain(..).collect()` with
   `std::mem::take(&mut queue.commands)`. Assert the two original commands in
   order, result length two and source empty after transfer. Do not touch
   production queue or capacity behavior.
3. **Live reinstall unit test.** Add a named
   `test_reinstall_after_parameter_edit_preserves_loaded_state` in the
   existing `#[cfg(test)] mod tests` of `crates/oxurack/src/lib.rs`. Add a
   small test-local `OxurackModule` with `KIND = "behavior_counter"`, an
   `increment: Int` schema, `StepState { increment: i64, total: i64 }`, a
   spawner that reads each RON override and inserts state, a real system on
   the supplied schedule that adds each instance's increment, and registered
   setter/getter for `increment`. Reuse the static
   `tests/fixtures/behavior-installation.ron` through public `install` and
   `load_patch`; identify `alpha`/`beta` via `PatchHandle`. Local type/helper
   names are CC's discretion. Do not add an App accessor or production hook.
4. **Temporal oracle.** Keep `rack` alive. Assert initial totals `(0,0)` and
   increments `(2,3)`; run direct `FixedUpdate` twice through its existing
   test-only world/App access and assert `(4,6)`. Through
   `ParameterRegistry::set_parameter`, set alpha increment to 4; read back
   alpha 4 and beta 3 with the registered getters. End all resource/world
   borrows, *then* call `rack.try_install::<Fixture>()`. Immediately verify
   totals remain `(4,6)` and getter values remain `(4,3)`. Run one more direct
   `FixedUpdate`; assert `(8,9)` and one increment per instance. These exact
   observations reject state reset, accessor replacement and duplicate system
   registration. Keep the existing collision, independent-rack and public
   `into_app` integration tests. In
   `crates/oxurack/tests/module_installation.rs`, rename only the earlier
   pre-edit reinstall test to
   `same_type_reinstall_before_edit_keeps_system_single_and_accessors_registered`;
   do not weaken
   its assertions or turn it into the after-edit oracle.
5. **Assignment-aware checker.** Edit only the assignment assertion and
   necessary output wording in this slice's `artifacts/verify-planning.py`.
   Keep every existing import/carryover/ledger/link check running. Add a
   compact `check_current_assignment(plan_text: str, slice_dir: Path)` helper
   or equivalent; `main()` calls it on the real slice plan. Accepted metadata
   format is the single, exact line
   `Current assignment: ` followed by a backticked project-relative path,
   and the existing `## Assignment history` table whose `Prompt` cell is the
   prompt basename. Count all lines beginning `Current assignment:` and
   require exactly one. Require the path to match
   `arc09-executable-rack/slice01-module-behavior-installation/cc-prompt.md`
   or the same prefix plus `cc-prompt-iterationNN.md` (two digits); reject
   absolute/parent/sibling paths. Require its resolved regular file to stay
   inside this slice and one matching history row. Require the initial
   `cc-prompt.md` and issued `cc-prompt-iteration01.md` to exist; when
   iteration02 is current, its file must exist too. Do not hardcode
   iteration02 as forever current or infer current from the largest suffix.
6. **Checker negatives without real-tree mutations.** Exercise the extracted
   helper with temporary slice copies/stubs and record the exact script,
   inputs and outputs in `artifacts/iteration02-verifier-fixtures.md`:
   missing current field, nonexistent current file, field/history mismatch,
   duplicate/ambiguous current fields, out-of-slice path and a valid alternate
   numbered iteration with matching file/history. Each bad case must fail;
   the valid alternate must pass. Then run the *complete* real checker to
   observe all original structural checks and links. Retain prior failed
   verifier logs; do not assert they passed retroactively.
7. **Provenance and bubble-up.** Correct exactly the ten nonexistent `cwd`
   cells in `artifacts/validation-results.tsv` to the actual
   `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`
   path, based on the raw logs and CC's original execution record. Preserve
   every prior command, exit code, failure, retry and log path. Keep
   `source_revision = a7ce6e61536ca96b6aeb2227ee06e430ed8df162` as
   the HEAD at those historical executions;
   add explicit dirty/uncommitted `source_state` to distinguish the then-current
   working tree from that baseline HEAD. Claim byte identity with `d43078e`
   only if independently supportable; otherwise label that mapping as CC
   attestation/uncertain. Document the correction basis in
   `artifacts/test-evidence.md`, without rewriting raw logs. Append new rows
   for new runs with their actual HEAD, dirty/clean state and named logs.
   Append an iteration02 section to `closing-report.md` with all six row
   dispositions and a **Bubble-up to arc09** answering delivered capability,
   newly found gaps, and scope-as-specified versus delivered/deferred. Keep
   sample/seed/MIDI and Q-02/Q-03 with their existing successors.

Borrow/order sketch for step 4, **illustrative and uncompiled**:

```text
let handle = rack.world().resource::<PatchHandle>().clone();
let alpha = handle.modules["alpha"]; let beta = handle.modules["beta"];
rack.world_mut().run_schedule(FixedUpdate); // twice, then assert 4/6
{
  let world = rack.world_mut();
  world.resource_scope::<ParameterRegistry, _>(|world, registry| {
    registry.set_parameter(world, alpha, &kind, "increment", Int(4))?;
    assert getter(alpha)==Int(4) and getter(beta)==Int(3);
  });
} // every world/resource borrow ends before try_install
rack.try_install::<Fixture>()?;
assert totals == (4,6) and getters == (4,3);
rack.world_mut().run_schedule(FixedUpdate);
assert totals == (8,9);
```

Do not copy the sketch as compiled Rust. A local helper is fine, but it must
preserve the public load path, the before/after ordering and the observable
assertions. The parser may use `re.fullmatch`, a bounded Markdown-row parser
and `Path.resolve`; these private details are local discretion if the same
positive and negative inputs yield the specified results. Unexpected need for
a public API, new file outside scope, changed gate or behavior is a stop
condition: send the exact conflict to CRC for CDC routing before affected work.

## Files, validation, evidence and return

**Corrective source allowlist — exactly these four paths, if changed:**

```text
crates/oxurack-graph/src/lib.rs                 # find_port test helper only
crates/oxurack-core/src/bridge.rs              # queue test only
crates/oxurack/src/lib.rs                      # existing test module only
crates/oxurack/tests/module_installation.rs    # one misleading test name only
```

The earlier I-01/I-03/I-04 verdict is candidate-specific to `d43078e`;
do not replay or edit its other production/doc files. Planning edits may touch
this slice's `artifacts/verify-planning.py`, `artifacts/validation-results.tsv`,
`artifacts/test-evidence.md`, new individually named iteration02 logs and
`artifacts/iteration02-verifier-fixtures.md`, `closing-report.md`, and the
slice plan's assignment disposition. Preserve all issued prompts/directives,
old raw logs, CRC verification and ledger statuses; CC does not close its
own rows. Do not create a separate planning tree.

Run and log **each** command separately from the source checkout, with cwd,
HEAD, dirty/clean state, toolchain, actual exit and raw output. Run focused
checks first, then the full unchanged matrix:

```text
cargo test --locked -p oxurack --lib test_reinstall_after_parameter_edit_preserves_loaded_state
cargo test --locked -p oxurack --no-default-features --lib test_reinstall_after_parameter_edit_preserves_loaded_state
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

Also run the bounded checker negatives from step 6 and preserve their evidence.
`make check` is not the all-targets Clippy gate. Restricted MIDI/DNS failures,
if encountered, must be recorded before authorized host reruns; keep both
attempts and their actual status. No ignored physical/timing tests, dependency
upgrade, coverage-target waiver, or inference of real MIDI acceptance. A
failing required gate keeps I-05 and the slice open for CRC review. Do not
call a target passed because a wrapper or narrower command succeeded.

Commit the source correction first, then planning evidence, with `git add --`
and `git commit --only` naming each changed literal path after inspecting each
cached diff/name/status. Omit untouched allowlisted files; preserve unrelated
changes. The first validation rows run on dirty precommit state; after the
source commit, add clean-candidate targeted/full gate rows sufficient to
support a candidate-specific CRC review. Before closing the packet, capture
the exact source/planning commits and clean states; a metadata follow-up commit
may record its predecessor hash without rewriting raw outputs.

Return through Duncan the active prompt path, source branch/worktree/commit,
planning evidence commit, six-row proposed walk, all required command statuses,
checker negative results, provenance correction basis and scope-as-specified
versus delivered. Update `closing-report.md` as a preserved later attempt,
without erasing iteration01 failures or claiming CRC acceptance. CRC will
independently verify I-02/I-05/I-06 and affected I-01/I-03/I-04 regressions;
slice02 remains held until all six rows and the bubble-up close.
