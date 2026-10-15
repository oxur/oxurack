# CDC directive14 — edit and observe a live session

2026-10-14. Sofie/CDC → Duncan → CRC. Three contributors, Expedited Mode.
Response to [escalation14](crc-escalation14.md), following [directive13](cdc-directive13.md).
Design baseline: source `e582aed6e87c234799921cc66a5dd55214f76afd`, planning
`b1f2081`; main `cd3119c` contains Arc09–12 and the README/user runbook updates.
All three worktrees were clean at inspection. Slice01's five LC rows remain
CRC-accepted; this is not another correction or independent CDC re-review of 01.

## Decision and sizing

**Issue Slice02 for executable session editing and ordered observation.** It adds
load/unload/restart/remove, cable edits, current labels/notes/tags, tag search and
watch to the existing rack-live owner. It also delivers the retained actual
PatchUnloaded receipt and runtime-observed common ordering across replacement.
No event service, output worker, durable audit backend or GUI is required.

Was:02 also owned a new persistent per-note history model and terminal completion.
Now: those become explicit Arc07 increments, preserving all obligations:

| Execution order | Owner | Delivery / re-entry |
| --- | --- | --- |
| 1 | slice01-live-controls | CRC-accepted at e582aed; unchanged |
| 2 | slice02-session-observation | This design: edit/watch the active session; current full-list notes and exact tag queries |
| 3 | slice05-note-history | Persistent note identity, per-note CRUD, explicit authorship and revision history; after 02, before rich-note claims or Arc07 closure |
| 4 | slice06-terminal-completion | Actual interactive Tab completion and line/history editing with watch-safe presentation; after 02/05, before completed REPL claims or Arc07 closure |
| 5 | slice03-controller-config | MIDI-CC mapping, user configuration, retained hierarchical/expanded tag and formatter policy; existing owner retained |
| 6 | slice04-external-transport | Distinct power/START/STOP/CONTINUE and clock source over the RT bridge; existing owner retained |

Numeric IDs remain stable; execution order follows the table. Do not create05/06
directories now. This split separates a runtime owner change, a persistent data
migration and terminal editing integration, each with a runnable user outcome.
It avoids bundling three different compatibility problems into one CC context.
Authority: Duncan's existing authorization to replan for solid engineering, his
playable-first recalibration and explicit selection of live controls; no feature
is dropped or converted to optional proof. Project v3.49 / Arc07 v2.3 record it.

CRC owns02's detailed initial prompt and implementation readback/oracles under
guide07. This directive is the binding design, not that CC prompt. The source
recipe below is recommended where it leaves private mechanics open. Consequential
behavior changes return to CDC; CC may choose private helpers within the contract.

## D1 — one facade journal, independent pull cursors

**Binding.** `Oxurack` owns one bounded in-process observation journal that survives
its internal App replacements. Consumers have independent cursors; reading never
consumes another reader's records or Bevy's existing typed messages. No callback,
subscription thread or shared mutable rack is exposed. Observations cover the
provided facade mutations and corresponding CoreCommand paths. `into_app` ends
this facade contract; arbitrary ECS writes, user-generated typed Messages and
standalone core Worlds do not acquire a cross-lifetime guarantee.

Default retention: **1,024 records**. This is an entry bound, not a promise that
arbitrary label strings occupy a fixed byte budget. Each record owns its names,
endpoints and values; no public `Entity`, World borrow or pointer escapes. Each
has a monotonically increasing sequence (first 1), operation/batch identifier,
`last_in_batch`, lifetime ID and the completed-sample boundary at mutation.
Lifetime0 is the initial empty rack; each successful load/restart/unload increments
it. A no-op empty unload does not. Identity is local to this facade session, never
a persistent patch UUID or globally meaningful counter.

Use distinct newtypes for sequence/lifetime/cable ID. Module identity is
(lifetime, exact instance name) for the currently supported operations. Cables
also receive a monotonic, never-reused ID within that lifetime; endpoint names and
current occurrence are included for presentation. Removing occurrence0 must not
change the ID of surviving occurrence1. No serialized cable IDs or richer edge
kinds are introduced. Resolve removal payloads before despawn. If future APIs
permit same-name module replacement within a lifetime, they must extend identity
before publication; that API is not part of 02.

