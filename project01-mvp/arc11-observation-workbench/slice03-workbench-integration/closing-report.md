# CC intake and proposed-done report — terminal Scope workbench

## Iteration01 intake — executable clear/continue recipe

2026-10-11. Active assignment: `arc11-observation-workbench/slice03-workbench-integration/cc-prompt-iteration01.md`, returning to CRC through Duncan. S is `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`, branch `codex/oxurack-module-behavior-installation`, clean at `22500fae234d5dbba599c7683f8893aabe70f47b`. P is `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch `planning`, clean at `0981ab5d5b9bb0fab718cd9ebd19e24718f5b05e`. Main remains clean at `a7ce6e6`. Assignment paths and source/planning heads match the prompt; no material drift.

### Required-reading coverage

| Prompt order | Loaded extent and consequence |
| --- | --- |
| 1 | Iteration prompt, sibling `slice-plan.md`, `ledger.md`, and `crc-verification.md`: complete. B-01–B-03 are accepted; B-04/F-01 alone remains. Only README plus a new inline test in the parent example are authorized. |
| 2 | P `cdc-directive09.md` and `arc11-observation-workbench/arc-plan.md`: complete. Slice03 is the approved third increment; no composition/CDC acceptance or structural change is authorized. |
| 3 | P `cdc-directive08.md`, “Checks and correction policy from Slice03 onward”: complete. Run the affected example modes, lint, strict affected Clippy and diff check; retain applicable unaffected CRC evidence and distinguish old receipts from new runs. |
| 4 | S `CLAUDE.md` and `crates/oxurack/Cargo.toml`: complete. Keep S/P separate; available example dependencies need no manifest change. |
| 5 | S `README.md`, Scope commands and lifecycle/clear/continue section (lines 117–247): complete. Replace the batched advance and explain generated-event recording versus live runner sending. |
| 6 | S `crates/oxurack/src/lib.rs`, `start`, `stop`, `sample_time`, Scope methods, `drain_midi`, and full `advance_samples` docs/body (lines 266–455): complete. Per-sample admission is conservative; a drain borrow ends before mutable rack recording; errors latch and stop the rack. |
| 7 | S parent example `prepare_rack` and full existing inline test module (lines 175–188, 300–848): complete. Fixture paths are `Path`; prepare returns a Manual, stopped rack. Keep all production portions unchanged. |
| 8 | S LFO fixture, complete; `oxurack-mod-midiout/src/lib.rs` event/buffer/fault APIs (lines 92–163), complete, plus `oxurack-midi/src/wire.rs` `MidiWire` definition and byte serializer (lines 16–140). `MidiOutputEvent` carries sample and message; `to_bytes()` returns the three-byte observation asserted by the test. Public buffer APIs expose drain, fault, and admission behavior. |
| 9 | Installed `rust-guidelines/SKILL.md` and `testing/SKILL.md`, complete. Use behavior-oriented assertions and repository-native validation. |
| 10 | Installed `rust-guidelines/guides/03-error-handling.md` EH-01 and `guides/11-anti-patterns.md` AP-33, required sections complete. Propagate real errors with `?`; do not ignore backpressure or let an error count as success. |

No conditional dependency was encountered. An initial combined source read was truncated; each required extent was recovered in bounded ranges. The fixture and type/API sections are complete; the MIDI wire definition and serializer were loaded from the newly encountered message-type dependency before finalizing the evidence record.

### Contract readback before source edits

The 100-sample batch is unsafe because each next sample checks pending MIDI admission. In the discriminating continuation from sample 200, sample 251 generates `90 3E 15`; after 52 more samples, the batch reaches sample 252 with that one event pending, returns `OutputBackpressure(completed_samples: 52)`, and latches the fault. The repaired recipe advances one sample at a time. It must collect the `drain_midi()` iterator into owned events so its exclusive rack borrow has ended before calling `record_scope_midi`; every event is copied as `Generated`, then the next sample is requested. The checked `advance_samples(1)?` must preserve all runtime errors.

`stop`/`start` preserve time and module phase. `clear_scope` removes retained records and overwrite count while capture stays enabled, so this is a continuation at absolute samples 200–299, not a reseed. Only a new rack/process loading the fixture begins from the seeded initial phase. Production admission, fixture, playback, Scope APIs and CLI remain protected. The focused executable test will assert 300 completed samples, clean fault/buffer, 100 signals at 200–299, exactly one Generated event at 251 with bytes `90 3E 15`, 101 records total, and no overwrite/Sent records. The README loop and test loop must match by direct review.

### Iteration01 repair, checks, and handoff

Source S commit `72fe6b52197b7a138afc3d5d7ac970ddfdf66981` changes only `README.md` and the test region at the end of `crates/oxurack/examples/turing-machine-midi.rs`. The README replaces the unsafe batched advance with the per-sample advance/drain/collect/copy sequence, stops afterward, and explains that it records Generated events while live sending stays with the existing runner. The inline test runs the accepted fixture to sample 200 with normal drain/copy, stops and clears, then exercises the same continuation loop. It observes 300 completed samples, capture still enabled, no output fault or pending MIDI, exactly 100 signal records at samples 200–299, one Generated MIDI event at sample 251 with bytes `90 3E 15`, 101 total records, no Sent record, and zero overwrites. Direct diff review confirms the README and test continuation loops match. No production code, fixture, manifest, lockfile, Scope/playback code, or prior tests changed.

| Check from S | Result |
| --- | --- |
| `cargo test --locked -p oxurack --example turing-machine-midi` | 26 passed, 0 failed, 0 ignored; includes the continuation oracle. |
| `cargo test --locked -p oxurack --no-default-features --example turing-machine-midi` | 26 passed, 0 failed, 0 ignored; includes the continuation oracle. |
| `make lint` | First run: Clippy passed, format check failed on the new test signature. Applied `rustfmt --edition 2024` to the authorized example file; rerun passed Clippy and format. |
| `cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings` | Passed. |
| `git diff --check` | Passed on the exact two-file source delta before commit. |

Cargo emitted the existing `block v0.1.6` future-incompatibility advisory during tests/lint/Clippy; no warning was suppressed and no dependency changed. `make test`, documentation generation, example build, and binary fixture demonstrations were not rerun: this iteration's checks do not require them and the source delta has no production, fixture, or public API effect. CRC's `22500fa`/2026-10-11 workspace, Scope/LFO/MIDI, build, rustdoc and fixture-output receipts are proposed for retention with that original revision/date; they are not new executions. The restricted CoreMIDI failure and unchanged host retry remain at their original receipts. No supporting artifact was needed or created.

#### Four-row walk

| Row | CC result at `72fe6b5` | Gate |
| --- | --- | --- |
| B-01 | Done, CRC independently accepted at `22500fa`; no relevant implementation changed. | Retain prior CRC evidence. |
| B-02 | Done, CRC independently accepted at `22500fa`; no relevant implementation changed. | Retain prior CRC evidence. |
| B-03 | Done, CRC independently accepted at `22500fa`; production/finalizer/runner code unchanged. | Retain prior CRC evidence. |
| B-04 | CC proposed-done: README continuation now drains each sample and the executable recipe test passes both feature modes; lint, strict Clippy and diff check pass. | Remains open pending CRC review; do not treat CC attestation as acceptance. |

#### Inventories and Bubble-up

- S inventory: clean at `72fe6b52197b7a138afc3d5d7ac970ddfdf66981`, branch `codex/oxurack-module-behavior-installation`; only `README.md` and `crates/oxurack/examples/turing-machine-midi.rs` in the source commit.
- P inventory before the return commit: branch `planning` at `0981ab5d5b9bb0fab718cd9ebd19e24718f5b05e`; only this `closing-report.md` and sibling `ledger.md` are authorized planning changes. Main remains clean at `a7ce6e6`.
- Bubble-up: no architecture, scheduler, ownership, drain, fixture, or parent-plan change. Slice03 and Arc11 remain open; B-04 awaits CRC, then the existing formal composition/CDC gate. No arc closure, merge, push, release, or live endpoint witness is claimed. No new artifacts are required.

2026-10-11. Assignment: `cc-prompt.md`, initial pass. Status: CC proposed-done;
all four B rows remain open for CRC. This report is in P and records the intake
that preceded dependent source edits. S and P remain separate.

## Checkout and assignment state

- S: `/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`, branch `codex/oxurack-module-behavior-installation`, HEAD `6c56e8460b693c285b8052633165e66129318604`, clean at intake. This is the prompt's accepted Scope source pin.
- P: `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch `planning`, HEAD `92129389999342be3fed7fa048c855138b3d63bf`, clean before this report. It contains the issued open set; Slice02 acceptance is recorded at `819cecd`.
- Main checkout `/Users/oubiwann/lab/oxur/oxurack` is on `main` at `a7ce6e6`, clean; not an implementation target.
- The initial prompt, slice plan and ledger were read completely: `cc-prompt.md` (323 lines), `slice-plan.md` (87 lines), `ledger.md` (12 lines). All four B rows are open. The initial prompt and issued history are preserved.

