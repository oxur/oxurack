# Slice04 — direct MIDI playback, cleanup and first musical trial

Version 1.2 — 2026-10-10. Status: open; software accepted, Operator witness pending.
Project01 / Arc10, Three-Contributor Workflow, Expedited Mode.
Current assignment: `arc10-turing-machine-midi/slice04-output-runtime/cc-prompt.md`.
Accepted predecessor: `2942c594a445b5de3ef8f131cdedaa6095aa1529`, clean on
`codex/oxurack-module-behavior-installation`. Planning predecessor `f19e2f1`.
Authority: [directive08](../../cdc-directive08.md), [design v1.2](../design.md)
§§4–5 and accepted [Slice03 review](../slice03-midi-output/crc-verification.md).
The directory keeps its planned output-runtime name; this is a direct runner,
not the retired asynchronous RT service. CDC owns design, CRC details/reviews,
Duncan relays assignments and witnesses endpoint use.

## Outcome and definition of done

The existing `turing-machine-midi` example lists endpoints and plays its real
seeded Clock→TM→MIDI-out patch on an explicitly selected destination, paced at
1ms logical samples with slowdown under late scheduling. Finite completion,
Ctrl-C and failures stop advancement, discard unsent events and attempt cleanup
of successfully sent notes plus channel CC123/120. Original performance and
cleanup failures stay visible. The offline trace command remains compatible.

[cc-prompt.md](cc-prompt.md) is the concrete implementation contract: CLI names,
selection/preparation order, sender/owned state, successful-send commit boundary,
cleanup order/idempotence, interrupt boundaries, pacing algorithm, source recipe,
worked Rust shapes, test oracles and return scope. All five O rows in
[ledger.md](ledger.md) own acceptance. A real software or hardware endpoint and
Duncan's musical observation is O-05; a mock/byte trace does not satisfy it.
If unavailable, leave O-05 open with its blocker while returning completed
software. Only CDC may explicitly defer T-04 under design§5 and authorize Arc11.
No implicit deferral, arc close or release acceptance from child tests.

## Scope, inherited work and verification

Runner component: existing umbrella example, private helpers/inline tests under
`examples/turing_machine_midi/`, umbrella dev dependencies/necessary Cargo.lock
edges and README; adjacent umbrella regression tests allowed. No public library
API, new runtime/scheduler/backend framework, asynchronous worker, clock/transport
output, reconnect or DAW/system endpoint configuration. Existing Clock/TM/core/
sink/RT/wire production and literal seed42 musical fixture remain inherited.
The one ctrlc notification thread only writes a stop flag; MIDI/rack ownership
stays on the runner thread. No hard-real-time, OS-call timeout or SIGKILL cleanup
claim. Static monophonic distinct-channel sink configuration bounds note ownership.

Focused tests invoke the real loaded-rack playback loop through one narrow sender
seam, exercise failed sends/cleanup/cancellation and pure exact selection/deadline
helpers. The actual example provides CLI/trace controls and endpoint use. Gates:
example tests default/no-default, MIDI integration default/no-default, make test,
make lint, all-feature/all-target umbrella Clippy, strict make docs, example build
and diff check. Retain known restricted MIDI failures with host retries and three
RT ignores; no inherited freshness, validators, benchmarks, coverage or soak gate.
The prompt's test table names precise bytes and boundaries; no test-count quota.

Artifacts: none required beyond normal source/example/docs and canonical close
records. Optional supporting failure/retry logs or test evidence live in this
slice's `artifacts/`, created only as needed. CC records concise intake/readback,
commands, five-row walk, applied Rust choices and bubble-up in closing-report.md;
CRC independently records acceptance in crc-verification.md. Issued prompt is
preserved. Commits use only explicit actual path lists, source before evidence.

## Author readiness — 2026-10-10

1. **Source-grounded:** inspected accepted example/fixture/oracle, facade load,
   manual advance, drain/fault and stop, sink buffer and messages, manifests,
   Make recipes. Pinned registry inspection confirmed midir 0.11.0 owner-consuming
   connect/mutable send and ctrlc 3.5.1 fallible process-global handler; checked clap
   derive/Arg relationships. No API name guessed from current online versions.
2. **Design-complete:** resolved exact-name/index selection, pre-start errors,
   sent-note commit before trace write, all-message cleanup on failure, terminal
   repeated finish, cancellation during a batch, late pacing and witness policy.
   These detail existing CDC decisions; no architecture amendment is needed.
3. **Guideline-applied:** framework 2.6.0 guide07/template and Proportionate Proof;
   Rust 2.0.0 AP-23/AP-25/AP-33, EH-01, API-23/API-39, TD-05/DC-05, CLI-05/CLI-43
   read and applied to owned state, recoverable I/O, exact validation and graceful
   interrupt. Private application error aggregation is distinguished from library
   APIs; the signal guide's forced second interrupt is not adopted.
4. **Executable:** concrete dependency/example/helper sites, real borrow-ending
   drain and unconditional stop/discard/finish sequence provided. Actual sender
   adapter and inline test route preserve single ownership; illustrative sketches
   are explicitly not compiled. Pure deadlines need no new test clock service.
5. **Falsifiable:** failed off versus failed new-on reveals incorrect note ledger;
   cleanup faults cannot hide remaining CC attempts or primary error. Duplicate
   exact names/substring negatives, delayed real-rack output, cancellation within
   a drain and late deadline values distinguish the intended behavior. Operator
   witness separately proves endpoint use; mocks prove only software behavior.
6. **Coherent/portable:** prompt/plan/five-row ledger agree with design§§4–5 at
   source `2942c59`. Required reading scopes current parents/source/library symbols;
   installed equivalent skill paths and complete intake/recovery are explicit.
   Historical packets/matrices reference-only. A bounded example/owned lifecycle,
   relevant guide sections and inline tests fit one slice without a proof-tool
   project; full required skill reads leave implementation/recovery headroom.

## Assignment history

- 2026-10-10: CRC opened/issued initial cc-prompt.md after independently closing
  Slice03 at `2942c59`. Five O rows open, including explicit Operator witness O-05.
  No corrective pass and no Arc10/Arc11 advancement by this issue.

## CRC software review — 2026-10-10

[Independent review](crc-verification.md) accepts O-01–O-04 at `65522f1`; all
required software gates and original trace compatibility reproduce. O-05 stays
open for Duncan's named endpoint, receiver/routing, musical and stop observation.
Actual host outputs exist; this is not an unavailable-endpoint disposition.
No corrective prompt, slice close, arc acceptance or Arc11 advancement. Issued
assignment is preserved; source remains clean and unchanged by CRC.

Version history: v1.1 records partial acceptance without changing the contract.

## Operator runbook — 2026-10-10

Duncan requested exact witness commands/operations.
[Operator runbook](artifacts/operator-runbook.md) supplies routing, offline
reference, finite playback, Ctrl-C, fresh-session replay and report-back steps.
The 5,000-sample offline reference was executed at unchanged `65522f1`: 34 rows,
SHA-256 `918f4883252988093f737571cbb862aa29a33b737d0db026c98a7d7fea652d17`.
No endpoint was connected and O-05 remains open. Version1.2 adds this supporting
artifact without changing acceptance or issuing a new CC assignment.
