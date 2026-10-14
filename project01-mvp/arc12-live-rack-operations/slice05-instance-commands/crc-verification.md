# Slice05 independent CRC review

2026-10-13. Sofie/CRC, Three-Contributor Workflow, Expedited Mode.
Reviewed source `7404fe227e561746dbda76543b59893feeacfe15`, parent
`ec85ebe99c09b8a2ea64d55da0346af4ae317a67`, and CC planning return
`a3f350e66833196edba0c642c88c677ca6114406`. Initial assignment `cc-prompt.md`,
SHA256 `a1ab69c378b51f8b9b9cd003cac59f1a314c9934d39e62d1205337135633db94`,
is unchanged from issuance `4b8119a1ebb2cb8ddf7dfebbdd816925505f8177`.
Both worktrees clean at intake; protected main remains a7ce6e6.

**Verdict: IC-01/IC-02/IC-03/IC-05 done; IC-04/IC-06 open. Slice05 stays open.**
Two implementation mismatches were independently reproduced: the required queue
API does not compile, and an existing sibling file defeats the example's promised
missing-load setup. Final cleanup/documentation and the assigned reset-failure
observation also need completion. No architecture or acceptance amendment.
Iteration01 is issued; zero corrective passes executed, first correction1/5.

## Intake, scope and source review

CC records all 19 reading items, contiguous recovery, dependency-version checks,
applied Rust guidance and a source-cited contract readback. This is attestation;
it does not establish understanding. Its description of opaque command IDs does
not match the actual public tuple field. The complete source delta is 14 paths:
README.md; core error.rs/lib.rs/module.rs/topology.rs; graph lib.rs; MIDI-out
lib.rs; facade lib.rs/lifecycle.rs/persistence.rs/commands.rs/handles.rs;
tests/instance_commands.rs; examples/rack-session.rs. All are in the initial
allowlist. Planning delta is exactly closing-report.md and ledger.md. No fixture,
manifest, dependency, lockfile, standalone algorithm, RT or main change.

CRC inspected the full delta, complete new commands/handles/example/integration
test bodies, and their actual constructor/installation/commit, ownership,
silence/accounting, typed-event and metadata seams. Required framework/PM,
verification/testing, guide07/template and relevant Rust API/EH/TD sections were
applied. Framework2.6.0/Rust-guidelines2.0.0 remain installed. No source repair by
CRC; all extra witness code and logs are transient outside the repository.

## Reproduced behavior and six-row walk

| Row | CRC status | Independent evidence and limits |
| --- | --- | --- |
| IC-01 | done | Native core ordered A→B→C/self/parallel retirement observes all four disconnects before hook, one hook then one ModuleRemoved; index/membership, owned-grandchild cascade and nested-active rejection pass. Direct graph incoming/outgoing/self/parallel mappings and newer same-name mapping tests pass independently of cable disconnects. Facade removal admission, survivor Scope/control, selected Scope disable, loaded-empty header and first-fault tests pass in both modes. Inspection confirms stopped/drained preflight precedes selected append and infallible commit, no survivor/time/queue rebuild. |
| IC-02 | done | Native wrong-kind/same-kind-different-Rust-type, cross-rack, removal, same-name reload, restart, failed-load preservation and bound-port/knob cases pass. Additional compiled external witness successfully uses bound unpatch/patch and knob read/write through real modules. Private Arc tokens rotate only on candidate commit, retire only selected name, and precede shared helpers. No public Entity/Arc; manual Clone/Debug and invariant marker verified. Documentation completion is tracked under IC-06, not a finding against this reproduced identity behavior. |
| IC-03 | done | Additional compiled external witness executes all eight variants; real typed ParameterChanged/LabelChanged/CableConnected/CableDisconnected include Command and DirectApi, loader parameter initialization remains PatchLoad. Cable graph changes are visible at sample0. Command save/load and restart retain global133 while Clock stays120; command PatchLoaded origin includes the path. Real running sample300 Panic emits exact NoteOff62/CC123/CC120 without advancing time. Invalid BPM and low-level four unsupported variants pass native tests. |
| IC-04 | open | FIFO/App-swap continuation, 64/65 admission, checked exhaustion, queued Panic tail cancellation, direct cancellation capacity and once-only collection pass. F-01: required public types/opacity differ; external consumer fails to compile while forged ID compiles. F-03: existing failed-reset test proves eventual command completion, not the assigned immediate pending-ID/command/fault/event/time preservation observation. |
| IC-05 | done | Actual sample300 sink removal emits exactly [(300,80 3E 00),(300,B0 7B 00),(300,B0 78 00)] and preserves time. Zero-sink drain, first fault, selected other-dirty-sink preservation, channel reuse, logical count/capacity and unchanged fixed storage pass. Existing 128-event peak/129th rejection/reset tests rerun. No endpoint or hardware silence claim. |
| IC-06 | open | All native gates, original fixture hashes and 2000-sample session/fresh/no-graph equality reproduce. F-02: predictable supposedly missing sibling can be an existing valid patch and aborts the example; final lifetime cleanup is never requested/recorded. F-04: new fallible public APIs lack required error/lifetime/cancellation rustdoc; ModuleRemoved still says no producer exists. These are bounded corrections, not release or hardware gates. |

