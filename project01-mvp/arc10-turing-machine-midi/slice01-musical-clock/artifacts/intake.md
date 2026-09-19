# Slice01 CC intake and contract readback

Date: 2026-09-19. Workflow: Three-Contributor Workflow, Expedited Mode.
Role: CC implementation return; independent CRC reproduction remains pending.

## Revisions and cleanliness

The exact assignment is
`arc10-turing-machine-midi/slice01-musical-clock/cc-prompt.md`.

- Source worktree: `codex/oxurack-module-behavior-installation`, clean at
  `37deeace90e7fe2dff633ada9d58ef9051577c3d`, the pinned Arc09 accepted
  baseline.
- Planning worktree: `planning`, clean at
  `622cb0d1fd592d46e3e982e53467ee93aef56349`, the CRC acknowledgement of
  directive07 and the unchanged issued Clock prompt.
- Planning baseline before the issue:
  `f6c778ea2f252e9dafa4d80ba459d9774a74beb6`.
- CDC acknowledgement/issue: `8162fccbfca7ddfc657b7462e399a99130536a3a`;
  the current planning commit is `622cb0d1fd592d46e3e982e53467ee93aef56349`.

The pre-edit state and branch checks are retained in
`artifacts/logs/00-preflight-state.log`. The source checkout matched the
prompt's required baseline and had no unrelated changes. Main remains untouched.

## Required reading record

The complete 234-line prompt, 171-line Slice01 plan and 12-line ledger were
read in full. The 56-line directive07 and 57-line Arc10 plan were read in full.
The introduction and section 1 of the 121-line Arc10 design were required for
this assignment; sections 2–5 were read as reference-only and were not treated
as current implementation authority. The 557-line project plan was queried and
read around the current roadmap and the CDC Arc09 acceptance / Arc10 opening
amendment (lines 224–247 and 475–557); the 561-line current status was read
around its current banner and Arc10 section. The 46-line planning `AGENTS.md`
was read in full.

The collaboration-framework entrypoint v2.5.0 and project-management
entrypoint v2.18.0 were loaded. The project-management README and relevant
scales, canonical-worktree, planning-top-down, closing-slices and
confirmation routes were loaded. Work-verification v2.5.0 was loaded with
ledger discipline, evidence strength, row closure, silent-drop and independent
verification guides. The execution rules applied here are: preserve the issued
prompt; keep planning on the planning worktree; keep CC source and planning
commits separate; use explicit file lists; retain failures and unrun gates; and
leave all rows proposed-done until CRC reproduces them.

Engineering-methods implementation-prompt authoring v2.5.0 was loaded through
the required-reading/intake and evidence-strength sections. Rust-guidelines
v2.0.0 was loaded with API-23/API-39, TD-05, DC-05, the Rust entrypoint and
the relevant library, error, type, project-structure and testing patterns.
Testing discipline v1.1.1 was read in full. The guidance versions are recorded
in `artifacts/logs/00-guidance-versions.log`.

The source witnesses were loaded at the pinned baseline:

- `oxurack-core/src/module.rs`: required trait, context, spawn helper and
  registry sections, including the full `OxurackModule` contract and registry
  context-spawner path.
- `parameter.rs`, `error.rs`, and `port.rs`: parameter value/schema/registry
  definitions and setter/getter calls, `CoreError`, `Port`, `CurrentValue` and
  `spawn_port_on_module` definitions.
- `tick.rs`: `TickPhase`, `SampleTime`, `SimulationRunning`, sample latch and
  running gate; `CorePlugin` behavior was checked against existing tests.
- `patch.rs`: `Patch`/`ModuleConfig` data and the complete
  `apply_patch_to_world` implementation, including full override-map delivery
  to `spawn_with_context` before initial parameter messages.
- `oxurack/src/lib.rs`: the complete production `Oxurack` implementation
  through `into_app`, including install/load/driver/stop/start/sample APIs and
  the existing repaired Arc09 behavior.
- `oxurack/tests/sample_pipeline.rs`: imports/types, all three fixture module
  implementations, fixture path/load helpers and tests through
  `live_reentry_has_no_zero_elapsed_catch_up_sample`; the remaining ordering
  test was also inspected as a relevant existing regression.
