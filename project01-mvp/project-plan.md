# Project 01 — oxurack MVP

| Field | Value |
| --- | --- |
| project | project01-mvp |
| status | active — Arc10 and Arc11 accepted; P-07 instrument/workbench milestone done; Arc12/Slice01 CRC accepted at fabfa1cb; Slice02 metadata/events issued; Arc12 composition remains open |
| depends-on | No other project required for the baseline MIDI rack; local Rust/Bevy/MIDI toolchain and test hardware |
| blocks | A reproducible modular MIDI recording workflow and an accepted MVP |
| related | underack; Lykn embedding (optional, externally gated); original v1 roadmap |
| source baseline | `a7ce6e61536ca96b6aeb2227ee06e430ed8df162` on `main`; original migration baseline retained below |
| planning home | `planning` branch, `.worktrees/planning/project01-mvp` |

## Current direction — v3.34, 2026-10-12

Duncan requested a recalibration during Arc10/Slice02 review: this is unreleased
open-source music software, and excessive defensive infrastructure, proof tooling
and reporting have delayed usable behavior. [Directive08](cdc-directive08.md)
remains the playable-first/proportionate-proof policy, applying framework **2.6.0**.
[Directive10](cdc-directive10.md) accepts Arc11 at `72fe6b5` and opens
[Arc12 live rack operations](arc12-live-rack-operations/arc-plan.md), starting with
current-configuration persistence. Duncan explicitly selected this continuation.
[Directive11](cdc-directive11.md) now resolves the initial CC return at a3633bc:
the TM parameter getter must expose configured base, and parallel cables need
logical patch-vector order through propagation/save/reload. CRC issued
[iteration01](arc12-live-rack-operations/slice01-live-persistence/cc-prompt-iteration01.md);
Was: five LP rows open after issuance. Now: CRC independently accepted all five
at fabfa1cb, including native host and fresh-process replay;
[Slice02 metadata/events](arc12-live-rack-operations/slice02-events-and-metadata/cc-prompt.md)
is the next issued CC assignment. These were gaps in CDC authoring, resolved
without weakening the base-value or stable-replay oracles.
Arc11's three children/14 rows and four composition rows are independently done;
P-07 is done with the reproduced workbench plus Arc10's preserved musical witness.
P-01–P-06 and release obligations remain open. Main remains `a7ce6e6`.

Was: CRC assembled Arc11 and CDC acceptance was pending. Now: the workbench is
accepted; live parameter edits/save/fresh replay are now independently delivered.
The next executable increment edits live metadata, emits correct notifications
and saves an annotated musical patch. Exact engine-state checkpointing is
still a retained feature, not silently included in configuration persistence.
Inspection surfaced append-only patch loading; Arc12/Slice04 owns actual replacement
repair. First-slice save isolates the active handle and demonstrates fresh reload.
The older sections below are a dated history; their old "current" assignments
and superseded future design/gates must not be inherited into new prompts.

The immediate product is a Turing Machine playing through the Bevy rack, followed
by Clock/LFO/Scope as a useful testing workbench. Plan and assess progress by
runnable musical capabilities. The broad reconstructed catalog is a retained
backlog/release envelope, not a prerequisite to testing this first instrument.

| Order | Deliverable | Boundary |
| --- | --- | --- |
| Arc10 | Playable seeded Clock/TM/MIDI rack, four children/22 rows; T-01–T-05 CDC accepted at65522f1 | Actual software endpoint witness; release timing remains open |
| [Arc11](arc11-observation-workbench/arc-plan.md) | LFO modulation, bounded Scope and terminal/CSV workbench | CDC accepted at72fe6b5; 14 child/four composition rows done |
| [Arc12](arc12-live-rack-operations/arc-plan.md) | Live persistence, metadata/events, cable/graph operations, coherent runtime lifecycle | Slice01 accepted; Slice02 issued; four-slice successor ownership retained |
| Arc13 and Arc07 | Remaining catalog and CLI/REPL/controllers | Preserve inventory, Chord and module-state checkpoint candidate |
| Arc08 / release | Coverage provenance/95%, hardware jitter, benchmarks, soak, broader musical acceptance | Retained obligations, not M1/M2 entry gates |

**Was:** Arc10 had three remaining slices, including a dedicated asynchronous
output service and standalone endpoint acceptance. **Now:** two remaining slices;
unissued Slice05 is folded into Slice04, with fresh-process replay in Slice03.
Arc10 design v1.2 replaces the mandatory batch/control/ack/join machinery. Arc12's
runtime API owns future concurrent output/reconnect if needed; old exact mechanics
are alternatives to evaluate, not inherited compulsory work. No future directories
or acceptance records are created merely to hold that intention.

Real endpoint witnessing remains required to claim demonstrated M1 playback.
Software MIDI is an acceptable first destination. If none is available, CDC may
close software composition with T-04 explicitly deferred to Arc08's musical/device
acceptance and proceed to Arc11; M1 witness remains outstanding. P-03's physical
hardware timing and P-04's final musical session are unchanged.

For all remaining prompts, the author does the larger design thinking and supplies
concrete types, wiring, examples, failure policy and discriminating test oracles.
CC gets tactical freedom inside approved behavior. Directive08 specifies bounded
reading, checks and evidence retention. New validation tools need a named gap;
planning/reporting is not itself the default next deliverable. Future arc planning
must use this policy, not mechanically copy the old packets.

### Design handoff history — current

| Date | Packet | Predecessor / initiation | Status | Affected authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-10-12 | [Arc12/Slice02](arc12-live-rack-operations/slice02-events-and-metadata/cc-prompt.md) | Slice01 CRC acceptance fabfa1cb/4c680b4; approved Arc12 successor | Initial CC implementation issued | M-01–M-05 open | Duncan relays prompt; CC returns to CRC |
| 2026-10-11 | [cdc-directive11.md](cdc-directive11.md) | Duncan relayed CC a3633bc/9fdb2cd failure report | CRC acknowledged; iteration01 issued | LP-01–LP-05 open; configured getter and logical cable order authorized | Duncan relays iteration01, CC repairs, CRC independently reviews |
| 2026-10-11 | [Arc12/Slice01 iteration01](arc12-live-rack-operations/slice01-live-persistence/cc-prompt-iteration01.md) | directive11; source a3633bc; author planning37d41f6 | CRC accepted2026-10-12 at fabfa1cb/4c680b4 | All five LP rows done; original prompt/failed history retained | Approved Slice02 issued; arc/project remain open |
| 2026-10-11 | [cdc-directive10.md](cdc-directive10.md) | Arc11 CRC packet; Duncan selected live persistence | CDC accepted; CRC handoff issued | W-01–W-04 and P-07 done; Arc12 LP-01–LP-05 open | Relay Arc12/Slice01 full prompt, then independent CRC review |
| 2026-10-11 | [cdc-directive09.md](cdc-directive09.md) | directive08; Duncan's Arc10 composition handoff | CRC acknowledged; LFO correction independently accepted | T-01–T-05 and all five L rows done | CRC accepted Scope at `6c56e84`; CRC closed terminal Slice03 at `72fe6b5` and assembled Arc11 packet; Duncan relays closing-report.md, crc-verification.md, ledger.md to CDC |
| 2026-10-11 | [Arc11 CRC composition](arc11-observation-workbench/closing-report.md) | directive09; final child closure `3aa15b4`/source `72fe6b5` | CDC accepted via directive10 | All 14 child/W-01–W-04 rows done; P-07 done, P-01–P-06 open | Arc12/Slice01 issued |
| 2026-10-10 | [cdc-directive08.md](cdc-directive08.md) | directive07; Duncan's project recalibration request during Arc10/Slice02; directive relayed by Duncan | CRC acknowledged after live-state check | Unissued Slice03 onward; Arc10 T-01–T-05 and P-07 interpretation, future prompt/proof policy | CRC closed Slice04 at 65522f1 after musical witness; Arc10 assembly ready for CDC |

