# Slice03 test evidence

This packet records CC implementation evidence for Arc09 Slice03 at source
commit `642313a62db437ee2bf0b3d5b97fcdcffa16782a`. It is proposed-done
evidence for CRC review; it is not an independent CRC or CDC acceptance.

## Implemented contract

- `ModuleSpawnContext<'a>` is public and re-exported by both `oxurack-core`
  and the umbrella `oxurack` crate. It carries the master seed, borrowed
  instance name, `ModuleId`, and derived instance seed through named
  read-only accessors.
- `OxurackModule::spawn_with_context` is a default method that forwards to
  the required legacy `spawn` method. Registered modules receive the context
  hook; direct `ModuleRegistry::insert` registrations clear the hook and use
  their public legacy spawner.
- Patch validation still completes before the per-module context is
  constructed. The context is created immediately before the corresponding
  spawn call and is borrowed only for that synchronous call.
- `derive_seed` now follows the locked FNV-1a byte stream from the slice
  golden reference. The direct `ahash` dependency and its unused lockfile
  chain were removed. `ModuleId` remains derived from master seed zero and
  the instance name.
- No TM engine, graph, RT/device, scheduler, or prior-slice manifest source
  was changed.

## Golden seed and trace evidence

The committed implementation matches the six plan vectors in
`seed-golden-reference.md`:

```text
(0, "")                  = a854da736b8befab
(42, "tm_a")             = 21819f1bfbd4fcea
(42, "tm_b")             = 21819e1bfbd4fb37
(43, "tm_a")             = 5365fb0ec2a0228b
(0, "vco_1")             = fda42c6e50ff0d2a
(u64::MAX, "é")          = 199408757f0d299d
```

The seeded probe fixture uses the plan's `(42, tm_a)` and `(42, tm_b)` seeds
and records the sorted, indexed trace:

```text
0|tm_a|4fffb24cd3888b92|2
0|tm_b|98a8f97c490c3bfe|3
1|tm_a|2cc8263b24bccaa8|2
1|tm_b|ebb0290dbb044775|3
2|tm_a|a5198b340227b2dc|2
2|tm_b|4bbaa57382d07c6f|3
```

The core integration test checks the trace against hard-coded plan seeds,
checks that patch order does not change per-name output, and checks that a
changed master seed or instance name does change output. It also launches the
same test executable twice with the exact child filter and compares both child
traces to the committed expected trace. See
`logs/13-core-seeded-context.log`.

## Compatibility and failure evidence

The core integration tests cover:

- context fields and parameter values being visible during construction;
- the default context hook forwarding to legacy spawn;
- direct registration preserving the public registration fields and falling
  back to legacy spawn after hook removal;
- a typed spawn failure followed by an independent load, showing no leaked
  context state;
- stopped schedules not advancing RNG state, resume continuing the owned
  state, and a fresh load restarting from the fixture seed;
- public umbrella loading through `Oxurack::load_patch`.

## Validation outcomes

The complete command matrix is in `validation-results.tsv`. The focused
results and package suites pass. The exact workspace commands were run first
in the restricted sandbox and retained the expected four `oxurack-rt` MIDI
initialization failures; host retries passed both feature configurations. The
restricted `make check` and `cargo outdated` attempts are retained separately
from their host retries. The restricted dependency probe failed at registry
DNS; the host retry completed and reported available dependency updates
without changing the lockfile.

The required gates passed on the host retry: `make check`, Clippy with
`-D warnings`, formatting, strict docs, and both workspace feature modes.
Clippy and all-features rustdoc emitted the existing future-incompatibility
notice for `block v0.1.6`; no new warning or error was emitted.

## Evidence boundaries

These are CC-authored implementation tests and replay artifacts. They do not
claim independent reproduction, semantic acceptance, or ledger closure. The
six slice ledger rows remain open for CRC review, and Arc10 remains held by the
planning instructions.
