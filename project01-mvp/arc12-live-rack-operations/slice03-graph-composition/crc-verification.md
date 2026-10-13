# CRC review — live graph composition

2026-10-12, Sofie/CRC, three contributors, Expedited Mode. **Initial return is
not accepted; Slice03 stays open and iteration01 is issued.** This review found
missing required behavioral evidence, not a reproduced production defect. No
source repair, endpoint connection, scope amendment or Slice04 opening by CRC.

## Candidate and authority

Source clean at `981964bc5ebc034b42e81dd8d3222f1213c1a9bb`; planning intake clean
at `544b89d296ef7635b9a6f15f06a64b6775468277`; protected main unchanged at
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. Assignment reviewed:
`arc12-live-rack-operations/slice03-graph-composition/cc-prompt.md`, SHA-256
`28b9d8c6676860ae0b17c4b9346fe1479829e56b3a345a79b75c992abbab011e`.
Five opening rows and five CC proposed-done rows; none removed or accepted here.

Refreshed current plans, directives08/11, framework2.6.0, guide07/template,
PM closing/iteration mechanics, independent verification and Rust2.0 topic
guidance. Inspected all eleven source-diff paths and the new topology/facade/
integration tests, compared selector resolution with label.rs, and inspected
existing persistence tie-order witnesses. CC's itemized intake/readback is
attestation; coherent prose does not discharge absent execution cases below.

The preflight/owned commit sequence, transform table, checked active ordinal,
one-only removal and optional synchronous maintenance agree with the approved
design on inspection. label.rs was left unchanged: duplicated resolution and
effective-order keys currently agree. Shared extraction was an implementation
recommendation; no refactoring iteration is required merely for that choice.
Graph sorting retains endpoint prefix before ordinal/fallback. Legacy loader
compatibility and old-entity retirement remain Slice04/final Arc12 owners.

## Reproduced checks at 981964b

All commands ran in the assigned source worktree. Transient receipts:
`/var/folders/dt/kc8ndn6d3kx8c1jnxr05drsh0000gn/T/oxurack-crc-981964b-o5_6yw7v`.
These temporary logs are diagnostic support, not new durable slice artifacts.

| Command | Exit and observations |
| --- | --- |
| `cargo test --locked -p oxurack-core` | 0; 423 unit, 5 integration, 9 seeded integration, 2 doctests passed |
| `cargo test --locked -p oxurack-graph` | 0; 66 unit and 1 integration passed |
| `cargo test --locked -p oxurack --lib --test cables --test labels --test persistence --test scope --test lfo --test midi_output --example rewire-live-patch --example annotate-live-patch --example save-live-patch --example turing-machine-midi` | 0; 46 library, 3 cables, 4 labels, 2 persistence, 6 Scope, 12 LFO, 2 MIDI and 26 runner tests passed; other example harnesses 0 tests |
| Same facade command with `--no-default-features` | 0; 44 library, otherwise same counts; graph absent |
| `make test`, restricted | 2; RT 146 passed, 4 CoreMIDI initialization failures, 3 existing ignores |
| Unchanged `make test`, host-access retry | 0; full workspace passed; RT 150 passed, 3 existing ignores |
| `make lint` | 0 |
| `cargo clippy --locked -p oxurack-core -p oxurack-graph -p oxurack --all-features --all-targets -- -D warnings` | 0 |
| `RUSTDOCFLAGS='-D warnings' make docs` | 0 |
| `cargo build --locked -p oxurack --example rewire-live-patch --example turing-machine-midi` | 0 |
| `cargo build --locked -p oxurack --no-default-features --example rewire-live-patch` | 0 |
| `git diff --check` | 0 |

The restricted failures were `midi_io::tests::test_list_input_ports`,
`test_list_output_ports`, `test_open_input_nonexistent_returns_error` and
`test_open_nonexistent_port_returns_error`: CoreMIDI could not initialize. The
host retry is a separate observation, not a rewrite of the failed attempt.
Existing RT priority/timing ignores and block0.1.6 future advisory remain visible.

## Executed musical recipe

