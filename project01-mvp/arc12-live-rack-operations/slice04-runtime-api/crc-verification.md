# Slice04 — independent CRC review

2026-10-12. Sofie/CRC. Three contributors, Expedited Mode.

## Candidate, authority and disposition

Reviewed source `f6fdeb513331a8d8962876bdf4766cedfce63f59`, against accepted
predecessor `deafdd228fb44720878c700f8ff62b2a59ddbad6`; CC planning return
`622e9948c443748bddbdc9c944b50aa5398fc1b1`. Both worktrees were clean at intake.
Main remains `a7ce6e61536ca96b6aeb2227ee06e430ed8df162`.
The issued `cc-prompt.md` (SHA-256
`285eecac303b5aa849f49072b1c56d940af20f1bb6c8b2cb9ee0abb97d14fd60`),
directive12 D1/D2/D4 and its narrow D3 honesty exception govern acceptance.

**Return for one collected corrective iteration. Six LF rows remain open.**
Native gates and the actual musical recipe reproduce successfully. The remaining
acceptance gaps concern explicitly assigned interaction tests, not a reproduced
failure of the lifetime implementation. One minor diagnostic defect is observed.
No architecture amendment, production lifecycle repair, Slice05 opening, or
ancestor acceptance is claimed. The next assignment is
[cc-prompt-iteration01.md](cc-prompt-iteration01.md), correction1/5.

## Source and intake review

CRC inspected the entire eleven-path delta, new lifecycle implementation, MIDI
buffer/consumer/cleanup, all eight lifecycle integration tests, affected inline
fixtures and the actual example. All eleven changed paths are inside the issued
fourteen-path allowlist. `oxurack-core/src/patch.rs` changes documentation only;
its loader/validator bodies, fixtures, manifests, lockfile, graph production,
standalone engines and native endpoint code are unchanged.

The isolated candidate, ordered installation recipes, typed stopped/pending
checks, complete cleanup staging and sole caller-owned drain fit CDC12. Preparing
read-only old cleanup before candidate construction is a tactical ordering
difference from the sketch; it does not mutate the old owner and no defect is
established from that difference. Ordinary returned errors all precede App swap.
The panic/global-effect exclusions are documented rather than disguised as rollback.

CC's intake/readback is attestation in its closing report. It captures the main
design, but its return overstates test coverage for LF-02–LF-05. In particular,
the single installation test observes one successful load, and the fault test
injects a synthetic fault into an unplayed rack; neither establishes the full
reported interaction matrix. Passing gates do not fill those coverage gaps.

## Fresh independent commands

All commands ran in the assigned source worktree at f6fdeb5, without source edits.
Transient complete logs: `/tmp/oxurack-crc-slice04-18qxy8th/`; these are convenient
local receipts, not a durable dependency for acceptance. The outcomes below and
source tests are the retained review record. Gates were run sequentially.

| Command | Independent result |
| --- | --- |
| `cargo test --locked -p oxurack-core` | exit0;424 unit,5 full-cycle,9 seeded-spawn,2 doctests passed |
| `cargo test --locked -p oxurack-mod-midiout` | exit0;7 unit passed |
| `cargo test --locked -p oxurack --all-targets` | exit0;48 facade unit,8 lifecycle integration, all other integration/example targets passed |
| `cargo test --locked -p oxurack --no-default-features --all-targets` | exit0;46 facade unit,8 lifecycle integration, all other integration/example targets passed |
| `make test`, restricted | exit2; RT146 passed,4 CoreMIDI initialization failures,3 pre-existing ignores; failure retained |
| unchanged `make test`, host access | exit0; RT150 passed,0 failed,3 existing ignores; workspace suites passed |
| `make lint` | exit0 |
| `cargo clippy --locked -p oxurack-core -p oxurack-mod-midiout -p oxurack --all-features --all-targets -- -D warnings` | exit0 |
| `RUSTDOCFLAGS='-D warnings' make docs` | exit0 |
| `cargo build --locked -p oxurack --example save-live-patch --example turing-machine-midi` | exit0 |
| `cargo build --locked -p oxurack --no-default-features --example save-live-patch` | exit0 |
| `git diff --check` | exit0 |

