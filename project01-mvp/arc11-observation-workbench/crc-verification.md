# Arc11 — CRC operational composition readiness

2026-10-11, Sofie/CRC; Three-Contributor Workflow, Expedited Mode.
Candidate source `72fe6b52197b7a138afc3d5d7ac970ddfdf66981`, clean;
CC return `2162fdb82ce48daf0b11793176ded4d835b8c16c`; child acceptance planning
`3aa15b4`. Main remains clean at `a7ce6e6`. CRC made no source changes.

**Verdict: CRC composition proposed-done for W-01–W-04; independent CDC gate
pending. Arc11 and project criteria remain open.** This is operational assembly,
not independent acceptance of CRC's own assembly. [Closing report](closing-report.md)
and [ledger](ledger.md) give the child, capability and deferred-owner walks.
Required CDC record: `cdc-verification.md`; its existence/verdict is not inferred.

## Inputs and evidence applicability

Read directive09, current directive08 policies, project/arc plans and ledgers,
all three child CRC verdicts, and current CC iteration01 intake/return/prompt.
Child denominator is 5+5+4=14; three of three children independently closed.
Source review at Slice03 established the full terminal/CLI/finalizer delta;
current `22500fa`→`72fe6b5` delta is only README and one inline recipe test.
Scope/playback/core/module algorithms, fixtures, dependencies and prior tests
are unchanged by that correction. Production regions are unchanged.

Retain the independent `22500fa`/2026-10-11 workspace result 960 passed, 0 failed,
3 existing RT ignores across 29 summaries; strict rustdoc; no-default Scope 6;
preflight CLI and accepted CSV-only compatibility demonstrations. These remain
historical independent receipts in [Slice03 verification](slice03-workbench-integration/crc-verification.md),
not new executions at `72fe6b5`. Historical LFO and Scope acceptance remain at their
named source pins. Fresh candidate checks below supplement that evidence.

## Fresh native checks on `72fe6b5`

All commands executed from implementation worktree S, all exit 0:

| Command | Result |
| --- | --- |
| `cargo test --locked -p oxurack --example turing-machine-midi` | 26 passed, 0 failed, 0 ignored |
| `cargo test --locked -p oxurack --no-default-features --example turing-machine-midi` | 26 passed, 0 failed, 0 ignored |
| `cargo test --locked -p oxurack --test scope --test lfo --test midi_output` | Scope 6, LFO 12, MIDI 2 passed; no failures/ignores |
| `make lint` | Clippy and formatting pass |
| `cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings` | Pass |
| `cargo build --locked -p oxurack --example turing-machine-midi` | Pass; rebuilt candidate executable for composition demonstrations |
| `git diff --check`; `git diff 22500fa..72fe6b5 --check` | Both pass |

The example tests include literal loaded saw-up 0..3, event bytes/delivery labels,
immutable snapshot rendering, output-error ordering, actual runner send failure
and cancellation after successful send, phase-preserving clear/resume, and the
corrected README continuation at 200→300 with Generated 251/90 3E 15. Scope tests
prove sample-indexed Clock/LFO/cabled input/Raw literals, stop during Consume,
mixed FIFO/clear/lifecycle/atomic rejection and capture-on/off MIDI. LFO tests
prove the independent accelerated probability/MIDI oracle and a disconnected
cable control that differs; merely seeing a changing number is not that proof.

CC's initial formatting failure and successful rerun are preserved in its return.
CC restricted CoreMIDI failures and unchanged host retries remain historical
attestation, alongside independent CRC host workspace evidence at `22500fa`.
Three existing RT ignores and block0.1.6 future-incompatibility advisory remain;
no warning suppressed, new ignore, coverage/freshness campaign or endpoint use.
Native logs `/tmp/oxurack-crc-72fe6b5-*.log` and
`/tmp/oxurack-crc-arc11-focused.log` are transient convenience outputs.

## Runnable composition demonstration

From S, after the build above, run the README fixture without capture and then
this combined command twice in fresh processes (choose distinct CSV paths):

```bash
./target/debug/examples/turing-machine-midi \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --samples 10000 > /tmp/arc11-plain.trace

./target/debug/examples/turing-machine-midi \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --samples 10000 --scope-view --scope-rows 12 \
  --scope-csv /tmp/arc11-first.csv --scope-capacity 100000 \
  --scope-port clock_main.pulse --scope-port lfo_main.bipolar_out \
  --scope-port tm_main.write_modulation --scope-port tm_main.note \
  > /tmp/arc11-first.trace 2> /tmp/arc11-first.view
```

