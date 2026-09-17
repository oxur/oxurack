# CDC directive04 — CRC authors the detailed implementation handoff

Issued 2026-09-17 by Sofie/CDC to CRC through Duncan. Proactive return initiated
by Duncan's request to refresh the installed prompt-authoring guidance and have
CRC use it to prepare the next CC prompt. Predecessor:
[cdc-directive03.md](cdc-directive03.md). No escalation04 exists or is implied.
Observed planning baseline: `9013653f1a327890d1e444d2fca9c80110277c5d`;
source main: `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. Both worktrees were clean
before this planning amendment; no arc09 execution/review packet was present.
Project v3.1 / slice01 v1.1 record this authoring assignment.

## Decision and authority

Approve the new authoring discipline immediately for CDC and CRC, for both initial
and corrective prompts. This directive replaces directive03's instruction to relay
the initial prompt unchanged. CRC is now assigned to expand the existing, approved
arc09/slice01 design into a detailed replacement CC assignment. CDC retains the
architecture and first-slice design; this is explicit authorization for CRC to
detail that design, not to select new public behavior, scope or acceptance policy.

Preserve directive03 and the initial cc-prompt.md byte-for-byte. Hold further
execution/relay under that prompt until the replacement is ready. If work has
already begun in another context, preserve it, reconcile its source/evidence state
with Duncan, and target the actual outstanding work; do not assume an unstarted
replacement or restart accepted work. No executed refinement is inferred from a
new filename, and the five-pass budget is not reset.

## Required author reading and application

Refresh [collaboration-framework SKILL.md](/Users/oubiwann/.codex/skills/collaboration-framework/SKILL.md)
(installed version 2.3.0), then read the entire
[From Slice to Implementation Prompt guide](/Users/oubiwann/.codex/skills/collaboration-framework/knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md)
and its [authoring template](/Users/oubiwann/.codex/skills/collaboration-framework/knowledge/engineering-methods/templates/cc-implementation-prompt.md). Also load the project-management
README's required filename/handoff sections and the relevant Rust and testing
entrypoints and chapters. Resolve installed equivalents in CRC's context if these
absolute paths differ; verify all guide paths supplied to CC are accessible there.
An unavailable required guide must be disclosed and substituted before claiming
readiness. Merely telling CC to load skills does not satisfy author responsibility.

The guide's sections 1–7 govern preparation. Apply relevant Rust API/trait,
ownership/borrowing, library error, panic-counterpart and documentation guidance;
record exact chapters actually read, their strength where applicable, the chosen
policy, its rationale and proof. Do not invent pattern IDs or promote contextual
recommendations into universal requirements. The worked registration fragment is
illustrative and uncompiled; it is not evidence that our source supports a recipe.

## CRC's concrete authoring assignment

1. Inspect the actual source checkout/revision/dirty state and the approved packet.
   Inventory the real Oxurack constructor/install path, OxurackModule, CoreError,
   metadata/spawner and parameter accessor wiring, patch loading, into-App access,
   existing test helpers and feature boundaries. Verify the schedule API against
   the locked Bevy source. Record observed symbols/callers and their intended deltas.
2. Use [execution-contract.md section 1](arc09-executable-rack/slice01-module-behavior-installation/artifacts/execution-contract.md#1-module-behavior-installation--binding-for-slice01)
   as the authoritative current-slice design. Sections 2–5 describe successors and
   do not enlarge this assignment. Preserve the approved signatures, per-rack
   kind/type identity, pre-mutation collision error, idempotence, legacy install
   compatibility and hook-panic boundary. Label binding decisions, recommended
   implementation shapes, local discretion and any unresolved issue separately.
3. Supply an ordered file-and-symbol recipe: add the default trait hook/error;
   initialize private installation bookkeeping in every relevant construction
   path; check same-type/conflict cases before mutable registry access; reuse
   existing metadata/spawner/accessor wiring; end temporary resource/world borrows
   before calling the App hook; record successful registration afterward; delegate
   and document the convenience API. Include a representative algorithm/borrow
   sketch, explicitly labelled uncompiled unless actually checked. Explain why
   global bookkeeping, per-instance hooks, silent replacement and rollback claims
   for arbitrary hook panics violate the approved contract. Local helper names
   remain CC's choice. Reconcile any infeasible design with CDC before issuing.
4. Design concrete tests through the real RON-load/public rack path. Map I-01–I-06
   to setup, action, exact expected observable and plausible bad implementation
   rejected. For a two-instance fixture, select explicit increments (for example
   2 and 3), state the initial values and resulting totals after each direct fixed
   schedule run; unrelated Update must not advance them. Specify parameter/state
   preservation after reinstall, exact collision variant, observable preservation
   of original metadata/spawner/setters/getters/systems, independent racks, legacy
   no-op-hook compatibility and feature-on/off behavior. A hook counter supplements
   real execution; it cannot replace it. Derive oracles from the contract before CC
   writes the implementation, using verified fixture APIs.
5. Give the doc-only repairs a precise recipe from the retained rustdoc log and
   source comments: three unresolved and eight redundant link diagnostics across
   the three already allowed legacy files. Preserve useful docs and prohibit lint
   suppression/behavior changes there. Include docs for the new APIs and panic
   boundary. The original eight-path allowlist, no-dependency-change rule, complete
   command matrix, failed/unrun evidence requirements and Q-02/Q-03 ownership in
   [slice-plan.md](arc09-executable-rack/slice01-module-behavior-installation/slice-plan.md#required-checks) remain binding.
6. Compose a usable prompt using the template. Put the implementation spine,
   decisions, test expectations, stop conditions and return contract in the prompt.
   Detailed attachments must name exact sections, applicable revisions and why CC
   needs them; broad reading lists are insufficient. Directive04 governs authoring
   and relay; directive03 plus execution-contract section 1 govern approved behavior;
   the slice plan and ledger govern scope/gates. Material contradictions require
   reconciliation, not a precedence shortcut or CC choosing the easier interpretation.
7. Record all six readiness results in the existing slice-plan assignment record,
   with section pointers: source-grounded, design-complete, guideline-applied,
   executable, falsifiable, coherent/portable. This is author self-review, not new
   independent acceptance. Resolve material gaps before relay. Length is not the
   target; a second competent implementer should not have to choose different
   observable behavior, API or failure policy from the same packet.

## Files, history and return

Create the next unused preserved prompt in the existing slice root. At this
baseline the intended path, **not yet authored or issued**, is:

`arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration01.md`

Use that path if still unused; reconcile newer assignments if present. Do not
rewrite cc-prompt.md, create a new slice or pre-create a placeholder. In the slice
plan record predecessor, authoring-improvement reason, whether the original was
executed, actual refinement count and replacement disposition. Replacing an
unstarted prompt is not an executed correction pass. All six I rows remain open;
no new implementation, acceptance or verifier record is produced by this directive.

CRC may update this slice's plan with source reconnaissance, applied-guidance table,
recipe/test detail and readiness record, create the replacement prompt, acknowledge
this directive in the project plan, and update current handoff/status pointers.
Use the existing slice artifacts home only for supporting evidence that is actually
needed. Keep issued design/prompt files and prior evidence intact. No source code
changes, acceptance-target changes or extra scope are authorized by this authoring
assignment. Any necessary design amendment returns to CDC through Duncan.

Inspect and commit only the individually named planning files CRC actually changes;
use explicit-path staging and `git commit --only` after cached-diff review. Preserve
unrelated changes. Return the planning commit, exact new project-relative prompt
path as plain copy/paste text, source revision/state, six readiness results with
section pointers, deviations/blockers and instruction to relay to CC. Routine
within-design authoring needs no extra approval round. Existing implementation
review and Operator gates remain; no push/merge/release. Future CRC assignments
must meet this same authoring standard. Next project exchange is 05 unless reserved.
