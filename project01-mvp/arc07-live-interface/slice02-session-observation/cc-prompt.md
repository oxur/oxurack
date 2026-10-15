# CC implementation — edit and observe a playing rack session

2026-10-15. CRC → Duncan → CC; return to CRC for independent review.
Three contributors, Expedited Mode. Initial assignment, correction0/5:
`arc07-live-interface/slice02-session-observation/cc-prompt.md`.
Implement usable session editing and ordered watch, not a discovery/report-only pass.
Authority: [CDC directive14](../../cdc-directive14.md), answering escalation14;
accepted Slice01/Arc12 behavior remains the baseline. This prompt supplies the
source-grounded integration recipe; directive14 D1–D3 supplies binding behavior.
D4/D5 describe **later05/06**, not work to implement now.

S=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch `codex/oxurack-module-behavior-installation`, pinned
`e582aed6e87c234799921cc66a5dd55214f76afd`.
W=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning`, branch planning;
P=W/project01-mvp; A=P/arc07-live-interface; D=A/slice02-session-observation.
K=`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`.
Planning issuance descends from `42b00f5c6adf2317a94fbbf18e0ef56919fc06a5`.
Root main `cd3119ca7ca3884d4a6705904cb1dc42f9d87bad` is a separate checkout,
not the implementation target. Recheck both worktrees/HEAD/status; unexpected
implementation drift returns to CRC, not copied receipts or reset files.

## Outcome, boundaries and reading

Extend rack-live: load/unload/restart/remove, cables, patch/unpatch, labels/current
full-list notes/tags, exact tag search and bounded ordered watch. One facade-owned
journal survives App replacements; real producers feed it in actual order. The
same synchronous owner drives samples, edits, MIDI drain/delivery and watch. No
new event/output thread, callback bus, audit backend, GUI, RT transport, catalog
vocabulary, note schema migration or terminal editor. Execution order after02 is
05 note history →06 real completion →03 controller/config →04 external transport.
Do not create future slice directories or claim those requirements delivered.

| Order | Exact input at the roots/pin above | Scope / reason |
| --- | --- | --- |
| 1 | W/AGENTS.md; this prompt; D/slice-plan.md; D/ledger.md; P/cdc-directive14.md | Required-full: authority, current assignment, all five SO rows and resolved compatibility boundaries |
| 2 | P/project-plan.md Current direction through roadmap table; A/arc-plan.md Current executable plan through EOF | Required-section: current source, dependencies and retained05/06/03/04 ownership |
| 3 | K/collaboration-framework/SKILL.md; K/project-management/guides/README.md; K/engineering-methods/guides/07-implementation-prompt-authoring.md; K/rust-guidelines/SKILL.md | Required-full: workflow, Expedited Mode, complete intake/readback and domain guidance |
| 4 | K/rust-guidelines/guides/02-api-design.md API-02/03; guides/05-type-design.md TD-02/03; guides/03-error-handling.md EH-01; guides/07-concurrency-async.md CA-01/02/07; guides/14-cli-tools/04-output-and-ux.md CLI-27 | Required complete named sections with examples/rationale: owned records/paths, typed identity/errors and one owner/stream discipline |
| 5 | S/crates/oxurack-core/src/patch.rs apply_patch_to_world/build_port_map and label emission tests; event.rs notification definitions/dispatch_core_command/set_module_parameter; topology.rs all production before cfg(test); label.rs all production before cfg(test) | Required complete symbols/sections and referenced types/helpers: actual emission, validation, ordered removal and label resolution |
| 6 | S/crates/oxurack-core/src/lib.rs production exports/plugin; error.rs CoreError; cable.rs Cable/CableOrder; tick.rs SampleTime; port.rs Port; module.rs Module/ModuleKind | Required complete named types/helpers: initialization, exact endpoint resolution and existing errors |
| 7 | S/crates/oxurack/src/lib.rs all production before cfg(test); lifecycle.rs all production plus failed-candidate tests; persistence.rs all production; commands.rs/labels.rs/cables.rs all production | Required complete sections, follow referenced helpers: App ownership/admission, source-aware dispatch and snapshot semantics |
| 8 | S/crates/oxurack/examples/rack-live.rs; rack_live/input.rs; rack_live/control.rs production plus owner-turn, native accounting, resume/save and error/shutdown tests | Required full first two files; required named production/tests in control: real owner integration, same sender, fairness and error precedence |
| 9 | S/crates/oxurack/examples/turing_machine_midi/playback.rs Session and drain helpers; workbench.rs write_view; S/crates/oxurack/src/scope.rs ScopeError/configure | Required complete named sections/helpers: cleanup and typed fallback; do not rewrite these reusable inputs |
| 10 | S/crates/oxurack/examples/patches/turing-machine-lfo.ron and turing-machine-midi.ron; crates/oxurack/Cargo.toml; crates/oxurack-core/Cargo.toml; Cargo.lock ron/oxurack package stanzas; Makefile build/test/lint/docs targets; README live-rack section | Required full fixtures/manifests, bounded lock/Make/README sections: real behavior, installed ron0.12.1 and native gates |
| 11 | A/slice01-live-controls/crc-verification.md Iteration01 independent verification through EOF; P/arc12-live-rack-operations/cdc-verification.md Historical decisions and remaining owners | Required-section: retained independent baseline and why a measured common stream/unload receipt is now needed |
| 12 | Other old prompts/legacy docs; D4 note-codec/Rustyline URLs in directive14; root-main runbook | Reference-only: no need to implement05/06, browse terminal APIs, or copy main into S |

Before dependent edits, record loaded extents/source state, any recovered truncation,
and a short source-cited contract readback in D/ledger.md execution notes. Hashes,
headings and another context's reading are not complete intake. Explain candidate
privacy, staging overflow versus mutation success, non-destructive cursors, and why
MIDI cleanup precedes fresh Scope. No structured-data queries or extra receipt file
are assigned. Load newly encountered binding definitions before dependent edits.
After coherent preflight proceed; missing/conflicting required material goes to CRC.

## Author reconnaissance and engineering guidance

At e582aed: core patch.rs emits module/label/parameter/cable/rack/summary messages;
its parameter loop is HashMap order, rack labels currently Notes before Tags, and
PatchHandle is installed only after the loader returns. Topology disconnect
currently despawns before writing its typed notification. labels/event helper
writes occur only after successful validation/setter. Facade lifecycle commit
replaces/drops App; loaded identity stores version/seed/BPM without origin. Command
label/cable branches bypass some public facade wrappers. These are the actual
integration sites; watching snapshots or concatenating message buffers will fail.

Rust guidance applied by CRC: API-02/03 SHOULD → borrow input selectors, return
owned records and store concrete PathBuf origin; TD-02/03 SHOULD → opaque distinct
sequence/batch/lifetime/cable IDs and private cursor position/owner; EH-01 MUST →
typed read/query errors without panic or mutation failure for observation exhaustion.
Retain CA-02 MUST/CA-07 SHOULD channel ownership, CA-01 SHOULD with the existing
explicit blocked-stdin exception, and CLI-27 MUST stdout MIDI/stderr replies/watch.
No async runtime or new library dependency is implied. Use core's existing thiserror
or manual std::error::Error in the facade, which has no thiserror dependency.

## Recommended public and internal shapes

These are **new API shapes**, not existing source. The behavior/data fields below
are binding; use these public names/signatures, with ordinary rustdoc/standard
traits. Private helper names/organization may adapt. If a public shape cannot
express the contract, return a concrete alternative to CRC before changing it.
Code blocks are design sketches, not compiled drop-in implementation.

```rust
// core observation.rs, re-export from core and umbrella where public:
pub struct ObservationCableId(u64); // get(); Copy/Eq/Ord/Hash/Debug, no reuse
pub struct ObservedCable {
    pub id: ObservationCableId,
    pub source: (String, String), pub target: (String, String),
    pub occurrence: usize, pub transform: Option<CableTransform>,
}
pub enum LogicalTarget { Rack, Module(String), Cable(ObservedCable) }
pub enum OwnedChange {
    ModuleAdded { module: String, kind: ModuleKind },
    ModuleRemoved { module: String, kind: ModuleKind },
    CableConnected { cable: ObservedCable, source: ChangeSource },
    CableDisconnected { cable: ObservedCable, source: ChangeSource },
    ParameterChanged { module: String, param: ParameterName,
        previous: Option<ParameterValue>, new: ParameterValue, source: ChangeSource },
    LabelChanged { target: LogicalTarget, kind: LabelKind,
        previous: Option<LabelValue>, new: LabelValue, source: ChangeSource },
    PatchLoaded { patch_name: String, source: PatchSource,
        module_count: usize, cable_count: usize, label_count: usize },
}

