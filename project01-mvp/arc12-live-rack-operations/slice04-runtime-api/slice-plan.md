# Arc12 Slice04 — explicit rack lifetimes

Version1.4 — 2026-10-13. Status: closed, CRC accepted at
`ec85ebe99c09b8a2ea64d55da0346af4ae317a67`.
Current assignment: none; accepted closing assignment is
`arc12-live-rack-operations/slice04-runtime-api/cc-prompt-iteration03.md`.
Three contributors, Expedited Mode; CDC12 unchanged. Six LF rows done.
Three corrective passes executed3/5. Prior evidence retains original revisions
and dates; see crc-verification.md. Source/main unchanged by CRC. Slice05 may
open; CDC retains arc acceptance and parent/project gates remain open.

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
| 2026-10-12 | arc12-live-rack-operations/slice04-runtime-api/cc-prompt-iteration01.md | CRC review f6fdeb5; F-01–F-06; original assignment preserved | Returned1242a60/9081268; CRC accepts LF-05/LF-06, four rows remain open; superseded by iteration02, no slice acceptance |
| 2026-10-12 | arc12-live-rack-operations/slice04-runtime-api/cc-prompt-iteration02.md | CRC review1242a60; remaining F-01–F-04; preserved prior assignments | Returned03825e5/18abe11; CRC accepts LF-01/LF-02, retains LF-05/LF-06; two rows open; superseded by iteration03 |
| 2026-10-12 | arc12-live-rack-operations/slice04-runtime-api/cc-prompt-iteration03.md | CRC review03825e5; remaining F-03/F-04; preserved prior assignments | Returned ec85ebe/972e3b7; CRC2026-10-13 accepts LF-03/LF-04, retains four rows; Slice04 closed, Slice05 may open |

## Iteration01 review / iteration02 readiness — 2026-10-12

CRC reproduced native gates, exact host retry and actual default/no-graph/fresh72
trace plus ordinary300/six CLI failures at1242a60. LF-05 candidate/error/event
boundary and LF-06 runnable capture/recipe are independently done. Remaining
F-01 resource/scalar continuation/normalized membership, F-02 ordered every-App
and both-instance replay, F-03 reset repeat/state/retrigger/overflow and F-04
current restart/real-fault candidate/empty/phase interactions stay open. No
implementation failure/design amendment is claimed. Four test-only paths;
accepted example/core/doc/fixture evidence retained under CRC's pinned rationale.
See crc-verification.md and iteration02 for exact current findings and commands.

Guide07 readiness: source-grounded1242a60 delta/private/public seams inspected;
design-complete CDC12 unchanged; guideline-applied actual Rust AP-08/09/12/42/44
and testing/retention; executable existing fixtures/owner access/buffer/Scope APIs;
falsifiable before-retirement resource, ordered every-App, literal reset4/overflow129,
real fault/current restart/phase controls; coherent portable ordered bounded
manifest with itemized intake/readback,55–75k packet allowance and preserved
prompts. No new proof machinery or endpoint gate; no source changes by CRC.

## Iteration02 review / iteration03 readiness — 2026-10-12

CRC independently reviews03825e5/18abe11. All required native checks and exact
host retry pass; restricted CoreMIDI failures retained. LF-01 isolation/structure
and LF-02 ordered actual multi-App behavior accepted; LF-05/LF-06 retained with
historical provenance. LF-03 first-reset state/fault comparison and LF-04 reset
versus uninterrupted control remain unproved. The accepted saved-lifetime replay
function was replaced rather than preserved; its restoration and the remaining
immediate snapshot-error observations are assigned in iteration03. No implementation
failure or structural amendment; two remaining rows open,04/05 and parents held.

Guide07 readiness for iteration03:

1. Source-grounded: inspected actual four-path03825e5 delta, nested RetainedState,
   first-fault timing, both-reset control, removed1242a60 function, current reset/
   restart/snapshot/consume paths. All gates reproduced; actual unreset CC74/75
   control run1300 grounds note62 at251, release376 and NoteOn64 at626.
2. Design-complete: CDC12 unchanged. Only first-reset preserved state, uninterrupted
   control with one literal release exception, original replay preservation and
   complete snapshot failure boundary; two test paths only, no product/API repair.
3. Guideline-applied: full guide07/template, testing01/03, verification03/04/05,
   iteration02/03 workflow and Rust2.0 entrypoint/full anti-patterns read. Actual
   AP-12/18 intentional owned snapshots/short borrows, AP-09/42 useful test errors,
   AP-44 private access and AP-35 explained fixture boundaries inform the recipes.
4. Executable: existing inline helper before/after reset, same two musical racks,
   exact saved function from1242a60 plus current capture helpers; disabled-cable
   observation avoids unsupported snapshot unwrap. No consumed owner reused or
   nonexistent Virtual overstep. Sketches labelled uncompiled, control run actual.
5. Falsifiable: expected pending1→4/0→3 with all other fields equal; saved first
   fault before mutation; only subject reset; exact cleanup300, only control
   release376 exception, all events after shared626 equal including later releases;
   complete error boundary and both restart and unload/reload remain exercised.
6. Coherent/portable: bounded ordered manifest with full active prompt/plan/ledger,
   scoped current review/design/source and original function, existing intake home,
   approximately35–50k packet/recovery allowance. Seven fresh gates retained;
   unaffected independent evidence stays dated/pinned, four accepted rows preserved.
   No new proof tooling, source/main edits by CRC, endpoint or permission gate.

## Final review and bubble-up — 2026-10-13

Iteration03 meets the remaining original observations. All six rows done; assigned
capability delivered, no silent drops or new structural/design amendment. Retain
native/musical evidence at original dates/revisions; fresh affected gates and host
retry pass. Artifacts none. Removal/handles/commands/global BPM remain Slice05;
transport/typed forms remain Arc07/13, with inherited/arc gates open. Full row walk
and bubble-up are in crc-verification.md; CC evidence stays separately attributed.

## Version history

- v1.4 — 2026-10-13: iteration03 accepted at ec85ebe; six LF rows done, Slice04
  closed after three corrections. Routine advancement to authorized Slice05.

- v1.3 — 2026-10-12: iteration02 independently reviewed at03825e5; LF-01/LF-02
  accepted, LF-05/LF-06 retained; LF-03/LF-04 open. Iteration03 issued for the
  collected remaining original observations/preservation in two test paths.


- v1.2 — 2026-10-12: iteration01 independently reproduces native/musical gates;
  LF-05/LF-06 done, four rows retain specifically unproved interactions.
  Iteration02 issued with four test paths and accepted receipts retained.

- v1.1 — 2026-10-12: independent CRC native/musical reproduction, six collected
  interaction-proof findings and accurate diagnostic repair assigned in iteration01;
  six LF rows stay open. No design/ancestor acceptance or source change by CRC.

- v1.0 — 2026-10-12: CRC acknowledges directive12 and opens the approved lifecycle
  increment with six rows, concrete implementation/test recipes and native gates.
  Source/main unchanged; instance commands remain unopened Slice05.
