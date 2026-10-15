# Live interface

Status: active — v2.3, 2026-10-14; Slice01 CRC-accepted at e582aed;
CDC directive14 resolves 02 and assigns CRC its prompt authoring. Both arc rows
remain open; note history and terminal completion have named successor slices. The current
executable plan below supersedes this historical outline.

## Capability and dependencies

A CLI/REPL and controller mapping let a musician load, inspect and change a running rack.

Dependencies: arc04 navigation, arc05 runtime operations and usable arc06 modules.

## Document-backed slice breakdown

No historical slice document to move here. Candidate work remains at arc level until selected.

Slice numbering records the historical order within this arc. Foundation precedes
dependent work; an existing document is not a newly issued assignment.

## Remaining work and boundaries

Outline only: patch load/save CLI and clock selection; REPL parser/commands/completion/watch; nanoKONTROL2 or generic MIDI-CC configuration; tag queries and user configuration where selected. No dedicated legacy slice document exists; plan slices when this arc is near.

The [project roadmap](../project-plan.md) and [backlog](../artifacts/status-and-backlog.md)
govern current interpretation. Historical docs retain their original text and links.
This migration neither reruns implementation validation nor closes this arc.
The sibling [ledger](ledger.md) records the composition checks to establish on resumption.

## Version History

- v1.0 — 2026-09-17: Reconstructed capability and document ownership during the
  Operator-authorized planning migration; future slice detail deliberately deferred.

## Operator priority amendment — v1.1, 2026-09-17

Following audible TM integration, prioritize the smallest workbench that can
control and inspect Clock/LFO/Scope with simulation-time capture and MIDI traces.
Scope capture belongs to arc06; presentation belongs here. Full four-pane TUI,
GUI and general REPL/controller features remain later unless specifically selected.
See the [dependency map](../arc08-validation-and-release/slice01-restart-baseline/artifacts/turing-machine-dependency-map.md).

### Version History — priority amendment

- v1.1 — 2026-09-17: Operator's TM-first/workbench-second decision sets interface
  sequencing; arc08 planning exposed the need to decouple display from capture.


## Transport re-entry — v1.2, 2026-10-12

[CDC directive12](../cdc-directive12.md) retains Arc05's distinct power versus
MIDI transport, START/STOP/CONTINUE wire behavior and relevant pause/resume aliases
here. Re-enter before a transport UI or external master/slave controller is
advertised; compose the actual RT bridge and endpoint owner. Arc12 facade
start/stop preserve execution state; do not change them to match the historical
memo. Arc12 restart is seeded reconstruction, not MIDI START. Arc12/Slice05 owns
bounded queued load/save and synchronous command results, so that feature is not
deferred here. Original Arc05 rows remain deferred until their full successor
walk is supported. This routing is design, not acceptance of transport behavior.

## Current executable plan — v2.3, 2026-10-14

**Active.** Was: directive13's four-increment outline combined observation,
persistent note history and terminal completion in 02. Now: accepted 01 and
[CRC escalation14](../crc-escalation14.md) expose three distinct integration
boundaries. [CDC directive14](../cdc-directive14.md) resolves 02's public journal,
lifecycle/Scope/MIDI contracts and command grammar; note history moves to 05 and
real completion to 06 without deleting either requirement. Duncan's prior
replanning authority, playable-first policy and live-controls selection govern.

Capability: play, inspect, persist and control the current modular instrument
from a usable terminal and later MIDI controls, with honest lifecycle/observation
and distinct external transport. Depends on accepted Arcs10–12; blocks project
P-01/P-04 live musical workflow. Related: Arc13 typed/catalog work and Arc08 timing/
release. Keep A-07-01/A-07-02 open until the full capability composes.

| Execution order / slice | Product increment / inherited responsibilities | Dependency/state |
| --- | --- | --- |
| 1 — slice01-live-controls | Play/pause/reset, scalar live edits, Scope/save | CRC accepted e582aed; five LC rows done |
| 2 — slice02-session-observation | Interactive lifecycle/topology/current metadata editing, exact tag queries and bounded ordered watch across replacement; actual unload receipt and runtime common-order proof | Directive14 resolves design; CRC authors detailed initial prompt next |
| 3 — slice05-note-history | Persistent per-note IDs, CRUD, explicit authorship and retained revisions; legacy/new notes compatibility | After02, before rich-note claims/arc close; model in directive14, actual codec probes before prompt |
| 4 — slice06-terminal-completion | Actual Tab completion, line/history editing and watch-safe terminal presentation | After02/05, before completed REPL claims/arc close; real PTY/stream integration probe before prompt |
| 5 — slice03-controller-config | MIDI-CC/controller mapping, user configuration, expanded/hierarchical tags and formatter policy | Existing scope retained; detail before issue |
| 6 — slice04-external-transport | Separate power/external START/STOP/CONTINUE and clock-selection over RT bridge | Existing scope retained; design before issue; hardware timing remains Arc08 |

IDs 03/04 are retained; execution order is explicit. Only 02 is ready for CRC's
implementation-prompt authoring. Do not create future placeholder directories.
These increments deliver runnable capabilities, not separate planning/proof-only
slices. No generic event service or asynchronous output owner is a prerequisite.
Catalog-specific enums/convenience forms, port labels, CableKind/knob edges and
exact checkpoints remain Arc13 before those public vocabulary claims. Full GUI/
TUI remains unselected. Both arc rows remain open until all retained scope composes.

### Accepted first increment