## Required reading coverage

The following required extents were loaded completely. Paths below are relative to
P or S as named above unless absolute.

| Order | Path and extent | Applicability/readback |
| --- | --- | --- |
| 1 | P `arc11-observation-workbench/slice03-workbench-integration/{cc-prompt.md,slice-plan.md,ledger.md}`, all | Binding terminal/CLI/finalizer contract, tests, exact three-file source allowlist, commit protocol, and four open rows B-01–B-04. CRC owns independent acceptance; this report cannot close rows. |
| 2 | P `cdc-directive09.md`, all; P `arc11-observation-workbench/arc-plan.md`, all | Directive09 authorizes Arc11's LFO → bounded Scope → terminal workbench sequence. Arc plan marks Slice03 as an immutable post-stop inspection on accepted Scope, with no new drain, async output, scheduler, or public API; P-07 and CDC composition remain open. |
| 3 | P `cdc-directive08.md`, “Prompt quality for all remaining work” and “Checks and correction policy from Slice03 onward” | Concrete recipes/test oracles and affected native gates are required. Use focused behavioral proof, `make test`, `make lint`, changed-crate all-target Clippy and strict docs; do not run `make check`, coverage, benchmarks, or an invented validator campaign. Preserve failures/blocked/ignored results and rerun only affected checks after a correction. |
| 4 | S `CLAUDE.md`, all; S `crates/oxurack/Cargo.toml`, all | Project conventions route skill use through installed local roots and require separate planning/source worktrees. Existing example dependencies already include clap, midir, ctrlc and the required modules; no dependency or manifest change is needed. |
| 5 | S `crates/oxurack/examples/turing-machine-midi.rs`, all | At `6c56e84`, `Cli` uses clap derive; `prepare_scope` only enables CSV and configures before opening its file (lines 87–95), runners call it before starting/connecting (141–145, 193–196), and `finish_scope` currently attempts CSV only (97–128). Offline output is the stdout MIDI trace (148–167). Live `playback::play` returns after cleanup before outer stop/finalization (193–231). These are the integration points to extend. |
| 6 | S `crates/oxurack/src/scope.rs`, `ScopeSelector`, `MidiDelivery`, `ScopeRecord`, `ScopeCapture` and public methods / CSV implementation (15–145) | Selectors are stable caller order (100–103); `records()` promises only `impl Iterator` in insertion order (106–109). Capacity is a single mixed FIFO counting signal and MIDI entries; overflow drops oldest records and may retain partial samples (72–77). CSV exports only retained rows (111–145). `ScopeRecord`/delivery are non-exhaustive; current signal values are scalar, and MIDI is a separate record. |
| 7 | S `crates/oxurack/src/lib.rs`, Scope methods and lifecycle/sample/drain APIs (265–365), plus S `crates/oxurack-core/src/tick.rs`, `SampleTime` definition and methods (60–92) | `start`/`stop` preserve module state and time; `clear_scope` only clears rows/overwrite count while retaining capture; configuration is stopped-only and validates exact selectors; `drain_midi` is the sole explicit runner drain. `SampleTime.completed_samples()` is authoritative and advances after Observe; a record maximum cannot replace it. |
| 8 | S `crates/oxurack/examples/turing_machine_midi/playback.rs`, `MidiSender`, `Session`, `PlaybackFailure`, `play`, `drain_generated`, `run_samples`, and named tests/helpers | Session records `Sent` only after a successful software send (109–133); cleanup attempts happen in `finish` (135–161); `play` stops, final-drains generated events, finishes cleanup and flushes trace before returning (199–220). `drain_generated` records generated copies. Existing fast-rack/Recorder/event tests establish independent oracle, failure-prefix, cancellation, and cleanup boundaries; playback stays protected and unchanged. |
| 9 | S `crates/oxurack/examples/turing_machine_midi/oracle.rs`, both protected patch fixtures, all | Oracle independently derives the seeded MIDI trace; LFO fixture supplies `lfo_main.bipolar_out`; original fixture supplies the pinned 2,000-sample comparison. No fixture or oracle edits. |
| 10 | S `crates/oxurack/tests/scope.rs`, patch/rack/run helpers and `capacity_clear_resume_and_mixed_midi_fifo_are_bounded` (helpers and 272–365) | Existing real installed-LFO sequence establishes capacity-3 retention of samples 2/3/4 and overwrite count 2; clearing keeps capture enabled, resumed samples continue at absolute sample 5, and mixed MIDI/signal entries share FIFO capacity. Reuse the behavior, do not edit this test. |
| 11 | S `README.md`, musical MIDI and Scope capture sections (110–183) | Existing commands and CSV meanings provide compatibility baseline; revise this section with the terminal view, bounded/retained semantics, clear versus new process and a small continuation example. |
| 12 | Installed `rust-guidelines/SKILL.md` and `testing/SKILL.md`, all | Rust task workflow and testing gate selection loaded. Existing clap derive, `Box<dyn Error>`, and standard writer patterns govern; skill default dependencies/runtime do not override the repo. |
| 13 | Installed Rust guides: `11-anti-patterns.md` AP-33; `03-error-handling.md` EH-01; CLI `02-argument-parsing.md` CLI-05; CLI `04-output-and-ux.md` CLI-21 and CLI-27 | AP-33/EH-01: propagate report write/flush failures as recoverable errors and never silently suppress them. CLI-05: keep clap derive and parse-time validation. CLI-21/27: opt-in human view to stderr while MIDI trace stays on stdout and CSV remains a file format. |
| 14 | Installed `collaboration-framework/knowledge/testing/guides/03-validation-gates.md`, all | Proportionate proof: focused real execution plus repository-native changed-surface gates; state limitations and do not turn evidence helpers into an extra product. |

