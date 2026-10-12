# CRC independent acceptance — Arc12 Slice02

2026-10-12, Sofie/CRC. Three contributors, Expedited Mode. All five M rows
independently accepted at source `20c60efa63c4fbded5c0036a3bcd10ff2c4ba4d9`.
Planning intake `d8761d0ee6383fe5a830585ef457f584d8ca4383`, including CC return
`ae2c1884d797f99dd9295237cefa489c8a157765`. Initial prompt SHA-256:
`d8cdefd5cf8dd993191c5e38521a556656fc65d35f7f971abcab73984cc5dffb`.
Source and planning were clean at intake; protected main remains a7ce6e6.

## Review and row walk

Inspected all new production paths: core target/value validation, active-handle
module/port resolution, cable occurrence sorting, actual previous value capture,
mutation and exactly-one emission; typed errors and exports; SetLabel dispatch;
facade direct delegation; runnable example and README. The commit changes exactly
the nine authorized paths. No engine, fixture, persistence, graph or scheduler edit.
Read core selector tests, complete facade inline labels tests and complete public
labels integration tests. Native runs below independently reproduce their behavior.

| Row | Independent evidence and disposition |
| --- | --- |
| M-01 | Done: raw reads/clear/equal, rack full-list notes, validated sorted tags, all supported targets, active/stale exclusion and parallel ordinal/Entity selection. Source resolves actual ChildOf ports through PatchHandle; no public world escape required. Both facade feature modes and core tests pass. |
| M-02 | Done: shared helper reads actual previous components before editing; DirectApi and explicit Command routes emit once, typed failure returns before mutation/emission. Mixed operation-boundary tests and cable identity tests pass. Tests exercise complementary fields/cases rather than every possible cross-product; source inspection completes the route/field review. No automatic queue consumer or global cross-type message order claimed. |
| M-03 | Done: raw component values, default graph fallback and fresh snapshot/save/reload tests pass. Actual annotated RON preserves version1.0, seed42, BPM120, parameters/transforms and all fixed metadata. Fresh graph builders read ECS labels. No engine-state checkpoint or in-place old-entity retirement claimed. |
| M-04 | Done: actual Clock/LFO/TM/MIDI control test passes at completed200 and pending252, preserving time/driver/transport, full Scope records, MIDI admission and sole drain. Event251 remains pending exactly once; continuation through sample1000 agrees. Reviewed helpers never tick, drain or reset state. |
| M-05 | Done: annotation reaches300 samples; two fresh annotated traces and original fixture agree at70 rows/hash, Scope stdout agrees; original MIDI13-row hash retained. Native checks pass on the host retry. CC separately accounts for six inherited Arc03 rows; those historical deferred rows and final composition ownership remain intact. |

CC intake/readback coherently covers constraints and actual source. Its reading
receipt groups some skills/guides instead of itemizing every loaded extent; that
is limited CC attestation, not independent proof of comprehension. CRC reviewed
actual behavior/source and reproduced checks. Future prompts require itemized
extents and conditional applicability in the same closing record.

## Native reproduction

All commands executed in the assigned source worktree at20c60ef; no source edits.
Temporary receipts: `/tmp/oxurack-crc-20c60ef-uiv3sk6e` (01.log–09.log,
results.json, host-make-test.log and replay/). These are transient diagnostic files,
not promised durable artifacts; this record retains commands/results and limits.

| Command | Exit and observation |
| --- | --- |
| cargo test --locked -p oxurack-core | 0; 417 unit, 5 integration, 9 seeded, 2 doctests passed |
| cargo test --locked -p oxurack --lib --test labels --test persistence --test scope --test lfo --test midi_output --example save-live-patch --example annotate-live-patch --example turing-machine-midi | 0; 44 lib, 4 labels, 2 persistence, 6 Scope, 12 LFO, 2 MIDI, 26 runner tests passed; save/annotation examples have zero unit tests |
| Same facade command with --no-default-features | 0; 42 lib; other denominators unchanged |
| make test (restricted) | 2, Cargo101; RT146 passed, four existing CoreMIDI initialization failures, three existing ignores. This attempt failed. |
| make test (exact unchanged host-access retry) | 0; full workspace passed; RT150 passed, zero failed, three existing ignores |
| make lint | 0; includes formatting |
| cargo clippy --locked -p oxurack-core -p oxurack --all-features --all-targets -- -D warnings | 0 |
| RUSTDOCFLAGS='-D warnings' make docs | 0 |
| cargo build --locked -p oxurack --example annotate-live-patch --example turing-machine-midi | 0 |
| git diff --check | 0 |

The four failed restricted tests were midi_io::tests::test_list_input_ports,
test_list_output_ports, test_open_input_nonexistent_returns_error and
test_open_nonexistent_port_returns_error. Existing RT priority/timing ignores and
block0.1.6 future-incompatibility advisory remain visible. No gate was suppressed.

## Executed fresh-process demonstration

Ran the prompt's actual cargo-run sequence: annotate the original LFO patch to a
temporary annotated.ron, inspect that file, run it twice for10000 samples, run the
original LFO for10000, run annotated with Scope view, and original MIDI for2000.
Every successful command exited0. All four LFO traces are byte-identical70 rows:
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
Original MIDI13 rows:
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.
Scope view: completed10000, retained10070, overwritten0, signals10000,
generated70, sent0. Annotation prints the raw labels and completed_samples300.
Missing patch, malformed RON and save-to-existing-directory each exited1 with
diagnosis and empty stdout. No MIDI endpoint opened or musical/hardware witness.

## Disposition and bubble-up

Slice02 closes with five done rows, no corrective iteration or design amendment.
Arc12 R-01–R-04 and project P-01–P-06 remain open; Slice03 is the next approved
product increment. Arc03 historical deferred criteria remain final Arc12 assembly
obligations; metadata delivery alone does not certify lifecycle/removal or the
old cross-type patch-load event timeline. Original CC report/prompt remain intact.

Nonblocking documentation follow-up owned by Slice03: LabelChanged rustdoc says
rack previous values are always present, but preserved PatchLoad events use None.
Qualify that sentence as runtime writes; retain PatchLoad history semantics.
ChangeSource::Command's adjacent description should include SetLabel/cable dispatch.
Production behavior is correct; this does not invalidate the five accepted rows.
