# Arc11 Slice02 — bounded Scope capture and CSV export

Version 1.1 — 2026-10-11. Status: closed; five S rows independently done at6c56e84.
Current assignment: `arc11-observation-workbench/slice02-scope-capture/cc-prompt.md`.
CRC authors/reviews; CC implements; Duncan relays. Three contributors,
Expedited Mode. Directive09 releases this slice after accepted LFO Slice01 at
`c4b517c89f51d44c0eb1bec4a7b891984d3e0a92`; closure planning `acb97eb`.

## Outcome and scope

Let the existing seeded musical example export a bounded, sample-indexed view
of selected signals and actual MIDI events without altering its performance.
Scope is an observer owned by the umbrella rack, available with and without
the graph feature. It is not a new signal-generating module, scheduler or MIDI
consumer. Add a small public capture API, scalar observation at Observe, caller
MIDI-copy admission, and CSV flags in the existing example. A terminal inspection
view remains Slice03; no generalized telemetry/streaming, UI, dependencies,
patch schema or module algorithm changes.

## Binding capture and lifecycle contract

`ScopeSelector` identifies exact module instance and port names. Resolve against
the current PatchHandle and actual child Port entities, without graph dependency
or label fallback. Validate all selectors/settings while stopped before replacing
capture state: positive capacity, loaded patch, existing unique selectors, supported
Float/Bipolar/Gate/Raw kinds and matching CurrentValue. Duplicate selectors and
Midi ports are rejected. An empty selector list permits MIDI-only capture.
Configuration while running is an error. Failure leaves existing capture intact.
Typed ScopeError is separate from core errors; no new core fault protocol.
Reserve the queue fallibly with try_reserve_exact; report allocation failure
before replacing existing capture. No hard real-time latency guarantee is claimed.

Store one mixed ordered queue of records, with capacity measured in **records**,
not frames, milliseconds or bytes. Each selected signal contributes one record
per running sample; each copied generated/sent MIDI event contributes one record.
At capacity, discard oldest before appending and increment overwritten count.
Never exceed capacity; a retained window may start partway through a sample.
Selection labels are owned once in a stable table; records reference selection
indices, not freshly cloned strings. No per-tick I/O or shared mutex. Logical queue length is the capacity
guarantee; allocator reserve bytes are not a second observable record budget.

Observe copies CurrentValue after Consume in selector order, using SampleTime's
zero-based last_index for that sample. It inherits the core's latched running
gate, including completion of a sample stopped during Consume. It does not check
raw SimulationRunning again, mutate source ports, drain MIDI, or advance clocks.
There is exactly one observer per rack; reconfiguration replaces its resource,
not systems. Internal disappearance/type corruption after validation is an
invariant failure. Legitimate scalar nonfinite values are copied without rewriting.

Stop/resume retains selection, queue and counters; no records while already
stopped. clear_scope clears records/overwritten count only, preserving capture,
module state and absolute sample time. disable_scope removes capture state;
recording MIDI while disabled is an explicit no-op. Successful load_patch disables
the old capture to prevent stale entity selections; failed loads retain it.
Reconfigure after loading another patch. A fresh rack starts without capture.
into_app retains the configured resource and observer; callers inherit the core
invariant obligations as they already do for other installed systems.

## MIDI ownership and delivery labels

The runner remains the sole drain owner. Copy its complete drained batch into
Scope as Generated, in existing buffer order; then copy each successfully sent
performance event as Sent, after sender.send returns Ok and before trace writing.
All generated events retain their original event.sample. Cancelled/failed/unsent
suffixes may appear Generated without Sent; a failed send must never claim Sent.
Offline capture records Generated only. Held Value::Midi is never an event source.
Session cleanup traffic is outside the rack performance stream and excluded.
Document that Sent means software send success, not endpoint/audio acknowledgement.
Playback compatibility concerns logical samples/ordered events and cleanup;
capture overhead is not a hardware wall-clock timing guarantee.

End-of-play discard of any remaining rack buffer also records its drained events
as Generated if capture is enabled. Scope never steals or reorders MIDI. Preserve
existing cancellation, primary failure, note ownership/cleanup and stdout trace.
The unchanged public drain_midi API does not automatically capture: other callers
explicitly copy their own drained events via the Scope API.

## CSV and runnable example

Add optional `--scope-csv PATH`, repeatable `--scope-port MODULE.PORT` (split at
last dot; both parts nonempty) and `--scope-capacity N` (positive; default100000
records when capture enabled). Port/capacity flags require scope-csv; all conflict
with list-outputs. No scope flags preserves existing behavior exactly. Omitting
scope-port requests MIDI-only capture; README supplies useful explicit selectors.
Resolve selectors/settings and open the CSV destination before starting samples
or connecting an endpoint. File creation overwrites the explicitly selected path.

CSV header: `sample,record,module,port,value_kind,value,status,data1,data2`.
Rows are retained insertion order. record is signal, midi_generated or midi_sent.
Signal kind is float/bipolar/gate/raw, value uses Rust Display (Gate true/false);
MIDI rows use kind midi, blank module/port/value, and uppercase two-digit hex
bytes. Escape textual fields correctly for comma, quote, CR and LF. Write/flush
after stopping/cleanup, even when playback failed after starting; preserve the
primary failure and also disclose any export failure. Export performs no capture
mutation. Print capacity/retained/overwritten summary to stderr; stdout remains
the exact existing performance trace. Empty captures export the header only.

## Verification and exit

Five ledger rows map to literal Observe values/timing; bounded window/lifecycle;
validation atomicity; real drained MIDI and unchanged performance; and public
CSV/example plus native gates. The prompt supplies concrete shapes, recipes and
commands, including no-default graph independence and a fake-sender failure
through the actual live runner. No actual endpoint is required for this change.
All five rows require independent CRC acceptance. Arc11/project remain open;
Slice03 follows only on accepted capture. No supporting artifacts are required
for routine passing checks; necessary durable outputs belong in sibling artifacts/.

## Author readiness

CRC at c4b517c inspected facade construction/load/advance/drain, PatchHandle,
Port/ChildOf, SampleTime and CorePlugin phase/gate setup, actual offline/live
runner and MIDI event type. The binding Arc11 ownership/design is preserved;
local selector, mixed record capacity, lifecycle and CSV choices are resolved
above. Rust EH-01, OB-02, PF-23, DC-01 and CLI-05/CLI-27 applied in the prompt.
Worked snippets are labeled illustrative; actual APIs verified locally. Literal
values, negative controls, partial send results and unchanged trace oracles are
specified. Reading is bounded to this packet/relevant source and guidance,
leaving implementation/recovery headroom. All six readiness checks pass.

## Assignment history

| Date | Prompt | Predecessor / reason | Disposition |
| --- | --- | --- | --- |
| 2026-10-11 | arc11-observation-workbench/slice02-scope-capture/cc-prompt.md | directive09; accepted LFO at c4b517c; next approved workbench increment | Accepted; CC return f2adf44, source6c56e84; CRC reproduced all five rows |

## Version history

- v1.1 — 2026-10-11: CRC accepted all five rows at6c56e84; bounded capture/CSV and native gates reproduce. No design findings; Slice03 authoring released.

- v1.0 — 2026-10-11: Opened on accepted LFO source; bounded Observe/MIDI copy
  capture plus runnable CSV export, leaving the terminal view to Slice03.