## Contract readback

**Retained window.** Scope is one bounded mixed FIFO, not separate queues. The
view computes counts, span, latest selected values and tails solely from retained
records. `overwritten` is the eviction count, not a recoverable description of
lost records. A full FIFO can begin mid-sample or contain Generated without Sent;
therefore no evicted-history reconstruction, generated/sent pairing, or audio
acknowledgement may be implied. `completed_samples` comes from `rack.sample_time()`.
Signal and MIDI tails each contain at most requested `rows` in insertion order,
independently of the latest-per-selector section. `records()` is only an
`Iterator`; implementation must use forward scans and avoid cloning the capture.

**Streams and value labels.** Existing sample-indexed performance trace remains
byte-identical on stdout. The opt-in plain terminal view goes to stderr after
stop/cleanup; CSV remains the existing schema in its selected file. Selector and
path strings use Rust Debug escaping. Current Float/Bipolar/Gate/Raw variants
render with their scalar values, gates as true/false; MIDI bytes are uppercase
hex. Unsupported future non-exhaustive record/delivery/value variants return a
rendering `io::Error`. A held `Value::Midi` is not an event stream. `Sent` means
the live software sender accepted that event, not that a synth acknowledged it.

**Enabling and preflight.** Capture is enabled by `--scope-csv` OR
`--scope-view`; CSV and view compose. View-only opens no file. Port/capacity
require either output, rows require view, and all Scope options conflict with
`--list-outputs`. Rows parse within 1..=1000, default 12 when viewing; capacity
remains positive with default 100000. Exact last-dot selector parsing and stopped
configuration stay intact. Configure and open a requested CSV before rack start
or native connection so selector/file failures prevent playback. No Scope flags
preserve disabled behavior and CSV-only remains compatible; no selectors means
MIDI-only capture.