Expose an API equivalent to this **proposed shape**, not existing source:

```rust
let mut cursor = rack.observe(ObservationStart::Now);
// Another consumer can start at OldestRetained; neither drains the other.
let page = rack.read_observations(&mut cursor, NonZeroUsize::new(64).unwrap())?;
// page.gap reports a missing half-open sequence range, then page.records
// contains the retained ordered suffix. The cursor advances only on success.
```

`Now` starts after the current final sequence; `OldestRetained` starts at the
oldest retained sequence (with no claim about older history). Cursors carry a
private facade identity and reject cross-facade use with a typed error. Clone is
an independent reader at that position. Reads have no mutation/MIDI/time effects.
A slow reader gets the exact missing range plus retained records, not a silent
jump or a mutation failure. Page limits may split batches; batch IDs/end markers
make this explicit. Reset/replacement does not reset cursors. No consumer ack or
backpressure stalls the instrument. Checked counter exhaustion permanently marks
the observation stream unavailable with a typed read error; it must not wrap or
misreport an already successful rack mutation as failed. Document this boundary.

Observe ModuleAdded/Removed, CableConnected/Disconnected, ParameterChanged,
LabelChanged, PatchLoaded, **PatchUnloaded**, and successful reset. Include
RunningChanged for actual start/stop transitions and GlobalBpmChanged for accepted
BPM writes; no record for a redundant start/stop. Preserve existing change-source
payloads; existing successful equal-value parameter/label writes still emit just
as their typed messages do. Reads, Scope samples/configuration, queue bookkeeping
and save I/O are not structural observations; replies continue to report saves,
rejections and cancelled commands. This is not a universal execution audit trail.

### Producer/commit spine

Add an owned common change enum and optional bounded staging resource in core,
with a single helper used at each **actual emission point**, alongside the existing
typed message write. Do not reconstruct chronology by later concatenating typed
buffers or diffing snapshots. Preserve those message types and graph cursors.
For a successful facade operation, transfer staged records into its journal in
emission order. All facade/command aliases converge on the same helpers; nested
calls must not double-publish. Stage counters track all emitted positions even
when the 1,024-entry staging window overflows, so transfer preserves sequence
holes and public reads report them. A candidate's staging belongs only to it.
Discard it on failure; transfer it only after successful lifetime commit.

Recommended core home: new `src/observation.rs`, exported via core `lib.rs`;
facade new `src/observation.rs` owns cursor/public envelopes and transfer logic.
A core World without the optional staging resource retains current behavior.
Capture owned targets at the call site: loader modules are not yet all in
PatchHandle; disconnect currently despawns before its typed message. Explicit
loader/config and pre-removal values solve that; post-despawn lookup does not.

Load notification order: patch-vector module order, each ModuleAdded then its
nonempty labels in DisplayName/Description/Tags order, then its parameter messages
in **lexicographically sorted parameter-name order**. Sort notification iteration
only; do not redesign module spawning/setter semantics. Cables follow patch-vector
order, each CableConnected then its nonempty labels in the same field order.
Rack fields follow DisplayName/Description/Tags/Notes; PatchLoaded is last.
Existing HashMap parameter iteration in patch.rs must be made explicit for this
notification contract. Observer payloads preserve actual emitted values/sources;
no synthetic event is added for an absent label or unprovided default parameter.

## D2 — lifetime and failure matrix

Add facade patch information with lifetime and owned origin (`File(PathBuf)` or
`InMemory`), retaining the supplied path without requiring canonicalization.
Command load knows the actual Path even though its existing PatchSource is Other;
keep that typed-message provenance and store the real origin separately. Restart
retains origin while its existing load provenance remains InMemory. Save does not
rename origin; CLI may separately display last successfully saved path. No implicit
wall time, OS username or new random patch identity.

A transition is one publication batch, with old and new lifetime IDs where needed:

| Operation | Literal public observation order / state |
| --- | --- |
| Initial successful load | New-lifetime granular load records, PatchLoaded last; no invented old PatchUnloaded |
| Label → parameter → cable edit | Their actual records in that call order, same lifetime/sample unless real sampling intervened |
| Remove module | Incident CableDisconnected records in existing prepared-removal order, then ModuleRemoved; no record for surviving modules |
| Replace loaded rack, including same names/path | PatchUnloaded(old identity, old sample, reason=Replace, successor lifetime), then candidate granular records at new sample0, PatchLoaded last |
| Restart | Same retirement/load order, reason=Restart, origin preserved, new lifetime/sample0; seeded configuration rebuilt |
| Unload | One PatchUnloaded(old identity, old sample, reason=ExplicitUnload, successor empty lifetime); no per-entity synthetic removals |
| Unload already empty | Admission checks still apply; no observation or lifetime change |
| Reset/Panic success | One Reset observation at current lifetime/sample, including clean/no-MIDI reset; running/time/phase unchanged; queue cancellations remain command outcomes |
| Failed load/candidate/edit | No successful mutation records, no new lifetime, old state and origin preserved; existing Result/command reply reports failure |

PatchUnloaded is the **owned facade receipt** spanning Worlds, not an Entity-bearing
message injected into a new World. It includes old origin, version, master seed,
current metadata BPM and lifetime. No claim is made that old World subscribers
survive its destruction. Accepted directive12 admission, candidate isolation,
cleanup and fault semantics remain unchanged. Publish after commit succeeds;
no candidate can be observed merely because it constructed modules. A failed
candidate's local IDs/records cannot consume global journal sequence positions.

Staging or public retention can discard the beginning of a very large batch;
the reader's explicit gap plus retained PatchLoaded boundary is the honest result.
Do not reject valid patches merely because their observation batch exceeds 1,024.
Old MIDI cleanup keeps its old sample boundary; a new Scope must not label that
cleanup as a new-lifetime sample. Panicking user hooks remain outside the existing
rollback contract; do not add a panic containment subsystem.

## D3 — executable commands, one MIDI owner, truthful Scope

Retain Slice01 grammar and behavior. New targets use separate tokens so names
containing dots work: `rack`, `module NAME`, or
`cable SRC_MODULE SRC_PORT DST_MODULE DST_PORT OCCURRENCE`. Occurrence is a
nonnegative integer in the current ordered endpoint group. `cables` lists these
selectors and observation IDs in current CableOrder order. A displayed occurrence
is positional and can change; it is not the stable observation ID.

| Command | Binding behavior |
| --- | --- |
| `load PATH` | Replace from trimmed path tail; requires paused/drained; stay paused |
| `unload` / `restart` / `remove MODULE` | Existing facade lifecycle operation; require paused/drained; stay paused |
| `cables` | List current exact endpoints, occurrence, ID and transform; read-only |
| `patch SRC PORT DST PORT [TRANSFORM]` | Add using existing topology validation; default None; optional RON `Option<CableTransform>` such as `Some(Affine(factor: 0.5, offset: 0.0))` |
| `unpatch SRC PORT DST PORT OCCURRENCE` | Remove exactly this current occurrence through facade; no implicit first-cable fallback |
| `label get TARGET FIELD` | Current raw value, FIELD display_name/description/tags/notes |
| `label set TARGET FIELD VALUE` | String fields take a RON string; tags/notes take a RON string list; notes only on rack. Full-value replacement using existing validation/events |
| `label clear TARGET FIELD` | Write the matching empty string/list, preserving the current field/target validity rules |
| `find any ["tag", "another tag"]` / `find all [...]` | Exact case-preserving membership; include rack/modules/cables; reject empty list; duplicates have set semantics |
| `watch on` / `watch off` | Default off; on starts at Now; repeated on while enabled preserves cursor; off discards that CLI cursor, not library history |
| `scope-select [("MODULE", "PORT"), ...]` | Explicit paused-only fresh capture with existing capacity; empty list means MIDI-only; invalid request leaves previous capture and remembered selectors intact |

Use `ron = "0.12"` as an oxurack **dev-dependency** for the example's structured
tails, matching the existing core dependency/lock resolution. This avoids inventing
a second quotation/list/transform parser. Preserve Slice01's raw-tail path and
scalar setter grammar; RON applies only to the new structured commands. Missing,
extra, unsupported or malformed input is a recoverable reply. Text can contain
spaces/escaped newlines via RON. The existing 8,192-byte input bound still applies.

