# Slice02 Turing Machine adapter — iteration01 CC closing report

## Return and provenance

Assignment: `arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt-iteration01.md`.
This is corrective pass 1/5, returned as CC proposed-done evidence under the
Three-Contributor Workflow and Expedited Mode. The preserved initial CC report
is [iteration00/closing-report.md](artifacts/iteration00/closing-report.md); its
claims were not promoted by this correction.

Source commit: `b2ffb16d342d9a6cc800bab2713813d9646fb505`

The source began clean at `9ba55dd0a4c8aa7d67fe26c30f878d89ead34aa2` on
`codex/oxurack-module-behavior-installation`. The planning worktree began clean
at `32e693466827eeab1c842931d74772820c2f6032` on `planning`. Source was
committed first; planning evidence is committed separately after this report.
All six ledger rows remain `open`; CRC review is the acceptance gate.

## Corrective result

The correction changed tests only in the four authorized paths:

- `crates/oxurack-mod-turingmachine/src/rack.rs`
- `crates/oxurack-mod-turingmachine/src/engine.rs`
- `crates/oxurack-mod-turingmachine/src/quantizer.rs`
- `crates/oxurack/tests/turing_machine.rs`

No production sections, manifests, lockfile, fixture, core/Clock/RT code,
issued prompts, plans, CRC review/probe/receipts, or iteration00 logs changed.
The byte audit against `9ba55dd` confirms the production prefixes of
`rack.rs`, `engine.rs`, and `quantizer.rs` are identical. The integration file
contains only tests. The source commit contains exactly those four allowlisted
paths and leaves its worktree clean.

## Six-row walk

| Row | Iteration01 CC observation | Status |
| --- | --- | --- |
| A-01 | Default TM suite: 112 passed, 0 failed/ignored. The normal dependency tree contains no Bevy or `oxurack-core`. Rack feature suite: 129 passed, 0 failed/ignored. The unchanged standalone engine remains the independent oracle. | open; CC proposed-done, CRC pending |
| A-02 | Exact 32-port name/direction/kind/policy and nine-parameter/default/registration checks pass. Every registered setter/getter has explicit valid readback. Spawn and registered setters reject the wrong variants, numeric/Scale/range populations with actual module/parameter attribution; spawn entity counts and complete state/port snapshots are preserved. Rejected setters continue identically to a separately configured standalone engine. | open; CC proposed-done, CRC pending |
| A-03 | A real Produce schedule test publishes sentinel values through all 28 outputs, including all-None, Some(0), validity gates and idle holding. The public oracle compares initial, delayed published and held values on every one of 24 samples for both TMs. | open; CC proposed-done, CRC pending |
| A-04 | Scripted samples 0–10 compare complete pending snapshots and outputs against independently driven engine actions. Isolated move/reset, consumed lower-priority edges, held high gates, all specified modulation offsets, real two-cable Bipolar Sum, stopped state/SampleTime, pending publication, pause after publish, and reinstall continuation are covered. | open; CC proposed-done, CRC pending |
| A-05 | Fresh child canonical rows match byte-for-byte across runs. Seed and name controls compare corresponding row contents; renamed primary also matches the standalone engine configured from the renamed context, while secondary output remains equal. Reversal and independent unrelated-observer insertion/removal preserve named traces and SampleTime. | open; CC proposed-done, CRC pending |
| A-06 | Required intake/readback, current test evidence, validation matrix, raw logs, protected-region audit and bubble-up disposition are recorded. Structural checker passes are limited to their predicates and do not accept Rust behavior. | open; CC proposed-done, CRC pending |

The key repaired tests are `schema_declares_complete_port_and_parameter_inventory`,
`registered_setters_and_getters_cover_every_valid_value_and_alias`,
`every_invalid_spawn_variant_and_range_is_rejected_without_orphans`,
`every_invalid_spawn_and_registered_setter_is_atomic_with_seeded_continuation`,
`real_produce_publication_maps_options_and_holds_all_values_on_idle`,
`consume_priority_consumes_losing_edges_and_matches_scripted_engine`,
`held_modulation_reconciles_targets_and_registered_base_edits`,
`two_real_sum_cables_drive_write_modulation_through_core_phases`,
`pause_preserves_consumed_high_edges_pending_outputs_and_sample_time`, and the
five umbrella integration tests. Full setup/oracle details are in
[iteration01 test evidence](artifacts/test-evidence.md).

## Validation, failures and limitations

All assigned source/umbrella/core package commands pass: default TM (112), rack
TM (129), focused umbrella Turing Machine tests in default and no-default modes
(5 each), umbrella default/no-default, and core (414). Clippy with denied
warnings, formatting, both strict rustdoc commands and the source diff check
pass. The full command and exit matrix is in
[validation-results.tsv](artifacts/validation-results.tsv), with command output
in individually named `artifacts/logs/iteration01-*.log` receipts.

Restricted all-features and no-default workspace runs each exit 101 after 146
passed, 4 existing CoreMIDI initialization failures and 3 ignored RT tests.
The four failures are `test_list_input_ports`, `test_list_output_ports`,
`test_open_input_nonexistent_returns_error`, and
`test_open_nonexistent_port_returns_error`. Restricted `make check` exits 2 on
the same tests. Required host retries pass for both workspace matrices and
`make check`; each records 150 RT tests passed and 3 ignored. Restricted
`cargo outdated` exits 1 because DNS cannot resolve `index.crates.io`; the host
retry exits 0 and reports updates without changing dependencies. The
`block v0.1.6` future-incompatibility warning is retained where emitted.
No failures or ignored tests were suppressed.

Planning validator default mode and `--require-return` mode each pass with all
built-in same-predicate negative controls. Those checks establish packet
structure only. The standalone engine oracle and integration tests provide the
behavioral evidence; CRC must independently reproduce and decide acceptance.
No production defect was observed. There are no architecture, scope or
acceptance changes proposed here.

## Bubble-up to arc

F-01–F-05 are repaired proof/evidence gaps within the existing Slice02 contract;
this execution found no demonstrated need to change Arc10 design, mapping, slice
order or acceptance policy. No design escalation is requested. Slice03 MIDI
output remains held pending CRC's review and acceptance of A-01–A-06. No Arc10
composition, physical endpoint, release or wider project-gate acceptance is
claimed.

## Evidence homes

- [Iteration01 intake and contract readback](artifacts/intake.md)
- [Iteration01 test evidence](artifacts/test-evidence.md)
- [Command/result matrix](artifacts/validation-results.tsv)
- `artifacts/logs/iteration01-*.log` — raw focused, full-matrix, retry, audit and checker receipts
- [Open ledger](ledger.md) — CC proposed-done notes only; every status remains open
- [Preserved iteration00 report](artifacts/iteration00/closing-report.md)

The planning evidence commit follows the source commit with an explicit path
list. No CRC acceptance is implied by this return.
