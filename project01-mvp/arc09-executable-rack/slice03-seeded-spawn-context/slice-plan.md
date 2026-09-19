# Slice03 — seeded spawn context and fresh-process replay

Version 1.1, 2026-09-18. Status: closed; six rows independently accepted.
Workflow: CDC design / CRC review / CC implementation, Expedited Mode.
Source predecessor: clean `5c1ce2ccb5e0ba36ac9fd2605a047e0ff45928b2`
in `.worktrees/codex-oxurack-module-behavior-installation`.
Depends on [accepted slice02](../slice02-simulation-samples/crc-verification.md#iteration01-independent-close--2026-09-18).
Authority: [CDC execution-contract §3](../slice01-module-behavior-installation/artifacts/execution-contract.md#3-seed-handoff-and-reset)
and [arc09](../arc-plan.md). Material changes to API compatibility, seed
bytes, restart semantics or arc scope return to CDC and the Operator.

## Outcome and boundaries

A validated patch's master seed and instance identity reach each registered
module before its state is constructed. Each instance receives a deterministic
derived seed from a fixed, versioned FNV-1a byte stream. The same patch under
the same locked build yields identical per-instance traces in separate
processes, regardless of spawn order; changing master seed or instance name
changes that instance's stream. Legacy modules implementing only `spawn`
continue to load, and direct `ModuleRegistration` literals remain valid.

This slice does not build the TM adapter, Clock, MIDI path, live snapshot,
generic restart command, RT/device integration or full transport. A fresh
patch load constructs new state from the seed; stopping an existing rack does
not reconstruct it. Arc10's TM adapter must call `TuringMachine::with_seed`
from this context and reapply parameters for seed restart;
`TuringMachine::reset()` alone is not equivalent. Inherited coverage and
hardware gates retain their separate owners.

## Binding public API and data contract

Add public `ModuleSpawnContext<'a>` in core, borrowing `instance_name: &'a str`
and storing `master_seed: u64`, `module_id: ModuleId`, and
`instance_seed: u64` privately. Its inherent infallible constructor
`new(master_seed: u64, instance_name: &'a str) -> Self` computes derived
values; expose named read-only accessors for all four fields. Re-export from
core and curated umbrella. It is an ephemeral construction input, not an ECS
resource or persisted snapshot.

Add `OxurackModule::spawn_with_context(world: &mut World, context:
&ModuleSpawnContext<'_>, parameters: &HashMap<String, ParameterValue>) ->
Result<Entity, CoreError>`, with default forwarding to
`Self::spawn(world, context.instance_name(), parameters)`. Keep required legacy
`spawn` and its signature, public `ModuleSpawner` alias,
`ModuleRegistration` field layout and direct `ModuleRegistry::insert` behavior.
A registered module uses the new hook from patch application; a manually
inserted registration falls back to its legacy spawner. The recommended
private implementation is a parallel context-spawner map in `ModuleRegistry`
keyed by kind; direct `insert` clears a stale hook. Do not use a mutable
global seed resource or infer seed from the world after engine construction.
No rollback of existing partial world mutations on spawn failure is promised;
context itself must not leak to a later module or patch.

`derive_seed(master_seed, instance_name)` uses FNV-1a 64-bit, wrapping at each
multiply, over exact bytes in order: ASCII `oxurack.module-seed.v1`, one zero
byte, `master_seed.to_le_bytes()`, UTF-8 name byte length as u64 little-endian,
then unnormalized UTF-8 name bytes. Offset basis is
`14695981039346656037`; prime is `1099511628211`. Keep the helper signature
and `derive_module_rng` returning `SmallRng` seeded from the new value.
`ModuleId::from_instance_name(name)` remains `ModuleId(derive_seed(0, name))`,
so numeric IDs change. Document this compatibility amendment: there is no
prior cross-version sequence guarantee. SmallRng stream replay is bounded to
the pinned implementation/dependency state, not future rand versions.
Slice02's observable sort keys remain names, not numeric IDs.

Independently calculated byte-level golden values:

| Master | Name | Expected `derive_seed` |
| ---: | --- | --- |
| 0 | empty string | `0xa854da736b8befab` |
| 42 | `tm_a` | `0x21819f1bfbd4fcea` |
| 42 | `tm_b` | `0x21819e1bfbd4fb37` |
| 43 | `tm_a` | `0x5365fb0ec2a0228b` |
| 0 | `vco_1` | `0xfda42c6e50ff0d2a` |
| `u64::MAX` | `é` (two UTF-8 bytes) | `0x199408757f0d299d` |

These constants came from the specified byte stream independently of the
current `derive_seed`; the calculation is recorded in
[seed-golden-reference.md](artifacts/seed-golden-reference.md). Do not update
expectations to match implementation.

## Implementation route and source scope

1. Replace AHasher in core `rng.rs` with the exact FNV loop, update docs and
   add fixed vectors and Unicode/boundary tests. Remove now-directly-unused
   `ahash` from core `Cargo.toml`; change `Cargo.lock` only as Cargo requires.
2. Add context, default hook and private registered-context dispatch in core
   `module.rs`. Preserve public registration literals and legacy spawner calls;
   test direct replacement via `insert` and default hook. Update ModuleId docs.
3. In core `patch.rs`, after validation and before each spawn, construct the
   context from `patch.master_seed` and module name; dispatch through registry.
   Preserve labels, messages, parameter application, cables and failure
   semantics. Test pre-construction visibility, both patch orders and a
   failed spawn followed by an independent load.
4. Re-export context in core and umbrella `lib.rs`; leave `Oxurack::load_patch`
   and stopped construction unchanged. Test public RON/umbrella load.
5. Add a real two-instance RON patch and a fresh-process integration fixture
   under core tests. A context-aware probe seeds `SmallRng` in
   `spawn_with_context`, emits a sorted sample-indexed trace and exercises
   identical, changed-seed/name and reordered inputs. A test runner subprocess
   may use `current_exe`, exact child filter and an environment marker. Compare
   two independent child processes and a committed expected fixture. No OS
   RNG in patch-driven construction, no sleep.

Literal source allowlist: core `src/rng.rs`, `module.rs`, `patch.rs`, `lib.rs`;
umbrella `src/lib.rs`; bounded new integration tests/fixtures under
`crates/oxurack-core/tests/` and `crates/oxurack/tests/`; core `Cargo.toml`
and `Cargo.lock` only for direct ahash removal. Changes to `oxurack-graph`,
TM, other manifests, RT/device paths, other seed consumers or accepted sample
scheduler require CRC review before editing. Evidence belongs in this slice's
`artifacts/`; the CC closing report belongs in the slice root.

## Acceptance and validation

The [six-row ledger](ledger.md) is binding. Prove fixed vectors, direct
context fields, default legacy dispatch, real patch/umbrella propagation,
same/different/reordered traces and fresh-process replay. Old AHasher must
fail the fixed vectors; same-process equality alone is insufficient. Test
errors without inventing atomic rollback. Preserve slice02 manual/live
behavior and name ordering after ModuleId changes. Run focused, package,
workspace all/no-default, all-targets Clippy, format, strict rustdoc,
`make check`, separate dependency audit, diff check and planning checks.
Record restricted failures and host retries with actual exits/logs. The
retained global 95% target is not claimed by passing new tests.

## Assignment history

Current assignment: `arc09-executable-rack/slice03-seeded-spawn-context/cc-prompt.md`

| Date | Prompt | Predecessor | Reason | Disposition |
| --- | --- | --- | --- | --- |
| 2026-09-18 | cc-prompt.md | accepted slice02 at source `5c1ce2c`; CDC §3 | Implement seed handoff, fixed derivation and fresh-process replay | Accepted by CRC at source `642313a`; six rows done; arc09 composition assembly follows |

Executed corrective refinements: 0 of 5. The [CC closing report](closing-report.md)
and [CRC independent close](crc-verification.md) preserve the implementation
and review evidence. Arc10 remains held for CDC's arc09 composition gate.

## Author readiness — CRC

| Check | Result / pointer |
| --- | --- |
| Source-grounded | Ready: inspected `module.rs` trait/registration/legacy spawner, `patch.rs` validation/spawn loop, `rng.rs`, ModuleId, umbrella load, graph direct registrations and TM `with_seed` at `5c1ce2c`. |
| Design-complete | Ready within CDC §3: explicit context/default hook, private dual dispatch, failure/non-rollback boundary, versioned bytes and compatibility amendment are fixed above. |
| Guideline-applied | Ready: [CC prompt](cc-prompt.md#source-reconnaissance-and-applied-guidance) maps Rust API-08/API-24, EH-01 and testing/final gates to symbols and proof. |
| Executable | Ready: route names APIs/files, RON fixture, subprocess harness and scoped manifest/lock changes; CC owns private helper names. |
| Falsifiable | Ready: six independent vectors reject AHasher, preconstruction probe rejects late seed injection, legacy tests reject breakage, and fresh-process/reorder/changed-input traces reject process-local seeds. |
| Coherent and portable | Ready: prompt carries required reading/data, bounded intake, pinned predecessor, exact source scope, validation and return contract. |

## Version History

- v1.0 — 2026-09-18: CRC opened the already planned third arc09 slice after
  independently closing slice02, detailing CDC §3 without expanding arc scope.
- v1.1 — 2026-09-18: CRC independently reproduced source `642313a`, closed
  T-01–T-06, and advanced to formal arc09 composition assembly.