Six opening rows accounted for: four done, two open, no deferrals/no-ops or
criterion softening. Accepted behavior is pinned to this source/date; a later
correction may retain it only after checking the actual delta and dependencies.

## Fresh native gates

All commands run from the assigned source worktree at7404fe2:

| Command | Result |
| --- | --- |
| cargo test --locked -p oxurack-core | exit0;426 unit,14 integration,2 doctests |
| cargo test --locked -p oxurack-mod-midiout | exit0;10 unit |
| cargo test --locked -p oxurack-graph | exit0;68 unit,1 integration |
| cargo test --locked -p oxurack --all-targets | exit0;63 facade unit,14 instance-command,11 existing lifecycle; all other integrations/examples, including2 session tests, pass |
| cargo test --locked -p oxurack --no-default-features --all-targets | exit0;61 facade unit and corresponding integration/example matrix pass |
| make test, restricted | exit2; RT146 passed,4 CoreMIDI initialization failures,3 existing ignores |
| unchanged make test, host access | exit0; RT150 passed,0 failed,3 existing ignores; workspace and doctests pass |
| make lint | exit0 |
| cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack-graph -p oxurack --all-features --all-targets -- -D warnings | exit0 |
| RUSTDOCFLAGS='-D warnings' make docs | exit0; does not enforce existence of # Errors sections |
| cargo build --locked -p oxurack --examples | exit0 |
| cargo build --locked -p oxurack --no-default-features --examples | exit0 |
| git diff --check | exit0 |

Restricted failures are midi_io::tests::{test_list_input_ports,
test_list_output_ports,test_open_input_nonexistent_returns_error,
test_open_nonexistent_port_returns_error}: MIDI support could not be initialized.
Host retry does not erase the failed attempt. Existing block0.1.6 advisory and
no-default cables.rs unused_mut warning remain unsuppressed.

Transient native logs: tmpdir `oxurack-crc-slice05-04_yls3q`,01–12.log and
results.jsonl; host `/tmp/oxurack-crc-7404fe2-host-make-test.log`. These paths
are conveniences, not durable acceptance dependencies.

## Musical recipe and targeted witnesses

