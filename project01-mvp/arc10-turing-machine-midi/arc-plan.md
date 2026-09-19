# Arc10 — Turing Machine MIDI in the rack

Version 1.1 — 2026-09-19. Status: open; Slice01 accepted, Slice02 issued by CRC.
Depends on: Arc09 at source `37deeace90e7fe2dff633ada9d58ef9051577c3d`.
Blocks: M1 endpoint playback and Arc11 observation workbench.
Workflow: Three contributors, Expedited Mode; Duncan relays CDC/CRC/CC packets.
Authority: Operator's TM-first selection and [directive07](../cdc-directive07.md).

## Capability and boundaries

Load a static seeded RON patch containing Clock → Turing Machine → MIDI-out,
reproduce ordered MIDI offline across fresh processes, and play it on a selected
physical endpoint with explicit failure/stop cleanup. Clock is pulled forward
from the workbench because TM needs a real pulse source. M1 is sample-quantized;
it does not satisfy the retained P99 <500 microsecond release timing target.
Arc09 installation, five-phase samples and pre-construction seed context are
accepted foundations. No new simulation or independent RT clock authority.

The detailed [M1 design](design.md) refines the existing execution contract.
Slice01's exact Clock contract lives in its [plan](slice01-musical-clock/slice-plan.md).
Later slices receive full API/test recipes before issue; their directories do
not exist yet. CRC details them within this design, escalating incompatible
changes through Duncan to CDC. No scope is delegated through an empty placeholder.

## Slice sequence

| Slice | Capability / dependencies | Current state |
| --- | --- | --- |
| slice01-musical-clock | Validated standalone sample Clock plus installed Gate output, fractional timing, edit/pause semantics; consumes Arc09 | Closed; CRC accepted source `a19cfd1` after iteration01 |
| slice02-turing-machine-adapter | Opt-in rack adapter around existing seeded engine, full capability inventory, scalar snapshots and rising-edge stepping; consumes Clock | Open; [CC assignment](slice02-turing-machine-adapter/cc-prompt.md) |
| slice03-midi-output | Coherent snapshot consumption, ordered atomic batches, offline capture and bounded refusal/cleanup contract; consumes adapter | Planned; CRC authors after Slice02 acceptance |
| slice04-output-runtime | Output-only runtime, batch/control lanes, session owner and explicit lifecycle/recovery; consumes sink | Planned; CRC authors after Slice03 acceptance |
| slice05-endpoint-acceptance | Integrated static fixture, fresh-process MIDI oracle, real selected endpoint witness and lifecycle tests | Planned; physical device/Operator evidence required before final close |

Each slice is bounded to one integration boundary. If full adapter mapping or
runtime recovery will not fit with reading/review headroom, CRC returns a sizing
escalation before issuing; do not omit mappings or weaken cleanup to fit.

## Composition, inherited work and gates

[ledger.md](ledger.md) opens five composition rows. Expected slice denominator is
five; each actual slice owns its own rows. Do not infer future row counts from
Slice01's six. Arc close needs CRC assembly and CDC independent reproduction;
physical endpoint observation remains an additional explicit Operator gate.

[Successor routing](../artifacts/cdc-successors03.tsv) remains intact: original
RT/public API/catalog obligations are partially addressed here, while external
clock/timing, live persistence/runtime mutation, remaining catalog, interfaces,
coverage provenance/95% hardening, benchmarks, soak and musical acceptance keep
their Arc08/12/13/07/project owners. Deferred historical rows are not auto-closed.
Arc11 reuses this Clock and adds LFO/Scope; no second clock design is implied.

## Version history

- v1.1 — 2026-09-19: CRC accepted Slice01 after one corrective pass and opened
  the fully inventoried Slice02 adapter packet. Arc composition rows remain open.
- v1.0 — 2026-09-18: CDC accepted Arc09's repaired composition and opened the
  existing five-slice M1 roadmap. First-slice timing/edit semantics and later
  batch/control ownership are explicit; no inherited criterion removed.
