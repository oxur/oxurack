# Slice02 Turing Machine adapter — CC closing report

## Return

Slice02 is returned as CC proposed-done evidence under the Three-Contributor
Workflow and Expedited Mode. The source implementation is committed at
`9ba55dd0a4c8aa7d67fe26c30f878d89ead34aa2` on
`codex/oxurack-module-behavior-installation`; the source worktree is clean.
The planning packet is committed separately after this report. CRC must
independently reproduce the six open ledger rows before acceptance or Arc10
advancement.

The adapter adds the complete 32-port, nine-parameter Turing Machine module
behind the opt-in `rack` feature. It uses context-derived seeded construction,
strict Scale and parameter validation, private cached-port state, supplied
schedule systems, rising-edge priority, delayed coherent snapshots, and
nonaccumulating write modulation. The existing standalone engine, default
dependency tree, `midi-io` path, core, Clock, RT and MIDI source remain outside
the source change.

## Row walk

| Row | CC result | Evidence | Status |
| --- | --- | --- | --- |
| A-01 | The opt-in rack feature compiles and passes its focused suite; the default package tests and normal dependency tree remain free of Bevy/core. Patch construction uses the context seed and the standalone engine remains the independent reference. | `artifacts/logs/01-tm-default-test.log`, `02-tm-normal-tree.log`, `03-tm-rack-test.log`, `04-umbrella-tm-test.log` | proposed-done; CRC pending |
| A-02 | All 32 ports and nine parameters are registered with strict types, ranges, Scale conversion, getters/setters and action mappings. Wrong variants, malformed scales, boundary values, lexical unknowns and invalid no-mutation cases pass. | `artifacts/logs/03-tm-rack-test.log`, `04-umbrella-tm-test.log` | proposed-done; CRC pending |
| A-03 | The adapter publishes every StepOutputs field as one delayed scalar snapshot, with explicit validity mapping and held values between steps. The public fixture matches a separately configured seeded standalone engine. | `artifacts/logs/03-tm-rack-test.log`, `04-umbrella-tm-test.log` | proposed-done; CRC pending |
| A-04 | Wide-gate rising edges, reset/move/clock priority, lower-edge consumption, modulation reconciliation, stop/resume pending publication and reinstall behavior pass. | `artifacts/logs/03-tm-rack-test.log`, `04-umbrella-tm-test.log` | proposed-done; CRC pending |
| A-05 | Manual/live grouping, reversed module and cable order, unrelated allocation, fresh child-process replay, changed seed/name controls and standalone comparison pass. | `artifacts/logs/04-umbrella-tm-test.log` | proposed-done; CRC pending |
| A-06 | The ten-path source allowlist, intake, raw receipts, validation matrix, checker controls, documentation and scope limits are recorded for independent review. | `artifacts/test-evidence.md`, `artifacts/validation-results.tsv`, `artifacts/logs/17-planning-validator.log` | proposed-done; CRC pending |

## Validation and limitations

The source-specific gates, umbrella tests, normal dependency tree, workspace
Clippy, formatting, strict rustdoc and diff check passed. The focused rack
suite reports 120 passed and zero ignored; the public Turing Machine integration
suite reports 5 passed and zero ignored. The workspace restricted matrices
each report 146 passed, 4 failed and 3 ignored, with the same four existing
`oxurack-rt` MIDI initialization tests failing because the host cannot
initialize MIDI support. `make check` reaches that same host-limited failure.
No RT/MIDI source was changed. `cargo outdated` is retained as network-blocked
because the restricted environment could not resolve `index.crates.io`.

The dependency future-incompatibility warning for `block v0.1.6` is retained
in the successful Clippy and rustdoc logs. These limitations do not establish
an adapter failure, and they remain visible for CRC/CDC review. No acceptance,
release, physical MIDI, Slice03 or Arc10 composition claim is made.

## Evidence inventory and handoff

* `artifacts/intake.md` — required reading, provenance, Rust-guidance decisions
  and contract readback.
* `artifacts/test-evidence.md` — implementation behavior, independent oracle,
  denominator accounting and validation limits.
* `artifacts/validation-results.tsv` — every required command, exit code,
  result category and raw-log path.
* `artifacts/logs/` — individually named command receipts, including restricted
  workspace/Make failures and the restricted freshness DNS failure.
* `ledger.md` — six rows updated only to CC proposed-done language; all remain
  `open` pending CRC.

The planning evidence commit is supplied with the final handoff after the
explicit planning file-list commit. The source commit remains
`9ba55dd0a4c8aa7d67fe26c30f878d89ead34aa2`.