- Root and adjacent manifests were read in full; the Makefile build/test/lint/
  format/common-checks/check/check-deps/ensure-binstall targets were read.
  `Cargo.lock` package name/version/source triples were extracted before edits
  into `artifacts/logs/00-lock-packages-before.tsv`.

Line-count and witness extents are retained in `artifacts/logs/00-source-extents.log`.
No required file was unavailable or left at a truncated boundary. The
conditional concurrency/async/unsafe guidance was not activated: this slice
has a synchronous bounded state machine, no threads, locks, unsafe code,
device resource or teardown task.

## Contract readback

C-01 is a standalone library contract. `ClockConfig::try_new` must reject
non-finite BPM first, then BPM range, PPQN range, pulse-width range and the
integer low-gap overlap condition. It canonicalizes BPM to milli-BPM once and
uses the bounded integer `D/R` accumulator. A fresh epoch rises at sample zero;
the exact width is high and every valid rate has at least one low sample. The
137 BPM / 4 PPQN rising indices and the 1000 BPM / 30 PPQN boundary are
discriminating oracles against rounded-period and overlap implementations.

C-02 is an edit and lifecycle state machine. Equal canonical configuration is
a true no-op. Any successful change stores the new config and sets one pending
restart; the next executed sample is forced low, then the following sample is
the fresh epoch. Multiple edits coalesce to the last valid config, changing
back does not cancel the pending separator, and an equal setter after a real
change does not cancel it. Invalid edits preserve config, phase, pending state
and output. Stop pauses without resetting the engine or held Gate; start
continues the next sample.

C-03 binds the adapter to the actual rack. `ClockModule` registers one Gate
output and one Produce system on the supplied schedule, calls `next_sample`
once per instance and writes `CurrentValue`. Core's existing latched sample
gate owns stop completion; the Clock adapter must not add its own phase-by-phase
running checks. The integration fixture must use public install/load/start/
advance/stop paths, real RON cables and a Consume/Observe edge observer, then
retain the wrapper until reinstall/resume checks are complete before calling
`into_app`. Controlled live groups, reversed spawn order and unrelated entity
allocation must preserve exact named traces.

C-04 binds validation and parameter ownership. Spawn must decode the entire
unordered override map into one candidate, reject unknown keys in lexical order,
validate all values and the cross-field overlap before creating any module or
port, and report the actual instance/parameter through existing `CoreError`
variants. Setters must copy current canonical values, strictly decode the one
edited variant, validate the replacement, and commit only after success; getters
read the engine, not schema defaults. Parameter registry borrows must end before
World mutation. `Patch.bpm` remains metadata: Clock's per-module `bpm` owns
rate, so an omitted Clock bpm in a patch with metadata bpm=90 still runs at the
Clock default 120.

C-05 binds packaging and gates. The new standalone crate has no Bevy/core
normal dependency and empty default features; `rack` is the only optional
adapter feature. Existing versions are reused, Cargo.lock receives only the
new package and consequent local edges, and the umbrella gains only a rack
enabled dev-dependency plus its fixture test. Docs must state millisecond units,
0.001 BPM resolution, first pulse, low-gap rule, restart/edit/no-op behavior,
standalone/rack features and per-module tempo. All required locked package,
workspace, Clippy, format, rustdoc, Make, outdated and diff gates remain active.

C-06 is an evidence contract. The six rows must each appear in the return,
with exact source/planning commits, explicit changed paths, dependency diff,
raw command logs, results TSV, current checker positive/negative results and
known environmental failures. CC evidence is attested/proposed-done only;
CRC owns independent reproduction and any close or Slice02 advancement.

The easily missed interaction is the edit-while-high boundary: if a high Gate
is changed during a rate edit, the next executed sample must be the one low
separator before the next rising edge. A direct phase reset, a repeated rounded
period, a setter that accepts fields independently, an adapter that reads
`Patch.bpm`, or a stopped-state reset can all look plausible while violating
the contract. The other important lifecycle boundary is that the core's
latched sample must finish even when the shared running resource changes; the
Clock system participates in Produce only and must not invent a second stop
authority.

