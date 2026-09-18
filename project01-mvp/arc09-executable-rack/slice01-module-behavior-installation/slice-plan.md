# Slice01 — module behavior installation

Version 1.5, 2026-09-17. Status: open; corrective iteration02 ready for CC relay.
Workflow: CDC design / CRC independent review / CC implementation; Expedited Mode.
Source baseline: `a7ce6e61536ca96b6aeb2227ee06e430ed8df162` (main).
Planning predecessor: `390d2aa2c3449f8a6e4bbaff3df6c64d821ce4ea`; authority is the
commit introducing this open set and [directive03](../../cdc-directive03.md).
Depends on accepted arc08/slice01 observations, not on fictitious product acceptance.

## Outcome and design

Calling rack installation registers module behavior as well as metadata exactly
once per type, supports many patch instances, and rejects kind collisions without
mutation. Existing modules and install chaining remain compatible. Follow section 1
of [execution-contract.md](artifacts/execution-contract.md) exactly: default
register_systems hook taking App and InternedScheduleLabel; try_install; per-rack
ModuleKind/TypeId bookkeeping; CoreError::ModuleKindConflict; legacy convenience
panic only for a programmer's conflicting kind. Preserve all other sections as
future designs; they are not implementation scope for this slice.

In scope: these APIs, behavior tests through the real rack/patch path, accurate
API docs, and comment-only repairs of the 11 existing rustdoc diagnostics recorded
in [CDC's log](../../artifacts/cdc-review03-rustdoc.log). CoreError uses its existing
thiserror dependency; no new dependency or Cargo change is needed.
Out of scope: new scheduler or phase semantics, seed changes, module adapter,
Clock/LFO/Scope/MIDI-out, device access beyond existing test prerequisites, full
runtime API, coverage hardening, dependency upgrades, merges or releases.

## Implementation and verification steps

1. Confirm pinned source and clean/known worktree state. Create an isolated source
   branch/worktree under the repository convention (suggested branch
   codex/oxurack-module-behavior-installation); do not change main or merge planning
   ancestry. Record actual path and commit. Planning records stay in the existing
   planning worktree. Inspect any applicable AGENTS.md before editing.
2. Add the default hook and conflict error, implement try_install bookkeeping and
   delegate the existing install convenience. Check the conflict before any
   registry/accessor/system mutation. No per-instance hook call during patch load.
3. Build a test module with a real Produce system and configurable per-instance
   state. Load a file fixture containing two instances through Oxurack::load_patch,
   consume into App for direct schedule control, and inspect actual output/state.
   A hook-call counter alone does not demonstrate execution.
4. Test repeated installation and repeated parameter edits, a conflicting module
   with observably different spawn/setter/getter/system functions, independent
   racks, legacy no-op-hook modules, and graph-enabled/disabled builds. Tests must
   fail for duplicate systems, replacing the original kind, wrong-schedule
   registration, or a hook invoked on each spawn. Direct FixedUpdate execution
   here proves schedule wiring only, not future exact-time sample semantics.
5. Fix only doc comments in length.rs/write_knob.rs/graph lib for the observed
   links. No lint allowances; no deleting useful docs or adding a graph→umbrella
   dependency to resolve the cycle. The graph's reference to its dependent
   umbrella method may be prose/code instead of an invalid intra-doc link.
6. Run the gates below, self-review exact source/planning diff, account for every
   I row, commit explicit paths and return a proposed-done packet through Duncan.

## Required checks

Record command, cwd, revision, toolchain, actual exit status and raw output for
all attempts. Negative controls are the behavioral cases above, not blanket
mutation testing or numerical coverage inflation. Required gates:

- `cargo test --locked -p oxurack --test module_installation`
- `cargo test --locked -p oxurack --no-default-features --test module_installation`
- `cargo test --locked -p oxurack-core` and `cargo test --locked -p oxurack`
- `cargo test --locked --workspace --all-features`
- `cargo test --locked --workspace --no-default-features`
- `cargo clippy --locked --workspace --all-features --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `RUSTDOCFLAGS=-Dwarnings cargo doc --locked --workspace --all-features --no-deps`
- `RUSTDOCFLAGS=-Dwarnings cargo doc --locked -p oxurack --no-default-features --no-deps`
- Native `make check` after tool preflight; retain its result and the separately
  captured `cargo outdated --root-deps-only --ignore-external-rel` exit/output.
  Never run make deps; available upgrades are not permission to update dependencies.
- `git diff --check`; planning verifier from this slice's artifacts.

Use normal sandbox escalation for existing MIDI initialization tests if restricted
runs fail, preserving both results. Never mark an inaccessible test passed. Do not
run ignored physical/timing tests or claim the historical 95% gate is satisfied.
Known global coverage is below target with unresolved profile warnings (Q-02/Q-03);
full coverage remediation belongs to those explicit successors. New behavior still
requires all targeted positive/negative tests. Unexpected failures cannot be waived
by this plan: fix within scope or report/escalate with affected rows. A required
blocked or failing gate keeps I-05 and slice acceptance open until reproduced or
explicitly dispositioned through the governing review/Operator contract.

## Evidence and exact file scope

Default supporting home: this slice's `artifacts/`. Design/checker inputs remain
preserved. CC creates `artifacts/validation-results.tsv`, `artifacts/test-evidence.md`,
and individually named `artifacts/logs/` outputs. Results table includes command,
cwd, source revision, toolchain, exit code, result and log path. Include exact
source and planning commits and explicit file lists in `closing-report.md`.

Source allowlist (literal repository-relative paths):

```text
crates/oxurack-core/src/module.rs
crates/oxurack-core/src/error.rs
crates/oxurack/src/lib.rs
crates/oxurack/tests/module_installation.rs
crates/oxurack/tests/fixtures/behavior-installation.ron
crates/oxurack-mod-turingmachine/src/length.rs
crates/oxurack-mod-turingmachine/src/write_knob.rs
crates/oxurack-graph/src/lib.rs
crates/oxurack-core/src/bridge.rs
```

The two Turing Machine paths are doc-comment-only. Graph lib.rs keeps its
doc-comment boundary except the exact `find_port` test-helper edit authorized
by directive05; bridge.rs permits only that directive's queue test edit.
Inline tests in the first three original source files are allowed. No
manifest/lockfile/new dependency edits. If another path is
necessary, explain why to CRC before expanding this allowlist; design changes go
to CDC. The RON fixture is source test data; logs are planning artifacts.
Planning allowlist: this slice's ledger.md, slice-plan.md (assignment status only),
closing-report.md, the two named evidence files and each separately enumerated log.
Directive05 also permits CC's narrow assignment-check update to
artifacts/verify-planning.py and named iteration02 verifier-fixture evidence.
Do not edit issued cc-prompt.md, execution-contract.md or another contributor's
verification. No close or verifier placeholders at opening.

## Assignment history

Current assignment: `arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration02.md`

This [corrective iteration02](cc-prompt-iteration02.md) is issued under
[directive05](../../cdc-directive05.md). The last executed assignment was
[iteration01](cc-prompt-iteration01.md), source `d43078e`, with
[CRC partial review](crc-verification.md): I-02/I-05/I-06 open. The original
`cc-prompt.md` remains held and unexecuted.

| Date | Prompt | Predecessor | Reason | Disposition |
| --- | --- | --- | --- | --- |
| 2026-09-17 | cc-prompt.md | none | CDC response to escalation03; first TM prerequisite implementation | held by directive04; replacement preparation assigned to CRC |
| 2026-09-17 | cc-prompt-iteration01.md | cc-prompt.md | Newly installed guide requires source-grounded recipe, explicit test oracles and bounded CC intake | CC proposed-done at `d43078e`; CRC partial review, original unexecuted |
| 2026-09-17 | cc-prompt-iteration02.md | cc-prompt-iteration01.md | CDC-approved after-edit reinstall test, exact Clippy/checker repairs and evidence correction | issued for CC relay; not yet executed, 0 of 5 corrective passes used |

Executed refinements: 0 of 5. Preserve later cc-prompt-iterationNN.md files.
CC owns execution; CRC checks exact commits, reproduces I-01–I-06 and records
crc-verification.md. On acceptance CRC may detail arc09/slice02 under the approved
design. New arc, altered contract or unresolved failure returns through Duncan.

## Version History

- v1.0 — 2026-09-17: First implementation open set after the accepted baseline;
  bounded registration and doc repair, explicit negatives, no seed/timing scope creep.

## Authoring amendment — 2026-09-17

[Directive04](../../cdc-directive04.md) authorizes CRC to expand the existing design
using the installed implementation-prompt-authoring guide/template. CRC inspected
clean source `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`, the locked Bevy 0.18.1
schedule source, the eleven-diagnostic rustdoc log, and the directive03/04 packet.
No intervening CC execution packet or source change was present. The replacement's
“Inspected source and applied engineering guidance” records the actual symbols,
Rust guide chapters/strengths, rationale and checks. Its “Binding contract and
recommended implementation sequence” and “Behavioral fixture and falsifiable
oracles” supply the ordered route and exact positive/negative observations.
Initial prompt/design are preserved; I-01–I-06, eight source paths and all
validation requirements remain unchanged. Executed refinements remain 0 of 5.

CRC author readiness self-review, with pointers into the issued replacement:

| Check | Result and evidence pointer |
| --- | --- |
| Source-grounded | Ready. “Inspected source and applied engineering guidance” identifies constructor/install/patch paths, parameter registry, CorePluginFixed, feature boundary and locked Bevy APIs; “Required reading and intake” makes CC recheck the pin. |
| Design-complete | Ready. “Binding contract and recommended implementation sequence” fixes public signatures, per-rack identity, collision ordering, idempotence, hook-panic and legacy convenience policy; private helper names alone remain discretionary. |
| Guideline-applied | Ready. “Inspected source and applied engineering guidance” maps Rust trait/borrow/error/API/docs rules and testing guidance to decisions, rationale and proof, with exact chapters and strengths; the manifest assigns CC the bounded relevant sections. |
| Executable | Ready. The ordered seven-step recipe names the eight approved paths and concrete symbols, including borrow end before App hook; its pseudocode is expressly uncompiled, and the source allowlist plus return contract is explicit. |
| Falsifiable | Ready. “Behavioral fixture and falsifiable oracles” derives the `(0,0) → (2,3) → (4,6) → (8,9)` two-instance trace, conflict preservation, independent racks, legacy module and feature matrix; “Validation, evidence, commits and return” retains every gate. |
| Coherent and portable | Ready. “Required reading and intake” gives exact accessible roots, bounded extents, revision/readback and evidence home. The contract and exclusions align with execution-contract §1, directive04, this plan and I-01–I-06; source scope and future slices stay separate. |

These are author checks, not independent CRC acceptance of CC's future work.
The expected first execution still requires CC's actual readback, implementation,
tests and proposed-done evidence before CRC reproduction.
The preserved planning verifier currently asserts that `cc-prompt.md` is the
current assignment; it therefore reports `Assignment mismatch` after this
authorized replacement. Its historical source stays intact. CC must capture that
result as a known structural-check limitation, and I-05 cannot close until
CRC/CDC explicitly dispositions it under the existing gate. It does not block
the unaffected source implementation or behavior tests.

- v1.1 — 2026-09-17: Operator requested guide-based CRC prompt authoring; hold the
  earlier handoff for a detailed preserved replacement without changing design scope.
- v1.2 — 2026-09-17: Issued preserved iteration01 replacement with source/guidance
  reconnaissance and six-part author readiness; the first prompt was not executed.

## CRC review and design handoff — v1.3, 2026-09-17

[CRC verification](crc-verification.md) independently reproduced I-01, I-03
and I-04 at source `d43078e`. I-02 needs an after-edit live-state reinstall
test, I-05 has two baseline all-targets Clippy findings and an obsolete
planning-verifier assignment assertion, and I-06 needs validation-table
provenance and explicit bubble-up repair. The full gate remains unchanged.
CRC requests exact source/checker scope from CDC in project-level
[crc-escalation05.md](../../crc-escalation05.md); no new CC prompt is issued
until a response. Arc09/slice02 stays held.

| Date | Exchange | Predecessor | Status | Affected rows / next owner |
| --- | --- | --- | --- | --- |
| 2026-09-17 | crc-escalation05.md | cdc-directive04.md and executed iteration01 | awaiting CDC through Duncan | I-02/I-05/I-06 open; CDC decides D1–D3, CRC then prepares the next prompt |

Executed corrective refinements remain 0 of 5; iteration01 was the first
execution of the slice after the held initial prompt. A later executed
iteration02 would consume the first corrective pass.

- v1.3 — 2026-09-17: Recorded independent partial CRC verdict and design
  escalation; retained the six-row denominator and all original quality gates.

## CDC decision on escalation05 — v1.4, 2026-09-17

[Directive05](../../cdc-directive05.md) approves D1/D2/D3. Current executed
assignment remains iteration01, partially verified at `d43078e`; iteration02 is
not yet authored/issued. CRC now prepares it under directive04's six readiness
checks. Earlier awaiting-CDC statements above are superseded by this decision.

Source scope amendment: add `crates/oxurack-core/src/bridge.rs` for only the
queue test's drain/collect-to-mem::take edit; permit only the `find_port` test
helper's collapsed conditional in graph lib.rs in addition to its doc repairs.
Original eight paths retain their other bounds. I-02 may add a test-local fixture
and after-edit reinstall test in umbrella lib.rs's existing test module, retaining
the rack across direct schedule runs; no new public app-access API is authorized.

Planning scope amendment: CC may narrowly update artifacts/verify-planning.py
and add individually named corrective logs/negative-fixture evidence. CRC records
a single canonical current-assignment field when issuing iteration02, consistent
with its assignment-history entry. The full original checker contract is retained.
No prior failed log, issued prompt/design or CRC verdict is overwritten.

All original gate commands remain required, plus the after-edit library test in
both feature modes and the bounded parser negatives specified by directive05.
Three historical done rows remain anchored to CRC's old candidate; I-02/I-05/I-06
stay open pending the new candidate's independent review. Graph doc-only language
elsewhere has only the explicit behavior-preserving test-helper exception above.
The verifier remains a known failing gate until CC repairs and CRC reproduces it.

- v1.4 — 2026-09-17: CDC authorized exact test/checker corrections, identified
  the consuming into_app test constraint, and retained all quality gates and the
  0/5 executed-correction count until iteration02 runs.

## CRC corrective authoring and readiness — v1.5, 2026-09-17

CRC acknowledges directive05 against clean planning
`dcab7931f8379380e0054c9b44b7b7d82b9be597` and clean source `d43078e`.
The source reconnaissance in [iteration02](cc-prompt-iteration02.md#inspected-baseline-and-applied-guidance)
checked `into_app(self)`, unit-test access to the live rack, the two exact
Clippy sites, the integration fixture and the verifier's existing checks.
Directive05 approves only test/checker repairs; the behavior contract and all
six ledger criteria remain unchanged. Prior directive05 “not yet issued” text
above is the historical pre-authoring state.

| Author readiness check | CRC result and prompt pointer |
| --- | --- |
| Source-grounded | Ready: “Inspected baseline and applied guidance” names the real consuming API, fixture/test helpers and checker source at `d43078e`; “Required reading and intake” makes CC recheck them. |
| Design-complete | Ready: “Binding corrective scope and ordered implementation route” fixes the live-state sequence, four exact source paths, checker metadata format and failure cases; only test-local helper/parser shape remains discretionary. |
| Guideline-applied | Ready: the applied-guidance table ties CG-L-01, OB-19, AP-01 and testing guides to the all-targets gate, Vec transfer, warning policy and behavioral proof, with strength/rationale/check. |
| Executable | Ready: seven ordered file/symbol steps, an expressly uncompiled borrow sketch, four-path corrective source allowlist and explicit planning artifact scope show the route without a public API change. |
| Falsifiable | Ready: the after-edit `(0,0) → (4,6) → (8,9)` oracle, six bounded checker cases, full unchanged command matrix and raw failure/rerun requirements reject duplicate systems, stale metadata and green-wrapper inference. |
| Coherent and portable | Ready: the bounded manifest names accessible paths and states, intake/readback home and prior authority; this plan's one canonical assignment field matches its history row and the checker contract. Future sample/seed/MIDI work remains separate. |

This is CRC author self-review, not CC execution or independent acceptance.
Executed corrective refinements remain 0 of 5 until CC runs iteration02.
Arc09/slice02 stays held until CRC accepts the new candidate and closes all rows.

- v1.5 — 2026-09-17: Issued guide-authored corrective iteration02 with canonical
  assignment metadata and six readiness checks; retained all gates and row states.
