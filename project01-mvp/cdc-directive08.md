# CDC directive08 — deliver the playable rack; proportionate proof

2026-10-10, Sofie/CDC to CRC through Duncan. Three contributors, Expedited Mode.
Initiated by Duncan's request to re-evaluate over-caution from Arc10/Slice03 onward
and apply collaboration-framework 2.6.0. This is a proactive design return, not
an invented escalation. Inspected planning `32e6934`, implementation `9ba55dd`
in `.worktrees/codex-oxurack-module-behavior-installation`. Supersedes directive07
for unissued future work and directive04's authoring guidance where amended here.
Issued prompts and Slice02's active review/correction retain their own contracts.

## Assessment and decisions

The old plan put an asynchronous output subsystem and substantial proof machinery
between a working TM adapter and first use. I prescribed that architecture as CDC;
the resulting overhead is a design/planning problem, not simply CC performance.
The repository has real Clock/adapter work, but infrastructure completion is not
the same as a playable instrument. This is unreleased open-source music software.
Future implementation assignments authorize library/runtime code changes; stop
using a blanket "do not touch production code" fence. Keep precise component
scope and the existing independent reviewer, not speculative safety-critical gates.

1. **Approved:** Arc10 now has four slices. Slice03 delivers an installed MIDI-out
   module plus a runnable static-patch trace. Slice04 adds direct endpoint playback
   to the same example, cleanup and the first musical trial. Unissued Slice05's
   work is absorbed into these slices; there is no separate acceptance project.
2. **Approved:** use a simple single-owner, synchronously drained output path for
   M1. Retire the mandatory 32-message/256-batch ring, atomic control/ack protocol,
   bounded join and simulated consumer. Keep finite storage, deterministic order,
   explicit failures, correct note-offs and a stop path. Arc10 design v1.2 sections
   3–5 define the replacement behavior.
3. **Approved:** a real software MIDI endpoint is sufficient for the first witness,
   as is hardware. An unavailable endpoint is an honest gap, not a simulated pass;
   CDC may disposition that gap to Arc08 and start Arc11 after software composition
   review. Hardware timing and final musical acceptance remain project obligations.
4. **Deferred:** concurrent output service/reconnect to Arc12 runtime API when
   live operation needs it; optimize from measurements. 95% coverage, hardware
   P99 <500 us, 10/50/100-module benchmarks and 24-hour soak remain Arc08/release
   obligations, not prerequisite gates for M1 or M2. No result is newly accepted.
5. **Retained:** Slice02's issued iteration01, all its rows, and CRC's review.
   Do not edit that packet to fit this directive or restart its work. Repin the
   accepted Slice02 source before authoring Slice03.

## CRC's next assignment: author Slice03 after Slice02 acceptance

Acknowledge this directive in project handoff history after checking live state.
Then continue current Slice02 review. When it closes, create the normal Slice03
open set and issue CC implementation immediately; no extra design review is
required within this decision. Return the project-relative prompt path to Duncan.

Source reconnaissance already establishes the route at `9ba55dd`:

- `crates/oxurack/src/lib.rs`: `try_install`, `load_patch`, `start`, `stop`,
  `advance_samples`, and the existing completed-sample accounting.
- `crates/oxurack-mod-turingmachine/src/rack.rs`: `TuringMachineModule`, the
  `step`, note/velocity validity, gate and noise ports; actual scheduled publication.
- `crates/oxurack-core/src/tick.rs`: Produce → Propagate → MergeInputs → Consume
  → Observe. The sink reads the published snapshot in Consume, not engine internals.
- `crates/oxurack-midi/src/wire.rs`: `MidiWire` constructors and byte conversion;
  constructors mask values, so input validation must happen at the boundary.
- `crates/oxurack-mod-turingmachine/src/midi_io.rs`: existing synchronous send
  pattern, useful reference only. Do not bypass the rack using `MidiTuringMachine`.
- `crates/oxurack-rt/src/lib.rs`/`midi_io.rs`: existing runtime requires a clock
  mode; output opening is private and substring-based. Do not pretend it already
  offers the required neutral output session. Use `midir` directly in the runner
  with explicit selection; no prerequisite rewrite of those RT internals.

CRC must verify these against the accepted source, choose the exact crate/module
and facade API, and show the actual registration, resource ownership, drain and
RON wiring in the prompt. Small related facade/Cargo/example/test changes are in
scope; a new scheduler or generalized framework is not. Authorize component-level
edit scope plus necessary integration sites. CC may make adjacent private helper,
import and test changes within those components and must report them. List the
actual commit paths explicitly; no `git add .` or unrelated edits.

Also correct the source `CLAUDE.md`'s stale One-Contributor paragraph and stale
skill routing when Slice03 touches implementation instructions. Planning authority
already governs; this is a small integration edit, not a separate planning slice.

## Prompt quality for all remaining work

