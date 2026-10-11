# Arc10 CDC verification — accepted

2026-10-11, Sofie/CDC; independent of CC implementation and CRC assembly.
Verified candidate: `65522f116a12a9ac96d35cfe25ed9c184f5bddcb`.
Planning intake `e868b44`; both worktrees clean. Three contributors, Expedited
Mode; directive08 and Arc10 design v1.2 govern this review.

## Verdict

**T-01–T-05 done; Arc10 accepted and closed.** Four children, 22 accepted child
rows; five composition rows, zero deferred/no-op/open. No blocking finding or
corrective assignment. This accepts the playable static seeded rack and its
software-endpoint witness, not release timing or the whole MVP. Arc11 may open.

## Independent evidence and row walk

Executed from `.worktrees/codex-oxurack-module-behavior-installation` at the
verified candidate on 2026-10-11, all commands exit 0:

- `cargo test --locked -p oxurack --test midi_output --example turing-machine-midi`:
  2 integration and 11 example tests pass, no failures/ignores. Actual loaded
  rack vs independent engine oracle; unread-output accounting; real playback
  failure/cancel/cleanup/fresh-session paths and deadline/selection contracts.
- `cargo test --locked -p oxurack-mod-midiout`: 4 tests pass, no failures/ignores;
  literal transitions/CC order, admission in Update/FixedUpdate, lexical order,
  invalid-input prefix/state preservation; 0 doctests.
- `cargo build --locked -p oxurack --example turing-machine-midi`: passes.
- `cargo tree --locked -p oxurack-mod-turingmachine --edges normal`: standalone
  rand/thiserror dependencies, no Bevy/core normal edge.

Built executable `target/debug/examples/turing-machine-midi`, arguments
`--patch <fixture> --samples <count>`, launched as separate processes by a short
Python subprocess comparison (no new checked-in validator):

| Composition check | Independently observed result |
| --- | --- |
| Original source fixture, 5000 samples, twice | Identical 34 rows; SHA256 `918f4883252988093f737571cbb862aa29a33b737d0db026c98a7d7fea652d17` |
| `artifacts/two-sink-composition.ron`, 5000 samples, twice | Identical 68 rows; SHA256 `3607893d308f30a32acc0e80a57775de5808755b167094936c576aeb30d665da`; each sample's channel-1 sequence precedes channel-0 despite reverse spawn order. Expected bytes derived by regrouping the single-sink reference, changing only status channel bits; all rows equal. |
| Six preserved Operator traces under Slice04 `artifacts/operator-witness-2026-10-11/` | Offline/live/restarted 5000 files equal fresh 34-row replay. Fresh 60000 replay equals the 533-row reference; interrupted and expected-prefix files equal its first four rows. |

| Row | Disposition / basis |
| --- | --- |
| T-01 | Done, reproduced: real loaded single/two-sink fixtures and integration oracle establish composition, ordering and publication latency. Reviewed scheduling, facade admission and sink source. |
| T-02 | Done, reproduced plus retained child proof: fresh process byte equality and standalone dependency tree; unchanged Clock/TM implementations retain accepted mapping/seed proofs. No cross-version/cross-platform bitwise promise. |
| T-03 | Done, reproduced: loaded facade/runner and literal sink tests plus direct source review establish bounded output, retrigger/release, partial send failure, stop/interrupt cleanup and fresh recovery. No async recovery/atomic wire delivery guarantee. |
| T-04 | Done, Operator-witnessed with independently reproduced trace comparisons: named MIDI Bus1 → PatchWork/Kontakt omni trial and Duncan's expected musical/stop/restart observation are preserved in Slice04's final CRC witness. CDC opened no endpoint and does not claim to hear the session. |
| T-05 | Done, reconciled: independently counted child ledgers 6+6+5+5=22 done, inspected child verdicts, source integration and successor routing under directive08. Applicable native gates retained as below. |

## Source inspection and retained evidence

Inspected actual example and playback ownership/cleanup/pacing, independent
oracle/integration test, sink installation/configuration/consume path, facade
and sample-start changes. Confirmed errors stop/discard/finish and successful
send state is committed before trace writes. Ordinary synchronous output follows
the approved design. No new runtime or duplicate musical clock introduced.

`git diff --exit-code <accepted> HEAD -- <paths>` confirms no delta for Clock
since `a19cfd1`, TM since `b2ffb16`, or core/MIDI-out/facade-src/MIDI/RT since
`2942c59`. Earlier numeric/mapping/lifecycle independent evidence retains its
historical source/date. The current composition was freshly executed above.

Retain Slice04 CRC's full host workspace test/lint/all-target Clippy/docs and
feature checks at this exact `65522f1`, recorded 2026-10-10: source/dependencies
are unchanged and no finding undermines those checks. Do not label them fresh
CDC runs. Three existing RT ignores and the `block 0.1.6` future-compatibility
advisory remain disclosed; no new focused failure. Earlier restricted CoreMIDI
failures remain historical, alongside the successful CRC host runs.

The interrupted trace ends on a note-off and does not prove interruption while
a note was sounding. The witness establishes expected observed behavior; the
real-runner tests establish active-note cancellation/error handling. Neither
claims guaranteed silence after disconnect/forced termination or hardware jitter.

## Bubble-up and next work

M1's playable rack is delivered. P-07 still needs LFO/Scope; all seven project
rows stay open. Arc12 owns future concurrent output/reconnect and live API/
persistence; Arc13/backlog remaining catalog/Chord; Arc07 interfaces; Arc08
coverage provenance/95%, hardware P99 timing, benchmarks and soak. No historical
row is silently closed. See directive09 and Arc11's new plan/first slice.

What worked: a real executable fixture plus a narrow sender seam supplied useful
composition and failure proof without creating another verification subsystem.
No new evidence archive or tool; this record contains the new CDC results.
