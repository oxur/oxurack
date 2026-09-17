# Slice01 — module behavior installation

Version 1.1, 2026-09-17. Status: open; CRC preparing detailed replacement under directive04.
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
```

The final three entries are doc-comment-only. Inline tests in the first three
files are allowed. No manifest/lockfile/new dependency edits. If another path is
necessary, explain why to CRC before expanding this allowlist; design changes go
to CDC. The RON fixture is source test data; logs are planning artifacts.
Planning allowlist: this slice's ledger.md, slice-plan.md (assignment status only),
closing-report.md, the two named evidence files and each separately enumerated log.
Do not edit issued cc-prompt.md, execution-contract.md or another contributor's
verification. No close or verifier placeholders at opening.

## Assignment history

Current assignment: `arc09-executable-rack/slice01-module-behavior-installation/cc-prompt.md` — held for relay/execution pending
CRC authoring under [directive04](../../cdc-directive04.md). No replacement has been
issued; its intended next-unused path is cc-prompt-iteration01.md. Read the current
authoring amendment below before acting on the historical initial assignment.

| Date | Prompt | Predecessor | Reason | Disposition |
| --- | --- | --- | --- | --- |
| 2026-09-17 | cc-prompt.md | none | CDC response to escalation03; first TM prerequisite implementation | held by directive04; replacement preparation assigned to CRC |

Executed refinements: 0 of 5. Preserve later cc-prompt-iterationNN.md files.
CC owns execution; CRC checks exact commits, reproduces I-01–I-06 and records
crc-verification.md. On acceptance CRC may detail arc09/slice02 under the approved
design. New arc, altered contract or unresolved failure returns through Duncan.

## Version History

- v1.0 — 2026-09-17: First implementation open set after the accepted baseline;
  bounded registration and doc repair, explicit negatives, no seed/timing scope creep.

## Authoring amendment — 2026-09-17

[Directive04](../../cdc-directive04.md) authorizes CRC to expand the existing design
using the installed implementation-prompt-authoring guide/template. CRC owns source
reconnaissance, applied domain guidance, concrete implementation/test recipes and
a six-part readiness record here before issuing the replacement. This is pending
author work, not a completed readiness claim. Initial prompt/design preserved;
I-01–I-06, eight source paths and all validation requirements remain unchanged.
CRC must reconcile any intervening execution and record actual refinement counts.

- v1.1 — 2026-09-17: Operator requested guide-based CRC prompt authoring; hold the
  earlier handoff for a detailed preserved replacement without changing design scope.
