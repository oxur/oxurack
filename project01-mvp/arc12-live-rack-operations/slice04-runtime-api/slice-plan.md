# Arc12 Slice04 — explicit rack lifetimes

Version1.1 — 2026-10-12. Status: open, CRC return for correction; no acceptance.
Current assignment: `arc12-live-rack-operations/slice04-runtime-api/cc-prompt-iteration01.md`.
Three contributors, Expedited Mode. CRC acknowledges CDC directive12 at planning
67e9150; source predecessor deafdd228fb44720878c700f8ff62b2a59ddbad6.
Current correction source baseline: f6fdeb513331a8d8962876bdf4766cedfce63f59.
Depends on CRC-accepted Slice01–03. Main a7ce6e6 remains protected. Six LF rows
remain open; correction1/5 issued (zero correction passes completed). CC implements,
CRC independently reviews; CDC retains Arc12 acceptance. Slice05 stays unopened
until this child is accepted.

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

## Initial return and corrective assignment — 2026-10-12

CC returned f6fdeb513331a8d8962876bdf4766cedfce63f59 with planning622e9948.
CRC independently reproduced every native gate (restricted CoreMIDI failure and
passing exact host retry preserved), actual default/no-graph/fresh72-row musical
replay, ordinary save, six CLI failures and both original fixture hashes.
[CRC verification](crc-verification.md) identifies F-01–F-06: incomplete full-owner
projections, multi-lifetime hook proof, actual reset/dirty/capacity oracles, complete
admission/real-fault/current-restart/Live matrix, immediate notifications/candidate
output rejection, and actual example/capture tests. The retired-sample diagnostic
reports fresh0 instead of retired300. No lifecycle implementation defect is
claimed reproduced. All six LF rows stay open; initial proposed-done evidence is
retained. Iteration01 collects the missing observations in existing test homes
plus narrow example factoring/diagnostic repair, without a new proof platform.

## Iteration01 author readiness — Sofie/CRC, 2026-10-12

1. Source-grounded: eleven-path delta, lifecycle/MIDI production and complete new
   tests/example inspected at f6fdeb5. Existing LiveBehaviorCounter, raw sink,
   TimeUpdateStrategy, snapshot failure, typed messages and graph rosters provide
   viable observation seams. Native/CLI/CSV receipts independently reproduced.
2. Design-complete: CDC12 unchanged. Test-before/after boundaries retain the old
   owner; only supported per-App effects are promised. Six test/example paths,
   no library production repair prescribed, no05 or endpoint scope. Missing
   production proof is distinguished from a reproduced defect; failures return
   exact causes instead of weakened oracles.
3. Guideline-applied: guide07/template and correction-retention guide applied;
   Rust2.0 AP-08/09/42/44, CLI-34/35/37/38 and27 support actual sink/recipe checks,
   private access, fallible writers and isolated files. No dependency imported
   from guideline examples. All relevant actual entries inspected by CRC.
4. Executable: prompt six finding recipes name test locations/private/public
   access, normalized within-World identities, actual candidate output fixture
   without cables, raw note/peak bytes, real fault252, live clocks and parsed
   writer-based example. Stop/drain precedes replacements; into_app last only.
5. Falsifiable: literal unread note0/cleanup1; real CC/NoteOff dirty marking;
   complete80+48 vector/second reset; restart after temporary input deletion;
   actual Live zero/1ms transitions; immediate failure messages; actual parsed
   recipe and retained MIDI capture. Fresh CSV independently grounds probability
   default0 then.5 from1. Each rejects a named plausible omission.
6. Coherent/portable: bounded ordered manifest, full current plan/ledger/review,
   relevant preserved prompt sections and domain entries, concise intake/readback
   in existing close record. Fresh affected tests/native/example gates; unchanged
   strict-docs/original pins may retain CRC2026-10-12 f6fdeb5 evidence after region
   audit. No new artifact forest; six LF rows/parent obligations all mapped.

## Assignment history

| Date | Prompt | Predecessor / authority | Disposition |
| --- | --- | --- | --- |
| 2026-10-12 | arc12-live-rack-operations/slice04-runtime-api/cc-prompt.md | Slice03 CRC accepted deafdd228; CDC directive12 planning67e9150; Duncan relay | Returned f6fdeb5/622e9948; independently reproduced native/musical checks, incomplete interaction proof; superseded by iteration01, no acceptance |
| 2026-10-12 | arc12-live-rack-operations/slice04-runtime-api/cc-prompt-iteration01.md | CRC review f6fdeb5; F-01–F-06; original assignment preserved | Active CC test/evidence correction1/5; six LF rows open; Slice05 held |

## Version history

- v1.1 — 2026-10-12: independent CRC native/musical reproduction, six collected
  interaction-proof findings and accurate diagnostic repair assigned in iteration01;
  six LF rows stay open. No design/ancestor acceptance or source change by CRC.

- v1.0 — 2026-10-12: CRC acknowledges directive12 and opens the approved lifecycle
  increment with six rows, concrete implementation/test recipes and native gates.
  Source/main unchanged; instance commands remain unopened Slice05.
