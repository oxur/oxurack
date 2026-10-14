# CC iteration01 — restore the command API and finish the session recipe

2026-10-13. Sofie/CRC → Duncan → CC. Three contributors, Expedited Mode.
Execute this implementation/test/documentation correction, then return to CRC.
Assignment: `arc12-live-rack-operations/slice05-instance-commands/cc-prompt-iteration01.md`.
Predecessor: preserved `cc-prompt.md`, SHA256
`a1ab69c378b51f8b9b9cd003cac59f1a314c9934d39e62d1205337135633db94`.
Initial implementation7404fe2/a3f350e was independently reviewed: IC-01/02/03/05
done; IC-04/06 open. First correction1/5 issued, zero corrections executed.
Slice05/Arc12 remain open. CDC12 unchanged; no new design or approval gate.

S=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch `codex/oxurack-module-behavior-installation`, clean
`7404fe227e561746dbda76543b59893feeacfe15`.
W=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch planning;
review issuance descends from `a3f350e66833196edba0c642c88c677ca6114406`.
P=W/project01-mvp; A=P/arc12-live-rack-operations; D=A/slice05-instance-commands.
R=`/Users/oubiwann/lab/oxur/oxurack`, protected main
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
K=`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`;
F=K/collaboration-framework/knowledge. Framework2.6.0, Rust-guidelines2.0.0.
Confirm cwd/branches/HEAD/status; unexpected drift returns to CRC without reset.

## Required reading and pre-edit intake

Load complete required text in bounded contiguous ranges and recover truncation.
Record actual itemized states/extents/recovery and a concise source-cited contract
readback in a new iteration01 section of **D/closing-report.md before edits**.
Explain why public Pending/constructible IDs differ from the contract, why queue
pop precedes Panic, and why a filename suffix cannot guarantee a failed Load.
Hashes, headings, old summaries and another context's receipts are not reading.
Coherent intake permits implementation under existing authority. Do not stop at
an intake-only return. Missing/conflicting binding material blocks affected work.

| Order | Input and state | Required extent / reason |
| --- | --- | --- |
| 1 | W/AGENTS.md; S/CLAUDE.md | required-full: roles, source boundary, Rust/testing guidance |
| 2 | This prompt; D/slice-plan.md; D/ledger.md; D/crc-verification.md | required-full: current assignment, four accepted/two open rows and F-01–F-04 |
| 3 | D/cc-prompt.md, issuance4b8119a | required-section: opening metadata; Source-grounded implementation route and guidance; full sections2–5; Gates, exact scope and return. Retained removal mechanics in section1 reference-only unless touched by a discovered interaction. Exact current correction fence below supersedes its larger edit allowance |
| 4 | P/cdc-directive12.md | required-section: full D2; D3 Commands and global tempo plus reference/handle paragraphs; D4 Loader compatibility/source boundary,05/discriminating proof and guidance/readiness. Existing lifecycle/matrix authority retained, no new lifetime semantics |
| 5 | P/project-plan.md; A/arc-plan.md; A/ledger.md | required-section: project metadata/Current direction/Design handoff history-current; arc metadata/Capability/Five delivery slices/Binding design/Current lifetime contract/closure; full arc ledger. Parent gates and no new arc authority |
| 6 | P/cdc-directive08.md | required-section: Prompt quality and Checks/correction policy; proportionate proof/native gates |
| 7 | K/collaboration-framework/SKILL.md; F/project-management/guides/README.md; F/engineering-methods/guides/07-implementation-prompt-authoring.md; F/work-verification/guides/05-independent-verification.md | required-full: execution intake, correction/retention, review boundary |
| 8 | K/rust-guidelines/SKILL.md; K/rust-guidelines/guides/11-anti-patterns.md | required-full before Rust changes; use actual rules, not an attested checkbox |
| 9 | K/rust-guidelines/guides/02-api-design.md;03-error-handling.md;05-type-design.md | required-section: API-02/07/23; EH-01/14; TD-03/05/06/07, complete examples/rationale. Opaque IDs, exact enum and named errors/documentation |
| 10 | K/testing/SKILL.md; F/testing/guides/01-testing-discipline.md;03-validation-gates.md | required-full: native, discriminating checks; no new proof platform |
| 11 | S/crates/oxurack/src/commands.rs, handles.rs; S/crates/oxurack/tests/instance_commands.rs; S/crates/oxurack/examples/rack-session.rs | required-full at7404fe2: actual API, queue/test state and runner consumer to repair |
| 12 | S/crates/oxurack/src/lib.rs, lifecycle.rs; core event.rs/topology.rs; MIDI-out lib.rs | required-section: facade fields/constructor/exports/into_app; lifecycle production through cfg(test); all CoreCommand and ModuleRemoved definitions; prepare/commit_module_removal; prepare_silence_for/prepare_silence_selected/append_silence/silence_outputs. Actual integration, errors and final cleanup; inherited lifecycle test bodies reference-only |
| 13 | S/README.md; S/Makefile; S/crates/oxurack/examples/patches/turing-machine-lfo.ron | required-section: README live-persistence/handle/command/example recipes; Make test/lint/docs/build targets with prerequisites; required-full immutable LFO fixture. Exact native tools and nine baseline cables |