CRC advancement — 2026-10-12: independently accepted Slice01 correction at
fabfa1cb after both feature modes/native host gates and actual replay. Issued
approved Slice02 with current guide07/Rust2.0, explicit APIs/cable selection,
event/failure/persistence oracles and runnable recipe. No structural amendment.

CRC acknowledgement — 2026-10-11: directive11 and current project/arc/slice
contracts read at clean source a3633bc and planning37d41f6. CRC independently
reproduced the failing fresh-reference comparison, and inspected the passing
diagnostic that accepts the tie reversal. Issued exactly one corrective packet
with configured getter, shared logical ordering, real schedule/round-trip tests
and original save-preservation/native gates. Initial prompt and CC return retained;
all five LP rows open, source/main unchanged, no further design approval required
for these resolved repairs. Three contributors and Expedited Mode remain active.

CRC acknowledgement — 2026-10-11: directive09 and current Arc11/LFO contracts
read against clean planning `353766a` and source `a965386`. Duncan already
relayed CDC's initial prompt and CC returned its implementation. CRC reproduced
all native gates and musical fixture traces, accepted four of five child rows,
and issued iteration01 for the existing wrong-type input invariant and supplied
Update/FixedUpdate proof. No source repair by CRC, new design gate, merge or
endpoint audition. Scope authoring follows LFO acceptance as directive09 directs.

CRC acknowledgement — 2026-10-10: read directive08, this current plan, Arc10
arc-plan v1.3/design v1.2 and the amended project/arc ledgers. Live planning
was clean at `61671a3eb487cbc433f721d59d645ffd60ebfb19`, including CDC's
`9cbcd02` amendments and CC's corrective evidence. Implementation was clean at
`b2ffb16d342d9a6cc800bab2713813d9646fb505`. This is newer than directive08's
inspected `32e6934`/`9ba55dd`: CC has returned Slice02 iteration01 as
proposed-done. A-01–A-06 remain open pending independent CRC review; the issued
assignment and its gates remain unchanged. No slice/arc/project acceptance is
claimed by this acknowledgement. CRC will repin the accepted predecessor before
opening Slice03 and apply directive08's concrete prompt and proportionate-proof
policy to subsequent work. Three contributors and Expedited Mode remain active.

Version history: **v3.34 — 2026-10-12:** Arc12/Slice01 CRC accepted all five rows;
opened the approved metadata/events successor. Configured replay and cable order
now pass; no roadmap or project acceptance change.

**v3.33 — 2026-10-11:** CRC acknowledged directive11 and issued Arc12/Slice01 iteration01 at source a3633bc after bounded reproduction/source inspection. The prompt applies current guide07/Rust guidance to configured readback, logical cable order and original preservation/replay checks. No LP acceptance, source repair or roadmap change by CRC.

**v3.32 — 2026-10-11:** Arc12/Slice01 return exposed a historical effective-getter conflict and entity-allocation tie reversal. CDC reproduced and resolved both in directive11, expanding source scope for required dependencies without relaxing replay goals or adding a slice. CRC correction authoring next; five LP rows open.

**v3.31 — 2026-10-11:** Arc11 CDC reproduction accepts the workbench and P-07 milestone; Operator explicitly selected live persistence. Opened Arc12 and a complete first implementation packet. New source finding: prior load appends entities; actual replacement repair owned by Slice04, active-handle snapshot isolation in Slice01. Configuration persistence is distinguished from retained runtime checkpoints; no scope removed.

**v3.30 — 2026-10-11:** CRC assembled Arc11 composition at `72fe6b5` with fresh known-signal/modulation, trace/view/CSV, bounds/empty and native evidence. Three children/14 rows done; four W and all seven P rows remain open for independent gates. No roadmap amendment; CDC owns next arc.

**v3.29 — 2026-10-11:** CRC accepted Slice03 iteration01 at `72fe6b5`; all 14 Arc11 child rows done. Recipe defect resolved; prior independent production evidence retained at its original revision/date. Arc11/project rows remain open for the composition gate.

**v3.28 — 2026-10-11:** CRC accepted terminal B-01–B-03 at `22500fa` and reproduced native/runnable commands. B-04 remains open for F-01, a README batch/drain recipe defect. Narrow iteration01 issued; arc assembly held, parent rows unchanged.

**v3.27 — 2026-10-11:** Issued Arc11 terminal Slice03 on accepted6c56e84. Four B rows open; private immutable view and existing CSV/drain preserved. Arc11/project acceptance remains pending.

**v3.26 — 2026-10-11:** CRC accepted Scope Slice02 at `6c56e84`. All five S rows done; capture/CSV/native checks reproduce, no structural changes. Terminal Slice03 next; parent rows open.

**v3.25 — 2026-10-11:** Issued Arc11/Slice02 Scope capture on accepted c4b517c. Concrete bounded observation/MIDI-copy API and CSV runner increment; no new scheduler/drain/dependency. All five S rows and parent composition remain open.

**v3.24 — 2026-10-11:** CRC accepted LFO corrective source c4b517c, all five rows closed after fresh affected checks. No arc design change; Scope capture is next under directive09. Parent/project rows stay open.

**v3.23 — 2026-10-11:** CRC reviewed LFO source a965386, accepted four rows, and issued one focused correction for L-02. Directive09 acknowledged; Slice01/Arc11 remain open and Scope stays held. Parent/project criteria unchanged.

**v3.22 — 2026-10-11:** [CDC review](arc10-turing-machine-midi/cdc-verification.md) accepts all five Arc10 rows after fresh composition/runner checks and trace comparisons. Opened Arc11 and its complete LFO implementation packet. No additional runtime/verification layer; all seven project rows remain open, P-07 awaits the workbench.