Tag search gets a small public facade method with a named Any/All enum and owned
logical target results. Scan actual current labels, not the optional graph/cache.
Order: rack first, modules lexicographically by name, then cables by source module,
source port, destination module, destination port, occurrence. Return each target
once. Match exact current Tag membership; no hierarchy, case folding, query language
or note-body search. Works with graph disabled. Rejection/no-op behavior continues
the existing label contract; it does not synthesize a richer note identity.

Lifecycle commands must **not** auto-pause or auto-drain to hide admission errors.
The ordinary owner loop already drains its own samples; the user enters pause
first. On successful remove/replace/unload/restart, immediately deliver the complete
cleanup batch through the **same existing Session**; do not call Session::finish,
reopen the endpoint, or discard output until terminal shutdown. If delivery fails,
terminate through the existing cleanup/error path. A committed load followed by
output/Scope failure must not be described as an atomic rejected load.

Keep a private SessionView containing current origin/display label, remembered
Scope selectors/capacity, watch cursor and last-save path. It holds no World/MIDI
owner. Update current patch display only on successful load/restart/unload; Scope
must never keep the startup path after replacement. Show `<unloaded>` when empty.
When empty, status reports no patch/sample0 and the CLI rejects play until load;
help/watch/load/quit and error-reporting remain usable.

On quit/EOF, if watch is enabled and diagnostics remain writable, flush at most
the retained 1,024-record suffix with any explicit gap before final exit; no
unbounded wait for new records. A writer failure follows normal terminal cleanup.

After replacement: deliver old cleanup while Scope is disabled, then reconfigure
remembered selectors at the existing capacity. If any selector is invalid in the
new rack, explicitly report fallback to MIDI-only capture and remember the empty
selection. If configuration otherwise fails (e.g. allocation), report **load
succeeded; capture unavailable**, leave capture disabled, remain paused. Do not
undo a committed rack or pretend continuous capture. Unload leaves no capture;
remember selectors for the next load. Restart reconfigures a fresh capture. Remove
only reconfigures if removal disabled the old capture; otherwise preserve it.
`scope-select` lets the user establish new selections afterward.

Watch prints completed operation records to stderr, after command replies, at
most32 records per owner turn. Format each as a single escaped line containing
sequence/batch/end marker/lifetime/sample/kind/logical target/value/source. A gap
prints its exact range and instructs `status`/`cables`/label queries for current
state; never call the retained suffix complete history. Read-only commands do not
produce watch records, so there is no feedback loop. Reuse the corrected owner_turn:
at most8 input commands, one due sample opportunity, bounded watch output; printing
must not repeatedly rebase a running deadline or reintroduce the Slice01 starvation
bug. No wall-clock jitter guarantee. Writer errors use existing terminal cleanup.
Plain input mode may interleave echoed typing with watch lines; this is explicit
until06 supplies redraw coordination. No ANSI cursor control or fake completion
is required in 02, and stdout remains MIDI-only.

## D4 — persistent note design retained in 05

**Approved model; implementation deferred to slice05-note-history.** Notes are
rack-level, with stable patch-local `NoteId(u64)` starting 1, monotonically allocated
and never reused, even after deletion. IDs belong to saved configuration, unlike
session lifetime IDs. Equal texts are distinct notes. No identity inferred from
text or a shifting list index. A checked next-ID counter returns a typed error
before mutation on exhaustion; author is an explicitly supplied optional string,
never inferred from OS/user/time. Use `Option<String>` to distinguish anonymous
from named authors; reject whitespace-only explicit author names.

One canonical runtime NoteStore has either Legacy(Vec<String>) or a journal.
Journal fields: `next_id` and notes in creation/ID order. Each note has ID and a
nonempty ordered revision list; each revision has consecutive revision number
starting 1, `author: Option<String>`, and `text: Option<String>`. Some(text) is create/
edit (empty text allowed); None is a deletion tombstone. Deleted IDs cannot be
edited/reused. All previous revisions remain; no automatic timestamps/truncation
or restore/branch/merge system. Current text/deleted state is derived from the last
revision, not a second mutable source of truth.

