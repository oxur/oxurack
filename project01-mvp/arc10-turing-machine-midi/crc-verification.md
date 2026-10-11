# Arc10 CRC composition assembly — ready for CDC

2026-10-11, Sofie/CRC; Three-Contributor Workflow, Expedited Mode.
Candidate source `65522f116a12a9ac96d35cfe25ed9c184f5bddcb`; clean implementation.
Planning accepted-child predecessor `21fb8f9782297fc8f3b2f73eebd2d903e5441d78`.
This context independently reviewed CC's child implementations and now assembles
composition. It cannot independently sign off its own arc assembly; CDC's gate
is pending. [Closure candidate](closing-report.md) walks T-01–T-05 and successors.

## Fresh composition evidence

From the assigned source worktree, all exits0 on2026-10-11:

- `cargo test --locked -p oxurack --test midi_output`:2 passed. Actual loaded
  seeded24-sample trace equals separately driven engine with delayed publication;
  unread output stops before overwriting and completed count agrees with clocks.
- `cargo test --locked -p oxurack --example turing-machine-midi`:11 passed.
  Actual loaded runner, zero/finite/cancel/send-failure paths, terminal cleanup,
  successful-send note ledger and fresh replay; pure selectors/deadlines.
- `cargo test --locked -p oxurack-mod-midiout`:4 passed. Literal retrigger,
  idle/release/CC ordering, Update/FixedUpdate admission guard, reversed lexical
  order, valid-prefix preservation and failure state.
- `cargo tree --locked -p oxurack-mod-turingmachine --edges normal`: only
  rand0.10.1 and thiserror2.0.18 direct normal deps; no Bevy/core normal dependency.
- Existing built executable, original RON, `--samples 5000`, two new processes:
  equal34-row stream, SHA256 `918f4883252988093f737571cbb862aa29a33b737d0db026c98a7d7fea652d17`.
- Same executable with [two-sink-composition.ron](artifacts/two-sink-composition.ron),
  `--samples 5000`, two new processes: identical68 rows. The patch retains original
  Clock/TM/seed; duplicates midi_main on channel1 as a_extra_sink and cables the
  same signals. Source list spawns midi_main first, a_extra_sink second; lexical
  output must invert that order. Expected stream groups the independently checked
  original rows by sample, writes each group's channel1 bytes, then channel0 bytes.
  Every row matches, including first pairs `251|91 3E 15`, `251|90 3E 15`,
  `376|81 3E 00`, `376|80 3E 00`. This is actual RON/facade composition, not a
  copied runner. CDC can execute the same fixture with the existing binary.
- Read all six Operator trace files; fresh offline5,000/60,000 executions match
  both references. Finite/restarted traces identical; interrupted trace equals
  the first four long-reference rows. All copies preserved under Slice04.
- Direct child row count:6 Clock +6 adapter +5 sink +5 runner =22, all done.
  Inspected current design/arc/project ledgers, directive08 and successor mapping.

To replay the extra composition input from the assigned source worktree:

```bash
./target/debug/examples/turing-machine-midi \
  --patch /Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp/arc10-turing-machine-midi/artifacts/two-sink-composition.ron \
  --samples 5000
```

## Retention and scope

Source unchanged since Slice04 software acceptance on2026-10-10. Its independent
[record](slice04-output-runtime/crc-verification.md) reproduces default/no-default
example11/integration2 each, host workspace tests, make lint, strict all-target
Clippy, strict docs, build and CLI controls at this exact65522f1. Those gate results
are retained with their original date: only planning evidence/input fixture changed;
no source, dependency, criterion or execution change affecting them. No fresh
full-workspace/quality run is claimed today. Known CC restricted CoreMIDI failures,
three existing RT ignores and block0.1.6 future-incompatibility advisory remain.

Direct Git comparisons show no Clock crate delta after a19cfd1, no TM crate delta
after b2ffb16, and no core/sink/facade/wire/RT delta after2942c59. Earlier Clock
numeric/lifecycle and TM mapping/seed proofs remain at their accepted revisions;
current installed interaction is freshly exercised above. Slice04 adds only
example/private helpers, oracle extraction, README/dev dependency edges and its
adjacent integration test. No claim that historical tests were freshly rerun.

## Operator witness and disposition

Duncan's2026-10-11 run at65522f1 explicitly connects MIDI Bus1; PatchWork/Kontakt
omni receive, Electric Storm Deluxe/Dreamscapes, Bloom post. He confirms audible
music and everything behaving as expected across finite/interrupt/restart runs.
See [final child witness](slice04-output-runtime/crc-verification.md) for precise
provenance, screenshot, hashes and limits. Auditory/receiver outcome is
Operator-witnessed, file comparisons are CRC-reproduced; no hardware timing or
active-note-at-interrupt inference from the four-row prefix.

T-01–T-05 are operationally covered; CRC recommends done, with no proposed
arc deferral/no-op. All five arc statuses remain open until CDC independently
accepts composition/design conformance. Children are closed. All seven project
rows stay open; no source merge, release acceptance or Arc11 prompt by CRC.
The bubble-up in closing-report.md retains all successor obligations and adds
no design amendment. Needed next evidence: CDC's `cdc-verification.md` at this
same candidate, plus its verdict/next-arc assignment relayed through Duncan.