**v3.21 — 2026-10-11:** Duncan witnessed MIDI Bus1 → PatchWork/Kontakt omni, Electric Storm Deluxe/Dreamscapes. Slice04 closes with five O rows done; [Arc10 CRC composition](arc10-turing-machine-midi/crc-verification.md) assembles four children/22 rows for CDC. Arc10 and all project rows remain open; CDC owns Arc11 opening after its verdict.

**v3.20 — 2026-10-10:** [Slice04 CRC software review](arc10-turing-machine-midi/slice04-output-runtime/crc-verification.md) accepts O-01–O-04 at `65522f1`; O-05 awaits Duncan's named endpoint/routing and musical/stop observation. Slice04/Arc10 remain open; Arc11 is not advanced.

**v3.19 — 2026-10-10:** issued [Slice04 direct playback/cleanup](arc10-turing-machine-midi/slice04-output-runtime/cc-prompt.md) at accepted `2942c59`; five O rows open, Operator endpoint gate retained.

**v3.18 — 2026-10-10:** Slice03 accepted at `2942c59`, five child rows done; direct endpoint playback/cleanup is next, all parent rows remain open.

**v3.17 — 2026-10-10:** CRC accepted Slice02 at `b2ffb16`
and issued the [Slice03 MIDI-out/trace packet](arc10-turing-machine-midi/slice03-midi-output/cc-prompt.md);
all parent composition rows remain open.

**v3.16 — 2026-10-10:** Operator's review of Arc10 progress
triggered this smaller delivery sequence and proportionate proof policy; accepted
work, current Slice02 and the broader release backlog are retained. No evidence
or ledger status upgraded by planning changes.

## Purpose and definition of done — retained release envelope

Deliver a small, reproducible modular MIDI rack: assemble modules, route typed
signals, save/load seeded patches, drive an external synth or DAW, and operate
the rack live. No audio synthesis. Architecture remains documented in
[0005](artifacts/0005-oxurack-system-architecture.md).

For this best-effort migration, MVP retains the existing v1 capability envelope
from [the catalog/build roadmap](artifacts/0009-module-catalog-and-build-order-v2.md):
Turing Machine, Noise, LFO, Sequencer, Euclidean, Clock, Quantizer, Range,
Sample-and-Hold, Arpeggiator, and MIDI-out; deterministic persistence; master/slave
clocking; CLI/REPL; controller bindings; documentation and operational validation.
The catalog calls these 12 modules but includes MIDI-in, explicitly deferred to
v1.1. Treat MIDI-in as deferred, leaving 11 baseline rack modules.
This is a working scope reconstruction, not a fresh release commitment.

Completion requires a reproducible saved-patch performance, live editing and
controller interaction, demonstrated master/slave synchronization, measured
jitter against the legacy P99 <500 μs target on named hardware, actionable errors,
module documentation/examples, and an Operator-accepted musical recording session.
The original 95% coverage target, 24-hour soak, and 10/50/100-module benchmarks
remain tracked validation obligations; no result is inferred from old plans.
Release/tagging is a separate explicit Operator decision, not implied by the MVP name.

## Arc roadmap

Numeric order is a stable identifier. Dependencies determine execution order;
the RT and core foundations developed partly in parallel.

| Arc | Capability | Depends on | Migration assessment |
| --- | --- | --- | --- |
| [01 — RT and MIDI](arc01-realtime-midi/arc-plan.md) | MIDI I/O, queues, master/slave timing and transport primitives | Shared MIDI types; composes with 02 | Substantial historical implementation; hardware/timing acceptance unverified |
| [02 — Core and persistence](arc02-core-and-persistence/arc-plan.md) | ECS tick cycle, module infrastructure, parameters and patch round trips | 01 for bridge integration | Substantial historical implementation; integration and persistence gaps remain |
| [03 — Events and literate patching](arc03-events-and-literate-patching/arc-plan.md) | Observable changes and patches carrying labels, tags and prose | 02 | Historical notification/label substrate implemented; runtime edits remain |
| [04 — RackGraph](arc04-rackgraph/arc-plan.md) | Fresh rack navigation, diagnostics and parameter introspection | 02, 03 | v0 catalog appears implemented, including later getter follow-up |
| [05 — Public API](arc05-public-api/arc-plan.md) | Operate a rack through a coherent runtime API | 01–04 | Partial; proposed next active implementation arc |
| [06 — Module catalog](arc06-module-catalog/arc-plan.md) | Usable standalone and rack-integrated musical modules | 01, 02, 05 as needed | Standalone Turing Machine exists; adapter and other rack modules remain |
| [07 — Live interface](arc07-live-interface/arc-plan.md) | Patch CLI, REPL, inspection and controller bindings | 04–06 | Planned; no implementation-specific legacy document |
| [08 — Validation and release](arc08-validation-and-release/arc-plan.md) | Reproducibility, performance, usability and release readiness compose | 01–07 | Planned; no current acceptance claim |

No arc is newly marked closed. Historical implementation is useful evidence,
not an independently reproduced acceptance result. [The project ledger](ledger.md)
tracks composition; [the status and backlog register](artifacts/status-and-backlog.md)
accounts for the old inventory, remaining work, and explicit deferrals.

## Superseded migration restart suggestion

Begin by selecting a bounded arc05 runtime-operation slice with the Operator,
likely cable add/remove. The old inventory's first priority, bucket-getter, is
already represented by `7a6fcdb` and `d760049`; do not re-open it by accident.
Re-establish the workspace's actual validation baseline when implementation resumes.
The first usable-rack milestone also needs the Turing Machine adapter and MIDI-out
module in arc06. Detail those slices only when selected; this migration creates
no empty future slice directories or synthetic assignments.

## Boundaries and historical interpretation

GUI/TUI product work, web UI, plugin wrappers, MIDI 2.0, feedback cycles, parallel
execution, distributed racks, and dynamic module loading are outside the inherited
v1 envelope. Chord/module-state persistence and Lykn behavior are retained as
candidate extensions; they are not silently added to the MVP. See the backlog
for owners and re-entry conditions. Historical week estimates are not schedules.

Old plans and design memos are archaeology: their original text, links, document
states and implementation instructions remain intact, except rename history notes.
Their paths and supersession state are retained in [the manifest](artifacts/migration-manifest.tsv).
Read this roadmap, the implementation repository README, and Git history first.
Existing `workbench/` material was outside the requested migration and stays put.

## Superseded migration workflow and authority

The Operator explicitly selected One-Contributor mode on 2026-09-17: Sofie handles
design, implementation, and self-review in this conversation; Duncan is Operator.
This applies to project01-mvp until explicitly changed. Scope changes and existing
approval gates stay with the Operator. Self-checks are attested; no independent
review is claimed. No `cdc-verification.md`, `crc-verification.md`, or simulated
CC handoff is created. Future concrete slices receive current plans and ledgers;
use assignment prompts only for real handoffs if the workflow changes.

