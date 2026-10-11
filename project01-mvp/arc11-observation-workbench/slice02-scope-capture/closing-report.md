# Arc11 Slice02 — CC intake and proposed closure

## Intake readback (2026-10-11)

The issued prompt and sibling plan/ledger define the active five-row assignment.
Planning is clean at `deb8690c6a0cba5df1e28b91601120a3737f3e5d`; the source
worktree is clean at accepted predecessor
`c4b517c89f51d44c0eb1bec4a7b891984d3e0a92` on
`codex/oxurack-module-behavior-installation`. The prompt names this file as the
intake/closeout record, but it was absent from the clean planning checkout; this
file is created at that designated path before implementation edits.

Required reading completed within the prompt's bounds: the full assignment,
slice plan, ledger, directive09, Arc11 plan, source `CLAUDE.md`, umbrella Cargo
manifest, Scope-relevant sections of the umbrella facade, core patch/port/value/
tick APIs, MIDI output buffer/event API and event production, full example CLI
and playback runner, full LFO integration/oracle/fixture inputs, and the listed
Rust/testing/project-management/engineering-methods guidance. Source reads
included the complete `Value` implementation; unrelated tests in core sources
were not treated as required. The approved source scope is six paths only:
new `scope.rs`, facade `lib.rs`, new `tests/scope.rs`, the existing example CLI
and playback module, and root README. The required test evidence belongs in
those implementation paths; no module algorithms, core/RT/MIDI ownership,
fixtures, manifests, or terminal interface changes are authorized.

### Contract readback from pinned source

- **Selection and atomicity:** `PatchHandle.modules` maps exact instance names
  to entities (`crates/oxurack-core/src/patch.rs:497-503`); `Port` and
  `CurrentValue` are child-port components, and the spawn helper attaches them
  through `ChildOf` (`crates/oxurack-core/src/port.rs:106-153`). The facade's
  `load_patch` inserts the new handle only after patch application succeeds
  (`crates/oxurack/src/lib.rs:194-220`). Scope will resolve these exact entities,
  validate all selectors and fallibly reserve a local candidate before replacing
  the prior resource. Only Float/Bipolar/Gate/Raw with matching current values
  are admitted; failures preserve old capture state.
- **Capacity:** the plan defines capacity as the number of mixed signal/MIDI
  records, with oldest-first eviction and a single overwrite counter. It is not
  a sample/frame/byte limit. Selector labels are held once; records store their
  selection index.
- **Observe latch:** core `SampleTime` publishes the prior completed count as
  the zero-based current index (`crates/oxurack-core/src/tick.rs:57-95`). The
  sample-start gate is latched before phases and finalization occurs after
  Observe (`crates/oxurack-core/src/tick.rs:105-122`). The observer therefore
  reads `last_index` and does not independently check `SimulationRunning`, so a
  stop during Consume does not discard the already-started sample.
- **MIDI borrow ownership:** `drain_midi` returns a borrowing `Drain` whose
  early drop discards the remaining batch (`crates/oxurack/src/lib.rs:286-297`).
  The existing runner drains after sample advancement; it remains sole owner.
  Implementation will collect each bounded drained batch, end the borrow, copy
  all events as Generated in original order/sample, then mark only successful
  `Session::send_performance` calls Sent. Cleanup messages from `Session::finish`
  remain outside capture.
- **Failure/export and compatibility:** the plan requires opening the selected
  CSV before sample start/native connection, exporting and flushing after
  stop/cleanup even on playback failure, reporting both errors with playback
  first, keeping trace bytes on stdout, and printing capture bounds on stderr.
  CSV writes preserve retained FIFO order, escape comma/quote/CR/LF, and always
  include the exact header, including for an empty capture. No-capture output,
  clocks, module state transitions, drain API behavior, and cleanup ownership
  remain unchanged.

The pinned facade confirms that start/stop retain module state and sample time
(`crates/oxurack/src/lib.rs:258-279`), and `advance_samples` completes a sample
whose start gate was already latched (`crates/oxurack/src/lib.rs:305-321`). The
required behavior and verification rows are coherent with the existing source;
no authority change is needed before dependent edits.

## Implementation and CC proposed-done evidence

### Five-row walk

- **S-01 — proposed-done.** The public integration test loads the accelerated
  Clock/LFO/Turing patch and a test-only Raw publisher through the real module
  registry. Four samples assert the literal Clock pulse, LFO bipolar/unipolar,
  propagated affine Turing modulation, and Raw(17), in selector order and at
  indices 0–3. A second test transfers the configured rack into Bevy, stops in
  Consume, and observes sample 0 exactly once before the next stopped update.
