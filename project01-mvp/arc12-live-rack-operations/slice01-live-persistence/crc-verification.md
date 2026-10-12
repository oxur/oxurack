# Live persistence — CRC independent verification

2026-10-12. **Accepted: LP-01–LP-05; Slice01 closed.** Three contributors,
Expedited Mode. CRC Sofie independently inspected and reproduced the corrected
CC return; evidence strength: reproduced. Duncan relays handoffs. This is child
acceptance, not Arc12/project acceptance or a merge/release.

## Assignment and intake

Reviewed source `fabfa1cb4beab5fd79f05b88d4bbec34208a1cac`, branch
`codex/oxurack-module-behavior-installation`, in the assigned source worktree.
Planning intake `4c680b484e4d4d8ede949fa4cdcb736a1fab13a4`, branch `planning`.
Both were clean; main stayed clean at `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
Active assignment: sibling `cc-prompt-iteration01.md`, SHA-256
`bba4dfd12e73a3cc13bca06912323a25b48ec387df2039385b64ff7d0231e45c`.
Initial assignment/failed return and CDC directive11 remain historical evidence.

CRC read the current directive, plans, ledger, both preserved assignments and
CC's closing report, including iteration intake/readback and failed/retried
checks. The intake names framework2.6/Rust2.0, bounded source/guide extents and
the two interacting contracts: configured getter versus effective delayed output,
and logical cable order versus allocator order. Its receipts are attestation;
source review and independent runs below establish acceptance. No hidden recall
check or new planning verifier was used.

## Source review and per-row disposition

The source commit changes exactly the nine authorized paths. No fixture, engine
algorithm, schema, dependency or main change. Inspected actual getters/setters,
CableOrder loader bundle, both propagation ordering paths, snapshot extraction,
facade errors/provenance/preservation, public fresh-replay tests and examples/docs.

| Row | Independent basis | Disposition |
| --- | --- | --- |
| LP-01 | TM registered getter returns write_base; held CV tests separate configured .25 from effective .5 and delayed old .75. Facade previous/new is .5/.25 DirectApi once; rejected type/NaN preserves base, time and event count. Existing deep TM observation/control-continuation test proves rejected setters do not alter engine state. Existing LFO/Command/error tests pass. | done |
| LP-02 | Active PatchHandle extraction reads getters and ECS metadata/header, fails rather than caching defaults. Loaded [.75,.25] parallel transforms retain literal LastWins .25 through two fresh save/reload rounds. Real Update/FixedUpdate core tests oppose ordinal and Entity order, preserve lexical endpoint priority, and cover mixed/equal/absent fallback plus dirty refresh. Snapshot uses the identical effective key. Missing-getter/disabled-cable/active-membership tests pass. | done |
| LP-03 | Snapshot and save at200 and pending252 preserve SampleTime, driver, running flag, Scope count/overwrites and output admission. Event251 drains once and continuation matches control through1000. Failure tests retain destination on extraction failure and propagate I/O errors. Full Scope record contents are not directly compared in this test; inspected snapshot/save never access or mutate capture, and no tick occurs. | done |
| LP-04 | Public integration saves explicit configured TM .25 with LFO1 and matches two fresh saved racks to an independently edited fixture; original differs. Separate fresh-process example replay below is identical for saved1/saved2/reference and differs from original. Saved configuration restarts engines, not their internal cursor. | done |
| LP-05 | Both facade feature modes and actual save/trace/workbench recipe pass, as do focused/native/host gates. Existing fixtures retain their pinned hashes. README/adapter docs describe configured versus effective behavior. No endpoint/hardware timing claim. | done |

## Independent commands and outcomes

All source commands ran in the assigned source worktree at fabfa1cb. Every row
below is a fresh CRC run; no CC receipt is relabeled independent. Transient logs
were under `/tmp/oxurack-crc-fabfa1cb-20261012`; this record retains their concise
results, not a promise of durable log availability.

| Command | Native exit and result |
| --- | --- |
| `cargo test --locked -p oxurack-mod-turingmachine` | 0;112 tests,0 doctests |
| `cargo test --locked -p oxurack-mod-turingmachine --features rack` | 0;129 tests,0 doctests |
| `cargo test --locked -p oxurack-core` | 0;415 unit,5 integration,9 seeded-spawn,2 doctests |
| `cargo test --locked -p oxurack --lib --test persistence --test scope --test lfo --test midi_output --example save-live-patch --example turing-machine-midi` | 0;42 unit,12 LFO,2 MIDI,2 persistence,6 Scope,0 save-example,26 runner-example |
| Same facade command with `--no-default-features` | 0;40 unit;other target counts unchanged |
| Restricted `make test` | 2 (Cargo101);RT146 passed,4 CoreMIDI initialization failures,3 existing ignores; not a pass |
| Exact `make test` with host access | 0;full workspace/tests/docs pass;RT150 passed,0 failed,3 existing ignores |
| `make lint` | 0;includes formatting |
| `cargo clippy --locked -p oxurack-core -p oxurack --all-features --all-targets -- -D warnings` | 0 |
| `cargo clippy --locked -p oxurack-mod-turingmachine --features rack --all-targets -- -D warnings` | 0 |
| `RUSTDOCFLAGS='-D warnings' make docs` | 0 |
| `git diff --check` | 0 |

The restricted failures are `midi_io::tests::{test_open_input_nonexistent_returns_error,
test_list_input_ports,test_list_output_ports,test_open_nonexistent_port_returns_error}`:
MIDI initialization unavailable in sandbox. The unchanged host command passed.
Existing RT ignores: priority elevation effects on scheduler, timing-loop jitter,
and timing-loop no-drift. No ignore/warning suppression added. The existing
`block v0.1.6` future-incompatibility advisory remains visible. Workspace feature
unification runs439 core unit tests versus415 in the focused core command.

### Actual fresh-process recipe

Using actual `cargo run --locked -p oxurack --example save-live-patch -- --patch
crates/oxurack/examples/patches/turing-machine-lfo.ron --save <temporary edited.ron>`,
CRC observed completed_samples300 and configured LFO .5→1. Constructed an
independent reference from the original fixture, changing only frequency_hz to1.
Ran `cargo run --locked -p oxurack --example turing-machine-midi -- --patch <file>
--samples 10000` separately for saved1,saved2,reference,original. All exited0.

| Trace | Rows | SHA-256 / relationship |
| --- | --- | --- |
| saved1,saved2,independent reference | 71 each | `7ce170b0880cb70cc65b8212eaaea729178b210387add22e1697f157d1a53d54`; byte-identical, nonempty |
| original LFO fixture | 70 | `988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`; differs from saved |
| original MIDI fixture,2000 samples | 13 | `98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`; unchanged |

Also ran the saved10000-sample runner with `--scope-view --scope-port
lfo_main.bipolar_out`: exit0, identical71-row stdout,10000 signal observations,
71 generated MIDI events,0 sent,10071 retained/100000 capacity/0 overwritten,
first sample0/last9999. No endpoint opened. These are configuration-replay and
inspection observations, not hardware or exact checkpoint evidence.

## Bubble-up check and artifact disposition

Was: Slice01 open with failed replay and diagnostic tie reversal. Now: both
CDC11-authorized repairs satisfy the original contracts and all five rows close.
Arc plan/project/status receive dated routine acceptance/advancement updates;
no architecture, scope or acceptance amendment is needed. CableOrder append and
actual replacement retirement remain Slice03/04 obligations. Historical Arc02
carryover reconciliation and all four Arc12 composition rows remain open for
final assembly/CDC gate. ProjectP-01–P-06 remain open; P-07 stays accepted.

CC's closing report has intake, exact row evidence, self-review and bounded
remaining claims; this explicit bubble-up check supplies the reviewer disposition
without rewriting CC's report or erasing failed history. Artifacts: no new durable
support outputs; implementation/tests/examples/docs stay in source, canonical
review in this slice root, concise provenance retained here. CRC did not modify
source or main. Under Expedited Mode, CRC opens approved Slice02 after this close.
