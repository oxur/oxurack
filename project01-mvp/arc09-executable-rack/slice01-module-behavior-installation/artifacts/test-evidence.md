# CC intake and contract readback

Recorded before dependent implementation edits for
`cc-prompt-iteration01.md`. This is CC intake evidence and proposed-done
support; it does not close I-01--I-06 or substitute for CRC reproduction.

## Intake record

| Item | Source state | Loaded extent | Gap / disposition |
| --- | --- | --- | --- |
| Active assignment | planning `4e50a3a0171b44344f19e4bbf1b8307dafec741c` | `cc-prompt-iteration01.md`, lines 1-300, required-full | Complete; replacement prompt is active and predecessor remains reference-only. |
| Planning local authority | planning `4e50a3a0171b44344f19e4bbf1b8307dafec741c` | `AGENTS.md` lines 1-46; slice `slice-plan.md` lines 1-175; slice `ledger.md` lines 1-14, all required-full | Complete; six ledger rows remain open. |
| CDC directive04 | planning `4e50a3a0171b44344f19e4bbf1b8307dafec741c` | `project01-mvp/cdc-directive04.md`, lines 1-130, required-full | Complete; authoring amendment does not change implementation authority. |
| CDC directive03 | planning `4e50a3a0171b44344f19e4bbf1b8307dafec741c` | `project01-mvp/cdc-directive03.md`, lines 1-63, required-full | Complete; slice01 behavior and successor boundaries remain binding. |
| Execution contract | planning `4e50a3a0171b44344f19e4bbf1b8307dafec741c` | opening authority and §1, lines 1-59; §5 Q-01 paragraph, lines 192-197 | Complete for the assigned sections; future sections were not used as current scope. |
| Arc plan | planning `4e50a3a0171b44344f19e4bbf1b8307dafec741c` | `arc09-executable-rack/arc-plan.md`, lines 1-45, required-full | Complete; slice02/03 remain future designs. |
| Project plan | planning `4e50a3a0171b44344f19e4bbf1b8307dafec741c` | “Purpose and definition of done” lines 13-35; “Current roadmap — v3.0” through “Quality dispositions” lines 224-266; “Prompt-authoring amendment — v3.1” lines 284-320 | Complete for named sections; v3.2 return was visible and confirms this replacement is issued. |
| Prompt-authoring guide | installed collaboration-framework 2.4.0 | `knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md`, lines 1-368; required “Required reading and CC intake” lines 194-316 | Complete; bounded reads recovered all required content. |
| Expedited Mode | installed collaboration-framework 2.4.0 | `knowledge/project-management/guides/README.md`, lines 1-178; required section lines 87-122 | Complete; explicit-path commits and full validation remain required. |
| Rust entrypoint | installed `rust-guidelines` | `SKILL.md` “Document Selection Guide” lines 89-150 and “Strength Indicators” lines 477-525 | Complete; MUST/SHOULD/CONSIDER/AVOID distinctions retained. |
| Rust API/error guidance | installed `rust-guidelines` | API-23 (`02-api-design.md` lines 710-760); EH-01/EH-04 (`03-error-handling.md` lines 1-150), EH-14 lines 529-580, EH-18 lines 720-770, EH-29 lines 1133-1175 | Complete for named sections. |
| Rust ownership/traits guidance | installed `rust-guidelines` | OB-02 (`04-ownership-borrowing.md` lines 37-71), OB-12/OB-13 lines 417-486; TR-09 (`06-traits.md` lines 355-410) | Complete for named sections. |
| Rust anti-pattern/docs guidance | installed `rust-guidelines` | AP-09 (`11-anti-patterns.md` lines 247-290), AP-25 lines 782-815; DC-05/DC-06 (`13-documentation.md` lines 142-210), DC-12 lines 387-425 | Complete for named sections. |
| Testing discipline | installed `testing` | `guides/01-testing-discipline.md`, lines 1-91, required-full | Complete. |
| Validation gates | installed `testing` | `guides/03-validation-gates.md`, lines 1-82, required-full | Complete. |
| Core module/error/parameter/patch source | source branch `codex/oxurack-module-behavior-installation` at `a7ce6e61536ca96b6aeb2227ee06e430ed8df162` | `module.rs` lines 1-340; `error.rs` lines 1-80; `parameter.rs` lines 1-230; `patch.rs` lines 1-135, 480-575, 528-850, 820-930 | Complete named definitions/imports/docs; overlap was used to recover the full patch application function. |
| Umbrella/core/tick source | source branch at `a7ce6e61536ca96b6aeb2227ee06e430ed8df162` | `oxurack/src/lib.rs` lines 1-560; core `lib.rs` lines 120-190; `tick.rs` lines 1-90 | Complete named definitions, constructors, helpers, tests and schedule wiring. |
| Locked Bevy APIs | local Cargo registry Bevy 0.18.1 | `bevy_ecs/src/schedule/set.rs` lines 15-65 and 165-188 plus label macro lines 86-135; `bevy_app/src/app.rs` lines 300-360 | Complete named schedule-label alias/interning and `App::add_systems` definitions. |
| Rustdoc diagnostics and source comments | planning `4e50a3a0171b44344f19e4bbf1b8307dafec741c` plus source pin | `cdc-review03-rustdoc.log` lines 1-163; `length.rs` 1-100; `write_knob.rs` 1-75; graph `lib.rs` lines 1-25, 300-318, 848-864 | Complete eleven diagnostics and corresponding comments. |
| Predecessor prompt | planning `4e50a3a0171b44344f19e4bbf1b8307dafec741c` | `cc-prompt.md` reference-only | Not loaded; no implementation dependency. |

