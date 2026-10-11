# Arc10 closing report — playable seeded Turing Machine rack

> CDC acceptance, 2026-10-11: [independent review](cdc-verification.md) accepts
> T-01–T-05 at `65522f1`; Arc10 is closed. CRC's original proposal below is
> preserved as assembly history. Arc11 is now authorized by directive09.

2026-10-11, Sofie/CRC; Three-Contributor Workflow, Expedited Mode.
Source: `65522f116a12a9ac96d35cfe25ed9c184f5bddcb`, clean on
`codex/oxurack-module-behavior-installation`. Child acceptance planning predecessor:
`21fb8f9782297fc8f3b2f73eebd2d903e5441d78`. Main remains `a7ce6e6`.
Authority: directive08, Arc10 design v1.2 and arc-plan v1.9.

## Capability and verdict

The installed Clock→Turing Machine→MIDI-out rack loads a seeded static RON,
replays ordered MIDI across fresh processes and plays a deliberately selected
software endpoint with finite completion, interruption and best-effort cleanup.
Duncan witnessed music through MIDI Bus1 → PatchWork/Kontakt8 omni; Electric
Storm Deluxe (Melody), Dreamscapes, with Bloom post. He reports it sounded amazing
and everything played exactly as expected. The actual finite/interrupt/restarted
performance traces independently match offline references.

CRC proposes Arc10 acceptance: all four children and22 child rows are accepted;
all five composition rows are covered below. **Arc10 remains open pending CDC's
independent composition/design review and `cdc-verification.md`.** CRC's
[verification](crc-verification.md) is assembly evidence, not CDC sign-off.

## Four-slice walk

| Child | Delivered outcome | Final rows / accepted source |
| --- | --- | --- |
| [Slice01](slice01-musical-clock/crc-verification.md) | Validated standalone/installed musical Clock and lifecycle | C-01–C-06 done; `a19cfd1` |
| [Slice02](slice02-turing-machine-adapter/crc-verification.md) | Complete seeded TM adapter, all mappings and delayed snapshot semantics | A-01–A-06 done; `b2ffb16` |
| [Slice03](slice03-midi-output/crc-verification.md) | Installed MIDI sink, finite guarded drain and runnable RON trace | M-01–M-05 done; `2942c59` |
| [Slice04](slice04-output-runtime/crc-verification.md) | Exact endpoint selection, paced direct playback, cleanup and Operator musical witness | O-01–O-05 done; `65522f1`, Operator witness2026-10-11 |

22 done, zero deferred/no-op/open child rows; no missing fifth child. Directive08
absorbed the unissued Slice05 into03/04. Corrections and earlier failed/pending
reviews remain in their child records; this does not retroactively erase them.

## Five composition rows

| Row | CRC result and evidence |
| --- | --- |
| T-01 | Reproduced real loaded fixture against independently driven engine/byte oracle. Added one small [two-sink input fixture](artifacts/two-sink-composition.ron) to prove actual RON installation of multiple sink instances: channel1/name a_extra_sink emits before channel0/name midi_main despite reversed spawn order. Two fresh runs produce68 exact sample/byte rows, derived from the independently checked34-row single-sink reference. Clock/adapter latency properties retain unchanged child proof and are exercised through this real path. |
| T-02 | Two fresh original executable processes produce identical34-row5,000-sample traces, SHA256 `918f4883252988093f737571cbb862aa29a33b737d0db026c98a7d7fea652d17`. Fresh normal TM dependency tree excludes Bevy/core; standalone adapter/seed inventory proof retained where unchanged. |
| T-03 | Fresh real-facade admission and sink transition/ordering tests pass; real-runner send-failure, cleanup/cancellation and fresh replay tests pass. Inspected single owner, finite storage and stop/discard/finish boundaries. Actual Operator interrupt exits0, matches four-row prefix, and fresh session reproduces full trace. No concurrent recovery, queue-atomicity or hard-real-time promise. |
| T-04 | Named actual software endpoint, seed42/fixture/commands, receiver screenshot, omni setting, audible/stop/restart confirmation and independently compared saved traces. Operator-witnessed; CRC does not claim its own auditory reproduction. No endpoint deferral needed. |
| T-05 | Four child ledgers/reviews counted22 accepted rows; actual integrated source scope and directive08 checked. Applicable native gates retained at exact unchanged final source; fresh composition checks below. Original successor mapping and project/release obligations remain. |

## Proof and durable artifacts

Fresh CRC composition commands/results and retained validation rationale are in
[crc-verification.md](crc-verification.md). Source unchanged during witnessing
and assembly. Original fixture remains literal/unchanged. No new test framework,
checker, probe crate or dependency update. One two-sink RON is justified by T-01's
multiple-instance composition criterion; it is an input to the existing executable.

Slice04 owns the runbook and six byte-for-byte Operator trace copies plus receiver
screenshot in `slice04-output-runtime/artifacts/operator-witness-2026-10-11/`.
Arc artifacts add only `artifacts/two-sink-composition.ron`. Successful routine
build logs are not re-archived. Earlier child evidence remains in its owning homes.
This packet adds arc close/review records and routine plan/ledger/status updates;
issued prompts, existing source and historical successor records are preserved.

## Change log and bubble-up to project

Arc-plan history records Clock and adapter corrections, sink/runner delivery,
and directive08's approved replacement of the unissued asynchronous protocol
with synchronous single-owner output plus actual software OR hardware witness.
This final child adds no new architecture or acceptance amendment. The first
musical milestone is demonstrated; the whole project remains open.

Arc11 remains next: LFO with observable modulation, bounded Scope and practical
inspection workbench, reusing the installed Clock and rack trace. CDC owns its
arc/first slice after independently accepting this composition. Concurrent output/
reconnect and runtime persistence/API/events stay Arc12; remaining catalog/Chord
stay Arc13/backlog, and broader interfaces stay Arc07. Coverage provenance/95%,
P99<500us hardware timing, benchmarks and24h soak remain Arc08/release. Project
P-03/P-04 final hardware/session obligations are not discharged by this witness.
No legacy deferred row is implicitly closed; original successor mapping remains
`../artifacts/cdc-successors03.tsv`, interpreted under directive08. All seven
project rows remain open, including P-07's unfinished LFO/Scope portion.

## CDC handoff

Please independently review this candidate, arc ledger, CRC record, design v1.2
and child verdicts; reproduce proportionate composition at source `65522f1`.
No need to rerun every child suite. Record `cdc-verification.md` with verdict and
row dispositions. If accepted, close the five arc rows/Arc10 and author Arc11's
arc/first slice; if rejected, return one explicit directive through Duncan.
