# CC correction — make the clear/continue recipe executable

2026-10-11, Sofie/CRC. Three contributors, Expedited Mode. Execute
`arc11-observation-workbench/slice03-workbench-integration/cc-prompt-iteration01.md`,
then return to CRC through Duncan. Predecessor issued cc-prompt.md is preserved.
Only B-04/F-01 remains open; B-01–B-03 are independently accepted at `22500fa`.
Deliver the documentation repair and its focused executable example test.

## Baseline, required reading and authority

S=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/codex-oxurack-module-behavior-installation`,
branch codex/oxurack-module-behavior-installation, clean final source
`22500fae234d5dbba599c7683f8893aabe70f47b`; preceding source f0b0453.
P=`/Users/oubiwann/lab/oxur/oxurack/.worktrees/planning/project01-mvp`, branch planning;
CC evidencefcbe6b8 plus this issued review/correction. K=
`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills`.
Verify assignment/heads/diffs; reconcile material drift before editing.

Read complete required extents; recover any truncated output. Record state,
loaded extents and concise source-cited readback in a new dated iteration01
section of existing closing-report.md before dependent edits. Explain how a
100-sample batch can latch backpressure, drain borrow/copy sequencing and why
clear continues rather than reseeds. Continue without another permission once
preflight is coherent. Earlier completion does not cancel this assignment.

| Order | Paths | Required extent / why |
| --- | --- | --- |
| 1 | This prompt, sibling slice-plan.md, ledger.md and crc-verification.md | Required-full; current assignment, accepted rows, F-01 and retained evidence |
| 2 | P/cdc-directive09.md; P/arc11-observation-workbench/arc-plan.md | Required-full; approved ownership/third increment/CDC gate |
| 3 | P/cdc-directive08.md | Required-section “Checks and correction policy from Slice03 onward”; affected checks/retention |
| 4 | S/CLAUDE.md; S/crates/oxurack/Cargo.toml | Required-full; conventions/available dependencies |
| 5 | S/README.md | Required-section Scope commands, clear/continue snippet and adjacent lifecycle explanation |
| 6 | S/crates/oxurack/src/lib.rs | Required-section start/stop/sample_time/scope methods, drain_midi and advance_samples (including full docs/body); admission and exclusive borrow |
| 7 | S/crates/oxurack/examples/turing-machine-midi.rs | Required-section prepare_rack and current inline test module; existing helper/fixture/test conventions. Production code protected |
| 8 | S/crates/oxurack/examples/patches/turing-machine-lfo.ron; S/crates/oxurack-mod-midiout/src/lib.rs | Fixture required-full; MIDI file required-section MidiOutputEvent, its message type and public buffer/fault methods used by tests |
| 9 | K/rust-guidelines/SKILL.md; K/testing/SKILL.md | Required-full, active domain/testing routes |
| 10 | K/rust-guidelines/guides/03-error-handling.md EH-01; guides/11-anti-patterns.md AP-33 | Required-section; paths relative to rust-guidelines. Propagate errors, do not suppress backpressure |

The initial prompt's terminal/CLI/failure behavior remains binding and unchanged;
its “Terminal semantics and shape” and “CLI and output lifecycle” are reference
contracts if needed, not new implementation scope. No full historical packet,
new data query, evidence tool or intake validator needed. Conditional: load a
newly encountered governing API before use. Equivalent installed K is allowed.

## Finding and applied engineering

CRC verification F-01 independently ran the exact public recipe on a configured
Manual rack after 200 normally drained samples of the accepted LFO fixture. The
next100 batch stops at252 with OutputBackpressure(completed52, pending1).
At sample251 the rack produces90 3E 15. Postponing drain to after the batch fails
admission on the following sample. Starting at10000 happened to pass100 samples;
use200, not a silent interval, for the discriminating test.

Binding: preserve the existing single-sample admission/drain and clear semantics.
Rust EH-01 MUST/AP-33 AVOID: README uses ? for real errors and the test requires
successful completion/no fault, rather than discarding or treating Err as success.
Exclusive drain ends before record_scope_midi; collect the per-sample batch first.
No production bug fix, weakened backpressure guard, reset API or lifetime change.

## Repair and test recipe

1. README clear/continue block: explicitly show a Manual, already configured
   stopped rack continuing100 samples. Replace batch advance with this loop;
   include stop afterward and explain that it records generated events, while
   live sending uses the existing runner. The public API loop below was compiled
   and exercised by CRC in a temporary probe on22500fa; it is an implementation
   example, not a claim that this README/test repair has been executed yet:

```rust
rack.stop();
rack.clear_scope(); // Keep time, seed and module phase.
rack.start();
for _ in 0..100 {
    if rack.advance_samples(1)? == 0 {
        break;
    }
    let events: Vec<_> = rack.drain_midi().collect();
    for event in events {
        rack.record_scope_midi(event, oxurack::MidiDelivery::Generated);
    }
}
rack.stop();
// Inspect/export the retained continuation. A fresh rack/process reseeds.
```

2. Add one inline test in the existing parent example file; production portions
   unchanged. Load protected LFO fixture via prepare_rack (already Manual and
   stopped), configure lfo_main.bipolar_out capacity1000. Run200 samples with
   normal advance1/drain/copy. Stop, clear; assert completed200, capture enabled,
   records0/overwritten0. Start and execute the README loop100 with actual facade
   APIs, then stop. Assert completed 300, midi_output_error=None, buffer fully
   drained. Retained signals are exactly100 at indices200..299; exactly one
   Generated MIDI event at251 with bytes90 3E 15; no Sent; total101 records and
   overwritten0. This tests continuation, not a fresh-start silent interval.
   Use ScopeRecord matches; public enums non-exhaustive require wildcard arms.
   Bind fixture path as Path, not a string argument to load_patch. Do not call
   private is_running or expose a public world accessor. Existing actual-value
   renderer/lifecycle tests continue to establish phase continuity.
3. Keep README and executed loop synchronized by direct review, without adding
   a doc extractor/checker. The test exercises the new recipe on unchanged,
   already correct production code. The predecessor README loop is the negative
   control recorded by CRC; do not invent a claim that production changed.

Private test naming/local assertions may vary while preserving these exact
observations. Allowed source paths only README.md and the existing parent
example file, test region only. No workbench/playback/Scope/facade/core/fixture,
manifest/lock or prior test repair/visibility change. If the oracle differs,
report actual evidence before revising the fixture/behavior to make it pass.

## Checks and retention

Run from S:

```bash
cargo test --locked -p oxurack --example turing-machine-midi
cargo test --locked -p oxurack --no-default-features --example turing-machine-midi
make lint
cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings
git diff --check
```

New test must pass both modes and all existing example tests remain passing.
Check committed delta has only README recipe/context and new inline test.
B-01–B-03 plus native workspace/Scope/LFO/MIDI/build/rustdoc/fixture receipts in
crc-verification.md at `22500fa`/2026-10-11 are proposed for retention: no production,
fixture or dependency change can affect them. CRC assesses actual delta on
return. Keep date/revision, distinguish retained from executed results. Unexpected
production drift invalidates this retention and requires affected reruns/review.
No make check, freshness/coverage or live endpoint witness. Keep failed attempts,
three RT ignores and restricted CoreMIDI evidence at their original receipts.

## Commit and return

Explicit source commit:

```bash
git commit --only -m 'docs(example): drain each sample in Scope continuation recipe' -- \
  README.md crates/oxurack/examples/turing-machine-midi.rs
```

Separate planning commit exactly these existing files:

```bash
git commit --only -m 'docs(plan): return Scope continuation correction evidence' -- \
  project01-mvp/arc11-observation-workbench/slice03-workbench-integration/closing-report.md \
  project01-mvp/arc11-observation-workbench/slice03-workbench-integration/ledger.md
```

Preserve all previous report
sections and add iteration01 intake, repair/test/native results, four-row walk,
source/planning inventories and Bubble-up (no structural change expected).
B-01–B-03 remain done with prior CRC pointers; B-04 remains open with CC proposed-
done until CRC review. Preserve issued prompts and plan; CRC owns assignment
changes/acceptance. No artifacts expected; necessary durable supporting outputs
belong in sibling artifacts/ with explicitly named paths. Return exact commits,
both tree states, failures/unrun checks and retention proposal through Duncan.
No merge/push/release or arc closure claim.