Layout authorization comes from the migration request: a dedicated planning
worktree, project01-mvp, document-backed slices, and artifacts at the closest scale.
The requested history-preserving exception uses filtered document ancestry instead
of an empty orphan commit. Original implementation history remains intact.
Retrospective slice ledgers remain open as reconciliation aids; they do not require
reimplementing completed work or inventing old acceptance records.

## Version History

- v1.0 — 2026-09-17: Initial reconstructed roadmap and layout, based on README,
  legacy docs and Git through `5cad90a`. Grouped eight arcs; corrected stale
  getter and module-status assumptions; retained future inventory items explicitly.

## Current plan — v2.0, 2026-09-17

The Operator requested reconciliation/closure of open slices and arcs, explicitly
allowed carried-over closure, and selected **Three-Contributor Workflow in
Expedited Mode**. This supersedes the one-contributor and cable-first suggestions
above. The existing planning layout is confirmed by the Operator's request and
on-disk migration; no parallel planning tree is introduced.

Active work: [arc08/slice01 restart baseline](arc08-validation-and-release/slice01-restart-baseline/slice-plan.md).
This uses the validation arc early to gather evidence; final arc08 acceptance
still depends on arcs01–07 and all inherited system gates. Arcs01–06 are carryover
closure candidates, not accepted capabilities. Six historical slices have explicit
candidate reports; 24 historical ledger rows remain open pending review.
Arc07 remains planned. No new product scope is selected by this restart.
The reconstructed MIDI MVP remains the scope baseline unless the Operator changes
it. Science-rig and Chord alternatives stay candidates; CDC/Operator settle the
next feature milestone after baseline evidence, before a feature assignment.

### Roles, evidence and approvals

- CDC: Sofie, current restart-planning conversation; project/arc design,
  first-slice preparation, design escalation and independent arc composition review.
- CRC: Sofie in the Operator-selected GPT-5.6 Sol forked review context; directive02
  acknowledged at planning commit `cc41597`; independent CC review, historical
  carryover review, and routine approved slice advancement.
- CC: separate Operator-selected execution context, not yet assigned; execute the
  named prompt and return evidence through Duncan. Context identity is recorded at
  handoff; this plan does not silently launch contributor sessions.
- Operator: Duncan; relay packets, decide scope/milestone and retain release,
  musical acceptance and any existing hardware/approval gates.

CC evidence is attested until CRC reproduces it. New slice reviews use
`crc-verification.md`; arcs/project require CRC plus independent CDC records.
CDC cannot accept its own carryover assembly: CRC checks the administrative
proposal before CDC considers a separately assembled arc packet. Existing
historical evidence is preserved without upgrading its strength.

Expedited Mode: each contributor commits its own explicit file list; never use
blanket staging. CC commits before review; CRC commits review/planning changes.
Preserve issued prompts and use numbered refinements, at most five executed
refinement passes. Close/advance when evidence permits; stop affected work at
independent review, unresolved design changes or Operator gates. No push, merge,
release, dependency modernization, hardware interaction or scope reduction is
implied by this documentation assignment.

### Transition state and design handoff history

Source `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`; planning predecessor `f0cf3a5172e02c717c7f924c84156f5fe0405733`. No current behavioral
acceptance or live CC assignment existed in the reconstructed tree. All original
project/arc/slice rows and findings remain tracked; no prior iteration budget is
invented. Initial active prompt is `arc08-validation-and-release/slice01-restart-baseline/cc-prompt.md`.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | [cdc-directive01.md](cdc-directive01.md) | none; Operator restart request | superseded before CRC acknowledgement | R-01–R-06 and 24 inherited rows; carryover permitted, evidence gates retained | See directive02; no execution occurred under this packet |

### Version History — restart amendment

- v2.0 — 2026-09-17: Arc08 restart planning receives missing evidence and unresolved
  scope sequencing from arcs01–06/status; enables three contributors and Expedited
  Mode, prepares carryover candidates and first baseline slice. No MVP acceptance,
  timeline, scope reduction or release authority is inferred.

## Operator priority amendment — v2.1, 2026-09-17

The Operator selected **Turing Machine inside the Bevy rack producing MIDI first**,
then **Clock, LFO and Scope as a testing workbench**. This supersedes the unresolved
milestone choice above. Retain all other MVP capabilities as later work. Scope is
an explicitly selected diagnostic addition; a full TUI/GUI, Chord/state extension
or general cable-editing API is not automatically required for the first milestone.

The [dependency and execution map](arc08-validation-and-release/slice01-restart-baseline/artifacts/turing-machine-dependency-map.md) records source-supported hidden
prerequisites: module system installation, tick/edge/phase ownership, patch seed
handoff, ordered MIDI events and note cleanup, RT lifetime/backpressure, then bounded
Scope capture and a minimal observation interface. The next issued slice remains
arc08/slice01 baseline; it now validates these TM-first dependencies. Following
baseline, CDC details the smallest prerequisite implementation slice(s), then the
adapter and MIDI-out path. CRC may not invent the cross-arc API contracts.

Historical carryover candidates remain pending independent review. Capability
ownership remains with the named domains until a recorded successor assignment;
reopening the same arc with a new slice is allowed and does not resurrect an old
prompt. No original requirement is removed by focusing this sequence.

### Version History — priority amendment

- v2.1 — 2026-09-17: Operator response during arc08/slice01 preparation selected
  TM-to-MIDI, then Clock/LFO/Scope; dependency inspection showed execution/seed/event
  contracts must precede a useful adapter. Original broader backlog retained.

### Design handoff history — priority update

Current directive: `cdc-directive02.md`; current CC assignment:
`arc08-validation-and-release/slice01-restart-baseline/cc-prompt-iteration01.md`.
The preceding directive01/initial-prompt entries are preserved and superseded
before execution; neither was acknowledged or executed in this planning context.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | [cdc-directive02.md](cdc-directive02.md) | cdc-directive01.md | acknowledged by CRC against source `a7ce6e6` and planning `cc41597` | R-03 focused; R-01–R-06 still open; Operator selected TM-first/Clock-LFO-Scope-second | Duncan relays iteration01 to CC; CRC reviews the returned commit and evidence |

CRC acknowledgement: the directive, dependency map, active assignment, six-row
ledger, and source/planning revisions agree. The active CC prompt remains
`arc08-validation-and-release/slice01-restart-baseline/cc-prompt-iteration01.md`.
No ledger row, historical carryover, implementation behavior, or hardware result
is accepted by this acknowledgement.

### CRC return — 2026-09-17

CRC accepted the restart observation slice, closed all six historical slices as
carried over with twelve deferred S rows, and assembled six arc packets with
twelve A rows still open pending independent CDC review. The new design and
quality findings are in [crc-escalation03.md](crc-escalation03.md). This return
does not create the next implementation slice or accept an arc capability.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | [crc-escalation03.md](crc-escalation03.md) | cdc-directive02.md and CC `df6ec0a` | sent for Operator relay to CDC | R-01–R-06 done as evidence; 12 S rows deferred; 12 A rows open | CDC independently reviews six arc assemblies and designs TM prerequisite slice(s) |