Ran the actual cargo-run example with original LFO input, temporary saved RON,
then two fresh 10000-sample runner processes and Scope-view replay. Successful
commands exited0. Default example reached300 samples, occurrence0, expected
description/constant transform and graph counts8→9; no-default reached300 without
graph output. Saved RON contains the constant+.25 connection and description.
All three rewired traces agree, 82 rows, SHA-256
`f1849af1c62dd1718a37a29d97fa46cdbe4255ef9e9e12c211a7b76c202deadd`.
Scope retained10082, signals10000, generated82, sent0, overwritten0; observed
target modulation+.25. Original LFO10000 remains70 rows/hash
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`;
MIDI2000 remains13 rows/hash
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.
Missing patch, malformed patch and save-to-directory exited1 with diagnosis.
No endpoint opened; offline output is not hardware or lifecycle acceptance.

## Findings and row walk

**F-01 — G-01/G-04: required runtime parallel-order proof is absent.**
`tests/cables.rs::live_patch_unpatch_and_labels_follow_exact_occurrences` proves
occurrence shifting, labels and graph counts but never ticks parallel candidates
to assert their LastWins winner. Existing
`persistence::tests::parallel_cable_tie_order_survives_snapshot_and_fresh_load`
only saves loaded [.75,.25]; it never adds .5 then removes that runtime cable.
The required [.75,.25]→[.75,.25,.5]→[.75,.25] effective-value/save/reload sequence
is therefore missing. The raw topology fallback test removes one equal ordinal
but does not label/select the same opposed-ordinal/absent cable or test insertion
before an absent-order cable. The new graph fallback test also hardcodes allocator
order for equal ordinals instead of deriving the actual Entity comparison.

**F-02 — G-02/G-03: call-boundary failure and identity evidence is incomplete.**
`topology::tests::transforms_validate_all_supported_kind_pairs_and_numeric_edges`
mostly exercises private validation, not the public mutator's atomic failure
boundary. Cycle/fan-in/overflow tests observe selected index/dirty/message totals,
but don't cover both index directions, cached order, actual cable values or
exact event identities. Facade pending252 negative checks observe time/Scope/
admission but not graph/topology/messages/driver/running/continuation. Successful
command tests check provenance/ports but not actual created/removed cable IDs;
the explicit command→GraphMaintenance path has no actual command witness. These
are missing checks prescribed by the initial contract, not claims that mutation
currently leaks or messages are currently wrong.

**F-03 — G-04: changed-topology engine continuity is not exercised.**
`cables::tests::live_rewire_preserves_engines_scope_and_pending_sample` restores
the original Affine(.5,0), providing useful original-continuation evidence.
The constant+.25 integration test loads fresh engines; the example continues live
but does not compare LFO output to an untouched control. Neither establishes the
explicit required live constant+.25 target with unchanged LFO phase/Scope
continuation against that control. Preserve the passing same-transform test and
add the missing differing-transform comparison.

**F-04 — G-05: historical reconciliation remains too general.**
The report names eight Arc04 rows, but says whole foundation/navigation/diagnostic
reconciliation remains future work and asserts all18 queries are covered without
the requested query→native-test→observable mapping. The initial assignment makes
the concrete delivered-subset/remaining-owner walk part of this slice, while
final historical acceptance stays CDC's Arc12 gate. Complete that bounded walk
in the existing report; do not invent tests for already witnessed queries or
close historical ledgers. Arc05 lifecycle/handles/queue obligations remain Slice04.

| Row | Current disposition |
| --- | --- |
| G-01 | open; ordinary public occurrences reproduced; F-01 ordering/selector interaction remains |
| G-02 | open; inspected preflight and existing negatives pass; F-02 complete atomic boundary proof remains |
| G-03 | open; inspected provenance/immediate graph and existing tests pass; F-02 actual IDs/command maintenance/failure messages remain |
| G-04 | open; pending sample251 and fresh literal replay reproduce; F-01/F-03 interactions remain |
| G-05 | open; executable/native/hash checks reproduce; F-04 concrete inherited reconciliation remains |

## Bubble-up and next assignment

No architecture, slice breakdown or acceptance amendment is required. This is
one collected correction of the existing proof contract, iteration1/5. Source
production can remain unchanged unless the new tests reveal a real defect;
report that result before repair. The five rows, initial prompt and CC report
remain preserved. Current assignment is `cc-prompt-iteration01.md`; Slice04 and
Arc12 composition stay held. Future review may retain the independently reproduced
unchanged production/docs/build/demo claims above at this revision/date after
inspecting the test-only delta and inputs; affected tests must execute fresh.