// facade observation.rs; ID wrappers expose get(), not a mutable cursor:
pub struct ObservationSequence(u64);
pub struct ObservationBatchId(u64);
pub struct LifetimeId(u64);
pub enum PatchOrigin { File(PathBuf), InMemory }
pub struct PatchInfo {
    pub lifetime: LifetimeId, pub origin: PatchOrigin,
    pub version: String, pub master_seed: u64, pub bpm: f32,
}
pub enum UnloadReason { Replace, Restart, ExplicitUnload }
pub enum ObservationChange {
    Core(oxurack_core::OwnedChange),
    PatchUnloaded { old: PatchInfo, successor: LifetimeId, reason: UnloadReason },
    Reset { source: ChangeSource },
    RunningChanged { previous: bool, running: bool },
    GlobalBpmChanged { previous: f32, bpm: f32, source: ChangeSource },
}
pub struct Observation {
    pub sequence: ObservationSequence, pub batch: ObservationBatchId,
    pub last_in_batch: bool, pub lifetime: LifetimeId,
    pub sample: u64, pub change: ObservationChange,
}
pub enum ObservationStart { Now, OldestRetained }
pub struct ObservationCursor { /* private Arc identity and next sequence */ }
pub struct ObservationGap { pub from: ObservationSequence, pub to: ObservationSequence }
pub struct ObservationPage { pub gap: Option<ObservationGap>, pub records: Vec<Observation> }
pub struct CableInfo { pub lifetime: LifetimeId, pub cable: ObservedCable }
pub struct ObservationTarget { pub lifetime: LifetimeId, pub target: LogicalTarget }
pub enum TagMatch { Any, All }
```

OwnedChange has no Entity/World reference. Preserve PatchLoaded's typed source and
counts; its envelope carries new lifetime/sample0 and patch_info provides current
origin. Public data enums are non_exhaustive; derive Debug/Clone/PartialEq where
supported, ID types additionally Copy/Eq/Ord/Hash. Cursor Clone copies its independent
position while sharing a separate private facade identity Arc, **not** the existing
lifetime_token that intentionally changes on replacements. No serde promise for
notifications. Existing typed Bevy Messages/structs remain unchanged.

Add public facade methods:

```rust
pub fn observe(&self, start: ObservationStart) -> Result<ObservationCursor, ObservationError>;
pub fn read_observations(&self, cursor: &mut ObservationCursor,
                     limit: NonZeroUsize) -> Result<ObservationPage, ObservationError>;
