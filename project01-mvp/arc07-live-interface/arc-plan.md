# Live interface

Status: active — v2.1, 2026-10-14; Slice01 iteration01 issued, all acceptance
rows open. The current executable plan below supersedes this historical outline.

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

## Current executable plan — v2.0, 2026-10-14

**Active.** Was: a reconstructed interface outline, no issued child. Now: Duncan
explicitly selected live controls for the existing workbench after Arc12's return;
CDC accepted Arc12 at 27ca00f and opens this arc ahead of catalog expansion.
[Directive13](../cdc-directive13.md) records acceptance and inherited routing.
The earlier outline is history, not simultaneous implementation instructions.

Capability: play, inspect, persist and control the current modular instrument
from a usable terminal and later MIDI controls, with honest lifecycle/observation
and distinct external transport. Depends on accepted Arcs10–12; blocks project
P-01/P-04 live musical workflow. Related: Arc13 typed/catalog work and Arc08 timing/
release. Keep A-07-01/A-07-02 open until the full capability composes.

| Slice | Product increment / inherited responsibilities | Dependency/state |
| --- | --- | --- |
| slice01-live-controls | Play/pause/reset, scalar live edits, Scope and save while the existing instrument plays; executable terminal | Arc12; complete open set issued |
| slice02-session-observation | Interactive lifecycle/topology/metadata session editing, REPL completion and watch/subscriber observation; actual unload receipt and cross-type observer proof, tag search, per-note/richer metadata representation | Slice01; detail before issue, CDC resolves new observer/metadata semantics |
| slice03-controller-config | Bounded MIDI-CC/controller mapping with user configuration, remaining tag/formatter configuration | Slice01/02; detail before issue and specify device witness |
| slice04-external-transport | Separate power/external START/STOP/CONTINUE and clock-selection controls over actual RT bridge | Prior live session; design before issue; retains hardware timing owner Arc08 |

These are retained delivery responsibilities, not four prompts ready to execute.
CRC may review/close 01 and inspect 02; it must obtain CDC's resolved notification/
metadata design before issuing 02. No generic event bus or broad REPL framework
is a prerequisite to 01. Catalog-specific enums/convenience forms, port labels,
CableKind/knob-edge graph vocabulary and exact checkpoints remain Arc13; its
reconciliation re-enters before exposing those catalog/interface forms. Full GUI/
TUI remains unselected. Existing controller/interface requirements are retained.

### Binding first increment

Use the [Slice01 open set](slice01-live-controls/slice-plan.md) and its detailed
prompt. One synchronous owner drives samples/drains/sends; only stdin blocking
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
Per-note/authorship history/tag query UX re-enters 02; expanded/hierarchical tags
and formatter persistence re-enter 03. Arc05 external transport re-enters 04.
These obligations are neither silently dropped nor called delivered by 01.

### Workflow and handoff

CDC owns arc/first-slice design; CRC reviews 01 and escalates consequential02 design
through Duncan. Expedited Mode retains scoped commits, preserved prompts and
independent gates. Current assignment: slice01-live-controls/cc-prompt.md; no
other child assignment. Slice01's 5 rows are open, arc 2 rows open. CRC assembles
arc closure only when its whole capability and retained responsibilities compose.

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
