# Arc11 CDC verification — accepted

2026-10-11, Sofie/CDC, independent of CC implementation and CRC assembly.
Three contributors, Expedited Mode; collaboration-framework 2.6.0.
Source: `72fe6b52197b7a138afc3d5d7ac970ddfdf66981`, clean.
Planning intake: `c6fc330d644d47564c30f9b1d37ea2e2c57e98ca`, clean.

**W-01–W-04 done; Arc11 accepted and closed.** Three children, 14 accepted
child rows; four composition rows done, none open/deferred/no-op. No corrective
assignment. The Operator selected continuation into live persistence this turn.

## Independent reproduction

Executed from `.worktrees/codex-oxurack-module-behavior-installation` at the
source above; all final checks below exit 0. No source changes or endpoint use.

| Check | Result |
| --- | --- |
| `cargo test --locked -p oxurack --example turing-machine-midi --test scope --test lfo --test midi_output` | 26 example + 6 Scope + 12 LFO + 2 MIDI tests pass, 46 total; no failures/ignores |
| `cargo build --locked -p oxurack --example turing-machine-midi` | Pass |
| `git diff --check` and `git diff 22500fa..72fe6b5 --check` | Pass |
| Independent child-ledger count | 5 L + 5 S + 4 B rows, all done; three planned children match actual children |

Ran the built executable in fresh processes using the exact fixture and argument
sets in [CRC's composition commands](crc-verification.md#runnable-composition-demonstration).
A short Python stdlib subprocess/csv comparison, not a new repository tool, checked:

- Plain 10000-sample LFO fixture versus two captured runs: identical 70-row MIDI.
  SHA256 `988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
- Complete terminal views and CSV exports equal across fresh runs. Four selected
  ports, capacity100000: 40070 retained, 0 overwritten, sample span0..9999;
  40000 signal/70 Generated/0 Sent records. CSV 40071 lines, SHA256
  `dd3986a6300af3b43582f12338d91a8ad17fafb7f7d66f7c33873d4a6fdae368`.
- All 10000 TM write-modulation observations equal half the same-sample LFO
  bipolar output within1e-7. Clock true indices exactly0,125,...9875, 80 pulses.
  Every Generated sample/byte triple equals its stdout row in order; no Sent.
- Capacity3, one LFO selector: unchanged MIDI, retained3/overwritten10067,
  samples9997..9999. Empty run: no MIDI, unobserved selection, header-only CSV.
- Original 2000-sample fixture: SHA256
  `98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.

One initial CDC comparison failed because it constructed `sample bytes` while
stdout uses `sample|bytes`. Inspection of the actual trace resolved the checking
mistake; rerunning the complete comparison with the documented separator passed.
This was not a product defect. Temporary results are under
`/var/folders/dt/kc8ndn6d3kx8c1jnxr05drsh0000gn/T/oxurack-cdc-arc11-g7tomhlz`;
they are convenience outputs, not required durable artifacts.

## Row dispositions and evidence applicability

| Row | CDC disposition / basis |
| --- | --- |
| W-01 | Done, reproduced: real loaded modulation, fresh-process equality, every affine sample and exact Clock edges; focused tests include an independent accelerated MIDI oracle and disconnected-cable control. |
| W-02 | Done, reproduced: capture-off/on equality, exact copied events, bounded eviction and empty state, lifecycle and real runner cancellation/send-prefix tests. |
| W-03 | Done, reproduced: runnable combined terminal/CSV output with correct selector order, values, separate tails, sample/overflow labels, CSV schema and event order. |
| W-04 | Done, reconciled with fresh composition: 3 children/14 accepted rows, no missing child, accepted source pins and unchanged successor obligations. |

Inspected Scope selector validation/Observe-only reads/mixed FIFO/CSV, facade
installation and lifecycle, immutable terminal rendering, sole-drain copying and
successful-send ordering in the actual runner. Snapshot display does not advance
or mutate the simulation; cleanup traffic is not passed off as performance MIDI.

Retain CRC's fresh native/default/no-default/lint/all-target checks at this exact
candidate, separately from CDC execution above. Retain the independently recorded
workspace960 passed/3 existing RT ignores and strict docs at `22500fa` on
2026-10-11 in [Slice03 verification](slice03-workbench-integration/crc-verification.md).
Reviewed `22500fa..72fe6b5`: README continuation and one inline test only; algorithms,
fixtures, dependencies and workspace code are unchanged. Focused current tests
cover the added test and composition. No finding undermines the retained proof.
Earlier restricted CoreMIDI failures and successful host retries remain historical;
block0.1.6 future-incompatibility advisory remains disclosed. No new release gate.

## Project disposition and successor

P-07's specific TM/Clock/LFO/Scope criterion is now met: this reproduced workbench
composes with [Arc10's accepted musical witness and software composition](../arc10-turing-machine-midi/cdc-verification.md)
at65522f1. That witness remains Duncan's MIDI Bus1 → PatchWork/Kontakt observation
at its historical revision, not a claim that CDC heard or opened an endpoint now.
Current runner tests and unchanged original trace support its continued relevance.
CDC marks P-07 done on these combined independent and Operator-witnessed grounds;
P-01–P-06 and the project remain open. No hardware jitter, whole-catalog, saved/live
controller session, release or cross-platform sine bitwise acceptance is implied.

Arc12 live persistence/API/events, Arc13 catalog/Chord, Arc07 interfaces and Arc08
coverage/hardware/benchmark/soak obligations retain their owners. Directive10
opens Arc12's first runnable persistence slice. No hidden extra verification
platform or asynchronous service is prerequisite. What worked: actual executable
traces plus focused state/failure tests proved composition directly.