pub fn patch_info(&self) -> Result<Option<PatchInfo>, ObservationError>;
pub fn has_patch(&self) -> bool; // pure loaded-membership query, even if observations exhausted
pub fn cables(&mut self) -> Result<Vec<CableInfo>, ObservationError>;
pub fn find_tags(&mut self, mode: TagMatch, tags: &[&str])
    -> Result<Vec<ObservationTarget>, TagQueryError>;
```

ObservationError is a typed non_exhaustive error: WrongFacade and
CounterExhausted{counter: ObservationCounter}; counter distinguishes Sequence,
Batch, Lifetime, CableId and StagePosition. TagQueryError distinguishes EmptyQuery,
InvalidTag{tag, source: TagError}, Core(CoreError), Observation(ObservationError).
Use std Error/Display or existing core error infrastructure, no new runtime crate.
Check cursor facade identity first; wrong-facade/error reads leave it unchanged.
Now starts at next sequence;
OldestRetained starts at front or next when empty, with no claim about older records.
A page clones at most limit records then advances that cursor on success. Gap is
half-open [from,to), followed by retained records; page limits can split batches.
Read/query calls never tick, drain MIDI, change labels/history or mint cable IDs.
Cable listing on an empty rack is empty; tag queries require a loaded patch and
nonempty validated tag list. All public signatures need useful error/ownership docs.

## Producer → staging → commit → journal recipe

### A. Core actual producers, optional observation state

New core observation.rs owns OwnedChange, logical descriptors, optional
ObservationStaging and the Entity→ObservationCableId map/counter. Install that
resource only from facade build_base_app (including isolated candidates), not as
an unconditional new requirement on standalone core Worlds. It contains at most
1024 staged records, plus active-cable identity bookkeeping proportional to the
active rack. Keep local emitted-position count even when evicting staged records.
Clearing an operation's staged packet must not reset surviving cable identities.

Use one helper at every actual typed emission point, conceptually:

```text
emit(world, typed_message, owned_change):
    write existing typed_message exactly once
    if optional staging exists:
        allocate/check local position, append owned_change with that position
        if records.len >1024: evict oldest; do not reset emitted count