Wire compatibility: retain legacy `RackConfig.notes: Vec<String>` and add optional
`note_journal` containing the shape above, with serde default/omit-when-none. Exactly
one form may carry data: a journal plus nonempty legacy notes is rejected before
candidate commit. Runtime materialization uses NoteStore only; snapshot derives
the appropriate wire fields. Legacy load/save without note editing retains its
existing representation/version. First per-note edit deterministically imports
legacy strings in list order as IDs 1..N, revision 1, anonymous; duplicate strings
get distinct IDs. New writes then use journal only (legacy notes empty/omitted).
New journal files use patch schema version 1.1. Promote version 1.0 on first migration;
for other version strings, return a clear unsupported-migration error rather than
silently replacing a caller's version. Loading journal with a version other than
1.1 or malformed IDs/revisions/counter is rejected. IDs must be unique/nonzero,
next_id greater than every stored ID, first revision Some(text), subsequent revision
numbers consecutive, and any deletion final. Preflight multi-note ID/revision
allocation before changing a journal so a failed bulk edit is unchanged. Existing non-journal version
handling is unchanged. Old binaries are not promised to read1.1 safely; document
that limitation. Build actual RON legacy/new/mixed round-trip probes before 05's
prompt; no assumption that RON untagged enums provide this compatibility.

Legacy LabelValue::Notes stays a Vec<String> read/projection of active notes.
In Legacy mode its full-list writes retain today's behavior. In journal mode a
successful full-list write means explicit replacement: append anonymous deletion
revisions to active notes, then allocate anonymous new notes for each supplied
string; preserve prior history. Per-note APIs and `note list/add/edit/delete/history`
commands operate on IDs with optional explicit authorship. Full-list writes do
not silently match equal strings to old IDs. Emit owned NoteCreated/Edited/Deleted
receipts with before/after revision data, then one existing LabelChanged(Notes)
projection for consumer compatibility; load supplies one NoteLoaded receipt per stored note (including tombstones),
carrying its retained revisions before the Notes label projection and PatchLoaded;
it does not pretend loading replayed historical user edits. No duplicate records from wrapper calls. Detailed command
syntax and exact wire examples belong to 05's prompt, with these semantics binding.

This is a recorded move of per-note/authorship/history out of 02, not permission to
claim full literate editing after 02. Current flat tags/search remain 02; hierarchy/
formatter/user configuration remains03; port labels/edge vocabulary remain Arc13.

## D5 — real completion belongs to 06

**Deferred with an explicit owner and prerequisite.** Slice06 must supply actual
Tab completion, left/right/backspace editing and bounded in-memory command history
in a TTY, while piped stdin keeps02's line protocol. An output-only `complete`
command does not discharge it. Completion candidates come from an immutable owned
snapshot of current verbs/modules/ports/parameters/IDs refreshed after mutations;
the editor must never borrow the rack or block waiting for the main owner to tick.
History persistence/configuration remains03; completion does not execute a command.