CRC ran the issued four-command session/fresh/no-graph recipe with fresh saved
paths and2000 samples. All exit0; each trace has14 rows, SHA256
`40965cc5f2031cbd3b26bd7275e0012359800c959d9c08f3dd5139f3707a76a2`.
The default/no-graph session reports4014 mixed Scope records, global133/Clock120.
Original MIDI2000:13 rows,
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`;
LFO10000:70 rows,
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
Temporary session directory: `oxurack-crc-slice05-session-0h4agngd`.

F-01 external compile witnesses used fresh Cargo JSON artifact paths after
`cargo build --locked -p oxurack --example rack-session --message-format=json`,
then rustc --edition=2024 with those exact --extern artifacts and dependency path:

```rust
use oxurack::{CommandOutcome, CommandResult}; // E0432: no CommandOutcome
// Required: Vec<CommandOutcome>, outcome.result, CommandResult::Completed(...).
// Actual CommandResult is a struct, so Completed also gives E0599.
```

The separate `fn main() { let _forged = oxurack::CommandId(42); }` compiles exit0;
required opacity would reject it. No stale prebuilt facade was used. The real
behavior witness then compiles/runs exit0 and establishes IC-02/IC-03 above.
Its initial graph assertion incorrectly assumed five cables; the actual fixture
has nine. CRC corrected the witness to compare the pre-command count with +1
and back to baseline, then reran successfully. No source/oracle was changed to
fit a product failure. Temporary API directory: `oxurack-crc-slice05-api-lj6lwe_y`.

F-02 collision reproduction: in a fresh temp directory copy the immutable LFO
fixture to `collision.guaranteed-missing.ron`; run the session with --save
`collision.ron`. Exit1, no trace. Results are first Save Ok, second Load Ok,
third Load Err(PendingMidi{count:3}), remaining edits/save Ok. The guessed path
therefore changes the owner and is not a guaranteed expected failure.

Separate --samples300 session exits0 and stdout contains exactly
`251|90 3E 15`. Stderr records initial output and carried cleanup from the first
lifetime, then reports replay equal. There is no final/reference cleanup for
the newly held final-lifetime note: play only stops and run_with_output returns.
This is a missing example cleanup step, not a claim that hardware hung; no
endpoint was opened. Complete the initial prompt's final-cleanup status recipe
through the existing reset/drain, keeping ordinary stdout unchanged.

## Required correction

- **F-01 — IC-04, commands.rs:11–26/78 and lib.rs exports.** Restore opaque
  CommandId, non-exhaustive CommandResult enum and CommandOutcome{id,result},
  exact process_commands return. Replace private status+retained-command slots
  with Pending/Completed enum and reserve64 as specified. Preserve working
  pop-before-execute behavior, cancellation capacity, checked IDs and App ownership.
- **F-02 — IC-06, rack-session.rs:140/187–203.** Reserve an actually fresh private
  temporary missing-input directory instead of deriving a user-adjacent filename;
  never delete/overwrite an existing sibling. Emit/drain and phase-label final
  cleanup for both subject/reference via reset after ordinary replay comparison.
  Add discriminating existing-sibling and held-note300 runner tests.
- **F-03 — IC-04, commands.rs:227–245.** Extend the existing injected-accounting
  failed-reset test with owned observations immediately before/after Err, before
  process_commands: exact pending IDs/commands and next counter, sample/time,
  fault, pending MIDI, Scope config/records and typed messages. Do not snapshot
  the deliberately invalid World. Preserve eventual ordinary completion proof.
  No additional product defect was reproduced by this evidence gap.
- **F-04 — IC-06, new facade APIs and event.rs:142.** Document new public handle,
  reference, command/BPM/removal methods, named recoverable errors and relevant
  invalidation/admission/cancellation/into_app behavior. Correct ModuleRemoved's
  producer documentation. README supplies the actual default/no-graph/fresh recipe.
  Strict rustdoc passing alone does not fulfill EH-14.

## Bubble-up, retention and artifact inventory

The implementation delivers most of the assigned removal/identity/command
capability. Native musical behavior works; required public API and example
failure/cleanup/documentation details are incomplete. Repair within approved
CDC12, with initial prompt preserved. No structural arc/architecture change is
needed; routine plans/status record four done/two open and iteration01.

Arc12 R-01–R-04 remain open; no final-child close, composition assembly or CDC
acceptance yet. Arc05 A-05-01 removal/handles/queue obligations are partly
reproduced here but not fully reconciled; A-05-02 awaits five-child reconciliation.
Arc07 transport and Arc13 typed vocabulary retain their separate re-entry owners.
ProjectP-01–P-06/release gates remain open; P-07 and accepted ancestors unchanged.
No merge, push, endpoint or new arc authority follows.

Supporting durable artifacts: none, checked against both diffs. Canonical
reports/prompts stay in the slice root; ephemeral witnesses are not new tooling
deliverables. CC's report stays unedited. What worked: real-module retirement,
direct graph messages and musical replay supplied useful proof; an external
consumer and one existing-file negative case exposed gaps missed by green gates.

Planning handoff check: six current rows (four done/two open), exact eight
planning paths, active iteration01 identity and unchanged initial prompt digest
verified. No pre-staged files, source/main pins clean and unchanged; planning
git diff --check passes. The original CC closing report is unedited.