```

Build expensive owned payload only when observation is enabled where practical.
This helper must not introduce a fallible return into an otherwise successful
mutation. A checked counter failure marks staging/journal unavailable, never wraps.
Capture names/values before mutation destroys them and end temporary World borrows
before writing messages or borrowing another resource. Do not drain typed messages
or run App::update to collect changes. Existing graph readers keep their cursors.

- patch.rs: use module_config and cable_config explicitly while PatchHandle is
  incomplete. At ModuleAdded/labels/parameters/cables/rack/summary, pass real emitted
  values/sources to the helper. Assign cable IDs in patch-vector order, beginning1.
  Sort only parameter **notification** iteration lexicographically; spawning/setter
  initialization order is unchanged. Rack label order becomes DisplayName,
  Description, Tags, Notes (the directive intentionally changes the old order).
  Preserve nonempty-only labels, no notifications for unprovided default params,
  actual label_count and summary last. Update affected old order tests/docs honestly.
- event.rs::set_module_parameter and label.rs::set_label_value: stage only after
  success, using their actual previous/new/source, including accepted equal writes.
  Translate LabelTarget::Cable to the stable descriptor while it exists. Getter
  failure's current previous=None behavior stays unchanged.
- topology.rs::connect_cable: assign stable ID only after valid creation and include
  actual endpoint names/transform/current occurrence. disconnect/commit_disconnect:
  capture descriptor before despawn. Removal keeps prepared incident cable order,
  captures each current occurrence immediately before that cable disappears, then
  emits ModuleRemoved using the prepared name/kind. If two parallel cables are both
  removed, the second may now be occurrence0; their stable IDs still differ.
  Retire map entries only after payload capture. Failed edits consume no IDs/records.

### B. Facade publication and aliases, no doubles

New facade observation.rs owns VecDeque journal capacity1024, next sequence1,
next batch1, current lifetime0, separate session identity and permanent-unavailable
state. A single successful observed facade operation is one batch; no-op/read/save/
Scope/queue bookkeeping is not a batch. Frontend pause composes stop and reset,
therefore can legitimately produce two facade batches at the same sample.

Refactor public/direct and command aliases onto common source-aware methods:
parameter already has set_parameter_with_source; add equivalent private routes
for label/cables/BPM/reset as needed. The common operation transfers staging once
on success, with current completed sample/lifetime. Do not independently publish
again in execute_command/process_commands. Existing CoreCommand::RemoveCable still
means occurrence0; CLI explicit occurrence uses facade unpatch instead of changing
that older variant's meaning. CLI label/set/patch use existing command provenance;
explicit unpatch/remove use DirectApi where no matching command variant exists.
No new CoreCommand variant is required for this increment.

Public start/stop publish RunningChanged only when the actual running flag changes;
no record on redundant call. Each successful reset/Panic publishes one Reset, even
if no dirty voice, with source DirectApi/Command respectively. Each accepted BPM
write publishes previous/new/source, including an equal accepted value. Preserve
existing start/stop time/state, reset cancellation/fault behavior and queue ordering.

Each stage packet has total emitted count and retained (position,change) suffix.
Reserve/advance global sequence space for **all** emitted positions; do not renumber
the surviving suffix. Compute last_in_batch using original final position, not page
size. For normal mutation packets sample/lifetime are the current boundary; load
packet records are new lifetime/sample0. Each batch ID is assigned only at successful
publication; no failed candidate consumes global sequence/batch/lifetime positions.
Public and staging retention capacities are both1024; private reduced-capacity tests
must keep them aligned, so a large staged packet leaves an honest contiguous suffix.

Sketch of publication, not compiled code:

```text
publish(packet):
    first = journal.next_sequence
    checked_reserve(total positions, batch increment and any lifetime increment)
    if counter exhausted: permanently disable observation stream; mutation stays successful
    for (offset, change) in packet.retained:
        push envelope(sequence=first+offset, batch, last=offset+1==total, ...)
        evict front while retained >1024
    next_sequence = first+total
