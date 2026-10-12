# Arc11 — observation workbench composition return

2026-10-11, Sofie/CRC to CDC through Duncan. Three contributors, Expedited Mode.
Source candidate `72fe6b52197b7a138afc3d5d7ac970ddfdf66981`, clean;
child-closure planning `3aa15b4`, based on CC return `2162fdb`.

**CRC readiness verdict: the three accepted children compose into the promised
workbench. Arc11 remains open pending independent CDC acceptance of W-01–W-04.**
[CRC verification](crc-verification.md) records reproduction and retention;
[ledger](ledger.md) keeps the four composition rows open. CDC's independent
`cdc-verification.md` is required for closure and has not been authored by CRC.

## Capability and three-child walk

The playable seeded Clock/TM/MIDI rack now includes an installed LFO that actually
modulates TM write probability, bounded scalar/MIDI capture, and terminal/CSV
inspection. It uses the existing sample driver, phases and sole MIDI drain owner.

| Child | Outcome and independent record | Rows |
| --- | --- | --- |
| [Slice01 LFO](slice01-lfo/crc-verification.md) | Delivered and closed at `c4b517c`: five waveforms, typed CV/sync, registered parameters, actual modulation and seeded MIDI. Initial invariant/schedule gap repaired in iteration01. | L-01–L-05 done, 5/5 |
| [Slice02 capture](slice02-scope-capture/crc-verification.md) | Delivered and closed at `6c56e84`: exact validated selectors, Observe snapshots, bounded mixed FIFO, generated/sent copies, lifecycle and CSV. | S-01–S-05 done, 5/5 |
| [Slice03 terminal](slice03-workbench-integration/crc-verification.md) | Delivered and closed: B-01–B-03 accepted at `22500fa`, retained over documentation/test-only correction; B-04 accepted at `72fe6b5`. README continuation now drains each sample; executable test proves the event at 251 and continuation to 300. | B-01–B-04 done, 4/4 |

Three planned children, three delivered, 14/14 child rows independently done;
none deferred, dropped or silently omitted. Issued prompts and failed predecessor
findings remain in their child records.

## Composition walk

| Arc row | CRC result at candidate; still open for CDC |
| --- | --- |
| W-01 | Clock/LFO/TM/MIDI fixture runs twice in fresh processes with identical 70-row MIDI, terminal view and CSV. Clock pulses every 125 samples; sine reaches +1 at 500 and -1 at 1500; all 10,000 TM input observations equal 0.5 times same-sample LFO output. Accelerated loaded modulation oracle and disconnected-cable control pass. |
| W-02 | Capture-disabled/enabled stdout bytes agree. Four selectors retain 40,000 signal snapshots plus 70 Generated events, no Sent/overwrite; CSV events equal stdout in order/sample/bytes. Capacity 3 retains 9997–9999 and reports 10,067 overwritten records. Actual known-waveform, stop/clear/phase and runner-copy tests pass. |
| W-03 | Documented combined terminal/CSV command shows patch, completed samples, bounds, selector-ordered latest values and separate event/signal tails. Export has 40,071 lines and the accepted CSV hash. Empty run shows unobserved selectors/header-only CSV. View is immutable and post-stop/cleanup. |
| W-04 | All three child verdicts reconcile. Fresh affected tests/lint/strict Clippy/build and integrated runs pass; unaffected independent workspace/docs evidence retains its original `22500fa`/date. Deferred obligations and gates below keep explicit owners. |

There is no gap between the approved Arc11 capability and delivered product.
Offline Generated evidence is not native send or musical/audio evidence. Actual
runner sender-failure/cancellation tests exercise successful Sent prefixes and
cleanup exclusion; no new MIDI endpoint or physical timing witness was performed.
Arc10's accepted Duncan witness remains at `65522f1` and is not relabelled current.
Cross-platform sine byte equality is not claimed.

## Plan changes and project bubble-up

[Arc plan history](arc-plan.md#version-history) v1.0–v1.8 records initial design,
LFO correction/closure, capture API/closure, terminal issue, recipe correction and
this composition return. These are local detail/status changes within directive09;
no scheduler, ownership, dependency, sequencing or structural scope amendment.
The recipe defect revealed the need to demonstrate per-sample drain in public
continuation examples; it is repaired here without a new runtime mechanism.

Against the project roadmap, Arc11 delivers the requested workbench milestone.
It supplies the outstanding LFO/Scope observation evidence for P-07, alongside
accepted Arc10 musical playback. P-07 and all seven project rows remain open
until the applicable independent project disposition; CRC does not close them.
No new arc or roadmap reordering is proposed by this work.

| Retained obligation | Owner / re-entry |
| --- | --- |
| Live persistence, runtime API/events, concurrent output/reconnect if needed | Arc12, re-enter from demonstrated musical use |
| Remaining module catalog, including Chord | Arc13 |
| Full CLI/REPL/controller editing | Arc07 |
| Coverage provenance/95%, hardware timing, benchmarks, soak, broader musical/release acceptance | Arc08; P-03/P-04/P-05 retain their gates |
| Whole rack/MVP composition and legacy obligations | Project P-01–P-07; historical carried-over rows remain owned, not discharged by this arc |

Supporting durable artifacts: none new. Product fixtures, tests and README are
source; per-scale reports/ledgers and preserved prompts are first-class records.
Transient outputs under `/tmp/oxurack-crc-arc11-72fe6b5/` are convenience files;
commands, assertions and results are recorded in CRC verification for reproduction.

## Relay and independent gate

Duncan relays these project-relative paths:

```text
arc11-observation-workbench/closing-report.md
arc11-observation-workbench/crc-verification.md
arc11-observation-workbench/ledger.md
```

CDC independently checks the candidate's composed workbench and row walk,
records its verdict in `arc11-observation-workbench/cdc-verification.md`, and
returns through Duncan. Required changes use a new directive. CDC owns any next
arc/first slice; this return grants no merge, push or release authority.
