# CRC verification — Slice01 musical Clock

## Iteration00 review — returned for correction, 2026-09-19

Reviewer: Sofie/CRC, independent of CC implementation. Workflow:
Three-Contributor Workflow, Expedited Mode. Assignment reviewed:
`arc10-turing-machine-midi/slice01-musical-clock/cc-prompt.md`.

Candidate source: `3009b1efb8685e68d9fde9d8ca2bec4c40a0c979`.
CC planning evidence: `ee99804c19d4b6f9b26c5b193cc9fa3d162f50f9`.
Both worktrees were clean at intake.

CRC loaded collaboration-framework 2.5.0, including engineering-methods
prompt-authoring 1.15.0, and Rust guidelines 2.0.0. The initial packet does use
the current authoring contract: exact public signatures and the accumulator
pseudocode are in the required-full slice plan; the prompt supplies file/symbol
steps, author-applied Rust decisions and discriminating behavioral values. CC's
intake records the same installed versions and Rust sections. Absence of a Rust
code block in the prompt file itself is not a guide failure because the prompt
requires the complete slice plan that contains those shapes.

CRC reproduced these candidate checks:

- `cargo test --locked -p oxurack-mod-clock`: 6 unit tests and 1 doctest pass.
- `cargo test --locked -p oxurack-mod-clock --features rack`: 10 unit tests and
  1 doctest pass.
- `cargo test --locked -p oxurack --test clock`: 2 integration tests pass.
- The current planning checker passes its baseline and rejects all three
  negative controls.

Those passes establish the behaviors they exercise, but they do not cover the
complete ledger contract. Review found the following unresolved findings.

### F-01 — spawn validation attributes BPM failures to the wrong parameter

`crates/oxurack-mod-clock/src/rack.rs` decodes `bpm` as a float, then maps every
`ClockConfig::try_new` failure to `pulse_width_ms`. Non-finite and out-of-range
BPM therefore return `CoreError::InvalidParameterValue { param:
"pulse_width_ms", ... }`, contrary to the binding spawn-error policy.

CRC built a separate temporary public-API probe against the committed crates.
Its assertion that NaN, 0 and 1001 BPM report `param == "bpm"` fails. The raw
failure is preserved in
`artifacts/logs/40-crc-invalid-bpm-attribution.log`. This is an implementation
defect in C-04, not merely missing test evidence.

### F-02 — unknown-key precedence is not the specified two-stage policy

`decode_spawn_parameters` sorts all keys but decodes known keys as it walks.
With invalid `bpm` and unknown `zeta`, it reports the BPM failure before the
unknown key. The contract requires the lexically first unknown key to be
rejected before validating known values, followed by fixed bpm, ppqn, width and
overlap validation. The existing test contains two unknown keys and cannot
distinguish these policies.

### F-03 — C-01/C-02 boundary evidence is incomplete

The standalone tests cover the fractional 137/4 sequence and basic edit shape,
but omit the ledger's 120/4 golden rise list and the complete invalid matrix.
The edit test stops after the fresh high block and does not prove low sample 14
or the next rise at sample 254. No test applies an invalid adapter edit while a
restart is pending and proves the pending low separator, engine state and held
output survive.

### F-04 — C-03 integration evidence does not exercise the assigned modes

The umbrella tests use only the manual driver. They do not run controlled Live
`FixedTimesteps`, compare full named traces across single/grouped manual and
Live execution, or compare stop/resume with an uninterrupted reference after
removing stopped requests. The helper installs each module once before load,
despite the required twice-install pre-load oracle. Reversal assertions cover
only rise lists, so they do not establish full trace equivalence.

### F-05 — C-04 parameter failure and preservation matrix is incomplete

The rack tests exercise only the BPM setter, one NaN failure, one valid edit,
and getters. They do not prove all setter/getter pairs, wrong variants, spawn
range/non-finite attribution, missing entity, overlap attribution, zero
module/port creation on every spawn failure, map-order independence, or
phase/pending/output preservation after invalid edits.