- **S-02 — proposed-done.** Capacity three over five samples retains indices
  2,3,4 with values -.75,-.625,-.5 and overwrite count 2. A stopped advance adds
  nothing; clear resets only records/count and resume captures absolute sample 5
  at -.375. The mixed signal/generated/sent/signal sequence retains the final
  three records in FIFO order with one eviction. Reconfiguration starts empty;
  disable is idempotent; failed file load preserves capture; successful reload
  disables it. The successful reload fixture uses a different MIDI channel
  because existing patch loading retains previously spawned entities; no
  stronger entity rollback claim is made.
- **S-03 — proposed-done.** Tests cover no patch, missing module/port, duplicate
  selector, Midi kind, zero capacity, `usize::MAX` reserve failure, running
  configuration, and missing/wrong `CurrentValue`. Rejected replacements leave
  the previous records intact. The scope integration target passes with and
  without the graph feature. The implementation uses exact PatchHandle names
  and `ChildOf` child ports; no graph or label fallback is present.
- **S-04 — proposed-done.** Two fresh loaded copies of the LFO fixture produce
  the same non-empty 10,000-sample MIDI trace with capture disabled and enabled;
  every captured Generated event matches sample/bytes/order, and offline capture
  has no Sent records. The actual `play` runner's fake-sender test matches the
  independent MIDI oracle and capture-off trace. Additional runner tests cover
  successful sends, cancellation, a later send failure, Generated batches,
  successful Sent prefixes, terminal drain, and cleanup traffic excluded from
  Scope. A declared Midi-valued test port is rejected as a selector.
- **S-05 — proposed-done.** CSV tests cover the exact nine-column header,
  empty captures, scalar formatting, escaped module/port labels, MIDI bytes,
  and propagated writer errors. CLI tests cover scope flag requirements,
  positive capacity, last-dot selector parsing and list conflict. Built-binary
  checks confirmed malformed/orphan/list-conflict options and a bad output path
  exit nonzero before sample output. The README documents the offline four-port
  run and optional live invocation. The documented capture retained 40,070
  records (40,000 signal + 70 Generated MIDI), overwrote 0, emitted 40,071 CSV
  lines, and had byte-identical stdout to capture-disabled execution.

### Commands and receipts

All required commands were run from S at the accepted source predecessor plus
these six changes:

```text
cargo test --locked -p oxurack --test scope                         PASS (6)
cargo test --locked -p oxurack --no-default-features --test scope    PASS (6)
cargo test --locked -p oxurack --lib --example turing-machine-midi   PASS (32 + 15)
cargo test --locked -p oxurack --no-default-features --lib --example turing-machine-midi
                                                                      PASS (30 + 15)
cargo test --locked -p oxurack --test lfo --test midi_output        PASS (12 + 2)
make test                                                            PASS on host retry
make lint                                                            PASS
cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings
                                                                      PASS
RUSTDOCFLAGS='-D warnings' make docs                                 PASS
cargo build --locked -p oxurack --example turing-machine-midi       PASS
git diff --check                                                     PASS
```

The first restricted `make test` run failed four RT tests because CoreMIDI
initialization was unavailable (`Midi support could not be initialized`). The
unchanged host retry passed; the full run reported 3 pre-existing ignored RT
tests (priority permissions and two timing tests). `block v0.1.6` emitted its
existing future-incompatibility advisory. The first `make lint` and strict
clippy attempts found a simplifiable `map_or` and a test `filter_map`; both were
corrected without lint suppressions, and the final gates passed. No endpoint was
listed, opened or used. Coverage/freshness/benchmark campaigns and ignored RT
tests were not run, as directed.

The built example receipts match the prompt's pinned hashes:

```text
LFO fixture, 10,000 samples: 988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c (70 rows)
Original fixture, 2,000 samples: 98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b (13 rows)
LFO capture-enabled stdout: same LFO hash; cmp against disabled stdout passed
CSV lines: 40,071; stderr: capacity=100000 retained=40070 overwritten=0
```

### Self-review and delivered scope

Source changes are limited to the six authorized paths: `crates/oxurack/src/scope.rs`
(new), `crates/oxurack/src/lib.rs`, `crates/oxurack/tests/scope.rs` (new),
`crates/oxurack/examples/turing-machine-midi.rs`,
`crates/oxurack/examples/turing_machine_midi/playback.rs`, and `README.md`.
Planning changes are limited to this report and `ledger.md`; the issued prompt
and slice plan are unchanged. No manifests/lockfile, core/RT/MIDI algorithms,
fixtures, LFO/oracle files, dependencies, or terminal interface were changed.
No new design/dependency finding required Duncan escalation. Arc11's terminal
view remains Slice03 work after CRC acceptance.

Source commit: `6c56e8460b693c285b8052633165e66129318604`, with exactly the six
authorized source paths listed above; the source worktree is clean after commit.
The planning report/ledger are being committed separately on `planning`; its
exact hash and clean tree state are reported in the return. S-01 through S-05
remain **open**; these are CC proposed-done receipts, not independent CRC
acceptance or arc advancement.