No contradiction was found between the prompt, plan, directive, design,
current source APIs or author checks. No scope expansion or CRC/CDC escalation
is required before implementation.

## Iteration01 intake and contract readback

This corrective iteration was read from
`arc10-turing-machine-midi/slice01-musical-clock/cc-prompt-iteration01.md`
in full (209 lines). The CRC baseline was read from `crc-verification.md` in
full (103 lines), and the slice plan was read from `slice-plan.md` in full.
The predecessor prompt's source route, behavioral-oracle, validation and
scope sections were reread. The source candidate is `3009b1efb8685e68d9fde9d8ca2bec4c40a0c979`
and the planning intake containing this iteration prompt is
`3c6f491b5b1221cf7540fb9cda85782bcb100ab3`. Both worktrees were clean before
implementation; the raw status and HEAD records are retained in
`artifacts/logs/41-source-status.log`, `41-source-head.log`,
`41-planning-status.log` and `41-planning-head.log`.

The current source extents are 319 lines for `clock.rs`, 421 for `rack.rs`,
216 for the umbrella Clock test, 126 for the required fixture, and 715 for
the existing sample-pipeline reference. The fixture is a protected input and
must remain byte-identical. The exact source edit fence for this iteration is
therefore only `crates/oxurack-mod-clock/src/clock.rs`,
`crates/oxurack-mod-clock/src/rack.rs`, and `crates/oxurack/tests/clock.rs`.
Cargo manifests, `Cargo.lock`, the README and the fixture are protected from
changes. No TM, core/RT, MIDI, global-tempo, transport, UI, dependency or
production-test-hook work is authorized.

The repaired spawn contract is staged in two phases. First, all supplied
parameter names are inspected without decoding values; the lexically first
unknown name wins even when known invalid values are present. Second, known
fields are decoded in the fixed `bpm`, `ppqn`, `pulse_width_ms` order and the
complete candidate is validated once through `ClockConfig::try_new`. Spawn
errors are attributed to the field responsible for the ClockConfig error;
setter errors continue to be attributed to the edited field. Failed spawns
must not create a Module or Port, and invalid edits must preserve both the
Clock state and held `CurrentValue`.

The standalone oracle must cover the 120/4 width-3 fractional schedule and
its high/low windows; all non-finite and range failures; cross-field pulse
overlap; the full edit restart boundary through the low sample and next rise;
no-op/coalescing behavior; and return-to-original configuration. The rack
oracle must cover every parameter's valid and wrong `ParameterValue` variants,
all spawn failure categories and exact attribution, missing and non-Clock
entities, all getters/setters, pending restart behavior, invalid-edit
preservation, and overlap attribution.

The integration oracle must compare full named per-sample traces and
`SampleTime`, independently constructed from the fixture's rates and widths.
It must exercise repeated installation before load, manual and controlled
live grouping, the reversed fixture, pause/resume with stopped requests,
stop-while-high behavior, and reinstall after progress. Rise lists alone are
not sufficient evidence because grouping and held-value behavior can leave
them unchanged while corrupting intermediate samples.

Required guidance was rechecked before Rust edits: collaboration-framework
2.5.0, project-management 2.18.0, work-verification 2.5.0,
rust-guidelines 2.0.0 and testing 1.1.1. The Rust anti-pattern guidance was
read in full for input-path error handling, private invariant state, and
hot-path allocation/unsafe boundaries. The implementation-prompt-authoring
and testing-discipline guidance was read in full for discriminating oracles,
corrective scope, preserved failure evidence and independent verification.
No public fallible API changes are planned, so the conditional API-23/API-39,
TD-05 and DC-05 reload gate is not triggered. All six ledger rows remain
proposed-done pending CRC; this intake does not close or advance them.

The prior CRC findings F-01 through F-05 are reproduced as the direct repair
targets: spawn error attribution, lexical unknown precedence, incomplete
standalone coverage, incomplete rack coverage, and manual-only integration
evidence. No design or scope conflict was found, and no CDC escalation is
required. The previous probe and all prior validation attempts remain
historical evidence; this iteration appends new results without deleting or
rewriting them.