```

Gap reads compare cursor.next to first retained sequence. Counter arithmetic must
also represent the half-open cursor/end position: do not emit u64::MAX then wrap
its successor. Counter exhaustion (including cable/staging/lifetime counters) makes
observe/read/patch_info/cables return the typed error and leaves erroneous reads'
cursors unchanged; rack mutations and ordinary playback still work. has_patch and
loaded origin/configuration stay truthful even when observation IDs can no longer
be represented. No partially numbered success record or fake new lifetime is published.
This is a documented terminal observation boundary, not a mutation rollback policy.

### C. Whole-App lifetime commit, origin and visibility

Extend LoadedIdentity with PatchOrigin. All successful file loads retain supplied
PathBuf (no canonicalization); command load still has typed PatchSource::Other but
origin File(actual path). Restart retains old origin while load provenance stays
InMemory. save never renames origin. In-memory load source uses InMemory origin.
Prepare old PatchInfo/sample before dropping World. Candidate staging and cable IDs
are private to next App; discard on any candidate/admission error.

At successful commit publish one batch: optional PatchUnloaded(old info/sample,
reason, successor lifetime) **first**, then the candidate's ordered packet at new
lifetime/sample0, PatchLoaded last. Empty→loaded has no old unload receipt. Restart
uses reason Restart, replacement Replace; explicit unload publishes only the old
PatchUnloaded with successor empty lifetime and creates no synthetic entity removals.
Repeated empty unload performs existing admission checks but changes neither IDs
nor history. All successful load/restart/unload transitions increment lifetime;
reset/start/stop/remove/metadata do not. Candidate failure preserves origin/current
lifetime/history and all original state/admission guarantees.

Prepending retirement offsets the candidate's sequence positions by1, including
positions evicted from candidate staging; preserve its holes. Very large load must
succeed despite observation eviction. Do not publish until lifecycle commit succeeds,
change MIDI cleanup ownership, or inject old Entity removals into a new World.
User hook panic remains outside existing rollback, not a new containment project.

### D. Current-state cable/tag queries

New facade queries.rs (or observation.rs for the cable accessor) reuses actual
world labels, CableOrder and PatchHandle. cables returns descriptors in current
CableOrder order; occurrence follows existing label selection, ID lookup never
renumbers a surviving parallel cable. find_tags validates inputs with Tag::new,
deduplicates them as a set, scans Rack/active modules/cables without graph, and
returns each matching logical target once. Order: Rack, modules by exact name,
cables by (source module, source port, target module, target port, occurrence).
Validate empty list/Tag::new before loaded membership. Any/All is exact
case-preserving membership; empty query rejects; no prefix,
hierarchy, normalization, graph cache, text search or note-ID implementation.

## CLI/session integration and command matrix

Extend rack-live.rs and existing control.rs; a private rack_live/session.rs may
hold SessionView/parsing/command helpers. Input reader still only frames bounded
lines; do not add terminal/editor dependencies. Add `ron = "0.12"` to oxurack
**dev-dependencies**, matching core/locked0.12.1. The only lock delta should be the
oxurack→ron dependency edge (if needed); no package version refresh. An initial
`cargo check --offline -p oxurack --examples` can update that edge; subsequent gates
are --locked. Core already owns serde support; no new serde/runtime dependency.
CRC compiled the real0.12.1 Option<CableTransform>/string-list/selector tails and
verified trailing garbage rejection; do not replace them with a new parser.

SessionView owns current display/origin, remembered selectors/capacity, watch
cursor and last successful save path; it owns no rack/World/MIDI sender. Use
has_patch/patch_info and successful transitions to keep it current. All new target
selectors are separate tokens: `rack`, `module NAME`, or
`cable SRC_MODULE SRC_PORT DST_MODULE DST_PORT OCCURRENCE`; dots in names survive.
Parse the exact target/field tokens first, then pass the complete remaining tail
to ron::from_str, which rejects extra garbage. Preserve original raw path/scalar
setter tail grammar. All bad/unsupported user input is a recoverable reply.

| Command | Implementation / result |
| --- | --- |
| load PATH | CoreCommand::LoadPatch via real facade; trimmed path, paused/drained required, stay paused |
| unload / restart / remove MODULE | Corresponding facade method; exact no-extra grammar, paused/drained, no hidden pause/drain |
| cables | Current descriptors/IDs/transforms in CableOrder; empty listing when unloaded |
| patch SRC PORT DST PORT [TRANSFORM] | Existing AddCable command/validation; default None; optional RON Option<CableTransform>, e.g. Some(Affine(factor: 0.5, offset: 0.0)) |
| unpatch SRC PORT DST PORT OCCURRENCE | Explicit facade occurrence; never fall back to first |
| label get TARGET FIELD | Current raw label; FIELD display_name/description/tags/notes; clear empty versus absent remains honest |
| label set TARGET FIELD VALUE | RON String for text, Vec<String> for tags/notes, validated Tag set; CoreCommand::SetLabel; notes rack-only |
| label clear TARGET FIELD | Matching empty String/tag set/notes list through same setter; invalid field/target remains error |
| find any ["tag", "another tag"] / find all [...] | Small public find_tags, nonempty exact set, show lifetime/target/cable ID as applicable |
| watch on / watch off | Off at startup; on creates Now cursor; repeated on preserves position; off discards this CLI cursor only |
| scope-select [("MODULE", "PORT"), ...] | Paused-only fresh capture using existing capacity; [] means MIDI-only; invalid keeps current capture and remembered selectors unchanged |

After **successful** replace/restart/unload/remove, immediately drain/deliver all
cleanup through the same existing Session, never Session::finish/reconnect until
terminal shutdown. A send/trace failure is terminal with current combined cleanup
reporting; a committed mutation is not reported as an atomic rejected command.
Update origin/display on success only; scope view must use current origin, not the
startup path. Unloaded status shows no patch/sample0 and play rejects until load;
help/watch/load/quit and replies stay usable. A failed load remains in its actual
old state, not unloaded merely because parsing failed.

After replacement/restart, old Scope is gone: deliver old cleanup while disabled,
then configure remembered selection/capacity fresh. UnknownModule/UnknownPort or
other invalid selector in the new rack → explicit MIDI-only fallback and remember
[]; allocation/other configuration failure → say “load succeeded; capture
unavailable”, leave disabled/paused, do not undo commit. Unload leaves no capture
but remembers selectors. Remove reconfigures only when removal disabled old capture;
otherwise preserve all retained Scope. Existing ScopeError variants support this;
explicit scope-select validation failures preserve both old capture and memory.
Do not make old cleanup appear as a new-lifetime Generated record in new Scope.

Watch output is stderr, at most32 records once per owner turn, after command replies
and the one due sample opportunity. Escape strings/paths/values to a single line;
include sequence/batch/end/lifetime/sample/kind/logical target/value/source.
Use an explicit absent source marker for record kinds without a typed source,
not invented provenance; PatchLoaded retains PatchSource. Gaps
show exact [from,to) and instruct current status/cables/label recovery; do not call
a suffix complete history. Read-only queries produce no observer feedback. Watch
printing can delay wall time but never resets an eligible deadline; production
owner_turn tests must include an advancing diagnostic writer with watch backlog.
Watch read exhaustion gives one explicit unavailable reply and disables that CLI
cursor; it does not turn the successful mutation into failure or stop the music.
Writer failure remains terminal. Plain terminal echo may interleave with watch
until06; document it, no fake completion/ANSI redraw workaround.

On quit/EOF perform existing terminal stop/reset/delivery/fallback cleanup, then,
if watch enabled and diagnostics writable, flush at most the retained1024 suffix
plus any exact gap, including shutdown observations. No wait for future records.
Failure to write it participates in combined error reporting; it cannot skip MIDI
cleanup. Preserve cancellation and blocked-input reader shutdown semantics.

Runnable README/new `docs/user-testing-rack-session.md` must show a real interactive
session, watch/label/tag/cable operations, pause/load/status/Scope fallback/unload/
reload and stdout capture, with exact fixture commands. Source currently has no
docs directory/runbook; create this named session runbook, do not edit an absent
main-only workbench file or copy main's README wholesale. No endpoint witness gate.

## Discriminating tests: five rows, actual production paths

Use native inline/core tests and new facade integration tests; no permanent probe
crate, validator or receipt framework. Add `tests/fixtures/session-observation.ron`:

```ron
(
 version: "1.0", master_seed: 42, bpm: 120.0,
 modules: [
  (kind: "clock", instance_name: "z_clock", parameters: {"ppqn": Int(4), "bpm": Float(90.0)}, display_name: "Clock", description: "tempo", tags: ["workbench"]),
  (kind: "lfo", instance_name: "a_lfo", parameters: {"waveform": String("sine"), "frequency_hz": Float(1.0)}, display_name: "LFO", description: "modulation", tags: ["workbench"]),
 ],
 cables: [(source: ("z_clock", "pulse"), target: ("a_lfo", "sync_in"), transform: None, display_name: "Sync", description: "reset", tags: ["workbench"])],
 rack: Some((display_name: "Rig", description: "session", notes: ["one"], tags: ["workbench"])),
)
```

CRC compiled/loaded this **real** fixture with current Clock/LFO adapters: typed
counts ModuleAdded2/ParameterChanged4/LabelChanged13/CableConnected1/PatchLoaded1,
total21. That validates fixture/API feasibility, not the not-yet-written observer.
Literal expected new public stream for an initial load (cursor Now beforehand):

```text
1  ModuleAdded z_clock
2  Label z_clock DisplayName  3 Description  4 Tags
5  Parameter z_clock bpm     6 ppqn
7  ModuleAdded a_lfo
8  Label a_lfo DisplayName   9 Description 10 Tags
11 Parameter a_lfo frequency_hz 12 waveform
13 CableConnected ID1 z_clock.pulse -> a_lfo.sync_in occurrence0
14 Label cable DisplayName  15 Description 16 Tags
17 Label rack DisplayName   18 Description 19 Tags 20 Notes
21 PatchLoaded modules2/cables1/labels13
```

**SO-01 — actual common order and typed compatibility.** Collect the public cursor
through real load, not hand-inserted journal records. Assert all21 envelope sequence/
batch/lifetime/sample fields (batch1/lifetime1/sample0, only21 last), exact owned
values/sources/previous=None and the order above. z_clock precedes a_lfo by patch
order, while parameters are sorted; rack Tags precedes Notes. Page through a split
batch and check end marker only on real last record. Then actual direct label →
command parameter → direct cable edit, and command label/cable/BPM/reset aliases:
true order/provenance and no duplicates, including accepted equal writes. Inspect
independent Bevy typed readers/graph still see their events; standalone core without
staging behaves as before. No hand-built emitted stream or per-type counts alone.

**SO-02 — lifetime, failure and stable identity.** On musical fixture, keep one
cursor across dirty sample251+, pause/reset/drain, same-name replacement, restart,
unload twice, failed load and recovery. Assert exact retirement first/summary last,
old sample/origin/version/seed/BPM, new lifetime/sample0, restart origin retention,
File versus command Other provenance, save not changing origin. Rejected running/
pending-MIDI edits and a candidate that fails after privately spawning one module
leave history/lifetime/origin and immediate old state unchanged; reuse the existing
failed-candidate fixture/spawner technique, not just an early invalid filename.
Remove LFO with two incident cables and check disconnects then removal. For parallel
LFO→TM.write_modulation cables, remember initial cableID2 and newly appended ID10
from the existing9-cable fixture; removing occurrence0 leaves ID10 at occurrence0,
not ID2 relabeled. Removing both emits distinct IDs in prepared order. No stale
Entity or name lookup after despawn. Run graph and no-graph public paths.

**SO-03 — independent cursors, retention, checked exhaustion.** Two cursors and a
clone read independently; WrongFacade returns typed error without moving cursor;
Now versus Oldest and empty reads are explicit. After the21-record fixture load,
create slow/fast Now cursors at22 and perform1025 accepted set_bpm writes, with fast
reading each: fast has no gaps; slow reads gap[22,23), retained23..1046; a new Oldest
starts23 without claiming history, new Now at1047 is empty. Verify page limit and
last-in-batch independently. Load1100 real Clock modules with no supplied params/
labels/cables:1101 changes including summary, first retained78, cursor-before-load
gap[1,78), final1101 summary, successful rack1100 modules. This rejects renumbering
staging survivors or rejecting an over-capacity valid patch. Private counter-near-
limit tests exercise sequence/batch/lifetime/cable/stage checked boundaries without
huge loops: mutation remains successful, stream typed-unavailable, no wrapped IDs,
read cursor unchanged. Reading alone changes no samples/MIDI/Scope/snapshot.

**SO-04 — real session CLI, same sender/Scope/fairness.** Fake sender + real owner
proves cleanup exactly once across replacement/unload/restart/remove without
Session::finish/reopen; old cleanup has old boundary and no records in new Scope.
Invalid remembered port after load gives explicit MIDI-only fallback; unloaded
status/play rejection, failed reload preserves actual empty/loaded state, explicit
scope-select failure leaves capture/remembered selection intact. Keep source error
and postcommit capture-unavailable distinction. Advance fake clock while commands/
watch print: at most8 commands, one due sample,32 records; sustained backlog still
samples, due pause/quit wins, shutdown flush≤1024/no hanging reader. Broken watch
writer still enters native cleanup and combined error path.
Run a temporary actual subprocess smoke in default and no-graph builds with stdin
held open: play until real notes →watch on →label/set/cable edits →pause →load other
fixture →scope/status →play until notes →pause →unload →status/rejected play →failed
load →reload →quit, plus EOF/Ctrl-C. Capture stdout/stderr separately; require
nonzero samples/trace, single escaped watch lines, correct origin/fallback/empty
state and clean success where expected. Do not feed play/quit immediately and
call zero samples playback. No hardware endpoint is required or opened.

**SO-05 — metadata/query/music/persistence.** Read/set/clear current Rack/Module/
parallel Cable labels and full-list rack notes, including text with escaped newline.
Any/All exact tags across all targets, duplicates once, case-sensitive difference,
empty/invalid rejection and deterministic result order; graph disabled too.
Malformed RON/extra args/unsupported entity notes preserve state/history/time.
Musical subject/control at200 samples: same label/parameter/cable edits via CLI
versus direct APIs, watch/query/save only on subject; compare next1000 complete
MIDI/selected Scope, with nonempty events. Saved/fresh configuration vs independently
prepared edited patch2000 samples: nonempty identical trace/Scope; unedited negative
control differs. Watching/querying alone leaves continuing MIDI/Scope unchanged.
Retain legacy notes wire format/version, no note journal/IDs/authorship fields.

## Gates, exact scope and return

From S after the permitted ron dependency edge is updated, run and record:

```text
cargo test --locked -p oxurack-core --all-features
cargo test --locked -p oxurack-graph
cargo test --locked -p oxurack --all-targets
cargo test --locked -p oxurack --all-targets --no-default-features
cargo build --locked -p oxurack --examples
cargo build --locked -p oxurack --examples --no-default-features
make lint
cargo clippy --locked -p oxurack-core -p oxurack-graph -p oxurack --all-features --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked -p oxurack-core -p oxurack-graph -p oxurack --all-features --no-deps
make test
git diff --check
```

Retain restricted make test's four known CoreMIDI initialization failures, then
exact approved host retry (RT150 pass/3 existing ignores baseline). Keep block0.1.6
advisory and pre-existing no-default cables.rs unused-mut visible; do not suppress
or repair unrelated warnings. Fresh MIDI2000 and LFO10000 traces must retain hashes
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b` and
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
Retain01's unchanged engine/RT proof at its original revision/date, not as fresh
observer acceptance; changed owner/public integration gets the fresh tests above.
No coverage/soak/jitter/terminal-library campaign or permanent planning verifier.