## Current roadmap — maintained through v3.11, 2026-09-18

CDC reviewed CRC's actual `390d2aa` packet and resolved escalation03 through
[cdc-directive03.md](cdc-directive03.md). All six historical slices and arcs01–06
are now **closed, carried over**. Their 24 rows are deferred, not delivered;
[exact successor routing](artifacts/cdc-successors03.tsv) preserves every original
criterion and concrete re-entry. The earlier active/pending statements are history.
P-01–P-06 remain open. Arc08/slice01 is accepted as observation evidence only.

| Current owner | Scope / concrete next identifiers | State and dependency |
| --- | --- | --- |
| [Arc09 executable rack](arc09-executable-rack/arc-plan.md) | Three original slices plus slice04-sample-stop-accounting | Closed; CDC accepts four rows at 37deeace |
| [Arc10 TM/MIDI](arc10-turing-machine-midi/arc-plan.md) | slice01-musical-clock; slice02-turing-machine-adapter; slice03-midi-output; slice04-output-runtime | Current four-slice sequence: see v3.16 above; unissued Slice05 absorbed |
| arc11-observation-workbench | slice01-lfo; slice02-scope-capture; slice03-workbench-integration | Planned after Arc10 software composition; reuse Clock, bounded scalar/event observation and minimal inspection |
| arc12-live-rack-operations | slice01-live-persistence; slice02-events-and-metadata; slice03-graph-composition; slice04-runtime-api | Retains remaining original arcs02–05 obligations after M1/M2; original requirements remain the input when detailed |
| arc13-remaining-module-catalog | slice01-catalog-reentry, then per-module slices selected from the retained catalog | Planned after M2; preserves the original eleven-module denominator, standalone compatibility, MIDI-in and extension deferrals |
| Arc07 live interface | Minimal workbench presentation, then full approved CLI/REPL/controller needs | Existing planned domain; no mandatory four-pane UI or broader feature expansion |
| Arc08 validation | slice02-coverage-provenance; slice03-coverage-hardening; slice04-hardware-timing; later system/soak/resource validation | Existing active validation domain; no global quality target is lowered |

Arc09 has four accepted slice directories. Arc10 and its first Clock open set
now exist; unissued Arc10 slices and Arc11–13 remain roadmap destinations.
Dependencies, not arc numbers, set execution order. Feature completion of old arcs
is not a prerequisite fiction: known source foundations can be used with exact
current validation, while missing acceptance remains explicitly tracked.

CDC chose the contracts in the [execution design](arc09-executable-rack/slice01-module-behavior-installation/artifacts/execution-contract.md).
Slice01 was bounded to behavior installation plus its strict documentation gate;
seed/time/MIDI are separate reviewable changes. The P99 target, 95% target,
benchmarks, soak and real musical acceptance remain project gates. M1 audible
sample-quantized playback does not claim final low-jitter release readiness.

### Quality dispositions

- Q-01: arc09/slice01 owns 11 observed rustdoc diagnostics (3 unresolved, 8
  redundant) and its new API docs. CDC reproduced the failure; preserve earlier
  two-link reports as observations, not the complete error inventory.
- Q-02: arc08/slice02-coverage-provenance owns the nine profile mismatches, precise
  tool/source/denominator accounting and trusted reproducibility. Start before
  using coverage to select hardening work; no exclusions or warning suppression.
- Q-03: arc08/slice03-coverage-hardening owns behavior-based coverage remediation
  against the unchanged 95% floor after Q-02. CRC's 93.47% remains a failing,
  qualified observation. New code must have meaningful tests immediately.

### Design handoff history — historical v3.0

Current directive: `cdc-directive03.md`. Current CC assignment:
`arc09-executable-rack/slice01-module-behavior-installation/cc-prompt.md`.
Baseline iteration01 is accepted/complete as observation work, not a live assignment.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | [cdc-directive03.md](cdc-directive03.md) | crc-escalation03.md / directive02 | issued; awaiting CRC acknowledgement | 12 A rows now deferred; 24 exact successors; E-01–04 and I-01–06 newly open | Duncan relays to CRC; CRC acknowledges then relays the initial implementation prompt to CC |

### Version History — CDC design return

- v3.0 — 2026-09-17: CRC escalation03 and accepted baseline led to independent
  carried-over arc closure, concrete successor arcs/slices, executable-rack
  contracts and first implementation assignment. CDC rustdoc reproduction enlarged
  Q-01's known failure set; original quality/capability obligations retained.

## Prompt-authoring amendment — v3.1, 2026-09-17

Current directive: [cdc-directive04.md](cdc-directive04.md), initiated by Duncan's
request to apply the newly installed implementation-prompt-authoring guide. CRC
now prepares a detailed preserved replacement for arc09/slice01's initial prompt
before CC relay/execution. Prior current-assignment statements describe historical
handoffs; the initial prompt is held and no replacement is issued yet.
Directive03's architecture, successor ownership, criteria and validation still govern.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | cdc-directive04.md | cdc-directive03.md | acknowledged by Sofie/CRC at planning `de337fa4ff0833af689bce6dd90efc4aaeabf3f1` and clean source `a7ce6e61536ca96b6aeb2227ee06e430ed8df162` | I-01–I-06 remain open; Operator requested improved authoring | CRC authors preserved iteration01, records six readiness results, then Duncan relays it to CC |

