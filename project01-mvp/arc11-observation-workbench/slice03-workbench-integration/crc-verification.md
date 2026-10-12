# Terminal workbench — CRC verification

2026-10-11, Sofie/CRC, three contributors, Expedited Mode.
Current verdict: **accepted after iteration01 at `72fe6b5`; B-01–B-04 done; Slice03 closed**. See the dated correction review below. Arc11 remains open for CDC.

## Initial review — preserved history

Initial assignment `cc-prompt.md`; source final
`22500fae234d5dbba599c7683f8893aabe70f47b` (preceded by
`f0b045330c8bb74fb3267ba8ef0a515dfaf1012e`), clean; CC planning
`fcbe6b8`, clean at intake. Accepted predecessor Scope `6c56e8460b693c285b8052633165e66129318604`.
Verdict: **B-01–B-03 independently done; B-04 open for F-01**. Slice and Arc11
remain open; no arc composition packet or next-arc implementation issued.
CRC changed no source. Current correction is sibling cc-prompt-iteration01.md.

## Inspected assignment and source

Read current prompt/plan/ledger, CC intake/readback/return and directives08/09.
Inspected complete three-path delta, including renderer, CLI/finalizer/tests and
README. Exact authorized source paths: README.md, example CLI and new workbench
module. Library/Scope/playback/oracle/fixtures/manifests/prior tests unchanged.
CC intake/readback covers the binding retained window, ordering, streams,
preflight and failure/lifecycle semantics; its receipts remain attestation.
No supporting durable artifacts produced; temporary probes/logs below are
convenience outputs, with methods/results recorded here. Issued initial prompt
and CC report preserved; correction needs a new dated CC return section.

## Four-row walk

| Row | Independent result |
| --- | --- |
| B-01 | Done/reproduced. Actual loaded saw-up literals0..3, selector order, known MIDI bytes/labels, retained bounds/tails/empty/control-escaped labels and immutable time/record snapshots pass. Renderer scans only retained records, counts Generated/Sent separately, bounds both tails and reads SampleTime for completed count. Scope mixed FIFO regression and fresh bounded binary run pass. |
| B-02 | Done/reproduced. View-only/CSV-only/combined, rows bounds/orphan/list conflicts and preflight selector/file errors pass. Fresh stdout equal across modes; CSV identical to CRC's prior accepted Scope output and current CSV-only. No new drain/endpoint or feature dependency. |
| B-03 | Done/reproduced. Generic finalizer attempts write/flush for both reports in specified order with primary first. Successful report survives other error. Actual runner send-failure/cancellation tests prove stopped partial report and cleanup-before-finalization. Capture/time are not mutated by rendering. |
| B-04 | Open. Runnable terminal/CSV/bounds/fresh-process recipes and all native gates pass. README clear/continue snippet at `22500fa` lines218–224 advances100 before draining, which can latch output backpressure; F-01 must be repaired. |

## Fresh validation on22500fa

All commands from S; exits0, unless specifically described below.

| Command | Fresh result |
| --- | --- |
| `cargo test --locked -p oxurack --example turing-machine-midi` | 25 passed, no ignores |
| `cargo test --locked -p oxurack --no-default-features --example turing-machine-midi` | 25 passed, no ignores |
| `cargo test --locked -p oxurack --test scope --test lfo --test midi_output` | 6+12+2 passed |
| `cargo test --locked -p oxurack --no-default-features --test scope` | 6 passed |
| `make test` with host access | 960 passed across29 summaries, 0 failed, 3 existing RT timing/scheduling ignores; RT150 passed |
| `make lint` | Clippy/format pass |
| `cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings` | Pass |
| `RUSTDOCFLAGS='-D warnings' make docs` | Pass |
| `cargo build --locked -p oxurack --example turing-machine-midi` | Pass |
| `git diff --check` | Pass |

CC's four restricted CoreMIDI failures and unchanged host retry remain separately
attested in its report. CRC ran host make directly. block0.1.6 advisory retained;
no suppression, new ignore, endpoint access, freshness/coverage campaign.
Transient logs `/tmp/oxurack-crc-workbench-*.log`.

Fresh built LFO10000 no-capture/view/two fresh view/combined/CSV-only traces all
match70 rows/hash988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c.
View-only stderr matches repeat; combined starts with the same view body, then
existing CSV summary. Four selected ports retain40070, overwrite0, counts
signal40000/generated70/sent0, span0..=9999, caller-ordered latest sample9999,
tails12 signals and12 MIDI. CSV40071 lines/hash
dd3986a6300af3b43582f12338d91a8ad17fafb7f7d66f7c33873d4a6fdae368 equals both
current CSV-only and the retained `/tmp/oxurack-crc-scope-6c56e84/capture.csv`
from prior independent Scope execution. Serialization/capture production inputs
are unchanged; this comparison is fresh, prior file identity kept historical.
Capacity3 one-LFO view retains9997/9998/9999, overwritten10067, no retained MIDI,
and identical70-row stdout. Zero samples shows unobserved selector/empty tails
and header-only CSV. Original2000 still13 rows/hash
98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b.
Rows0/1001, orphan rows/ports and list conflict exit2/empty stdout; unknown
selector/bad CSV path exit1/empty stdout. Binary outputs transient under
`/tmp/oxurack-crc-workbench-22500fa/`.

## F-01 — clear/continue recipe violates per-sample drain admission

README218–224 tells an already configured rack to stop/clear/start, then
`advance_samples(100)?`, postponing drain/copy until afterward. Facade lib.rs
advance_samples docs and admission code require all MIDI drained before the
next sample, including samples inside a batch. A pending message latches a
terminal fault and stops progression. This is a runnable recipe defect, not a
production admission bug or an optional hardening gate (B-04).

