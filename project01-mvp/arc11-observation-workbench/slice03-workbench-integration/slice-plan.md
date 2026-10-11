# Arc11 Slice03 — usable terminal observation workbench

Version 1.0 — 2026-10-11. Status: open; four B rows.
Current assignment: `arc11-observation-workbench/slice03-workbench-integration/cc-prompt.md`.
CRC authors/reviews, CC implements, Duncan relays; three contributors, Expedited Mode.
Directive09 and Arc11's third increment authorize this terminal view on accepted
Scope source `6c56e8460b693c285b8052633165e66129318604`, closure planning `819cecd`.

## Outcome and boundaries

Extend the existing seeded musical example with optional `--scope-view` on
stderr, showing patch identity, completed sample count, retained bounds/counts,
latest selected signals and short signal/MIDI tails. CSV can be requested with
or without this view. This is a finite, post-stop inspection tool: no alternate
runner, live redraw, interactive editing, GUI, dependencies or public API changes.
Keep signal capture and MIDI copy ownership in accepted Scope/playback. The
original/no-view commands, CSV schema, fixtures and hashes stay compatible.

## Binding local decisions

New private example module `turing_machine_midi/workbench.rs` renders an immutable
ScopeCapture through a fallible Write. Exact display contract is in prompt's
“Terminal semantics and shape”. Display is deliberately plain and deterministic;
no ANSI, TTY behavior or per-sample I/O. Counts and latest values describe only
the retained FIFO, which may omit older data or start midway through a sample.
No statistics about evicted records or assumed generated/sent pairing. Display
at most N signal rows and N MIDI rows, N=12 by default, explicit1..=1000 via
`--scope-rows`; latest selector rows are separate, caller-ordered. Selector/path
labels use debug escaping to keep each output line intact.

Capture is enabled by csv OR view. Port/capacity require at least one; rows require
view; all Scope options conflict with list-outputs. CSV+view allowed, MIDI-only
view allowed. Default capacity100000 when enabled, preserving transactional
validation while stopped. Scope file open and config still precede start/connect.
After stop/cleanup, attempt both requested outputs and flush each, even after
playback failure or failure of the other output. Report primary failure first,
then CSV and view errors in order. An output error cannot turn playback failure
into success. New view errors are recoverable Results, not panics or suppressed
stderr writes. Existing diagnostic/summary policy remains otherwise unchanged.

No commands to reset the running rack are added. Documentation differentiates
clear_scope (only records/overwrite count), stop/resume (keeps phases/time) and a
new process/rack (fresh seeded run). CSV export does not reset capture. Reuse the
existing API and accepted lifecycle proof, with a small compile-checked usage
example in the renderer tests if helpful. Full live editing is Arc07.

## Rows, proof and return

B-01 terminal semantics/record oracles; B-02 CLI compatibility/preflight;
B-03 post-play failure/immutable lifecycle; B-04 usable recipes/native proof.
Native example tests in default/no-default, Scope/LFO/MIDI regression checks,
make test/lint, affected strict all-targets Clippy, strict docs and example build.
Fresh command comparisons and documented bounded view/CSV prove the product;
no new validator or mandatory live endpoint witness. Four rows remain open
until CRC independently accepts. Arc11 composition returns to CDC afterward;
no arc/project acceptance, merge, push or release in this assignment.

Supporting durable artifacts: none expected for routine checks. Any necessary
supporting output belongs in this slice's artifacts/, with explicitly named
commit paths. Canonical CC closing-report.md holds intake/results/row walk and
artifact inventory. Prior Scope/LFO evidence stays at its historical source.

## Author readiness

Source6c56e84 inspected: CLI prepare_scope/finish_scope, offline/live orchestration,
playback::play/drain_generated/Session, ScopeCapture accessors/CSV/record types,
SampleTime accessors and existing tests. Iterator exposes only Iterator, so tail
rendering uses forward scans, not assumed len/rev APIs. No library changes needed.
Guide07/template applied, Rust 2.0.0 CLI-05 MUST, CLI-27 MUST, CLI-21 SHOULD,
EH-01 MUST and AP-33 AVOID applied in prompt. Shapes/examples are illustrative,
not compiled claims. Private rendering and generic finalization expose writer
failures to tests while actual runners/fixtures establish integration. Bounds,
empty capture, failure ordering and stream compatibility are decided. Four-row
mapping is discriminating; required reading is a bounded packet plus exact
source/guide sections with room for implementation/recovery. Six readiness
checks pass; no unresolved consequential decision.

## Assignment history

| Date | Prompt | Predecessor / reason | Disposition |
| --- | --- | --- | --- |
| 2026-10-11 | arc11-observation-workbench/slice03-workbench-integration/cc-prompt.md | accepted Scope6c56e84 / closure `819cecd`; approved terminal increment | Active; CC implementation then CRC review |

## Version history

- v1.0 — 2026-10-11: Opened terminal inspection on accepted capture API and sole
  runner; local output/CLI/failure details resolved within approved Arc11 design.