**Failure ordering and lifetime.** Finalization runs once after stop and, for
live mode, after `playback::play` has drained and finished Session cleanup. Keep
the primary playback/trace failure first; then attempt CSV write, CSV flush, view
write, and view flush in that order, collecting each failure without skipping
later requested output. Any reporting error returns failure; none can turn a
primary error into success. No view drains MIDI, advances samples, mutates
capture/time, or accesses the endpoint. Generic writer injection proves write
and flush failures without filesystem-specific devices.

**Clear versus fresh run.** `clear_scope` clears retained records and overwrite
count while capture remains enabled; `start`/`stop` preserve state/time, so a
continuation keeps absolute sample indices and LFO phase. A fresh rack/process
reinstalls the seeded patch and begins a fresh deterministic run. Documentation
must explain this distinction and show a small stopped-rack clear/continue
sequence; no reset command is added.

## Implementation and proposed-done evidence

Source implementation is committed on the accepted S branch in
`f0b045330c8bb74fb3267ba8ef0a515dfaf1012e`; a focused cancellation-finalizer
test follow-up is committed at `22500fae234d5dbba599c7683f8893aabe70f47b`.
Together they change exactly these paths:

- `crates/oxurack/examples/turing-machine-midi.rs`
- `crates/oxurack/examples/turing_machine_midi/workbench.rs` (new)
- `README.md`

