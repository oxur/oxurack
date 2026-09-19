# CC corrective assignment iteration01 — Clock diagnostics and proof matrix

## Assignment identity and authority

Execute corrective pass 1/5 for project01-mvp / arc10-turing-machine-midi /
slice01-musical-clock. Exact assignment:
`arc10-turing-machine-midi/slice01-musical-clock/cc-prompt-iteration01.md`.
Predecessor: `arc10-turing-machine-midi/slice01-musical-clock/cc-prompt.md`.
CRC review: `arc10-turing-machine-midi/slice01-musical-clock/crc-verification.md`,
iteration00 section. Source candidate:
`3009b1efb8685e68d9fde9d8ca2bec4c40a0c979`; planning intake is the commit
containing this prompt. Three-Contributor Workflow, Expedited Mode. CC
implements and returns evidence; Sofie/CRC independently reviews.

All C-01–C-06 rows remain open. Repair F-01/F-02 and complete the missing
C-01–C-04 proof described below, then rerun C-05/C-06 gates. The original
Clock API, timing/lifecycle design, package shape, nine-path source allowlist
and exclusions still govern. No TM, core/RT, MIDI, global tempo, transport,
UI or dependency change is authorized. Preserve both issued prompts.

## Required reading and intake

Use P=/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp,
S=/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation,
F=/Users/oubiwann/.codex/skills/collaboration-framework and
R=/Users/oubiwann/.codex/skills/rust-guidelines. Expand aliases in recorded
commands. Before edits, record source/planning HEAD and cleanliness, confirm
this path is the current assignment, and append an iteration01 section to the
existing `artifacts/intake.md` with loaded extents and a concise contract
readback.

Read in this order:

1. This prompt and P/arc10-turing-machine-midi/slice01-musical-clock/
   `crc-verification.md`, `slice-plan.md`, and `ledger.md`: required-full.
2. The predecessor `cc-prompt.md`: required sections Source-grounded
   implementation route, Behavioral oracles, Validation and evidence, and
   Explicit scope and return. All binding contracts carry forward.
3. S/crates/oxurack-mod-clock/src/{clock.rs,rack.rs} and
   S/crates/oxurack/tests/clock.rs at `3009b1ef`: required-full. Inspect the
   fixture and the relevant `sample_pipeline.rs` helpers/tests through
   `manual_and_controlled_live_drives_match_grouping` and
   `last_wins_is_name_stable_with_spawn_permutation_and_unrelated_entity`.
4. R/SKILL.md and R/guides/11-anti-patterns.md: required-full; the Rust skill
   requires the anti-pattern chapter first for every Rust change. Also reload
   complete API-23 and API-39 in `02-api-design.md`, TD-05 in
   `05-type-design.md`, and DC-05 in `13-documentation.md` if any public
   fallible item changes.
5. F/knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md:
   required sections Concrete implementation recipe, Design the tests and
   their oracles, Corrective iterations. Read
   F/knowledge/testing/guides/01-testing-discipline.md in full.
6. S/crates/oxurack-core/src/{module.rs,parameter.rs,port.rs,error.rs}: required
   definitions for OxurackModule, Module, Port, ParameterValue, registries and
   CoreError. S/crates/oxurack/src/lib.rs and
   S/crates/oxurack/tests/sample_pipeline.rs: required sections for install,
   driver/start/stop/advance/into_app and controlled-Live helpers.

Recover truncated reads. Reading receipts remain CC attestation. Explain in
the readback why unknown-key detection is a separate first stage, why spawn
error attribution differs from setter attribution, and how full traces make
manual/grouped-Live and pause/resume claims falsifiable.

## Binding repair: deterministic spawn validation and attribution

Change `rack.rs` so spawn validation has two explicit stages:

1. Inspect all supplied names without decoding values. If any are unknown,
   return `UnknownParameter` for the lexically first unknown name.
2. Decode known fields in fixed order `bpm`, `ppqn`, `pulse_width_ms`; validate
   the complete candidate; attribute configuration errors as follows:
   `NonFiniteBpm | BpmOutOfRange -> bpm`, `InvalidPpqn -> ppqn`,
   `InvalidPulseWidth | PulseOverlap -> pulse_width_ms`.

Do not duplicate numeric validation outside `ClockConfig::try_new`. A suitable
private shape is:

```rust
fn spawn_error_parameter(error: ClockError) -> &'static str {
    match error {
        ClockError::NonFiniteBpm | ClockError::BpmOutOfRange => "bpm",
        ClockError::InvalidPpqn => "ppqn",
        ClockError::InvalidPulseWidth | ClockError::PulseOverlap => "pulse_width_ms",
    }
}

fn validate_spawn_config(instance: &str, raw: RawConfig) -> Result<ClockConfig, CoreError> {
    raw.into_validated().map_err(|error| {
        invalid_parameter(instance, spawn_error_parameter(error), error.to_string())
    })
}
```

This is a recommended implementation shape, not compiled drop-in code. The
binding properties are the two-stage precedence, fixed known-field order,
single ClockConfig validation authority and exact parameter attribution. Setter
validation continues to attribute any candidate failure, including overlap, to
the parameter being edited. Invalid spawn creates neither Module nor Port;
invalid setters preserve all Clock state and the held CurrentValue.