No structured-data query applies. Conditional: if unfamiliar dependency APIs
arise, check their locked local version before use; no new dependency is needed.
Old directives/other slices are reference-only except the explicit bounded inputs
above. Approximately45–65k tokens including reading, implementation and recovery;
chunk rather than skim. On compaction revalidate state and reload unavailable
contracts. The correction fits the existing slice; do not invent another tree.

## Verified baseline and guidance applied by CRC

commands.rs:11 exposes CommandId(pub u64); CommandStatus includes Pending; the
public CommandResult is a struct and CommandOutcome is absent. An external
consumer of the required API fails E0432/E0599; constructing CommandId(42)
compiles. FIFO execution/reset logic itself works. The actual runner at:140
derives a supposedly missing path from --save; a valid existing sibling causes
second Load Ok and third Load PendingMidi, aborting the session. At:187–203 it
compares ordinary replay but never resets/drains either final lifetime. A300-sample
run has final NoteOn62 at251 and no final cleanup status. Native gates and normal
session/fresh/no-graph traces pass. These observations are reproduced, not guesses.

CRC read/applied API-02 SHOULD (owned outcomes/borrowed calls), API-07 MUST
(private Arc implementation remains hidden), API-23/EH-01 MUST (typed ordinary
errors, no successful stubs), TD-03 SHOULD/TD-05 MUST (opaque ID), TD-06/07 SHOULD
(state-carrying enum/non-exhaustive public result), EH-14 MUST (errors and panic
boundaries in rustdoc). They support the already binding initial API. API changes
must reach exports, tests and runner together. No library runtime, toolchain,
dependency or new exception is imported from skill defaults.

## F-01 — IC-04: exact public API and bounded private states

Binding, unchanged from initial section4:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandId(u64); // private field, no public constructor

#[non_exhaustive]
#[derive(Debug)]
pub enum CommandResult {
    Completed(Result<(), CoreError>),
    CancelledByReset,
}

#[derive(Debug)]
pub struct CommandOutcome {
    pub id: CommandId,
    pub result: CommandResult,
}
// Oxurack::process_commands(&mut self) -> Vec<CommandOutcome>
```

Remove the unintended public CommandStatus surface, not the required types.
Update lib.rs exports and runner/integration/unit callers to `.result` and
CommandResult variants. No public Pending outcome. Compare submitted IDs in
integration tests, not external CommandId(n) construction or `.0` access.
An optional Display implementation may show the number. Do not add a public
constructor/accessor merely to rescue tests.

Use private `Slot::Pending { id, command } | Slot::Completed(CommandOutcome)`;
initialize VecDeque::with_capacity(64). Cancellation replaces a Pending slot,
dropping its command, preserves ID and leaves Completed slots untouched. Maintain
64 total outstanding including cancellations, checked increment before acceptance,
full/exhaustion errors with unchanged counter/slots, queue outside App, and
exactly-once FIFO result collection. Preserve the working pop-before-execute seam.

Worked illustrative body, not compiled drop-in code (private field names adapt):

```rust
pub fn process_commands(&mut self) -> Vec<CommandOutcome> {
    let mut outcomes = Vec::with_capacity(self.command_queue.slots.len());
    while let Some(slot) = self.command_queue.slots.pop_front() {
        let outcome = match slot {
            Slot::Completed(outcome) => outcome,
            Slot::Pending { id, command } => CommandOutcome {
                id,
                result: CommandResult::Completed(self.execute_command(&command)),
            },
        };
        outcomes.push(outcome);
    }
    outcomes
}
```

Reset converts the still-owned queue tail only after silence_outputs succeeds.
Taking the queue into a local vector before execution would conceal that tail.
Keep all current cancellation/capacity/exhaustion regressions. Add a real public
API doctest using Vec<CommandOutcome> and CommandResult::Completed; compile_fail
doctests reject CommandId(42), `.0` access and an exhaustive external match lacking
the wildcard on non-exhaustive CommandResult. No new trybuild dependency/harness.
Those checks reject the current API even though current native tests are green.

## F-02 — IC-06: reproducible missing load and final silence

Keep the session's real handles/removal, six-command FIFO, ordinary trace and
saved/fresh/no-graph comparison. At rack-session.rs:140 replace the user-adjacent
suffix with a newly reserved private temporary directory and a child path that
is never created. Do not remove or overwrite any pre-existing sibling file.

Recommended standard-library recipe (illustrative, not compiled):

```text
for ordinal in 0..1024:
    dir = temp_dir()/format!("oxurack-missing-{}-{ordinal}", process::id())
    create_dir(dir):
        Ok => own dir; missing = dir/"not-created.ron"; stop searching
        AlreadyExists => try next ordinal without altering existing dir
        other Err => return the actual I/O error