Scope/facade/core algorithms, playback/oracle, existing Scope/playback tests,
fixtures, manifests and lockfile were not changed. No endpoint was selected.
No API/dependency research beyond the required packet was needed.

### Behavior and runnable demonstrations

- `workbench::write_view` scans retained capture only, keeps caller selector
  order, reports span/counts/latest/two bounded tails, escapes paths/labels,
  returns `InvalidData` for unsupported future record/delivery/value variants,
  and does not alter `ScopeCapture` or `SampleTime`. Example tests cover the
  loaded saw-up sequence (samples 0–3: -1, -.875, -.75, -.625), exact `90 3C
  5A` Generated/Sent lines copied from an actual event, capacity-3 samples
  2/3/4 with two overwrites, empty/MIDI-only views, caller order, scalar
  variants/nonfinite values and control escaping.
- CLI tests cover view-only, CSV-only, combined outputs, repeat selectors,
  orphan flags, rows 0/1001, accepted bounds 1/1000, default 12, conflicts,
  last-dot parsing and positive capacity. Unknown selector and invalid CSV path
  fail before a sample starts. Generic failing writers prove the order
  primary → CSV write → CSV flush → view write → view flush and that either
  successful output survives failure of the other.
- The real `playback::play` test uses an accelerated loaded fixture and a sender
  failing on its second performance send. It observes the Generated batch,
  one successful Sent event, cleanup calls before finalization, cleanup excluded
  from Scope, and the primary error retained with the final view. Existing
  runner cancellation tests remain unchanged. A second real runner test sets
  cancellation after the first successful send, verifies no later samples or
  drain remain, and renders the stopped partial capture with generated/sent
  counts and cleanup calls excluded from Scope.
- `./target/debug/examples/turing-machine-midi --patch
  crates/oxurack/examples/patches/turing-machine-lfo.ron --samples 10000`
  with the four README selectors and `--scope-view --scope-rows 12
  --scope-capacity 100000` produced 70 stdout rows with SHA-256
  `988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
  View: `completed_samples=10000 capacity=100000 retained=40070
  overwritten=0`, counts signal=40000/generated=70/sent=0. Two fresh-process
  trace and view outputs compared byte-for-byte. The combined CSV+view run
  produced 40,071 CSV lines; its stdout equals view-only and CSV equals both
  CSV-only output and the accepted Slice02 executable's CSV bytes. Combined
  view body equals view-only output.
- Capacity 3 with only `lfo_main.bipolar_out` retained samples 9997–9999,
  reported 10067 overwritten records, completed=10000, generated=0, and kept
  the same 70-row stdout hash. A zero-sample combined run showed the selected
  port as unobserved, empty tails, and a one-line CSV header.
- The no-capture original 2,000-sample fixture still produced 13 rows with
  SHA-256 `98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.
  The 10,000-sample no-Scope LFO trace compared byte-for-byte with the viewed
  run. The accepted Slice02 baseline was extracted to `/private/tmp` from
  `6c56e8460b693c285b8052633165e66129318604`; its CSV matched the new capture
  byte-for-byte (40,071 lines, SHA-256
  `dd3986a6300af3b43582f12338d91a8ad17fafb7f7d66f7c33873d4a6fdae368`).

