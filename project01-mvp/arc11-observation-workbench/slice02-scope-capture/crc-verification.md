# Scope capture — CRC independent verification

2026-10-11, Sofie/CRC; Three-Contributor Workflow, Expedited Mode.
Verdict: **accepted; S-01–S-05 done; Slice02 closed**. No source edits by CRC.
Source `6c56e8460b693c285b8052633165e66129318604`, clean; predecessor
`c4b517c89f51d44c0eb1bec4a7b891984d3e0a92`. CC return planning
`f2adf4493c617629cd1e45d2d740d70262590d5a`, clean at intake.
Assignment: `arc11-observation-workbench/slice02-scope-capture/cc-prompt.md`.
Framework2.6.0, author guide07 and Rust2.0.0 applied.

## Review and five-row walk

Read the current plan/prompt/ledger and CC report; inspected all six committed
source paths against c4b517c. The commit changes exactly the authorized files.
Core, scheduler, module algorithms, fixtures, manifests and prior oracles are
unchanged. CC intake/readback addresses actual selectors, fallible reservation,
latched Observe, drain borrow, send labels and post-play error/export ordering.
Its receipts remain attestation; the code and checks below support acceptance.

| Row | Independent evidence / verdict |
| --- | --- |
| S-01 | Both feature modes reproduce four samples of actual Clock/LFO/cabled TM input/Raw17 with literal kinds/values and selector order. Consume-stop retains sample0, completes one sample, and stopped scheduling adds nothing. Observer uses last_index within the inherited core gate and only reads CurrentValue. Done/reproduced. |
| S-02 | Exact capacity3 window2/3/4 and overwritten2; clear/resume retains absolute sample5; mixed Generated/Sent/signal FIFO and valid reconfiguration/disable/reload pass. Queue evicts before push and has one shared record budget. Binary capacity3 run retains samples9997/9998/9999 and overwritten10067. Done/reproduced. |
| S-03 | Default/no-default tests reject no patch, missing names, duplicates, Midi, zero/unrepresentable capacity and running changes. Inline missing/wrong CurrentValue rejection passes. Source constructs and reserves candidate before replacing resource; rejected candidates retain prior capture; successful load disables bindings only after success. Done/reproduced. |
| S-04 | Real loaded10000 sample trace capture on/off identical; actual playback fake sender success/failure/cancel checks pass. Copy helper finishes drain borrow, admits whole Generated batch, then Sent only after successful send; cleanup excluded. Terminal discard uses same helper. No extra drain/held-MIDI observer. Done/reproduced. |
| S-05 | CSV schema/escaping/empty/failing writer and post-primary-failure export pass. Fresh executable capture40070 records/40071 lines, zero overwrites, unchanged70-row stdout; parsed CSV MIDI bytes/sample equal stdout. Empty run header only; missing selector, zero/orphan/list conflict/bad path reject without trace output. Both fixture hashes and all native gates pass. Done/reproduced. |

## Fresh checks on final source

Executed from the source worktree, exits0 unless stated; no endpoint opened.

| Command | Result |
| --- | --- |
| `cargo test --locked -p oxurack --test scope` | 6 passed |
| `cargo test --locked -p oxurack --no-default-features --test scope` | 6 passed |
| `cargo test --locked -p oxurack --lib --example turing-machine-midi` | 32 library +15 example passed |
| `cargo test --locked -p oxurack --no-default-features --lib --example turing-machine-midi` | 30 library +15 example passed |
| `cargo test --locked -p oxurack --test lfo --test midi_output` | 12 LFO +2 MIDI passed |
| `make test` with host access | 960 passed across29 result summaries, 0 failed, 3 pre-existing RT ignores; RT150 passed |
| `make lint` | Clippy and formatting pass |
| `cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings` | Pass |
| `RUSTDOCFLAGS='-D warnings' make docs` | Pass |
| `cargo build --locked -p oxurack --example turing-machine-midi` | Pass |
| `git diff --check` | Pass |

CC's restricted make attempt with four CoreMIDI initialization failures is
preserved in closing-report.md; its host retry remains CC attestation. CRC ran
make test directly with host access, separately proving the final source.
Three ignored RT scheduling/timing tests and block0.1.6 future incompatibility
advisory remain disclosed. No make check/freshness/coverage/hardware campaign.
Transient native logs `/tmp/oxurack-crc-scope-*.log` are convenience outputs,
not a required durable artifact or replacement for these receipts.

Binary: `./target/debug/examples/turing-machine-midi`. Two fresh LFO fixture
`--samples 10000` runs match70 rows and SHA256
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
Original MIDI fixture `--samples 2000` matches13 rows and SHA256
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.
The documented four selectors clock_main.pulse/lfo_main.bipolar_out/
tm_main.write_modulation/tm_main.note with capacity100000 produce40000 signal
and70 Generated MIDI records, no Sent, no overwrite. CSV parsed with standard
Python csv; exact9-column rows and MIDI equality checked. Capacity3 with one
LFO selector keeps3 records/overwritten10067 and the same stdout. Zero samples
produce only CSV header. Unknown module/bad path exit1; zero capacity/orphan
selector/list conflict exit2; all produce empty stdout before playback.
Transient binary files `/tmp/oxurack-crc-scope-6c56e84/`; results recorded here.

## Bubble-up, inventory and limits

Opening5 rows, CC walk5, CRC accepted5; no deferral/no-op/silent drop. Scope
supplies Arc11's assigned bounded observer and runnable CSV, preserving the sole
MIDI drain and musical traces. No design, sequencing, dependency or source-scope
change surfaced. Routine arc status update suffices before authoring Slice03.
Terminal inspection remains the next increment; Arc11's4 rows and project's7
rows remain open for composition/CDC gates. No physical timing/audio acceptance
or release readiness inferred from software send success.

Durable supporting artifacts: **none**. New source scope.rs/tests/scope.rs are
product/tests; canonical CC report/ledger and this CRC record are first-class
slice documents. Checked source/planning inventories contain no unplaced support
artifact. Issued prompt preserved; assignment history records acceptance.
What worked: actual loaded modules and independent literal oracles, one capture
resource, explicit caller MIDI copies and native checks established the behavior
without extra validator machinery.