The four restricted failures are `midi_io::tests::{test_list_input_ports,
test_open_input_nonexistent_returns_error,test_list_output_ports,
test_open_nonexistent_port_returns_error}` in oxurack-rt. Their host retry is not
relabeled as a restricted pass. Existing block0.1.6 future-incompatibility and
no-default unused_mut advisory remain visible; no warnings/tests were suppressed.
The 26 example tests belong to `turing-machine-midi`, not `save-live-patch`;
the latter currently executes zero tests in both facade all-target matrices.

## Fresh musical and CLI observations

Executed the issued section7 cargo-run commands with transient saves under the
receipt directory. The default lifecycle, no-default lifecycle and a separate
fresh `turing-machine-midi --samples 10000` run of the saved patch all exited0
and matched byte-for-byte:72 rows, SHA-256
`20d86b836d7540a2ac6bf725f5045dfe6304fa7a027d703f22e1fe93f27613ba`.
Lifecycle stderr reported3 retired cleanup events,20,000 scalar Scope records,
and2 final silence events; its internal zero-overwrite check passed. Source
inspection confirms explicit copied-MIDI recording. This command does not
independently inspect every retained MIDI record or port/sample pairing.

A separate fresh saved-patch run with `--scope-csv`, both named `--scope-port`
selectors and `--scope-capacity 21000` exited0. CRC inspected its complete CSV:
20,000 signal and72 midi_generated rows, zero overwrites, write_probability0 at
sample0 then.5 at every sample1..9999. This establishes the fresh-reference
observation; it does not substitute for checking the lifecycle recipe's own
capture. The new prompt uses this source-supported publication boundary.