CRC acknowledgement: directive03's architecture and successor routing remain
binding; directive04 replaces only its instruction to relay the held initial CC
prompt unchanged. The installed collaboration-framework is version 2.4.0 (newer
than directive04's 2.3.0 reference); the required authoring guide/template and
project-management routes are present. No CC execution packet or source change
exists at this baseline. Current source and planning worktrees are clean before
CRC's prompt-authoring edits.

- v3.1 — 2026-09-17: Added guide-based CRC authoring assignment and held the earlier
  CC handoff. No behavioral contract, ledger status or acceptance gate changed.

## CRC prompt-authoring return — v3.2, 2026-09-17

Sofie/CRC issued the preserved [arc09/slice01 iteration01 replacement](arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration01.md)
for Duncan to relay to CC. The original `cc-prompt.md` was never executed and stays
held; no executed correction pass is consumed (0 of 5). The [slice-plan author
readiness record](arc09-executable-rack/slice01-module-behavior-installation/slice-plan.md#authoring-amendment--2026-09-17)
covers source, design, guide application, execution route, test oracles and
portability. I-01–I-06 remain open for implementation and later independent CRC
review. Directive04 governs this relay; directive03 and the execution contract
still govern behavior. No source, acceptance or quality gate changed.

- v3.2 — 2026-09-17: Issued detailed first-execution CC prompt after the
  guide-based authoring check; retained all six open implementation rows.

## CRC slice01 review handoff — v3.3, 2026-09-17

Current exchange: [crc-escalation05.md](crc-escalation05.md), awaiting CDC through
Duncan. CRC independently reviewed source `d43078ea9c71bba10035699c9914bef3891f1d83`
and planning evidence through `2da485ebad1c685b847bcebf954921dd0aecedfa`.
The [slice verification](arc09-executable-rack/slice01-module-behavior-installation/crc-verification.md)
reproduced I-01/I-03/I-04; I-02/I-05/I-06 remain open. The unchanged
all-targets Clippy gate exposes two baseline test-code warnings beyond the
current source allowlist, while the historical planning verifier rejects the
authorized current assignment. CDC is asked to authorize exact repairs, not
to lower either gate. No arc09/slice02 assignment is issued pending the response.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | [crc-escalation05.md](crc-escalation05.md) | cdc-directive04.md / executed iteration01 | awaiting CDC through Duncan | I-02/I-05/I-06 open; source/checker scope decision | CDC answers with directive05; CRC acknowledges and prepares bounded correction |

- v3.3 — 2026-09-17: Recorded CRC partial verification and escalation for
  exact gate-repair authority; preserved arc09 and project quality contracts.

## CDC scope return — v3.4, 2026-09-17

Current directive: [cdc-directive05.md](cdc-directive05.md), answering
[escalation05](crc-escalation05.md). D1 approves two exact test-only Clippy
repairs; D2 approves the current-assignment verifier correction without dropping
any structural checks; D3 authorizes CRC's detailed iteration02 combining those
with I-02's after-edit reinstall oracle and I-06's provenance/bubble-up corrections.
CDC reproduced both failing gates; no additional implementation acceptance.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | cdc-directive05.md | crc-escalation05.md / directive04 | acknowledged by Sofie/CRC against clean planning `dcab7931f8379380e0054c9b44b7b7d82b9be597` and source `d43078ea9c71bba10035699c9914bef3891f1d83` | I-02/I-05/I-06 open; exact scope amendment, unchanged gates | CRC authors iteration02, CC repairs, CRC independently verifies before slice02 |

- v3.4 — 2026-09-17: Bounded corrective scope approved; no architectural or
  quality-target change, no slice/arc close. Previous pending-CDC status is history.

CRC acknowledgment: D1 authorizes only the named graph/bridge test-code
normalizations; D2 authorizes CC to repair the current-assignment checker while
retaining every existing structural check; D3 authorizes one combined corrective
assignment after guide-based authoring. The source and planning states above
match directive05's observed candidate. No CC iteration02 work is present yet;
CRC's prior I-01/I-03/I-04 verdict remains candidate-specific, and no gate or
target has been waived.

## CRC corrective handoff — v3.5, 2026-09-17

Sofie/CRC has issued [arc09/slice01 iteration02](arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration02.md)
under directive05 for Duncan to relay to CC. The [slice-plan author readiness](arc09-executable-rack/slice01-module-behavior-installation/slice-plan.md#crc-corrective-authoring-and-readiness--v15-2026-09-17)
records six checks and a single canonical current-assignment field. Source
remains clean at `d43078ea9c71bba10035699c9914bef3891f1d83`; I-02/I-05/I-06
remain open, and the three earlier done rows are tied to that prior candidate.
The next CC execution will be the first corrective pass (1 of 5 when run), not
an acceptance or arc09/slice02 advancement.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-17 | [cc-prompt-iteration02.md](arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration02.md) | cdc-directive05.md / iteration01 | issued for relay, not executed | I-02/I-05/I-06 correction within approved scope | Duncan relays to CC; CC returns committed source/planning evidence for CRC review |

## CRC slice01 close and slice02 issue — v3.6, 2026-09-17

Sofie/CRC independently reviewed CC's clean `6a31c0f` corrective source and
planning evidence `0f4561b`/`4fb9f2f`. [Slice01 verification](arc09-executable-rack/slice01-module-behavior-installation/crc-verification.md#iteration02-independent-close)
closes I-01–I-06. The after-edit reinstall oracle reaches `(4,6)` before
reinstall and `(8,9)` after one further fixed run. Full all-targets Clippy,
strict docs, format, planning verification and host-access workspace/make
checks pass. Restricted MIDI/timing and DNS failures remain recorded alongside
successful host retries; three RT tests remain ignored. The inherited 95%
coverage, physical MIDI and hardware timing gates remain with their existing
owners. No arc09 sequence or scope amendment was required.

CRC opened [arc09/slice02](arc09-executable-rack/slice02-simulation-samples/slice-plan.md)
within the existing CDC execution-contract §2. The [initial prompt](arc09-executable-rack/slice02-simulation-samples/cc-prompt.md)
defines the exact 1 ms sample API, five-phase pipeline, manual/live transitions,
raw-core compatibility and sample-indexed traces. Its six S rows are open;
arc09/slice03 and arc close remain future work. Current CC assignment relative
to project01-mvp:
`arc09-executable-rack/slice02-simulation-samples/cc-prompt.md`.

- v3.6 — 2026-09-17: Independently closed the first executable-rack slice and
  issued the next approved sample/phase slice with guide-authored prompt.

- v3.5 — 2026-09-17: Issued the bounded corrective assignment after directive05
  acknowledgment and six-part author readiness; no gate or source behavior changed.

## CRC slice03 close and arc09 composition handoff — v3.7, 2026-09-18

Sofie/CRC independently reviewed clean source `642313a` and CC planning evidence
`698d2d0`/`cc35c4b`. All six slice03 rows are done: the fixed FNV-1a byte vectors,
ModuleId migration, pre-construction context, legacy registration compatibility,
real core/umbrella patch paths and two-child-process replay reproduce. Package,
both host workspace feature matrices, Clippy, format, strict docs and host
`make check` pass. Three pre-existing RT tests remain ignored; physical MIDI,
timing, coverage and musical acceptance are not claimed.

All three arc09 slices are now CRC-accepted. CRC assembled the formal
[arc closing report](arc09-executable-rack/closing-report.md),
[operational verification](arc09-executable-rack/crc-verification.md), and
[composition ledger](arc09-executable-rack/ledger.md). E-01–E-04 remain open
until CDC independently verifies this assembly. No structural project-plan
change was discovered. Arc10 and its first slice remain CDC-owned and unopened
until that gate passes.

- v3.7 — 2026-09-18: Closed slice03 after independent CRC reproduction and
  assembled arc09 composition for CDC review; retained all successor and
  project acceptance gates.

## CDC composition return — v3.8, 2026-09-18

[CDC review](arc09-executable-rack/cdc-verification.md) reproduced installed seeded
composition and standard gates but found a mid-sample stop in a manual batch
returns 3/advances 3 ms while completing only 1 sample. E-01/E-03 are reproduced
on 642313a; E-02/E-04 remain open. [Directive06](cdc-directive06.md) adds a bounded
arc09 slice04 and specifies partial-completion/time semantics. No original product
capability is dropped. Three existing slices/18 rows retain historical acceptance;
final planned accounting is four/24, once CRC authors/completes the remediation.
Arc10 remains unopened; TM/MIDI then Clock/LFO/Scope stays the priority sequence.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-18 | cdc-directive06.md | arc09 CRC composition assembly / directive05 | issued; awaiting CRC acknowledgement | E-02/E-04 open; exact manual partial-result amendment | CRC authors slice04, reviews CC repair, reassembles for CDC before arc10 |

- v3.8 — 2026-09-18: Arc09 CDC composition exposed shared-stop/manual-batch
  interaction; plan amended to remediate it before advancing, preserving gates.

## CRC directive06 acknowledgement and Slice04 issue — v3.9, 2026-09-18

Sofie/CRC acknowledged [directive06](cdc-directive06.md) against clean source
`642313a62db437ee2bf0b3d5b97fcdcffa16782a` and planning
`6a1bc1340ae6eecb8ed606eb6c1aa8323f7a7119`. The CDC review and current source
agree: driver validation is correctly first, but the manual loop checks running
only at entry, so a shared mid-sample stop can over-report progress and advance
unused clocks. No additional design decision is unresolved.

CRC opened [arc09/slice04](arc09-executable-rack/slice04-sample-stop-accounting/slice-plan.md)
with six M rows and issued a guide-authored initial prompt. The exact one-source-file
scope preserves core's complete-sample latch while checking shared running before
each attempted sample, returning actual completed progress and proving clock/state
resume behavior with real patch construction. All six author-readiness checks pass;
the slice starts with zero executed corrective refinements.

Current CC assignment relative to `project01-mvp`:
`arc09-executable-rack/slice04-sample-stop-accounting/cc-prompt.md`.
The prior three slices/18 rows retain their candidate-specific acceptance; the
current arc denominator is four slices/24 rows with M-01–M-06 open. Arc10 remains
held until CC implementation, independent CRC closure/reassembly and independent
CDC acceptance.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-18 | [Slice04 CC prompt](arc09-executable-rack/slice04-sample-stop-accounting/cc-prompt.md) | directive06; source `642313a`; planning `6a1bc13` | issued for relay, not executed | M-01–M-06 open; E-02/E-04 remain open | Duncan relays to CC; CC returns committed source/planning evidence for CRC review |

- v3.9 — 2026-09-18: Acknowledged directive06 and issued the bounded Slice04
  repair assignment after six-part author readiness; no acceptance gate changed.

## CRC Slice04 close and Arc09 reassembly — v3.10, 2026-09-18

Sofie/CRC independently reviewed clean source
`37deeace90e7fe2dff633ada9d58ef9051577c3d` and CC planning evidence
`7efed01357a9332b5b00556daefcf8a0c1464d3a`. The one-file source diff implements
directive06's per-attempt running check and exact completed counter. All nine
focused tests pass in both feature modes: phase stops finish the latched sample,
return/count/time agree, stopped requests preserve state, start resumes seeded
state against an uninterrupted reference, and false-then-true follows final state.

[Slice04 CRC verification](arc09-executable-rack/slice04-sample-stop-accounting/crc-verification.md)
closes M-01–M-06 with no correction. Both full host workspace matrices, Clippy,
format, strict docs, native Make, separate dependency observation, source scope
and current planning controls pass. CC's restricted MIDI/DNS attempts and host
retries remain preserved; three RT tests remain ignored. No retained project gate
is inferred from those results.

All four arc09 slices/24 child rows are now CRC-accepted. CRC refreshed the
[arc closing report](arc09-executable-rack/closing-report.md),
[operational verification](arc09-executable-rack/crc-verification.md), and
[composition ledger](arc09-executable-rack/ledger.md) for source `37deeace`.
E-01–E-04 are open for independent CDC review of this exact candidate. Arc09 is
not closed, and Arc10 remains held; TM/MIDI then Clock/LFO/Scope remains the
priority sequence.

| Date | Packet | Predecessor | Status | Affected rows / authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-18 | [Arc09 repaired composition](arc09-executable-rack/closing-report.md#repaired-four-slice-assembly--crc-2026-09-18) | directive06; source `37deeace`; planning `7efed013` | CRC reassembled; awaiting independent CDC | M-01–M-06 done; E-01–E-04 open on final candidate | Duncan relays packet to CDC; CDC independently reviews before Arc10 |

- v3.10 — 2026-09-18: Independently closed Slice04 and reassembled the repaired
  four-slice Arc09 candidate; CDC composition gate and all project gates retained.

## CDC Arc09 acceptance / Arc10 opening — v3.11, 2026-09-18

Arc09 review07 independently accepts the repaired source `37deeace` and closes all
four E rows (four slices/24 child rows). The additional stop-accounting slice
resolved the previously missing partial-progress interaction; original accepted
records and the failed predecessor proof remain intact. No further dependency
or project scope change was exposed. P-01–P-07 remain open, including deferred
historical obligations not wholly discharged by this infrastructure arc.

CDC opens the already planned Arc10 with its five-slice design and complete
musical Clock open set. Exact fractional timing, per-module tempo authority,
one-low-sample rate-edit restart, immutable validated config and optional rack
packaging are now specified. These detail the planned Clock capability. Later
TM/MIDI batching/control/session responsibilities remain separate slices; source
has not been changed by CDC. This replaces earlier Arc09-held current banners.

Current directive: `cdc-directive07.md`. Current CC assignment:
`arc10-turing-machine-midi/slice01-musical-clock/cc-prompt.md`.

| Date | Packet | Predecessor | Status | Affected authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-18 | cdc-directive07.md | CRC Arc09 repaired composition / directive06 | issued; awaiting CRC acknowledgement | Arc09 E-01–04 done; Arc10 T-01–05 and Clock C-01–06 open | Duncan relays to CRC; CRC acknowledges and routes issued first prompt to CC |

- v3.11 — 2026-09-18: Arc09 repaired composition independently accepted;
  existing Arc10 roadmap activated with CDC-authored Clock first slice. Retained
  quality/physical/project gates and TM-first/workbench-second sequence unchanged.

## CRC directive07 acknowledgement and Clock relay — v3.12, 2026-09-19

Sofie/CRC acknowledged [directive07](cdc-directive07.md) against the clean source
baseline `37deeace90e7fe2dff633ada9d58ef9051577c3d` and CDC planning issue
`8162fccbfca7ddfc657b7462e399a99130536a3a`. CRC checked the complete Arc10 design,
Clock slice open set, issued prompt, source API anchors and planning verifier. No
contradiction or blocking omission was found. The CDC-authored prompt is preserved
unchanged and ready for Duncan to relay to CC.

Arc09 remains closed with four accepted slices/24 child rows. Arc10 and Slice01
are active; C-01–C-06 remain open and the correction count is 0/5. CC retains
proposed-done authority only. CRC will independently reproduce the returned source
and evidence before closing the slice or authoring Slice02 from a complete Turing
Machine source inventory.

Current CC assignment relative to `project01-mvp`:
`arc10-turing-machine-midi/slice01-musical-clock/cc-prompt.md`.

| Date | Packet | Predecessor | Status | Affected authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-19 | [Clock CC prompt](arc10-turing-machine-midi/slice01-musical-clock/cc-prompt.md) | directive07; source `37deeace`; planning issue `8162fcc` | CRC acknowledged; issued prompt relayed unchanged | C-01–C-06 open; 0/5 corrections | Duncan relays to CC; CC returns committed source/planning evidence for CRC review |

- v3.12 — 2026-09-19: Acknowledged directive07 and checked its revisions,
  complete Clock packet and source anchors; relayed the issued prompt unchanged.

## CRC Clock iteration00 return — v3.13, 2026-09-19

Sofie/CRC reviewed CC source `3009b1efb8685e68d9fde9d8ca2bec4c40a0c979`
and planning evidence `ee99804c19d4b6f9b26c5b193cc9fa3d162f50f9`.
The focused standalone, rack-feature and umbrella Clock tests reproduce, and
the initial packet is confirmed to use collaboration-framework 2.5.0 with the
new prompt-authoring guide and Rust guidance. CRC nevertheless reproduced an
invalid-BPM spawn diagnostic attributed to `pulse_width_ms` and found the
assigned controlled-Live, full-trace and parameter-preservation oracles absent.

Slice01 remains open with C-01–C-06 open. CRC issued corrective pass 1/5 within
the approved Clock design and original nine-path source fence; no arc/design
amendment or CDC escalation is needed. The initial prompt and CC evidence remain
historical iteration00 records.

Current CC assignment relative to `project01-mvp`:
`arc10-turing-machine-midi/slice01-musical-clock/cc-prompt-iteration01.md`.

| Date | Packet | Predecessor | Status | Affected authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-19 | [Clock iteration01](arc10-turing-machine-midi/slice01-musical-clock/cc-prompt-iteration01.md) | initial Clock prompt; source `3009b1e`; CRC iteration00 review | issued corrective pass 1/5 | C-01–C-06 open; existing design/fence retained | Duncan relays to CC; CC repairs diagnostics/proof matrix and returns committed evidence |

- v3.13 — 2026-09-19: Returned Clock iteration00 after reproducing a diagnostic
  defect and evidence gaps; issued guide-authored iteration01 without changing
  Arc10 architecture, sequence or scope.

## CRC Clock acceptance and Slice02 issue — v3.14, 2026-09-19

Sofie/CRC independently accepted corrected Clock source `a19cfd1` and CC
evidence `ef9d99f`. C-01–C-06 are done after corrective pass 1/5: the public
diagnostic probe, exact pulse/edit matrices, full Manual/Live traces, lifecycle
controls and strict host gates reproduce. Restricted CoreMIDI/DNS failures and
successful host retries remain recorded. This closes only Arc10 Slice01.

CRC then completed directive07's actual-source inventory for every existing
Turing Machine setting/action and `StepOutputs` field. The new Slice02 packet
fixes all nine parameters, 32 ports, Scale conversion, reset/move/clock priority,
nonaccumulating write modulation, one-sample coherent publication and seeded
fresh-process/standalone oracles before implementation.

Current CC assignment relative to `project01-mvp`:
`arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt.md`.
All A-01–A-06 rows are open; correction count is 0/5. Arc10 T-01–T-05 remain
open because no MIDI sink, runtime or endpoint has yet composed with the Clock
and adapter.

| Date | Packet | Predecessor | Status | Affected authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-09-19 | [TM adapter prompt](arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt.md) | accepted Clock `a19cfd1`; directive07 | issued initial pass | C-01–C-06 done; A-01–A-06 open | Duncan relays to CC; CRC reviews committed source/evidence |

- v3.14 — 2026-09-19: Accepted corrected Clock and issued the fully inventoried,
  guide-authored Turing Machine adapter slice; all composition/device gates retained.

## CRC Slice02 review and corrective handoff — v3.15, 2026-10-10

Duncan renewed Expedited Mode after the month-long pause. CRC verified that
CC's `9ba55dd` / `516afcd` / `2eb5ac1` packet is still the latest Slice02
candidate. Independent focused gates reproduce, but strict API/preservation,
complete publication/action/lifecycle and seed-control proof is incomplete.
All A-01–A-06 remain open. [CRC review](arc10-turing-machine-midi/slice02-turing-machine-adapter/crc-verification.md)
records the exact independent subset and a reproduced identity-only rename
false positive; no production seed failure is inferred.

Current CC assignment relative to `project01-mvp`:
`arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt-iteration01.md`.
It is a bounded test/evidence correction, authorized pass1/5, awaiting execution.
The initial packet is preserved; source remains clean at `9ba55dd`.
Slice03 stays held pending corrected candidate acceptance. No architecture,
acceptance, arc scope or deferred-feature ownership changes.

| Date | Packet | Predecessor | Status | Affected authority | Next action |
| --- | --- | --- | --- | --- | --- |
| 2026-10-10 | [TM adapter iteration01](arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt-iteration01.md) | initial adapter prompt; source `9ba55dd`; CRC review F-01–F-05 | issued correction, awaiting CC | All A rows open; existing design, test-only source fence | Duncan relays to CC; CC completes proof and returns committed evidence to CRC |

- v3.15 — 2026-10-10: Refreshed current status and issued Slice02 test/evidence
  correction after independent partial review. No higher-scale closure claimed.

## CRC Slice02 acceptance — 2026-10-10

CRC independently accepted the iteration01 correction at `b2ffb16`: all six
A rows are done, with [reproduction and bubble-up](arc10-turing-machine-midi/slice02-turing-machine-adapter/crc-verification.md#iteration01-review--accepted-2026-10-10).
No substantive design change arose. Directive08 governs immediate Slice03
authoring at this accepted source. Arc10 and all project composition rows
remain open; this is adapter acceptance, not MIDI playback acceptance.

## Slice03 CC handoff — 2026-10-10

Current assignment: `arc10-turing-machine-midi/slice03-midi-output/cc-prompt.md`,
at accepted clean source `b2ffb16`, planning predecessor `95bf441`. Five M rows
open. The packet implements directive08/design v1.2: installed sink, bounded
facade drain and runnable seeded trace, with code sketches and real behavior
oracles. A narrow ordering label on the existing core sample-start boundary
allows admission before the latch; no phase/driver/clock is added. CRC performs
independent review after CC return. Slice04 remains unissued.