if no reservation => return a clear reservation error
enqueue the original six commands using missing only for the expected failed Load
process once; remove the now-owned empty temporary dir; validate six typed results
```

Private helper/guard organization is local discretion. Normal I/O failures remain
visible; exceptional best-effort cleanup must not hide the primary error. No new
crate, random service, temp-file overwrite or filesystem API in the sample loop.
The other failed Load must not change the current owner; successful Load still
retains its queued tail. Expected normal outcomes stay [Ok,Err,Ok,Ok,Ok,Ok].

After comparing ordinary final-lifetime trace/scalars/copied MIDI with the fresh
saved rack, explicitly reset **both** stopped final racks, drain each cleanup,
record each event to Scope and phase-labelled status, and compare their cleanup
vectors. Never send cleanup into ordinary stdout or reset before capturing the
ordinary replay. Reuse reset/drain_midi/record_scope_midi; no hidden sender.

```rust
// Illustrative helper body; phase/status types and error return follow the runner.
rack.reset()?;
let cleanup: Vec<_> = rack.drain_midi().collect();
for event in cleanup.iter().copied() {
    let [a, b, c] = event.message.to_bytes();
    writeln!(status, "{phase}: sample={} {a:02X} {b:02X} {c:02X}", event.sample)?;
    rack.record_scope_midi(event, MidiDelivery::Generated);
}
```

Use phases `final-cleanup` and `reference-cleanup`. At samples300 with the supplied
fixture and saved TM base.5 each vector is exactly
[(300,80 3E 00),(300,B0 7B 00),(300,B0 78 00)]; stdout remains exactly
`251|90 3E 15\n`. A repeated reset emits nothing. Report Scope counts truthfully
after cleanup (or explicitly identify pre-cleanup counts); do not retain4014
as a post-cleanup count by hiding appended records. This proves emission/recording
only; there is no endpoint or hardware-silence test.

Extend the existing writer-based runner tests rather than adding tooling:

- Reserve a unique test directory; pre-create --save's former `.guaranteed-missing.ron`
  sibling with valid LFO fixture bytes. Session still succeeds, saved/fresh behavior
  matches, and that sibling's bytes are unchanged. Baseline7404fe2 exits1 here.
- Use samples300, not32, for a nonempty exact final trace and the literal cleanup
  for both phases; test repeat cleanup/idempotence through the real helper if
  factored. The baseline has no final/reference cleanup status and fails the oracle.
- Keep zero-sample/missing-input failures and ordinary2000 default/no-graph/fresh
  equality; include an unwritable/invalid save destination with nonzero result.

## F-03 — IC-04: immediate failed-reset observation

Extend commands.rs::tests::failed_reset_preserves_pending_command, which currently
only checks eventual success after Err. Use a real loaded fixture with selected
Clock/TM Scope and a short nonempty capture, then stop/drain. A deliberately
latched fault marker is allowed for preservation (identify it as test injection,
not a reproduced playback defect). Enqueue at least two distinguishable commands
and retain their IDs; despawn the MIDI sink without release as in the existing
test, so prepare_silence fails whole-rack reservation accounting.

Immediately before reset own a projection of pending IDs/commands and next_id;
SampleTime, Fixed elapsed/overstep, Virtual elapsed/delta/pause and driver/running;
pending MIDI and first-fault values; Scope selectors/capacity/overwrites/records;
and typed lifecycle/parameter/label/cable messages. Call reset, assert the typed
TickError::OutputCapacity reason, then compare the projection **before processing**
or any other operation. CoreCommand/first fault can use owned Debug strings where
PartialEq is unavailable; no pointers/references into a mutable World. Keep the
pending commands unchanged and later collect their original IDs in order with
ordinary Completed results, never CancelledByReset. Do not call snapshot_patch
while PatchHandle/cables deliberately refer to a despawned sink.

This closes the original immediate-failure oracle, not a new matrix or public
fault-injection feature. No product defect was reproduced here. Use a small
inline owned helper only if useful; no new public accessor/verifier framework.

## F-04 — IC-06: finish the new API documentation

Add useful rustdoc to new public handle/reference constructors/accessors/builders
and methods. Every new Result-returning function needs # Errors, reflecting
actual InvalidReference/StaleHandle/NoPatchLoaded/UnknownModule/type/member/
topology failures and delegation. Explain bound builders defer member resolution,
unbound exact names versus dotted rejection, invalidation and survivor validity.

Document command ID opacity; enum outcomes; total64/cancelled capacity; ID/full
errors; execution-time resolution; continued failures and once-only processing;
successful/failed reset cancellation and into_app abandonment. Document global
BPM separately from Clock and actual no-patch/invalid-BPM errors. remove_module
documents stopped/drained order, selected cleanup drain, Scope invalidation and
hook panic exclusion. New core prepare/commit and MIDI prepare_silence_for errors/
caller ordering need honest sections as applicable. Update ModuleRemoved's
producer comment: it now has a core retirement emission site; whole-App drop
still emits no per-instance events. No notification payload change.

README includes the actual default session, fresh saved replay and no-default
session/cmp recipe from initial section5; current text mentions both modes but
shows only default. Describe final cleanup as status, ordinary stdout unchanged.
Strict rustdoc passing verifies links/warnings, not the existence of these sections.

## Validation, retention, scope and return

Fresh final-source commands from S:

```bash
cargo test --locked -p oxurack --all-targets
cargo test --locked -p oxurack --no-default-features --all-targets
cargo test --locked -p oxurack --doc
make test
make lint
cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack-graph -p oxurack --all-features --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' make docs
cargo build --locked -p oxurack --examples
cargo build --locked -p oxurack --no-default-features --examples
git diff --check
```

Run initial section5's full default/fresh/no-graph2000 recipe; ordinary14-row trace
SHA256 remains40965cc5f2031cbd3b26bd7275e0012359800c959d9c08f3dd5139f3707a76a2.
Run the two discriminating runner cases and original MIDI2000/LFO10000 hashes
from initial Gates. Preserve every failure/correction/retry. Restricted make test
has four known CoreMIDI failures,146 pass/3 ignores; retry unchanged with host
access, expecting150 pass/3 existing ignores. No suppression/new ignores, endpoint,
make check/dependency-outdated campaign, new validator or coverage campaign.

Proposed retention, not CC acceptance: CRC7404fe2 dated2026-10-13 IC-01/02/03/05
behavior and standalone core/MIDI/graph receipts, only where actual delta preserves
them. Current affected facade/example/docs and workspace gates above are fresh.
Migration of queue API does not authorize changing installed engines, topology,
typed notification semantics, reset MIDI behavior or existing accepted tests.

Authorized source paths only:

- crates/oxurack/src/commands.rs — API/private queue and unit/doctests
- crates/oxurack/src/lib.rs — exports/documentation only
- crates/oxurack/src/handles.rs — documentation only; preserve behavior
- crates/oxurack/src/lifecycle.rs — reset/removal documentation only
- crates/oxurack-core/src/event.rs — ModuleRemoved producer documentation only
- crates/oxurack-core/src/topology.rs — new removal API documentation only
- crates/oxurack-mod-midiout/src/lib.rs — new selected-silence API documentation only
- crates/oxurack/tests/instance_commands.rs — required API migration and existing proof, retain all accepted behaviors
- crates/oxurack/examples/rack-session.rs — consumer migration, missing-path/final cleanup and runner tests
- README.md — corrected usable recipes/API guidance

No other production changes. A defect outside this fence returns its concrete
evidence through CRC/CDC before affected repair. Keep original fixtures/examples,
manifests/lockfile/dependencies, standalone/RT/main and all issued prompts intact.
Planning edits only D/closing-report.md and D/ledger.md; do not edit review/prompt/
plans. Durable supporting artifacts none expected; /tmp for routine logs, owning
D/artifacts/ only for a justified failure receipt with inventory, no passing archive.

Before commits inspect status/diff/staging. Stage only actual named new files if
any. Use git commit --only with every changed authorized path listed literally;
no directory-wide stage, no other contributors' files. Separate planning commit
names only the two allowed D records and any explicitly justified produced receipt.
Expedited Mode authorizes these scoped commits; no merge/push.

Return this prompt identity/SHA, source/planning commits and exact changed paths;
pre-edit itemized intake/readback, applied Rust rules, all attempts/results/unrun
checks, API/cleanup/collision/error-state observations, deviations/self-review,
six-row walk, retained-evidence rationale and artifact inventory/bubble-up.
IC-01/02/03/05 retain CRC acceptance subject to delta; IC-04/06 remain open with
CC-proposed evidence until CRC review. No Slice05, Arc12 or MVP acceptance. Arc05
inherited rows and Arc07/13 re-entry plus CDC composition gate remain unchanged.