No required item was inaccessible or truncated after recovery. The only
conditional/reference item was the predecessor prompt; it is reference-only
and intentionally not an execution authority.

## Contract readback

The binding behavior is from execution-contract §1 and the slice ledger. Add a
default `OxurackModule::register_systems(&mut App, InternedScheduleLabel)` hook
so existing implementations continue to compile, and add
`Oxurack::try_install<M>() -> Result<&mut Self, CoreError>` with a concrete
`CoreError::ModuleKindConflict { kind: String }`. `Oxurack` must keep a private,
rack-local `ModuleKind -> TypeId` identity map. A new kind reuses the existing
metadata, spawner, setter and getter registration, invokes the hook once with
`FixedUpdate.intern()`, then records success. A same-type reinstall is an
unchanged success. A different type claiming an installed kind returns the exact
typed error before any registry, accessor, schedule or resource mutation.

The existing `install` method remains the chaining convenience and delegates to
`try_install`; its documented panic is only the programmer-error counterpart
for a collision. Patch loading continues through `load_patch` and the existing
spawner path, so user-selected/dynamic input uses the fallible API and never
enters that convenience panic path. A hook panic remains a programmer failure;
there is no `catch_unwind`, rollback or transaction claim. `TypeId` is local
bookkeeping only and never enters persistence, seeds or event ordering. The
existing fixed schedule, `Produce -> Propagate -> Consume` ordering, timing,
seeding, `SimulationRunning`, and raw metadata-only `ModuleRegistry` behavior
stay unchanged. The only other source changes permitted are the exact
behavioral integration test/fixture and comment-only rustdoc repairs in the
three named legacy files; no manifest, dependency, scheduler, device, MIDI or
future-slice work is authorized.

The tests must prove the public RON-load path and actual per-instance behavior:
the two named instances start at totals `0/0` with increments `2/3`, one direct
`FixedUpdate` reaches `2/3`, `Update` does not advance them, and another fixed
run reaches `4/6`. After changing one parameter through the existing registry,
same-type reinstall must preserve state/accessors and the next fixed run must
reach `8/9`. A different Rust type with the same kind must return the exact
conflict and leave original metadata, spawner, accessors, hook behavior and
fixture state intact; independent racks, a legacy no-hook module, the
convenience panic, and graph-on/off behavior cover the remaining rows. A hook
counter is supplemental: **a hook counter alone does not prove installed
systems run**, so the oracle must inspect the loaded entities' actual state.

I-01 through I-06 remain open until CRC independently reviews the exact source
and planning commits. Every listed focused, package, workspace, feature,
clippy, format, strict-rustdoc, make/check, outdated, diff and planning
verifier attempt must retain its real status and raw log. The verifier's known
assignment mismatch is a structural limitation to preserve and explicitly
disposition; it cannot be silently converted to a pass. No ignored
physical/timing tests or historical coverage target waiver is allowed.

## Iteration02 intake and corrective contract readback

This intake was completed before dependent iteration02 edits. The active
planning assignment is `c12ab9d7f19daf5c7510e5697fb64129ddf5bbbb`; its clean
predecessor/CDC acknowledgement is `dcab7931f8379380e0054c9b44b7b7d82b9be597`.
The source worktree is clean at `d43078ea9c71bba10035699c9914bef3891f1d83`.
The first corrective pass is therefore active; the held initial prompt remains
unexecuted and iteration01 remains historical candidate work.