Reproduced using public facade/module APIs linked to the current build: install
Clock/LFO/TM/MIDI, load protected turing-machine-lfo.ron in Manual mode, configure
lfo_main.bipolar_out capacity100000, run200 single samples with normal drain/copy,
stop/clear/start, then execute README batch 100. Exact output:

```text
before=200
README advance100=Err(Tick(OutputBackpressure { pending_messages: 1, completed_samples: 52 }))
after=252 latched=Some(OutputBackpressure { pending_messages: 1, completed_samples: 0 })
```

This state is an ordinary stopped configured rack and catches the note at251.
A200→300 continuation using single-sample advance followed by collect/drain and
Generated copy each iteration compiles and executes on the same source:
completed 300, retained101, overwritten0, no fault, Generated event251 bytes
[144,62,21] (90 3E 15),100 scalar observations200..299. Starting at10000 did
not hit an event within the next100, so that probe passed and would have missed
the defect. The discriminating start200 matters. Initial temporary probe had
CRC-only compile errors (&str instead of &Path, private is_running call); fixed
before claiming runtime reproduction. No repository source altered.

Prescribe only README recipe repair plus one example inline compile/execution
test for this same continuation. The test proves the recipe on already-correct
production code; do not claim a production regression or that unmodified code
would fail a correctly drained test. No change to rendering/playback/Scope.

## Bubble-up and retention

Opening4 rows, reviewed4: three accepted, one documented repair. The terminal
product composes on the approved capture/runner; no design, dependency, phase,
drain or source-scope amendment surfaced. Arc11 structure unchanged; routine
status update/correction only. Arc/project acceptance and next-arc work remain
held pending B-04 then formal composition/CDC review.

If CC changes only the README recipe and one new inline test, propose retention
of B-01–B-03 and the unchanged production/native receipts at `22500fa`/2026-10-11.
Freshly run both example feature modes, formatting/lint, strict affected Clippy
and diff check; manually synchronize README loop with executed test. Workspace,
Scope/LFO/MIDI tests, build, rustdoc and pinned fixture outputs are unaffected
and may retain this independent evidence after CRC checks the actual delta.
Do not relabel retained results as new executions; rerun if unexpected changes
undermine applicability. No supporting artifact or extra checker required.

## Iteration01 review — accepted, 2026-10-11

Assignment `arc11-observation-workbench/slice03-workbench-integration/cc-prompt-iteration01.md`.
Clean source `72fe6b52197b7a138afc3d5d7ac970ddfdf66981`; CC return planning
`2162fdb82ce48daf0b11793176ded4d835b8c16c`, clean at intake. CRC read the
complete iteration prompt, CC reading/readback/results and current plan/ledger.
Full delta from `22500fa` contains exactly README.md and one new inline example
test; production regions, prior tests, renderer, playback, Scope, fixtures,
manifests and lockfile are unchanged. README and executable continuation loops
match by direct review. CRC made no source changes.

Fresh independent commands from S, all exit 0:

| Command | Result at `72fe6b5` |
| --- | --- |
| `cargo test --locked -p oxurack --example turing-machine-midi` | 26 passed, 0 failed, 0 ignored |
| `cargo test --locked -p oxurack --no-default-features --example turing-machine-midi` | 26 passed, 0 failed, 0 ignored |
| `make lint` | Clippy and formatting pass on first CRC run |
| `cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings` | Pass |
| `git diff --check`; `git diff 22500fa..72fe6b5 --check` | Both pass |

The new test runs the installed protected patch for 200 samples with per-sample
drain/copy, stops/clears, then executes the README continuation for 100 more.
It proves completed 300, cleared-but-enabled capture, signal indices 200–299,
exact Generated 251/90 3E 15, 101 retained records, zero overwrites, zero Sent,
empty output buffer and no latched fault. The preserved renderer test separately
proves actual phase continuity versus a fresh rack. Runtime errors propagate.
F-01 is resolved; B-04 is independently done. This is recipe proof against
unchanged correct production, not a newly fixed production regression.

### Retention and four-row closure

B-01–B-03 retain independent acceptance at `22500fa`/2026-10-11. The full source
delta establishes that no relevant production, dependency or fixture changed.
Retain the prior independent workspace 960 passed/3 RT ignores, Scope 6 + LFO 12 +
MIDI 2, no-default Scope 6, strict rustdoc and binary/CSV/preflight receipts at
that original revision/date. Those are not new executions. Both example modes,
lint, strict all-target Clippy and diff were freshly rerun for this correction.
Additional current build/focused tests and integrated demonstrations appear in
[parent CRC assembly](../crc-verification.md), for the arc composition purpose.
CC's first formatting failure/rerun, restricted CoreMIDI failures/host retries
and existing block0.1.6 advisory remain preserved with their original attribution.
No suppression, new ignore or endpoint access.

Opening 4 rows, final walk 4: B-01, B-02, B-03 retained independently done;
B-04 freshly done. No deferral/no-op/silent drop. Both issued prompts preserved;
iteration01 is the closed assignment. Supporting durable artifacts: none;
checks/logs in /tmp are convenience outputs, not required artifacts. CC's source
and planning inventories match the assignment. Source/main remain untouched by CRC.

### Bubble-up to Arc11

Slice03 delivers the assigned terminal inspection, combined CSV, honest bounded
window and executable clear/continue/new-process recipes. The only surprise was
the documented batch/drain defect, now repaired without changing production or
arc scope, ownership, scheduler, dependency or sequencing. No structural plan
amendment is needed. Routine child-status update suffices: all three children
are accepted,14/14 child rows. Arc11's four composition rows remain open for
CDC's independent gate; project/release obligations and next-arc authority remain.