## Required standalone and adapter tests

Extend tests in the existing homes. Test names and private helper names are
local discretion; the setups/actions/observations below are binding.

### C-01/C-02 standalone oracles in `clock.rs`

- Add 120 BPM / 4 PPQN / width 3 and assert rising samples exactly
  `[0, 125, 250, 375, 500]`, including high widths and intervening lows.
- Retain 137/4 and add the complete rejection categories: NaN, positive and
  negative infinity; BPM <=0 and >1000; PPQN 0 and 97; width 0 and 1001;
  overlap at 1000/30/2. Match `ClockError` variants.
- Strengthen the rate-edit sequence: after samples 0–2 are high, change to
  240 BPM / 1 PPQN / width 10. Assert sample 3 low, 4–13 high, 14–253 low and
  the next rise at 254. Keep equal/no-op and coalesced/back-to-original cases.

These values reject repeated rounded periods, missing low separation and an
epoch shifted by one sample.

### C-04 adapter failures in `rack.rs`

- Spawn NaN, infinities, 0 and 1001 BPM and match module name plus
  `param == "bpm"`. Exercise PPQN and width range/type errors and overlap with
  their exact parameter names. Count both Module and Port components before
  and after each failed spawn; both counts must remain unchanged.
- Mix an invalid known value with unknown keys in opposite insertion orders.
  Both calls must return the same lexically first unknown key. This rejects the
  current interleaved decode policy.
- Exercise wrong `ParameterValue` variants for each parameter.
- Apply valid changes through all three setter function paths and read all
  three getters after each compatible candidate. Prove a missing/non-Clock
  entity returns `UnknownModule`.
- Run the Produce system to establish output, apply a valid change to create a
  pending restart, snapshot the private Clock and held CurrentValue, then apply
  an invalid edit. Assert the snapshot is unchanged; the next execution is the
  pending low separator and the following execution starts high. Add an overlap
  edit and assert it is attributed to the edited parameter with the same
  preservation guarantee.

Do not expose private state or add a public test accessor. These adapter tests
live inside `rack.rs`, where ClockState is visible.

## Required rack integration matrix in `crates/oxurack/tests/clock.rs`

Derive `Clone + Debug + PartialEq + Eq` for the test-only GateTrace, or use an
equivalent test-only value shape. Build reusable runners that return the full
named trace plus `SampleTime`:

```text
manual(groups): install Clock and consumer twice before load; load/start;
                advance each group; optionally reinstall after progress
live(groups):   install twice before load; load/start; into_app;
                set TimeUpdateStrategy::FixedTimesteps(group); app.update
```

Compare equal-total executions for at least manual `[1105]`, manual grouped
`[1, 2, 7, 95, 1000]`, Live single/grouped steps and the reversed module fixture.
The exact grouping may differ if totals remain 1105. For every run, assert the
entire per-sample vectors against independently constructed pulse windows:

- slow 120/1 width3 rises 0,500,1000;
- fast 240/1 width2 rises 0,250,500,750,1000;
- default 120/1 width10 rises 0,500,1000 despite patch metadata BPM90;
- explicit 90/1 width10 rises 0,667;
- disconnected consumer remains low and has no rises.

Compare full traces and SampleTime across modes; rise lists alone are
insufficient. Keep the reversed fixture's unrelated consumer before the Clock
instances so entity allocation and spawn order differ.

For pause/resume, run an uninterrupted reference and a subject for the same
number of executed samples. On the subject, stop while a pulse is high, issue
zero and nonzero stopped requests, resume and finish. After excluding the
stopped requests, full trace and SampleTime must equal the reference. Reinstall
both module types after progress and show no duplicate execution or state reset.

The Live runner must follow the existing compiled sample-pipeline shape:
consume the wrapper only after install/load/start, insert/update
`TimeUpdateStrategy::FixedTimesteps`, and call `app.update()`. Do not invent a
World accessor or reuse Oxurack after `into_app`.

## Source scope, validation and return

The predecessor's nine-path allowlist remains binding. This repair should
change only:

- `crates/oxurack-mod-clock/src/clock.rs`
- `crates/oxurack-mod-clock/src/rack.rs`
- `crates/oxurack/tests/clock.rs`

Cargo manifests, lockfile, README and fixture must remain byte-identical. A
necessary additional path or contract change stops for CRC/CDC resolution.
No ignores, lint suppression, weakened assertions or production test hooks.

Rerun every predecessor validation command, not only focused tests. Add focused
commands for the new test names if useful. Preserve restricted failures and
host retries at full strength. Run the current planning checker after updating
iteration evidence. Append iteration01 sections to the existing intake,
test-evidence and closing report; update results.tsv without deleting prior
attempts. Record the independent CRC failing probe as predecessor evidence,
not as CC-authored proof.

Use exact source and planning file lists with `git commit --only`. Source and
planning commits remain separate. Planning writes stay within this slice's
existing ledger, closing report, intake/results/test-evidence/checker/logs and
iteration evidence; do not edit either issued prompt. Return exact commits,
changed paths, row-by-row status, complete commands/results, Rust-guidance
application, deviations and bubble-up. All rows remain CC proposed-done until
CRC independently reproduces the corrected candidate.
