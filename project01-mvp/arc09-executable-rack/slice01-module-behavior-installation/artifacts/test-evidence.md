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
