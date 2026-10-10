# Slice03 — installed MIDI output and executable seeded trace

Version 1.0 — 2026-10-10. Status: open; initial CC assignment issued by CRC.
Project01 / Arc10; Three-Contributor Workflow, Expedited Mode.
Current assignment: `arc10-turing-machine-midi/slice03-midi-output/cc-prompt.md`.
Accepted source predecessor: `b2ffb16d342d9a6cc800bab2713813d9646fb505`, branch
`codex/oxurack-module-behavior-installation`, clean at authoring.
Planning predecessor: `95bf441` (CRC Slice02 acceptance).
Authority: [directive08](../../cdc-directive08.md), [design v1.2](../design.md)
sections 1–3 and 5; Slice02's [independent acceptance](../slice02-turing-machine-adapter/crc-verification.md#iteration01-review--accepted-2026-10-10).
CDC owns arc design; CRC authors/reviews this approved successor; Duncan relays CC.

## Outcome and definition of done

Run `cargo run -p oxurack --example turing-machine-midi -- --patch
crates/oxurack/examples/patches/turing-machine-midi.ron --samples 2000` from the
implementation worktree. A real installed Clock → TM → MIDI-out patch streams
deterministic sample indices and MIDI bytes to stdout, without MIDI hardware.
The ordinary rack facade exposes a bounded per-sample drain. Five rows M-01–M-05
in [ledger.md](ledger.md) own acceptance; no fixed test-count target.

The [CC prompt](cc-prompt.md) owns the concrete API, wiring, validation,
admission/partial-sample contract, worked transition, source recipe and test
oracles. It implements CDC's single-owner design without an async service.
The only core scheduling addition labels the existing pre-Produce begin-sample
system so an unread-output guard can run before its running latch. It does not
add a phase, scheduler, clock, worker or independent execution authority.

Static M1 sink configuration is supplied at spawn and readable through registered
getters; live sink edits are successor work. One channel per sink within a rack,
one fresh patch per example run. Existing patch-load partial mutation is not
changed or advertised as transactional. The standalone Clock/TM engines and
adapters, seed derivation, RT bridge, wire masking semantics and graph contracts
remain inherited. Slice04 extends this exact example with direct endpoint
selection, pacing, sent-note ownership and stop/error cleanup. This slice does
not claim device playback, cleanup of physically sent notes or arc closure.

## Scope and validation

New framework module crate `crates/oxurack-mod-midiout/`; integration in core's
existing begin-sample label/error types and umbrella facade/Cargo/examples/tests.
Small README and CLAUDE.md corrections are authorized. Exact component scope,
commit-path reporting and preservation rules are in the prompt. Private helper
organization and adjacent regression fixes inside those components are local
discretion; changed public behavior, major scope or acceptance returns to CDC.

Use actual module and runner behavior tests, `make test`, `make lint`, changed
crate all-target Clippy, strict `make docs`, default/no-default facade integration,
standalone Clock/TM checks, two fresh example processes and `git diff --check`.
Do not inherit dependency freshness/tool installation, coverage/soak/jitter or
old planning validators. Existing host MIDI failures need disclosed host retries;
three ignored RT timing tests remain counted. Source-specific failures stay open.

Supporting evidence lives in this slice's `artifacts/`; concise intake, test and
command evidence belongs in CC's canonical `closing-report.md`. Optional raw logs
are useful for failures/retries; no new report validator or receipt framework.
CRC acceptance is `crc-verification.md`. All source/evidence commits are scoped
and explicit; CC proposed-done cannot close a row independently.

## Author readiness — 2026-10-10

1. Source-grounded: inspected accepted facade install/load/advance/stop methods,
   core latch and five-phase registration, module/port/patch APIs, published TM
   ports, MidiWire constructors, manifests and existing fixture/test runners.
   Checked Bevy 0.18.1 Mut::into_inner in pinned registry source for borrowed drain.
2. Design-complete: prompt fixes schema/config, lexical order, transition bytes,
   bounded admission, terminal failures, pre-latch guard and per-call completion
   accounting; no consequential decision is delegated to CC.
3. Guideline-applied: framework 2.6.0 authoring guide/template and Proportionate
   Proof; Rust 2.0.0 AP-23/AP-25/AP-33, API-23/API-39, EH-01, TD-05, DC-05 and CLI-05
   support the owned buffer, typed errors, validation, private state and thin CLI.
4. Executable: the workspace already includes `crates/*`; module hooks receive
   the actual schedule. A narrow core label gives the guard explicit precedence
   over the existing latch. The drain borrow ends before the next facade action.
   All sketches are illustrative, not compiled implementations.
5. Falsifiable: exact retrigger/idle/silent bytes, opposing spawn/name order,
   invalid data without masking, pre-advance unread-output state/time preservation,
   partial completion and independently assembled seeded MIDI trace reject the
   named mistakes. Fresh processes exercise the actual example.
6. Coherent/portable: prompt/plan/ledger agree at accepted `b2ffb16`; required
   shared reading is bounded to current sections. Skills resolve from the installed
   profile root, with equivalent-path discovery on CC's host. Historical packets
   and logs are reference-only; no new proof machinery is commissioned.

## Assignment history

- 2026-10-10: CRC issued initial `cc-prompt.md` after accepting Slice02 at
  `b2ffb16`; zero corrective passes. Five rows open. Slice04 remains unissued.