| Required item | Revision/state and loaded extent | Result or gap |
| --- | --- | --- |
| Active prompt, slice plan, ledger | planning `c12ab9d`; prompt lines 1-263, slice-plan lines 1-266, ledger lines 1-15; required-full | Complete; iteration02 is the current assignment, I-01/I-03/I-04 are CRC-done only at `d43078e`, I-02/I-05/I-06 remain open. |
| Planning instructions | planning `c12ab9d`; root `AGENTS.md` lines 1-46, required-full | Complete; three-contributor Expedited Mode, explicit scoped commits and planning/source separation retained. |
| CDC and CRC authority | planning `c12ab9d`; `cdc-directive05.md` lines 1-177 and `crc-verification.md` lines 1-70, required-full | Complete; D1/D2/D3 authorize only the four source corrections, checker repair, live reinstall proof and evidence repair. |
| Execution contract and arc plan | planning `c12ab9d`; execution-contract lines 1-201 opening and §1 loaded; arc-plan lines 1-45 required-full | Complete; production behavior/API/scheduler/seed/MIDI boundaries remain unchanged and slice02/03 stay future work. |
| Project plan | planning `c12ab9d`; project-plan lines 13-35, 224-266, 341-380 loaded as named sections | Complete; inherited gates and Q-01/Q-02/Q-03 ownership remain binding. |
| Predecessor prompt | planning `c12ab9d`; iteration01 lines 94-273 loaded for binding sequence, fixture/oracles and validation/return | Complete; the prior candidate and its gate matrix are preserved; the old assignment identity is historical. |
| Prior evidence | planning `c12ab9d`; closing-report lines 1-119, validation TSV lines 1-21 and test-evidence lines 1-81 loaded | Complete; prior failures/log paths remain evidence. Ten nonexistent historical cwd cells are confirmed by path existence checks and are corrected as metadata only. |
| Authoring/project-management guides | installed collaboration-framework 2.4.0; authoring intake lines 194-316, Expedited Mode lines 87-122, top-down iteration section loaded | Complete; fresh intake/readback and later explicit-path commits required. |
| Rust/testing guides | installed rust-guidelines; Strength Indicators lines 477-525, CG-L-01, OB-19, AP-01 loaded; testing guides 01 and 03 required-full | Complete; all-targets Clippy stays required, `mem::take` is the bounded Vec transfer, and no library lint attribute is allowed. |
| Source rack/test boundary | source `d43078e`, clean; `oxurack/src/lib.rs` lines 1-668, integration test lines 1-452, RON fixture lines 1-29, module trait lines 130-230 and parameter registry lines 95-180 loaded | Complete; `into_app(self)` consumes the rack, so the new after-edit oracle belongs in the existing unit-test module using private test-only world access. |
| Source Clippy sites | source `d43078e`, clean; graph helper lines 1010-1055 and bridge test lines 450-510 loaded | Complete; only the let-chain and `std::mem::take` test edits are authorized. |
| Planning checker | planning `c12ab9d`; `verify-planning.py` lines 1-88 required-full | Complete; all historical structural checks stay, only the current-assignment assertion/output may change. |
| Reference-only history | directive04, held initial prompt and crc-escalation05 were not loaded for implementation decisions | Conditional reference-only material; no unresolved conflict required them. |

No required item was inaccessible or left truncated after contiguous recovery.

The corrective contract is narrow. I-02 needs a test-local module with a real
fixed-schedule system and separate hook-invocation/system-run observations:
load the static public fixture, run two fixed schedules to `(4,6)`, edit
alpha through the registered setter to `4`, release every world/resource
borrow, reinstall the same type while `rack` is still owned, assert state and
getters remain `(4,6)` and `(4,3)`, then run once more to `(8,9)`. The existing
integration reinstall test remains public `into_app` coverage and is renamed
only to describe its before-edit boundary.

I-05 requires the full all-targets workspace Clippy command and complete
planning checker; `make check` and targeted Clippy cannot substitute for
either. The only source repairs are the diagnosed graph test let-chain and
bridge test `std::mem::take`, with no `#[allow]` or production lint policy.
The checker must parse exactly one canonical backticked current-assignment
field, validate its in-slice regular file and matching history row, retain the
initial/iteration01 prompts, and prove six temporary negative/alternate cases
without mutating the real planning tree.

I-06 requires correcting exactly the ten nonexistent cwd cells while retaining
their original commands, baseline HEAD, exits, failures, retries and raw logs.
Those historical runs were dirty/uncommitted; later `d43078e` identity is
reported only as CC attestation/uncertain unless fresh evidence supports it.
New rows must record their actual HEAD and clean/dirty state. The iteration02
report must explicitly bubble up delivered capability, newly discovered gaps,
and scope-as-specified versus delivered/deferred work. No sample, seed, MIDI,
coverage-target or future-slice obligation moves into this corrective pass.

## Iteration02 provenance correction record

The original 20-row table was inspected by resolving every `cwd` field. Exactly
ten cells (the focused no-default/package/workspace rows, their retries where
present, full Clippy, and the targeted core/format rows) named the nonexistent
`/Users/oubiwann/lab/oxurack/...` checkout. They were corrected to the actual
implementation worktree `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`.
Commands, exits, results, retry distinctions and log paths were preserved.

Each historical row now has `source_state=dirty/uncommitted` and retains
`source_revision=a7ce6e61536ca96b6aeb2227ee06e430ed8df162`, which was the HEAD
reported when the implementation tree was being validated. The later
`d43078e` identity is recorded only as CC attestation/uncertain mapping; it is
not substituted for the historical HEAD. Iteration02 dirty rows use the same
explicit state, and clean-candidate rows use their actual committed HEAD
`6a31c0f99241eccb0dcf0f23e9f64ac9dec2f94a` with `source_state=clean`.

The new rows retain both restricted and host outcomes. The clean restricted
`make check` run reproduced four MIDI initialization failures and one timing
failure; the authorized host retry passed. No ignored physical/timing tests
were run, and no raw log was rewritten to turn a failure into a pass.
