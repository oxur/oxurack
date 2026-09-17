# Restart baseline environment

Captured 2026-09-17 for `project01-mvp` / Arc08 / Slice01, iteration01.
This is the CC execution record for the assignment in
`cc-prompt-iteration01.md`. The evidence is attested and awaits independent
CRC review.

## Repository state

| Field | Value |
| --- | --- |
| Implementation cwd | `/Users/oubiwann/lab/oxur/oxurack` |
| Implementation branch | `main` |
| Implementation source commit | `a7ce6e61536ca96b6aeb2227ee06e430ed8df162` |
| Required source pin | `a7ce6e61536ca96b6aeb2227ee06e430ed8df162` |
| Planning cwd | `/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning` |
| Planning branch | `planning` |
| Planning HEAD at baseline capture | `c613050` |
| Planning predecessor named by the slice | `f0cf3a5172e02c717c7f924c84156f5fe0405733` |
| Cargo.lock SHA-1 before | `fcc8f7816cc14a1cb5f0df72016b13115404f85f` |
| Cargo.lock SHA-1 after | `fcc8f7816cc14a1cb5f0df72016b13115404f85f` |
| Implementation status before/after | clean / clean |

The source checkout was already at the required pin. No implementation files,
manifests, `Cargo.lock`, Makefile or tests were edited. `make check` was run
with `CARGO_NET_OFFLINE=true`; its first attempt was blocked by the sandbox's
Cargo registry cache permissions and the same command was rerun with the
normal tool escalation and passed. The failed attempt remains in
`logs/16-make-check-offline.log`, and the passed attempt is in
`logs/17-make-check-offline-escalated.log`.

## Tool and workspace inventory

| Tool | Observed value |
| --- | --- |
| rustc | `rustc 1.98.0 (88d9e12ae 2026-08-18)` |
| cargo | `cargo 1.98.0 (797e8a9bc 2026-08-05)` |
| cargo-binstall | `/Users/oubiwann/.cargo/bin/cargo-binstall` |
| cargo-outdated | `/Users/oubiwann/.cargo/bin/cargo-outdated` |
| cargo-llvm-cov | found by `make check-tools` |
| host | `aarch64-apple-darwin` |

`cargo metadata --locked --no-deps --format-version 1` reported six workspace
members:

| Package | Features observed |
| --- | --- |
| `oxurack` | `default`, `graph` |
| `oxurack-core` | `default`, `rt-bridge` |
| `oxurack-midi` | `default`, `reflect`, `serde` |
| `oxurack-rt` | none |
| `oxurack-graph` | none |
| `oxurack-mod-turingmachine` | `default`, `midi-io`, `midir`, `serde` |

The raw metadata is in `logs/08-cargo-metadata.log`; tool checks are in
`logs/01-make-help.log` through `logs/10-command-cargo-outdated.log`. Artifact
shape, scope, cached-diff and final planning self-checks are preserved in logs 54–61.

## Evidence interpretation

The locked build, lint, format, workspace test and package feature probes all
completed with exit code zero. Documentation with warnings denied failed on two
broken intra-doc links. Both coverage commands reached the `oxurack-rt` MIDI
tests and failed because MIDI support could not be initialized in this
environment, so no overall coverage percentage or denominator was produced.
The direct dependency check completed after escalation and reported available
updates; that is a dependency-gate finding, not permission to update the lockfile.
Three RT quality tests are explicitly ignored and remain unrun because they
require RT scheduling permissions. Physical MIDI, DAW, benchmark, 24-hour soak
and musical-session gates remain deferred under the retained Arc08/project
requirements.
