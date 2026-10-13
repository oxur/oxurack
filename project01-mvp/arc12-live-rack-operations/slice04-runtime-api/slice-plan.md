# Arc12 Slice04 — explicit rack lifetimes

Version1.0 — 2026-10-12. Status: open, implementation assigned; no acceptance.
Current assignment: `arc12-live-rack-operations/slice04-runtime-api/cc-prompt.md`.
Three contributors, Expedited Mode. CRC acknowledges CDC directive12 at planning
67e9150; source predecessor deafdd228fb44720878c700f8ff62b2a59ddbad6.
Depends on CRC-accepted Slice01–03. Main a7ce6e6 remains protected. Six LF rows
start open; iteration budget0/5. CC implements, CRC independently reviews; CDC
retains Arc12 acceptance. Slice05 stays unopened until this child is accepted.

## Delivered capability

Replace, unload and restart an actual musical rack without leaked prior entities,
channel reservations or partial failed candidates. A fresh isolated App is the
lifetime boundary. Stopped/drained admission precedes all fallible work; failed
candidates preserve the old owner. Start/stop continue state, reset appends narrow
MIDI silence without discarding unread events, restart reconstructs current saved
configuration at0. Ordered installation recipes support every new lifetime.

Extend the existing save-live-patch example with --lifecycle for actual parameter
and cable/label edits, save, unload, cleanup drain, reload, Scope and seeded replay.
The ordinary save example remains usable. No endpoint gate or second MIDI owner.
The prompt's signatures, operation matrix, staging/commit recipe, literal tests,
reading manifest and native gates are binding within directive12 D1/D2/D4.

## Boundaries and inherited work

Low-level core loader remains nontransactional and retains file validation policy;
facade isolation supplies failure preservation. Registration hooks run once per
App, including failed candidates, with supplied-world ownership and documented
panic/external-effect exclusions. No arbitrary World cloning/rollback promise.
Scope is disabled on successful replacement/restart/nonempty unload, preserved on
reset/stop/error. New lifetime fault clears; reset preserves the first fault.
Core Panic/SetBpm unsupported dispatch must return NotImplemented rather than success.

Removal, stale-safe handles, synchronous commands, bounded queued load/save/results,
reset cancellation and global BPM are Slice05. Separate MIDI transport/power is
Arc07; module-specific typed vocabulary and checkpoints retain catalog ownership.
Historical Arc05 A-05-01/02 remain deferred through their successor/composition walk.
No RON schema, engine algorithms, native RT transport, dependency or lockfile change.

## Verification and records

Six LF rows in ledger.md. Prove exact World membership, ordered hook replay,
late-spawner isolation, stopped continuation and failed-snapshot preservation;
128-message peak cleanup, literal bytes/timestamps, terminal recovery and actual
fresh seeded reconstruction. Use native core/MIDI-out/both facade feature modes,
workspace host retry, lint/strict Clippy/docs/build and real example/replay.
Fresh facade integration is required; previous receipts are not its substitute.
CC keeps itemized intake/readback, per-row observations, attempts/retries and
inherited walk in closing-report.md and ledger.md. Supporting artifacts: none
expected; /tmp for transient output, explain any necessary durable artifacts/
exception. Preserve issued prompt. Separate source and named planning commits.
No new verifier, coverage campaign, soak or hardware witness.

## Author readiness — Sofie/CRC, 2026-10-12

1. Source-grounded: inspected deafdd228 owner/install/load/clock/admission/Scope,
   persistence/getters, actual MIDI buffer/reservation/consume and tests, core
   loader/commands/errors/hook contracts, current save/rewire examples. CDC's leak
   witness/design return read; old handle-only replacement test is insufficient.
   Bevy0.18 World::try_query immutable API checked in pinned local dependency source.
2. Design-complete: directive12 resolves lifetime/fault/reset/transport boundary.
   Prompt fixes stopped-first admission, candidate output rejection, installation
   order, cleanup capacity/bytes/time/commit, empty unload and snapshot restart.
   No consequential choice is delegated to CC; successor05 is explicitly separate.
3. Guideline-applied: guide07/template, Rust2.0 anti-patterns and API-02 SHOULD,
   API-23/EH-01 MUST, TD-02/03 SHOULD, AP-08/09/42/44 AVOID, AP-43/55 CONSIDER,
   CLI-05/27 MUST and CLI-55 SHOULD inform owned staging, concrete errors, short
   borrows, narrow cross-crate helpers, reserved capacity and example streams.
   Panic exclusion/infallible new are explicit CDC compatibility choices.
4. Executable: fourteen exact source paths, worked erased installer, candidate
   apply/commit, immutable silence collection and caller recipe; native commands
   use existing packages/examples. Code sketches are explicitly uncompiled and
   name new seams versus verified existing symbols. Inline/private and consuming
   public test access are separated; no public World accessor for tests.
5. Falsifiable: same-channel replacement and valid[1,0] versus rejected[1,1];
   late valid spawner writes then errors; unread note0 followed by cleanup1;
   16 sinks80+48=128; CC-only/released/unused and repeat cases; typed busy/error
   precedence, retained fault, phase/Scope and seeded fresh reference; real edited
   save/unload/reload session in both modes and original trace hashes.
6. Coherent/portable: bounded ordered reading manifest with pinned inputs and
   intake extent/query coverage/readback in the existing closing record. Reading
   is chunkable with recovery budget; no historical full-project scan or new proof
   platform. Current04 contract is binding;05 is successor-only. Six open rows
   cover the scope; dependent edits follow coherent preflight without a new gate.

## Assignment history

| Date | Prompt | Predecessor / authority | Disposition |
| --- | --- | --- | --- |
| 2026-10-12 | arc12-live-rack-operations/slice04-runtime-api/cc-prompt.md | Slice03 CRC accepted deafdd228; CDC directive12 planning67e9150; Duncan relay | Initial implementation assigned; six LF rows open; no return or acceptance |

## Version history

- v1.0 — 2026-10-12: CRC acknowledges directive12 and opens the approved lifecycle
  increment with six rows, concrete implementation/test recipes and native gates.
  Source/main unchanged; instance commands remain unopened Slice05.
