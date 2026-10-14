# Arc07 Slice01 — live controls

Version 1.1, 2026-10-14. Status: open, initial CC return reviewed; iteration01 issued;
five LC rows open, no implementation acceptance. Three contributors/Expedited Mode.
CDC author; CRC reviewer; Duncan relays. Depends on Arc12 accepted 27ca00f.
Current assignment: [cc-prompt-iteration01.md](cc-prompt-iteration01.md).
Correction1/5 issued, 0 corrective passes executed; initial prompt preserved.

## Goal and boundary

Use the actual Clock/LFO/TM/MIDI workbench interactively: play, change scalar/bool/
string parameters during playback, inspect bounded Scope, save configuration,
pause with silence and quit cleanly. New rack-live example uses a single synchronous
rack/output owner and a bounded stdin-only helper thread. Default is paced trace;
explicit endpoint selection enables native MIDI. Startup is paused at sample 0.

The [initial prompt](cc-prompt.md) and current iteration carry the binding design,
source map, command matrix, failure/shutdown policy, test oracles and allowed
paths. This plan does not duplicate them. The only facade addition is public read-only is_running, needed
for actual current state rather than a frontend shadow flag. No new dependencies.
Unissued successor capabilities—subscriptions, interactive topology/replacement,
controllers/config, external transport—remain in the arc plan. Scale values remain
file-editable; no broad module vocabulary project is a prerequisite.

## Implementation and proof

Source: existing implementation worktree/branch at 27ca00f. Reuse playback Session,
endpoint selectors, drain_generated and Scope view; expose private helper seams
only as listed. New rack-live.rs plus rack_live/input.rs/control.rs contain CLI,
line parsing and owner loop. Old finite examples keep their behavior/hash oracles.

Five [ledger](ledger.md) rows bind command behavior, continuing playback, MIDI and
shutdown ownership, observable persistence/Scope, and executable/native proof.
Actual CLI smoke must keep stdin open while samples advance, not send play/quit
immediately and falsely call zero samples a live instrument. Native endpoint
selection is compiled/fake-sender tested; no new physical witnessing gate.
Artifacts: none expected; concise command/intake/evidence notes in ledger and
closing report. Optional necessary support goes only under this slice's artifacts/.

## Author readiness — CDC 2026-10-14

1. Source-grounded: inspected27ca00f facade/commands/lifecycle, both playback and
   workbench production code, fixture, parameter enum, LFO bounds and actual build
   dependencies. Found private is_running and explicitly authorized its read-only
   exposure. Shared playback tests require an oracle root sibling; prompt covers it.
2. Design-complete: exact command behavior, startup, pause-versus-reset, output
   streams, file policies, input bound 32/8192, 8-command fairness, failure precedence,
   cancellation/EOF and the detached blocked-reader exception are resolved.
3. Guideline-applied: prompt records EH-01, CA-01/02/07 and CLI-27, including the
   reason not to join a reader blocked in stdin at executable shutdown.
4. Executable: native existing helpers supply one nonterminal Session; prompt
   explicitly rejects repeated play(..., 1). Private cycles allow deterministic
   correctness tests; actual subprocess smoke covers waiting/input/exit behavior.
5. Falsifiable: unchanged musical third control rejects no-op edits; exact pause
   cleanup and sample continuity, malformed→valid input, bounded fairness and
   broken-writer cleanup reject plausible incorrect implementations.
6. Coherent/portable: prompt carries exact roots, bounded full/section/conditional
   readings, intake record, named source paths and separate commits. Packet is one
   frontend increment, estimated 45–65k tokens including source/test work with
   recovery headroom; no requirement to read every old arc or implement successors.

This readiness is CDC author judgment, not independent implementation acceptance.

## Assignment history

| Date | Assignment | Predecessor | Reason/status | Next owner |
| --- | --- | --- | --- | --- |
| 2026-10-14 | cc-prompt.md | Arc12 CDC acceptance27ca00f; directive13; Duncan selected live controls | Returned a603a24/9fdb513; CRC changes required, five rows open | Preserved; iteration01 supersedes assignment |
| 2026-10-14 | cc-prompt-iteration01.md | Initial a603a24/9fdb513; CRC F-01–F-05 | Active correction1/5 issued; none executed yet | Duncan relays to CC, then CRC independently reviews |

## Version history

- v1.0 — Arc12 acceptance and Operator selection open the first playable control
  surface; larger historical interface capabilities remain named successor slices.

## CRC initial review and corrective author readiness — 2026-10-14

[Independent record](crc-verification.md): native gates and ordinary CLI work,
but actual fairness/native trace/error probes fail; status/default Scope are
incomplete. Resume/save proof needs discriminating control continuation; no
runtime phase/save defect reproduced. Five LC rows remain open. No structural
arc/architecture change; Slice02 stays held under directive13.

Iteration01 readiness (CRC, not acceptance):

1. Source-grounded: inspected all six changed paths at clean a603a24; verified
   actual bpm/snapshot/reset APIs, CLI gating, delivery, turn and input shutdown.
2. Design-complete: preserve original behavior; eligible sampling survives long
   command work, ready pause/quit wins, one late sample then rebase; native
   send-before-trace; aggregate primary/cleanup and completed-reader errors.
3. Guideline-applied: EH-01/AP-33 error retention, CA-01 join versus explicit
   blocked-reader exception, CA-02/07 bounded sole ownership, CLI-27 both backends.
4. Executable: four authorized existing frontend/README paths; production-turn
   seam, unconditional startup Scope, full snapshot status, private failure result;
   no library/dependency changes. Sketches are labeled uncompiled guidance.
5. Falsifiable: three transient failing baseline probes, real default CLI failure,
   fake-time combined turn, literal native trace/cleanup, dual-fault result,
   nonempty 2000-sample fresh/negative control and actual resumed engines.
6. Coherent/portable: explicit roots, bounded full/section reading, existing ledger
   intake home, original prompt retained, affected mode/native/subprocess gates.
   Expected correction packet fits one pass with approximately55–75k tokens
   including reads/implementation/validation and recovery headroom; no proof
   platform, hardware campaign, new arc or subscriber design is assigned.

Version history: v1.1 — initial review rejects incomplete frontend/proof at
source a603a24; iteration01 details in-scope repairs without changing acceptance.

Pre-relay root-state check: main is independently updated to docs-only1954fad,
while assigned source stays clean a603a24. Iteration01's unrelayed draft corrects
that metadata before handoff; same assignment/scope/oracles, no second correction
or source operation. The original prompt remains unchanged.