Fresh actual outputs were stored under `/tmp/oxurack-crc-arc11-72fe6b5/` as
plain.trace, first/second.trace, first/second.view and first/second.csv.
All three traces match byte-for-byte; the two complete stderr views and CSV files
also match byte-for-byte. Observed outputs:

- stdout 70 rows, SHA256 `988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
- completed 10000; capacity 100000; retained 40070; overwritten 0; span 0..=9999;
  counts signal 40000/generated 70/sent 0. Latest selectors are caller-ordered,
  each at 9999; tails bounded to 12 signals and 12 MIDI.
- CSV 40071 lines, SHA256 `dd3986a6300af3b43582f12338d91a8ad17fafb7f7d66f7c33873d4a6fdae368`,
  identical to the accepted Scope hash. Exact nine-column schema and 70
  `midi_generated` records; serialized sample/bytes equal every stdout row in order.

Python standard csv parsing checked all 40,000 signal records. Actual Clock
`gate=true` indices exactly 0,125,250,…,9875 (80 pulses); all other Clock records
are false. The 0.5 Hz sine fixture observations include:

| Sample | LFO bipolar observation |
| --- | --- |
| 0 | 0 |
| 1 | 0.0031415876 |
| 500 | 1 |
| 1000 | -0.0000000000000020979814 |
| 1500 | -1 |
| 2000 | -0.00000000000034396997 |

At all 10,000 indices, observed `tm_main.write_modulation` equals 0.5 times
same-sample `lfo_main.bipolar_out` within 1e-7. Observe follows Consume and sees
the cable-resolved input; this is consistent with existing same-sample literal
Scope proof. The independent accelerated oracle/control separately demonstrates
that modulation changes the TM's emitted MIDI.

An initial CRC-only CSV inspection assertion selected record="generated" and
failed because the actual CSV spelling is `midi_generated`; corrected to the
public schema before asserting MIDI equality. A planned previous-index affine
comparison was discarded before execution after inspecting same-sample CSV
and the existing Observe test. No product defect or source edit arose; the
corrected observation-only analysis passed all assertions. No new repository
checker or evidence framework was created.

## Bounds, empty and compatibility demonstrations

Fresh candidate runs also used the same executable/fixture:

```bash
./target/debug/examples/turing-machine-midi \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --samples 10000 --scope-view \
  --scope-port lfo_main.bipolar_out --scope-capacity 3

./target/debug/examples/turing-machine-midi \
  --patch crates/oxurack/examples/patches/turing-machine-lfo.ron \
  --samples 0 --scope-view --scope-port lfo_main.bipolar_out \
  --scope-csv /tmp/arc11-empty.csv

./target/debug/examples/turing-machine-midi \
  --patch crates/oxurack/examples/patches/turing-machine-midi.ron --samples 2000
```

Capacity 3: completed 10000, retained 3, overwritten 10067, span 9997..=9999;
signal 3/generated 0/sent 0; latest 9999=-0.0031415876 and signal tail 9997/9998/9999.
stdout remains the same 70-row pinned trace. The view does not reconstruct evicted
MIDI. Zero samples: empty stdout, retained 0, selected value unobserved, empty
tails and header-only CSV. Original fixture: 13 rows, SHA256
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.

## Four-row recommendation and limits

W-01 repeatable actual modulation, W-02 bounded observation without changed MIDI,
W-03 usable terminal/export, and W-04 child/owner/gate reconciliation are all
CRC proposed-done, with fresh composition above plus justified retained child
evidence. All four remain open for CDC. No child or promised capability is missing.
Arc12/13/07/08 retain their obligations as enumerated in the closing report;
all seven project rows remain open. P-07 now has candidate workbench support,
alongside the accepted Arc10 Operator witness at its original `65522f1` revision.
No current endpoint playback, synth acknowledgement, physical jitter, release
readiness or cross-platform sine bit equality is inferred.

Inventory: no new supporting durable artifacts; this record and closing report
are the required arc-scale documents. CRC source/main unchanged. Duncan relays
the three project-relative packet paths; CDC records independent composition
and owns any subsequent arc/first-slice preparation after its verdict.