Read framework 2.6.0's `07-implementation-prompt-authoring.md`, its template,
`testing/guides/03-validation-gates.md` (Proportionate Proof), and relevant installed
Rust chapters. The skill root on CDC's host is currently
`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills/`;
resolve equivalent usable paths on CC's host before issue. Exact source and domain
reading is the author's responsibility, not homework passed down as an empty recipe.

Put substantive engineering in the prompt or exact current slice-plan sections:

- State the user-visible result and how Duncan runs it. Resolve binding ordering,
  validation, ownership and failure behavior. Explain why the chosen design fits.
- Give concrete files/symbols, proposed signatures/data shapes, construction and
  wiring steps, and representative Rust/pseudocode for the tricky transition.
  Mark examples as illustrative unless actually compiled. For Slice03 show the
  step-to-MIDI transition and buffer/drain integration; for Slice04 show send
  failure/cleanup ownership and the paced runner loop. A gate checklist is not
  a substitute for these examples.
- Name each meaningful test's setup/action/expected result and the mistake it
  catches. Example: channel 0, active note 60, next step note 60/velocity 100
  emits `[0x80,60,0]` then `[0x90,60,100]`; the following idle sample emits nothing;
  a silent step emits only `[0x80,60,0]`. Exercise the installed path as well as
  the small transition function. Do not compare output with itself as an oracle.
- Distinguish binding behavior, a recommended implementation, and local discretion.
  CC can adapt private mechanics, add a discovered regression test and fix adjacent
  implementation mistakes inside approved behavior. Public behavior, major scope
  or acceptance changes still return to CDC. Do not force an escalation for every
  changed helper or filename.
- Bound shared-document reading to current sections. Superseded directives,
  old complete prompt packets and historical validation logs are reference-only
  unless a specific dependency is named. Keep a concise intake/readback in the
  existing report; do not commission another intake validator.

Applied Rust guidance: AP-23 favors the ordinary owned buffer/direct loop until
measurement warrants optimization; AP-25 and EH-01 require recoverable I/O and
configuration errors as Result; AP-33 supports reporting send and cleanup errors
instead of silently discarding them. These are concrete design choices, not
instructions to import an async runtime from a skill's defaults. CRC applies the
remaining relevant API/ownership guidance to its exact proposed implementation.

## Checks and correction policy from Slice03 onward

Select adequate proof of the agreed behavior. Default implementation completion
uses the focused behavioral checks, `make test`, `make lint`, and all-targets
Clippy for the changed crates/features so examples/tests are covered. Check new
public documentation with `RUSTDOCFLAGS='-D warnings' make docs` when public API or
docs change. Preserve standalone/rack feature checks where affected. Record
exact commands, exits and useful summaries in existing close/review records.

The repository's `make check` also runs `check-deps`, which checks current available
updates and may install tooling. For these feature slices, the explicitly selected
build/test/lint/docs checks replace that aggregate invocation: dependency freshness
and upgrades belong to Arc08, or to work explicitly evaluating dependency upgrades/compatibility. Wiring an
existing local crate or feature does not trigger a dependency-freshness campaign. This
changes future planning gates, not the Makefile or Slice02's issued gates. Required
checks that fail or cannot run must be reported and dispositioned, never described
as passed. No warning suppression, ignored regression or fabricated device result.

Do not add a reusable docs/report validator, broad mutation suite, copied probe
crate/lockfile or new evidence format unless existing tests and direct inspection
cannot establish a named required property. Existing incidental checkers are not
inherited mandatory products. A short direct consistency check is enough for
ordinary Markdown. Keep raw logs only when they add reproducibility value (such
as a failure or environment issue); normal successful output needs command,
revision and result, not a second archive of the entire build.

CRC reviews the code and each acceptance claim independently. On corrections,
rerun affected behavior and integration checks; retain unaffected independent
results only under the framework's evidence-retention rule, with original
revision and rationale. Do not restart the entire proof campaign for a test-only
repair. At arc close CDC reproduces composition and reviews the delta/child
verdicts; it need not recreate every child test suite or invent a parallel harness.

A correction needs a failed agreed criterion, a credible defect or invalid proof
of a required property. Classify an additional convenience, hypothetical hardening
case or aesthetic preference as a follow-up unless it actually defeats acceptance.
Collect current findings into one coherent repair prompt with implementation
suggestions; do not dribble newly invented requirements into successive rounds.
New evidence can still expose a real blocker. There is no fixed target number
of tests, reports or iterations, and no entitlement to use all five iterations.

## Return and authority

CDC has updated the project plan, Arc10 plan/design/ledger, project ledger,
planning instructions and current status. No slice/arc is closed by this change.
CRC owns the normal Slice03 open set after acceptance; avoid a second directive
or design packet simply to repeat this one. Preserve earlier handoffs and evidence.
If baseline drift defeats the chosen route, name that concrete conflict through
Duncan; otherwise proceed within these decisions and report the runnable outcome.
