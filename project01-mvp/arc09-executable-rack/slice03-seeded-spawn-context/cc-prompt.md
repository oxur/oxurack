# CC assignment — arc09/slice03 seeded spawn context

2026-09-18. Three-Contributor Workflow, Expedited Mode. Sofie/CRC assigns
implementation and independently reviews CC's return. Current project-relative
assignment: `arc09-executable-rack/slice03-seeded-spawn-context/cc-prompt.md`.

## Outcome, pin and authority

Implement the approved [CDC §3 seed handoff](../slice01-module-behavior-installation/artifacts/execution-contract.md#3-seed-handoff-and-reset): fixed byte-level derivation, pre-construction per-instance context, legacy-compatible patch dispatch and fresh-process replay. The exact public API, vectors, scope and failure policy are in [slice-plan.md](slice-plan.md); six [ledger rows](ledger.md) define done. TM, Clock, MIDI, device and generic restart behavior are outside this slice.

Inspected clean source predecessor: `5c1ce2ccb5e0ba36ac9fd2605a047e0ff45928b2`
on `codex/oxurack-module-behavior-installation` in
`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`.
Planning predecessor is the CRC commit issuing this prompt. Confirm both
worktree heads and dirty state before edits; report drift to CRC before
dependent changes. Slice02 is closed; arc10 remains CDC-owned and held.

## Required reading and CC intake

Load required material into the executing context before dependent edits.
Use bounded contiguous reads and recover truncation; a filename, grep hit,
hash or another agent's report is not content. Execute the required-data
queries and inspect complete results; record command, denominator, errors
and text coverage separately in `artifacts/test-evidence.md`. Write a short
source-cited contract readback connecting T-01–T-06 to actual spawn paths,
compatible callers, errors and the cross-process oracle. Revalidate state
and reload unavailable contracts after context recovery. Coherent preflight
permits implementation without another approval.

| Order | Material | Required extent and purpose |
| --- | --- | --- |
| 1 | This prompt, `slice-plan.md`, `ledger.md`, `artifacts/seed-golden-reference.md`, current planning HEAD | Required-full: assignment, API, independent seed calculation and six rows. |
| 2 | Planning `AGENTS.md`, `../arc-plan.md` | Required-full: worktree, roles, arc sequence. |
| 3 | `../slice01-module-behavior-installation/artifacts/execution-contract.md` | Required-section: opening authority, complete §1 and §3, §2 name-order/sample boundary, §5 quality owners. §4 is future context only. |
| 4 | `../slice02-simulation-samples/crc-verification.md`, `../../project-plan.md` | Required-section: slice02 iteration01 close/bubble-up; project Purpose/DoD, active roadmap, quality dispositions and latest handoff. |
| 5 | Installed collaboration-framework `knowledge/engineering-methods/guides/07-implementation-prompt-authoring.md`, `knowledge/project-management/guides/README.md`, `guides/02-canonical-planning-worktree.md`, `guides/04-closing-slices.md` | Required-section: intake/data, discriminating tests, Expedited Mode and first-class close placement. |
| 6 | Installed `rust-guidelines/SKILL.md`, `guides/02-api-design.md` API-08/API-24, `guides/03-error-handling.md` EH-01, `guides/04-ownership-borrowing.md` OB-01/OB-12, `guides/15-cargo/07-lints-and-formatters.md` CG-L-01; installed testing `guides/01-testing-discipline.md` and `03-validation-gates.md` | Required-section Rust strengths/patterns; required-full testing guides. |
| 7 | Pinned source core `src/{rng,module,patch,lib}.rs`, umbrella `src/lib.rs`, TM `src/engine.rs`, core `Cargo.toml` | Required-section: complete named symbols/callers/tests below; TM is read-only. |
| 8 | Pinned source audit | Required-data: run `rg -n 'ModuleRegistration \{|\.spawner\b|ModuleSpawner\b' crates` and `rg -n 'derive_seed|derive_module_rng|ModuleId::from_instance_name|master_seed|with_seed' crates`; inspect complete results, record hit counts and exclusions, then read every affected production caller and direct registration literal. |
| 9 | Workspace `Cargo.lock` dependency projection | Required-data: run `rg -n '"ahash"|name = "oxurack-core"' Cargo.lock`; record every match and its enclosing complete `[[package]]` stanza, including dependency arrays. Check the direct dependency before and after the scoped removal; record result count and errors. |

Conditional: source drift, extra caller, undocumented dependency behavior or
out-of-scope edit triggers full affected definitions/tests and CRC
reconciliation before that step. Old slice prompts and migration material are
reference-only history.

## Source reconnaissance and applied guidance

At `5c1ce2c`, `Patch.master_seed` is parsed but unused by the
`apply_patch_to_world` spawn loop. `OxurackModule::spawn` takes world/name/
parameters; `ModuleRegistry::register<M>` stores a legacy `ModuleSpawner`
pointer in public `ModuleRegistration`. Patch application calls it after
validation, before labels/messages/cables. Three graph tests construct public
registration literals, and the accepted module-installation test calls
`.spawner` directly. `rng.rs` uses `AHasher::default`; ModuleId calls
`derive_seed(0,name)`. Umbrella `load_patch` delegates to core. Standalone TM
has OS-seeded `new()` and locked `SmallRng`-seeded `with_seed(u64)`; neither
is changed here.

| Guidance read | Applied decision and proof |
| --- | --- |
| Rust API-08 SHOULD / API-24 MUST | Named context with inherent constructor/accessors identifies master/name/ID/seed; test fields and re-exports. |
| Rust EH-01 MUST | Preserve recoverable spawner errors; do not panic on patch input or invent atomic rollback. Test a failed load followed by an independent load. |
| Rust OB-01 MUST / OB-12 SHOULD | Borrow validated name only during synchronous spawn; build owned RNG/state before borrow ends and scope registry/world borrows. |
| Rust CG-L-01 SHOULD; testing guides | Full all-targets Clippy, fixed vectors and fresh-process oracles reject process-local or OS seeds. |

## Binding implementation recipe

1. In core `rng.rs`, replace AHasher with FNV-1a over the **exact** slice-plan
   byte stream. Each byte does `hash = (hash ^ u64::from(byte)).wrapping_mul(PRIME)`;
   include domain zero, little-endian master and UTF-8 **byte** length. Keep
   `derive_seed` and `derive_module_rng` signatures; the latter seeds
   `SmallRng` from the new value. Remove only direct `ahash` from core
   `Cargo.toml`, and inspect Cargo-required lockfile changes. No other update.
2. In core `module.rs`, define borrowed private-field
   `ModuleSpawnContext<'a>` with the planned constructor/accessors. Constructor
   computes `ModuleId::from_instance_name(name)` and
   `derive_seed(master_seed,name)`. Add the default trait
   `spawn_with_context` forwarding to required legacy `spawn`. Preserve
   public `ModuleRegistration` literals and `.spawner`. Recommended internal
   dispatch: private context-spawner map in `ModuleRegistry`; `register<M>`
   stores the new hook, while direct `insert` removes a stale hook and uses
   the legacy spawner fallback. No global seed resource or runtime downcast.
3. In core `patch.rs::apply_patch_to_world`, keep validation first. Build
   context from `patch.master_seed` and instance name immediately before each
   spawn; dispatch through registry with manual-registration fallback. Retain
   labels, messages, parameter application, cables and `PatchHandle`. Do not
   retain context after synchronous call or promise rollback of prior world
   mutations on spawn error. A following independent load must receive only
   its own context.
4. Re-export context in core and umbrella `lib.rs`. Document changed
   `derive_seed`/ModuleId numeric values, no old cross-version stream
   guarantee, non-cryptographic hash and locked SmallRng compatibility.
   Slice02 name-based observable ordering and stopped `Oxurack::load_patch`
   behavior stay unchanged.
5. Add a real two-instance RON fixture and context-aware seeded probe under
   core tests. Its `spawn_with_context` initializes owned `SmallRng`/state
   from `context.instance_seed()` before returning, and applies an initial
   parameter. Run controlled core samples and record a sorted indexed trace.
   Use `current_exe`, exact child-test filtering and an environment marker to
   compare two distinct child processes with a fixed expected fixture.
   Changed master/name must differ; reordering patch modules preserves each
   name's trace; stop/resume continues, fresh load restarts. Add a bounded
   umbrella RON load test proving the context passes through public
   `Oxurack::load_patch` too. No OS RNG in patch-driven construction or sleep.

Private helper names and harness framing are local discretion. A materially
different public API, compatibility need or source path requires CRC review
before editing.

## Falsifiable oracles

| Rows | Setup/action | Exact expected observation | Rejected behavior |
| --- | --- | --- | --- |
| T-01/T-02 | Six slice-plan byte vectors, including max-u64 plus two-byte `é`; call public derive and ModuleId. | Six fixed hex values match; `ModuleId("vco_1")` is `0xfda42c6e50ff0d2a`. | AHasher, native-endian master, char-count length, missing separator or changed domain. |
| T-03 | Registered probe, legacy-only module, direct registration literal and replacement. | Probe sees four context fields before initial state; legacy default calls old spawn once; direct `.spawner` works; replacement cannot invoke old hook. | Late/global seed injection and public literal breakage. |
| T-04 | RON with `tm_a`/`tm_b`, master 42 and parameter; reverse order, vary seed/name, fail a spawner then independently load. | `tm_a` seed `0x21819f1bfbd4fcea`, `tm_b` seed `0x21819e1bfbd4fb37`; only affected per-name state changes; parameter appears at first observation; no context leaks. | Entity/order seed, partial-load leakage, late seed. |
| T-05 | Fixed RON, three samples, two child processes and committed trace fixture. | Both sorted indexed traces equal fixture; seed/name controls differ; stop/resume continues while fresh load restarts. | Same-process-only proof, OS RNG, unsorted output or reset substituted for reconstruction. |
| T-06 | Clean source/planning commits and complete matrix. | Required gates pass or retain exact restricted failure and host retry without waiving timing, coverage or physical MIDI. | Pass-only summary or weak substitute. |

For a SmallRng expected fixture, seed a reference instance with the
**hard-coded plan golden seed**, not a call to new `derive_seed`, using locked
rand; freeze and report its exact trace bytes before comparing patch-driven
output. Do not silently update expectations to match failed code. The old
AHasher must fail fixed-vector assertions.

## Scope, validation and return

Allowed source paths are exactly [slice-plan source scope](slice-plan.md#implementation-route-and-source-scope).
Do not edit TM, graph, RT/device, other manifests, scheduler or prior slices
without CRC review. `ModuleRegistration` compatibility and slice02 tests are
binding. Architecture/scope/failure changes return to CDC through CRC.

From the source worktree, run focused vector/spawn/process tests and record
each command. On final clean source, run separately:

```text
cargo test --locked -p oxurack-core
cargo test --locked -p oxurack
cargo test --locked -p oxurack --no-default-features
cargo test --locked --workspace --all-features
cargo test --locked --workspace --no-default-features
cargo clippy --locked --workspace --all-features --all-targets -- -D warnings
cargo fmt --all -- --check
RUSTDOCFLAGS=-Dwarnings cargo doc --locked --workspace --all-features --no-deps
RUSTDOCFLAGS=-Dwarnings cargo doc --locked -p oxurack --no-default-features --no-deps
make check
cargo outdated --root-deps-only --ignore-external-rel
git diff --check 5c1ce2c..HEAD
```

Inspect scoped manifest/lock diff and planning links/current assignment.
Record actual exits, cwd, HEAD and dirty/clean state, toolchain and raw logs
in `artifacts/validation-results.tsv` and `artifacts/logs/`; retain restricted
MIDI/DNS failures and host retries separately. A slice02 checker requiring
open rows is historical pre-acceptance evidence, not this slice's validator.
No warning suppression, ignored new test or narrowed matrix.

Write `artifacts/test-evidence.md` with intake/readback, golden provenance,
subprocess traces, compatibility audit and gate caveats. CC writes root
`closing-report.md` with six-row proposed walk, exact commits/paths, artifact
inventory and Bubble-up to arc09. Commit source and planning separately with
explicit path staging and `git commit --only` limited to assignment files;
inspect cached diffs and preserve unrelated work. Return exact revisions,
worktree states, outcomes and blockers to CRC. CC remains proposed-done until
CRC independently accepts the slice.