### Validation receipts and limits

| Command / demonstration | Result |
| --- | --- |
| `cargo test --locked -p oxurack --example turing-machine-midi` | 25 passed, 0 failed, 0 ignored (default features). |
| `cargo test --locked -p oxurack --no-default-features --example turing-machine-midi` | 25 passed, 0 failed, 0 ignored. |
| `cargo test --locked -p oxurack --test scope --test lfo --test midi_output` | 20 passed, 0 failed, 0 ignored. |
| `cargo test --locked -p oxurack --no-default-features --test scope` | 6 passed, 0 failed, 0 ignored. |
| `make test` restricted | Workspace run exits 2. The `oxurack-rt` suite reports 146 passed, 4 failed, 3 ignored. Four MIDI port tests fail because CoreMIDI reports `MIDI support could not be initialized`; other workspace suites and doctests pass. Raw log: `/tmp/oxurack-slice03-make-test-restricted.log`. |
| Unchanged `make test` host-access retry | Passed; the `oxurack-rt` suite reports 150 passed, 0 failed, 3 ignored, and the full workspace command completes successfully. Raw log: `/tmp/oxurack-slice03-make-test-host.log`. The 3 ignored tests are existing RT scheduling/timing cases. |
| `make lint` | Passed Clippy and formatting. |
| `cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings` | Passed. |
| `RUSTDOCFLAGS='-D warnings' make docs` | Passed; generated umbrella docs. |
| `cargo build --locked -p oxurack --example turing-machine-midi` | Passed. |
| `git diff --check` | Passed. |

All Cargo runs report the existing `block v0.1.6` future-incompatibility
advisory; no warning was suppressed and no dependency was changed. During
development, the first focused renderer attempt had 15 passed/4 failed because
the test fixture setup omitted saw-up/accelerated clock parameters; after
correcting the setup, the default and no-default runs each passed 25/25,
including the added cancellation-finalizer integration case.
The first `make lint` found formatting differences; exact changed Rust files
were formatted and subsequent lint runs passed. These failed attempts are
retained here rather than described as successful runs.

### Row walk, self-review and handoff

| Row | CC evidence and status | CRC boundary |
| --- | --- | --- |
| B-01 | Proposed-done at source `f0b045330c8bb74fb3267ba8ef0a515dfaf1012e`: literal loaded scalar sequence, stable selector order, exact independent MIDI bytes, bounded/mixed/empty/escaped cases and immutable snapshot/time are covered. | Open; CRC independently checks retained-window semantics and output literals. |
| B-02 | Proposed-done at source `f0b045330c8bb74fb3267ba8ef0a515dfaf1012e`: parser combinations/bounds/conflicts, preflight failures, unchanged CSV bytes and stdout trace are evidenced above. | Open; CRC checks CLI contract and baseline CSV/trace comparison. |
| B-03 | Proposed-done at source `22500fae234d5dbba599c7683f8893aabe70f47b`: injected write/flush errors, primary-first aggregation, independent report success, real second-send failure, cancellation and cleanup-before-view are evidenced above. | Open; CRC independently reviews finalizer ordering and runner integration. |
| B-04 | Proposed-done at source `f0b045330c8bb74fb3267ba8ef0a515dfaf1012e`: README recipes, pinned fixture hashes, fresh-process equality, baseline CSV equality, capacity-3 and zero-sample outputs, and all prompt gates are recorded above. | Open; CRC reviews runnable evidence and documentation. |

Self-review found no edits outside the three authorized S paths, no library,
playback, fixture, manifest or lockfile changes, and no new dependency, API,
endpoint witness or evidence tooling. Scope matches the assignment. The only
environment-limited result is the restricted CoreMIDI run; its identical host
retry passed. No physical MIDI endpoint was selected, as none is required.
No supporting artifacts are needed; raw failure/host logs remain in `/tmp`.
CC proposes these rows done but does not close them. CRC owns independent child
acceptance; Arc11 composition and CDC's integrated gate remain open. Bubble-up:
Slice03 adds a usable finite workbench view/export atop accepted Scope, without
changing the parent capture/runner design or claiming parent/arc acceptance.