## Row verdict

| Row | CRC disposition | Reason |
| --- | --- | --- |
| C-01 | open | Core algorithm looks conforming; required integer/boundary matrix is incomplete. |
| C-02 | open | Edit implementation looks conforming; exact continuation and invalid-pending preservation are unproved. |
| C-03 | open | Manual fixture passes; controlled Live, full grouping and uninterrupted-reference oracles are absent. |
| C-04 | open | F-01 is reproduced; F-02 and the missing failure matrix remain. |
| C-05 | open | Prior gates pass, but the corrected source must rerun the complete matrix. |
| C-06 | open | Candidate is not accepted; iteration evidence and current-assignment checks remain. |

Verdict: Slice01 remains open. CC's report is retained as proposed-done evidence
for iteration00. CRC issues `cc-prompt-iteration01.md` as corrective pass 1/5.
No architecture, scope, acceptance or source-fence change was found, so no CDC
escalation is required.

## Bubble-up check

The Clock remains the correct first piece of Arc10 and the approved API/timing
design remains viable. These findings concern implementation diagnostics and
missing proof inside the existing slice. Arc10's slice breakdown, sequencing
and scope do not change.

## Iteration01 acceptance — 2026-09-19

CRC independently reviewed clean source
`a19cfd1a134c5934d270dc0ba72334d8f167fe2f` and CC evidence
`ef9d99fe7ab4f38ca46a069c4e87cd4d2e10873f`. The correction from
`3009b1e` changes exactly `clock.rs`, `rack.rs`, and the umbrella Clock test;
the protected manifests, lockfile, README and fixture are unchanged.

The repaired adapter rejects the lexically first unknown key before decoding,
then validates known fields in fixed order and attributes each configuration
failure to its responsible parameter. CRC's separate temporary public-API
probe, which failed iteration00 on NaN BPM attribution, now passes. Source
inspection and focused tests also reproduce the complete 120/4 and 137/4
windows, edit continuation through rise 254, invalid-pending preservation,
all setter/getter and failure paths, full manual/controlled-Live traces,
pause/resume, reinstall, and reversed allocation controls.

CRC reproduced these gates:

- locked standalone, rack-feature, umbrella default/no-default and core tests;
- both full workspace feature matrices;
- normal dependency tree, all-target Clippy with denied warnings, formatting,
  strict workspace and no-default umbrella rustdoc, and `git diff --check`;
- the current planning checker with missing-row, stale-assignment and invalid-
  status negative controls;
- host `make check` and dependency freshness. The restricted attempts showed
  the recorded four CoreMIDI initialization failures and crates.io DNS failure;
  the corresponding host executions passed. Three existing RT timing tests
  remain ignored under their declared permission/timing conditions.

| Row | CRC disposition | Evidence |
| --- | --- | --- |
| C-01 | done | Exact integer/fractional windows and complete invalid matrix reproduce. |
| C-02 | done | Edit, invalid-state and pause/resume transitions reproduce. |
| C-03 | done | Complete named traces agree across manual/Live groupings and reorder/lifecycle controls. |
| C-04 | done | Independent probe and adapter matrix reproduce precedence, attribution and no-mutation behavior. |
| C-05 | done | Required package, workspace, lint, format, docs, Make and dependency gates pass on the host. |
| C-06 | done | Revisions, exact correction scope, durable evidence and checker controls agree. |

Verdict: Slice01 is accepted and closed at source `a19cfd1`; one of five
permitted corrective passes was executed. This proves the reusable Clock
prerequisite. It does not prove a TM adapter, MIDI output, physical endpoint,
release timing, global coverage or musical acceptance.

The bubble-up check found no Arc10 architecture or acceptance change. Slice02
may proceed under the existing design after its required complete engine
inventory and mapping are made explicit.