The accepted [Slice01](slice01-live-controls/slice-plan.md) established the
following behavior. One synchronous owner drives samples/drains/sends; only stdin blocking
is isolated into an input thread. Parameter edits/save/status occur between
samples with no acknowledgment tick. Synchronous I/O may delay wall time; no
catch-up burst and no unmeasured timing guarantee. Startup paused; frontend pause
composes stop + reset + delivery, while reset alone preserves running state.
Native output remains explicit opt-in, trace-only does not open MIDI resources.
Old finite runners and facade semantics remain intact.

### Historical residuals accepted for tracking

Arc12 [CDC verification](../arc12-live-rack-operations/cdc-verification.md) records
all 18 original rows. Arc03 PatchUnloaded's old unload-exists trigger is renewed as
Slice02's lifetime-aware observer requirement; runtime-observed cross-type ordering
also remains there before any ordered watch claim. Source summary-last plus typed
counts is sufficient for current synchronous consumers, not that stronger proof.
Directive14 supersedes the prior combined 02 destination: exact tag queries and
legacy full-list notes are 02; persistent per-note identity/authorship/history is 05;
real completion is 06. Expanded/hierarchical tags and formatter persistence remain 03;
Arc05 external transport remains04. All re-enter before their capability claims
and before Arc07 closure. Nothing is called delivered by naming its successor.

### Workflow and handoff

CDC resolves architecture; CRC now acknowledges directive14 and authors 02's
plan/ledger/detailed cc-prompt.md under guide07, then independently reviews its
implementation. Current CC assignment: none until that prompt is issued.
Source baseline e582aed; planning return follows b1f2081. Accepted01 evidence
remains preserved, and Arc07/project acceptance is unchanged. Subsequent prompts
follow the explicit dependencies and integration gates above. Source implementation
stays in its existing worktree; this CDC response changes planning only.

Version history: v2.0 — 2026-10-14, Arc12's working lifecycle/commands remove the
previous prerequisite; Operator prioritizes live controls over catalog expansion.
Open a concrete first frontend and explicit successor responsibilities, preserving
notification, controller and external transport work with their design gates.

### Design handoff history

CRC acknowledgment — 2026-10-14: Duncan relayed
[directive13](../cdc-directive13.md) after passing Slice01 to CC. Read the directive,
Arc12 CDC acceptance/residual dispositions, current project v3.46/Arc07 v2.0 and
Slice01's complete assignment at planning
`241cc203a2dd8df4f17813f6571ccf9b06869391` (predecessor: CRC Arc12 return
`a5a8e65`). Verified source HEAD
`27ca00f64c902b77c2c16d943f1a2b5f650f7f67`; CC already has in-progress frontend
edits in that checkout, which CRC leaves untouched. Protected main remains
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162` and clean.

Status: **acknowledged**. Arc12's five children/27 rows and R-01–R-04 are CDC
accepted; historical residuals retain the exact owners/re-entry in the CDC record.
Current assignment remains `slice01-live-controls/cc-prompt.md`, issued by CDC
and already relayed by Duncan; no duplicate or corrective assignment is issued.
LC-01–LC-05 and A-07-01/A-07-02 remain open. CC returns source/planning commits;
CRC then independently reviews the five LC rows and records the verdict in
Slice01's `crc-verification.md`. After Slice01 closes, CRC inspects Slice02 and
returns concrete observer/lifecycle/metadata design questions through Duncan to
CDC before authoring its implementation prompt. This acknowledgment changes no
implementation contract, acceptance status or successor design authority.

CRC review handoff — 2026-10-14: initial Slice01 return a603a24/9fdb513 is not
accepted; [CRC record](slice01-live-controls/crc-verification.md) reproduces working
ordinary controls/native gates and actual fairness/trace/error defects plus
observation/proof gaps. [Iteration01](slice01-live-controls/cc-prompt-iteration01.md)
repairs only the approved frontend, correction1/5 issued (0 executed). Five LC and
two A rows stay open. Duncan relays to CC; CRC reviews its return. The four-slice
structure and CDC13 residual/design boundaries remain unchanged; no02 prompt.

Version history: v2.1 — Slice01 initial review surfaces in-scope frontend repair
and proof needs; record corrective assignment without changing arc scope/design.

CRC closure/design return — 2026-10-14: accepted Slice01 iteration01 e582aed /
CC518897; five LC rows done, one corrective pass executed. [CRC record](slice01-live-controls/crc-verification.md)
contains fresh affected native/host/actual CLI/control proof and resolves F-01–F-05.
No structural amendment is needed for01. Directive13's post-close source inspection
is complete; [crc-escalation14.md](../crc-escalation14.md) is **awaiting CDC** through
Duncan, predecessor directive13 and accepted01 return. It requests observation
owner/identity/delivery/order, lifetime transition matrix, session/Scope/MIDI rules,
per-note/persistence/tag-query compatibility and completion/sizing decisions.
CRC has not approved a new architecture or issued02. CDC returns directive14;
CRC can then detail the approved successor. A-07-01/A-07-02 remain open.

Version history: v2.2 — independently close01 and record the required02 design
return; existing four-increment outline and inherited responsibilities unchanged.


CDC design return — 2026-10-14: [directive14](../cdc-directive14.md) answers
[escalation14](../crc-escalation14.md), predecessor directive13, inspected source
e582aed/planning b1f2081. Status: **issued for Duncan relay; CRC acknowledgment
pending**. D1–D3 authorize02's observer/session design and exact current metadata/
tag query scope. D4 assigns the persistent note model to 05; D5 assigns real
completion to 06 with a concrete integration probe before its prompt. Prior
Operator replanning/live-controls authority applies; no acceptance gate waived.
CRC owns02 authoring and returns its exact project-relative prompt path to Duncan.

Version history: v2.3 — escalation14 after accepted 01 reveals that the old02
combined three compatibility problems. Preserve all scope across02/05/06, retain
03/04 IDs, resolve02 architecture and re-entry, and advance actual session editing.