Authorized exact source paths (new files explicitly permitted):

- `crates/oxurack-core/src/observation.rs`
- `crates/oxurack-core/src/lib.rs`
- `crates/oxurack-core/src/event.rs`
- `crates/oxurack-core/src/patch.rs`
- `crates/oxurack-core/src/topology.rs`
- `crates/oxurack-core/src/label.rs`
- `crates/oxurack-core/src/error.rs`
- `crates/oxurack/src/observation.rs`
- `crates/oxurack/src/queries.rs`
- `crates/oxurack/src/lib.rs`
- `crates/oxurack/src/lifecycle.rs`
- `crates/oxurack/src/persistence.rs`
- `crates/oxurack/src/commands.rs`
- `crates/oxurack/src/labels.rs`
- `crates/oxurack/src/cables.rs`
- `crates/oxurack/examples/rack-live.rs`
- `crates/oxurack/examples/rack_live/control.rs`
- `crates/oxurack/examples/rack_live/input.rs`
- `crates/oxurack/examples/rack_live/session.rs`
- `crates/oxurack/tests/observations.rs`
- `crates/oxurack/tests/fixtures/session-observation.ron`
- `crates/oxurack/Cargo.toml`
- `Cargo.lock`
- `README.md`
- `docs/user-testing-rack-session.md`

New: both observation.rs files, queries.rs, optional session.rs, observations.rs,
fixture and runbook. Cargo.toml/Cargo.lock changes are only the approved ron edge.

Tests inline in modified files and the named integration/fixture paths. Do not
modify graph implementation, Scope/shared playback/workbench/oracle, old fixtures,
standalone modules, RT or note schema. Resolve a real missing capability with CRC
before broadening scope; do not simulate production emission or weaken an oracle.

Planning edits only D/slice-plan.md execution notes, D/ledger.md, D/closing-report.md;
optional necessary support only D/artifacts. No artifacts expected beyond canonical
records and temporary command logs; create support only for a named need. Preserve
issued prompt/directive/escalation, all failed attempts and existing CRC evidence.
Return per-row proposed-done evidence with actual failures/retries, intake/readback,
self-review, artifact inventory, bubble-up and exact source/planning hashes. All
SO rows remain open pending CRC; no Slice02/Arc07/project acceptance claimed by CC.
Separate source/planning commits with explicit actual changed paths and
`git commit --only -- <named paths>`; no wildcard/blanket staging, merge or push.
Duncan relays the return to CRC. Approved05 model and06 integration gates remain
retained successors, not work to quietly add to02.
