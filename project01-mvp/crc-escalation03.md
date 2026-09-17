# CRC escalation 03 — accepted baseline, arc packets and TM-first design gate

2026-09-17. Sofie/CRC to CDC through Duncan. Predecessor:
`cdc-directive02.md`; CC packet `df6ec0a6906c2b16c049db8f267145861b623c41`;
implementation source `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
Three-Contributor Workflow, Expedited Mode. This packet requests CDC design
and independent arc review; it does not issue an implementation assignment.

## Accepted and still open

Restart slice01 R-01–R-06 are **done as evidence observations**, with independent
CRC results in its `crc-verification.md`. All six historical slices are now
**closed, carried over**, each with S-01/S-02 **deferred** and an individual
CRC verifier. No historical capability is accepted as done. The twelve arc rows
remain **open**, with six distinct two-row CRC arc assemblies awaiting CDC's
independent verification; actual child counts are 0/0/2/3/0/1. The full 24-row
register remains the traceability denominator. Existing arc domain owners retain
the work until CDC records concrete successor IDs.

Arc packets for CDC independent review (each directory contains
`closing-report.md` and `crc-verification.md`):

| Arc | Closing report |
| --- | --- |
| 01 RT/MIDI | `arc01-realtime-midi/closing-report.md` |
| 02 core/persistence | `arc02-core-and-persistence/closing-report.md` |
| 03 events/literate patching | `arc03-events-and-literate-patching/closing-report.md` |
| 04 RackGraph | `arc04-rackgraph/closing-report.md` |
| 05 public API | `arc05-public-api/closing-report.md` |
| 06 module catalog | `arc06-module-catalog/closing-report.md` |

CRC reproduced the native checks, build, clippy, format, workspace tests and
feature probes. In restricted execution, three workspace variants failed the
same four MIDI initialization tests; with normal MIDI-access escalation all
three passed. Docs still fail on two rustdoc links. Clean direct coverage
reported 886/13,560 missed lines, **93.47% line coverage** versus the retained
95% target, with nine mismatched-profile-function warnings even after cleaning
generated coverage artifacts. Coverage result passes as a command, not as the
quality target. Three RT tests are ignored; physical MIDI/DAW, P99 timing,
10/50/100-module benchmarks, 24-hour soak and Operator musical-use acceptance
were not run. Arc08/P-05 must keep all these gates open and own bounded repair or
validation successors.

## First implementation design boundary

The Operator's M1 is a seeded standalone Turing Machine used **inside the Bevy
rack to produce MIDI at a real endpoint**. Source review confirms no rack adapter,
behavior-installation path, ordered MIDI-out module or Clock/LFO/Scope. Before
issuing the next CC prompt, CDC should choose and specify the smallest coherent
cross-arc contract for:

1. Installing module behavior exactly once per kind while spawning many instances;
   simulation versus musical tick ownership, 24-PPQN conversion, clock edges and
   produce/propagate/consume visibility (TM-01–03; arcs01/02/05).
2. Passing the patch master seed and stable instance identity before TM creation,
   with reset/reload semantics and fresh-process negative controls (TM-04; arc02/06).
3. Mapping the standalone engine's parameters/ports without losing capabilities;
   preserving ordered note-off/note-on, retriggers, silence, CC and multiple MIDI
   events through an offline sink and RT queue (TM-05–06; arcs02/06/01).
4. Owning bridge/device lifetime, start/stop/panic note cleanup, overflow and
   disconnection behavior, with a static validated patch and no partial audible
   startup (TM-07–08; arcs01/02/05).

The [dependency map](arc08-validation-and-release/slice01-restart-baseline/artifacts/turing-machine-dependency-map.md)
gives source anchors and test examples. Its API/event choices remain proposals.
The first successor should implement/test prerequisite execution contracts before
the adapter depends on them; CDC decides whether that is one or several slices
in existing arcs. A deterministic offline MIDI stream is a useful intermediate
gate, but physical endpoint playback remains M1 acceptance.

M2 follows with Clock, LFO and Scope (WB-01–03): one clock authority, bounded
sample/event capture after a specified consume boundary, and minimal inspection.
If M1 needs a clock source, pull only that necessary capability forward.
Scope presentation and the exact adapter packaging/event representation remain
CDC design decisions. Full TUI/GUI, Chord, arbitrary state checkpoints and broad
live cable editing are not automatic M1 prerequisites; their inherited backlog
ownership remains intact.

## Requested CDC return

Independently review all six arc closing reports and CRC records, then record
CDC verification and final carried-over disposition if complete. Select concrete
successor arc/slice IDs and issue the first TM-prerequisite design and CC prompt
with acceptance rows, source/plan pins and negative controls. Assign bounded
quality follow-ups for the rustdoc failures, below-target coverage and profile
warning without turning them into a bulk dependency update. Return the next
directive through Duncan; no project/arc capability is silently accepted here.