Preferred integration: Rustyline's Editor/Completer and ExternalPrinter (18.0.1
API inspected via its published docs). It supports external printing while input
is edited, but the trait's documentation calls its output stdout; the required
MIDI-only stdout boundary must be demonstrated, not assumed. Before06 prompt
issuance, CRC must run a small real PTY integration probe to establish stderr
routing, watch redraw, Ctrl-C/EOF and bounded shutdown, with stdout captured
separately. Return the concrete dependency/configuration choice to CDC if that
probe cannot preserve the contract; do not redirect process-wide descriptors,
share the MIDI sender or add an output service as an improvised workaround.
No new terminal dependency is authorized for02. This terminal-specific integration
gate is owned by 06; it does not block02's useful line-oriented watch/session edits.
References: [Editor](https://docs.rs/rustyline/18.0.1/rustyline/struct.Editor.html),
[ExternalPrinter](https://docs.rs/rustyline/18.0.1/rustyline/trait.ExternalPrinter.html).
The candidate API is verified; its required platform/stream integration is not.
The old exception permitting a detached stdin-only reader does not permit leaving
a raw-mode terminal guard behind. The06 probe must establish terminal restoration
and reader shutdown on quit, EOF, Ctrl-C and output failure before that design is
issued; a blocked editor cannot simply be abandoned as though it owned only stdin.

## CRC recipe, verification and return

Source findings behind this design at e582aed: facade lifecycle.rs `commit` drops
the old App; persistence.rs LoadedIdentity lacks origin; core patch.rs emits
parameters from HashMap iteration and PatchLoaded last; topology.rs disconnect
needs pre-despawn target capture; label.rs uses full-list notes; core event.rs
`set_module_parameter` emits accepted parameter deltas. rack_live/control.rs
`owner_turn`, `handle_input`, `deliver_drained` and `run` own the current ordering/
cleanup; rack-live.rs owns startup selectors and the fixed startup display path.
These were inspected, not invented public APIs. No source implementation changed
or new observer behavior was tested by CDC in this design return.

CRC's02 prompt should authorize the actual producer-to-consumer paths: core
observation/event/patch/topology/label/lib and necessary error support; facade
observation/lib/lifecycle/persistence/commands/labels/cables and tag-query helper;
rack-live.rs and rack_live control/input plus an optional session command module;
focused tests, README, user-testing runbook and oxurack Cargo.toml/lock only for the
matching ron dev-dependency. Refine this into exact named files after inspection.
No note schema migration, RT implementation, catalog redesign or completion
library belongs in 02. Do not repeat the old failure mode of permitting only the
frontend while forbidding the core emission changes the design requires.

Apply Rust API-02/03 SHOULD (owned records/PathBuf, borrowed inputs), TD-02/03
SHOULD (typed sequence/lifetime IDs), EH-01 MUST (recoverable cursor/input errors),
from the installed rust-guidelines chapters02/05/03. Use the current synchronous
owner, existing Result/thiserror style and optional graph; no generic async default.
Provide code shapes for producer → staging → commit → cursor → CLI and the exact
command/Scope matrix above. Guide07 readiness must include context headroom for
core integration, not just the parser. Reuse existing native tests and fake sender;
no new persistent verification tool or hardware/soak campaign.

Required discriminating acceptance groups for02 (CRC may name the rows):

1. **Literal common order:** actual public cursor observes a small fixed patch's
   exact interleaved module/label/sorted-parameter/cable/rack/summary sequence;
   label→parameter→cable direct and command calls retain their true order/source.
   Graph and independent typed cursors still work in default mode; observation
   works without graph. A hand-built expected vector or fabricated stream is not
   proof of producer integration.
2. **Lifetime identity/isolation:** same-name replacement, restart, unload twice,
   failed candidate after some private events, rejected live edit, and removal
   with two incident cables. Inspect actual old/new lifetime/sample and literal
   retirement/load order; failure changes neither cursor history nor current rack.
   Surviving parallel cable IDs do not follow shifted occurrence indices.
3. **Retention/readers:** two independent cursors, Now/Oldest behavior, page split,
   more than 1,024 changes, one slow reader's exact gap and current-state recovery;
   large single candidate exceeding staging capacity still loads with honest gaps.
   Wrong-facade cursor rejects without advancing it. Use small private test capacity
   where useful; no million-event stress harness.
4. **Executable session:** actual CLI waits while playing; watch on → label/set/
   cable edit → pause → load another patch → scope/status → play → pause → unload
   → failed load → reload → quit. Verify replaced label, explicit Scope fallback,
   empty status, old cleanup exactly once through the existing fake/native seam,
   no stale Scope records, continued command fairness and error recovery.
5. **Metadata/query/music:** actual current labels and exact Any/All queries across
   rack/module/parallel cables; save/fresh replay and unchanged live continuation
   versus direct-API control. Watching and querying alone do not alter MIDI/Scope.
   Malformed structured input leaves state intact. Full-list notes remain the
   existing schema; no accidental note-history completion claim.

Run current affected core/facade/graph unit/integration gates, facade all-targets
both feature modes including rack-live, example builds, lint/strict Clippy/docs and
workspace make test with the existing recorded CoreMIDI host retry if necessary.
CRC records command results and independently reviews the real composed workflow.
Retain accepted 01 evidence where unchanged; target changed integration behavior.

CRC next action: acknowledge this directive in arc/project handoff history, repin
source, then create `arc07-live-interface/slice02-session-observation/` with plan,
ledger and detailed initial cc-prompt.md. Relay its project-relative path through
Duncan. No new CC assignment exists until that authoring is complete. Keep Arc07
A-07-01/A-07-02 and project P-01–P-06 open; P-07 and prior arc acceptance unchanged.
Existing issued prompts and escalation14 remain immutable. Commit exact planning
paths separately from source under Expedited Mode; no merge/push is assigned.
