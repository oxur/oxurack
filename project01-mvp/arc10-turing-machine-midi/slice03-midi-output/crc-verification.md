# Slice03 CRC verification — accepted

2026-10-10, Sofie/CRC; Three-Contributor Workflow, Expedited Mode.
Independent of CC implementation. Assignment: `arc10-turing-machine-midi/slice03-midi-output/cc-prompt.md`.
Source: `2942c594a445b5de3ef8f131cdedaa6095aa1529`, clean on the assigned
implementation branch. Evidence intake: planning `cf9188d0941a7b6cd725b19cf3ca807051456c3d`, clean.
Compared all 14 changed source paths with accepted `b2ffb16`; inspected the full
new sink, facade/core delta, integration oracle, example/fixture, manifests,
lockfile delta and instructions/docs. No CRC source edits.

## Verdict and row walk

Five opening rows, five reviewed rows: M-01–M-05 **done, reproduced**. No deferral,
no-op, silent drop or blocking finding. CC's report remains its original attestation.

| Row | Independently checked evidence |
| --- | --- |
| M-01 | Actual schema/strict signed decode/getters and reservation-before-spawn inspected; sink tests pass. Loaded seeded Clock→TM→sink RON through the facade in default/no-default integration tests. |
| M-02 | Literal retrigger/idle/release/velocity fallback/zero/CC and Raw boundary assertions pass; lexical order beats reversed spawn order. Inspected validation-before-mutation and complete per-sink off/on/CC ordering, including gate-false CC validity. Constructors cannot silently mask used invalid data. |
| M-03 | Facade tests reproduce unread-output denial before either clock, completed count1, retained events/fault, ordinary stop/drain/resume and finalized Consume-error count. Real Update/FixedUpdate pre-latch guard tests pass. Earlier lexical prefix/failing state preserved. Checked bounded 5-per-static-sink storage, first-fault retention and no reset on start/drain in source. |
| M-04 | Two facade integration tests pass in each feature mode; seeded oracle drives the standalone engine separately and constructs literal MIDI transitions with delayed odd-sample publication. Two fresh built example processes emit identical 13-row traces, SHA-256 `98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`; zero samples empty, missing/malformed patches fail with stderr and no stdout. |
| M-05 | Required gates below reproduced; exact source/planning component scope, unchanged issued prompt, standalone isolation, docs and bubble-up checked. No endpoint or hardware claim. |

## Commands and results at the source above

Executed from `.worktrees/codex-oxurack-module-behavior-installation`; all exits0:

- `cargo test --locked -p oxurack-mod-midiout`: 4 passed.
- `cargo test --locked -p oxurack --test midi_output`: 2 passed.
- `cargo test --locked -p oxurack --no-default-features --test midi_output`: 2 passed.
- `cargo test --locked -p oxurack-mod-clock`: 7 plus 1 doctest passed.
- `cargo test --locked -p oxurack-mod-turingmachine`: 112 passed.
- `cargo tree --locked -p oxurack-mod-turingmachine --edges normal`: rand0.10.1/thiserror2.0.18; no Bevy normal dependency.
- `make test` on host: workspace passed, including facade28, core438, MIDI-out4, rack-enabled TM134 and RT150; three existing RT scheduling/timing ignores retained. Host run used escalation for CoreMIDI. CC's four restricted initialization failures remain disclosed in its unchanged closing report; CRC did not rerun that known environment failure or relabel it a pass.
- `make lint`: workspace Clippy and formatting passed.
- `cargo clippy --locked -p oxurack-mod-midiout -p oxurack-core -p oxurack --all-features --all-targets -- -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' make docs`: passed.
- `git diff --check`: passed.
- `cargo build --locked -p oxurack --example turing-machine-midi`: passed.
- Built binary with `--patch crates/oxurack/examples/patches/turing-machine-midi.ron --samples 2000`, twice: equal/nonempty; first rows `251|90 3E 15`, `376|80 3E 00`, `626|90 40 2C`, `751|80 40 00`. Same executable, samples0: exit0/empty; missing and malformed temporary RON: nonzero/stderr/empty trace.

Existing dependency `block0.1.6` emits a future-incompatibility advisory; no
new Clippy/rustdoc failure. Lockfile comparison removes/upgrades no existing
package/version pair; adds MIDI-out and clap's dependency closure, including
syn3 alongside existing syn2. No freshness campaign is an assigned gate.
Prompt SHA-256 matches `20dff54f31beaecefefca5745b7cad03b516d763b0170e5bc0ddd62b3652a35d`;
unchanged CC report matches `480062d687fc27151b67799f89b320054f9fadffe140701472652c6caeaf0391`.

## Bubble-up to arc

Delivered the assigned installed sink/bounded drain and executable seeded trace.
No discovered architecture, sequence, scope or acceptance amendment is needed;
Arc10 gets routine child-status/evidence updates before Slice04 is detailed.
Physical playback, sent-note cleanup and the musical witness belong to Slice04;
T-01–T-05 and all project rows remain open. Accepted Clock/TM production is
unchanged; prior adapter mapping/seed acceptance remains at its historical pin.
Artifact inventory checked: CC changed only the two canonical closing-report
and ledger files, no supporting artifact; CRC adds this canonical review record.
Temporary successful command output/traces are not durable supporting artifacts.

## What worked

The real RON fixture and separately driven engine oracle expose publication
latency, while small literal-byte cases expose MIDI transitions and the direct
schedule guards expose admission ordering. Required native gates cover the
actual example without a second validator or copied implementation harness.