The ordinary save example exited0 with the frequency edit/save lines and
`completed_samples=300`. Original MIDI2000 remained13 rows with SHA-256
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`;
original LFO10000 remained70 rows with SHA-256
`988563371b7473412325a22d6babb68cba1468b7719bef3512fafa431e8a687c`.
Missing patch, malformed temporary RON, and nonexistent save parent each exited1
with empty stdout in ordinary and lifecycle modes (six negative invocations).
No MIDI endpoint was listed, connected or played.

## Findings and required correction

All source anchors below are at f6fdeb5. These are serious proof gaps (F-01–F-05),
with a minor diagnostic issue in F-06; they do not establish latent production
failures. The prescribed iteration reuses current tests rather than a new verifier.

1. **F-01 / LF-01 — full failure and membership observations are incomplete.**
   `tests/lifecycle.rs:375` projects aggregate counts and handle names, not exact
   port/cable/index/graph membership. The late-failure test at:283 checks Scope
   count, then musical continuation, but never compares scalar captures. Its
   message check after1000 more samples compares only ModuleAdded lengths, not
   payload preservation or other typed queues. Observe payloads immediately after Err, complete
   normalized membership and Scope/control projections, and prove a subsequent
   channel1 load after that same late failure.
2. **F-02 / LF-02 — per-App replay is tested only across load.**
   `install_recipes_keep_order_reinstall_and_collision_contract` at:354 consumes
   the rack after one load/tick. It does not cover restart/unload/reload, live
   current-App reinstall/collision state, accessors after each lifetime, or
   execution counts after reconstruction. Reuse ordered hooks and the existing
   LiveBehaviorCounter fixture with inline private access.
3. **F-03 / LF-03 — low-level staging tests do not prove actual reset emission.**
   The peak test at MIDI-out:991 invokes `append_silence` directly, leaving dirty
   state uncleared; it checks length, not the80-event prefix/48 cleanup bytes and
   boundaries or second reset. Channel/released tests hand-set dirty state, so
   they cannot catch omission of production CC-only/NoteOff dirty marking.
   The facade test at lifecycle integration:169 tests presence of controls after
   draining all normal output, not the literal unread NoteOn→cleanup sequence.
   Exercise actual normal production and silence/reset, preserving clocks,
   Scope, fault and voice state, plus an invariant failure distinct from room.
4. **F-04 / LF-04 — admission/recovery/restart/Live matrix remains unproved.**
   At integration:144/:224 only load has running/pending rejection assertions.
   The restart check at:169 reads one parameter but does not delete the original
   file or compare fresh edited seeded replay. Inline lifecycle:180 injects a
   synthetic fault in an unplayed rack and recovers only via load. No new test
   exercises Live lifetime reconstruction, real-fault reset/recovery across all
   three operations, empty-cleanup/no-op fault preservation, or actual musical
   stop/reset continuation. Existing pre-lifecycle driver tests do not cover App
   replacement. Fill the explicit initial-prompt section5 matrix.
5. **F-05 / LF-05 — candidate output and lifetime message boundary are untested.**
   `validate_candidate` at lifecycle:140 has no rejecting-output/fault witness
   through a facade load. A post-continuation ModuleAdded length comparison
   does not establish fresh message payloads/provenance or absence of retired
   removal messages. Inspect all existing notification types immediately before
   and after success/failure. Check PatchLoaded-last by the unchanged loader's
   write ordering; separate Bevy typed queues are not a global event sequence.
6. **F-06 / LF-06 — executable recipe tests and complete capture checks missing.**
   `save-live-patch.rs` has no tests; `edited_saved_lifetime_replay...` at:439 is
   a parallel recipe. Its capture helper discards MIDI records and accepts any
   one effective-probability observation. Test actual parsed mode/recipe and
   sample/selection coverage, copied MIDI and settled probability. The actual
   CLI says `retired at sample 0` (example:116) by reading the new lifetime after
   unload; the retired rack completed300. Preserve that boundary before unload
   and report300 accurately, separately checking fresh sample0.

## Row walk, inventory and bubble-up

| Row | Independently reproduced part | Remaining owner / disposition |
| --- | --- | --- |
| LF-01 | Same-channel replacement, valid[1,0], rejected[1,1], late spawner Err and continued MIDI; source isolation | F-01 complete owner projections/control observations; open |
| LF-02 | One-load hook order, same-type/collision initial checks, two systems once in one tick | F-02 multi-lifetime/current-state/accessor proof; open |
| LF-03 | Reservation128, channel ordering, room-failure retry, cleanup lengths/idempotence in partial fixtures | F-03 real reset/dirty/maximal bytes/timestamps/phase/fault/atomic invariant proof; open |
| LF-04 | Load-only busy precedence, current parameter after restart, getterless Err, basic unload/reload, synthetic fault recovery | F-04 complete operations/fresh reconstruction/real fault/Live/continuation; open |
| LF-05 | Typed variants, exact core unsupported errors, documented hook/loader boundary; unchanged loader body | F-05 candidate rejection and complete immediate notifications; open |
| LF-06 | Both actual CLI modes/fresh replay, ordinary mode, six failures, original pins and every native gate | F-06 executable automated/capture checks and diagnostic; open |

Artifact inventory: original prompt preserved; slice-plan/ledger remain canonical;
CC closing report retained unchanged by CRC; new CRC record and iteration01 are
the only new slice artifacts. No permanent receipt forest/new checker. No silent
drop: all six rows and original section5 obligations map to this correction.
Arc05 A-05-01/02 remain deferred: configuration/metadata/cables are accepted
in01–03; lifecycle is awaiting this review; removal/handles/commands/global BPM
remain05, separate transport07 and catalog forms13. R-01–R-04 and project/release
gates stay open. Findings remain within the approved design, so no new CDC design
return is needed. Slice05 remains unopened until Slice04 is independently accepted.
