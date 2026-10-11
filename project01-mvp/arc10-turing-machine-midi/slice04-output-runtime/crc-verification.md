# Slice04 CRC verification — accepted

2026-10-10, Sofie/CRC; Three-Contributor Workflow, Expedited Mode.
Independent of CC implementation; no CRC source changes.
Current verdict (2026-10-11): O-01–O-05 done; Slice04 closed at unchanged source
`65522f1`. Operator supplies the musical witness; CRC reproduces trace comparisons.
Earlier pending-witness sections below retain their original dated context.
Assignment: `arc10-turing-machine-midi/slice04-output-runtime/cc-prompt.md`.
Source: `65522f116a12a9ac96d35cfe25ed9c184f5bddcb`, clean on the assigned
implementation branch; includes `829feb99279b11b3f46a11e513aba577eacd6fd1`.
Planning intake: clean `514d599c21dd25c07ee24e18d4e0850ac9af354d`.
Authority: directive08, Arc10 design v1.2 §§4–5 and the issued prompt.
Framework 2.6.0 and Rust 2.0.0 applied to review.

## Initial software verdict and five-row walk — 2026-10-10

Five opening rows and five reviewed rows: **O-01–O-04 done, reproduced;
O-05 open, awaiting Operator witness**. No blocking software finding or
corrective iteration. Slice04 and Arc10 remain open; no Arc11 advancement.

| Row | Independent evidence and disposition |
| --- | --- |
| O-01 | CLI conflicts and exact unique-name/duplicate/index tests pass. Built executable lists actual host ports; impossible index exits1 with empty stdout. Missing/malformed patches fail before playback; malformed-live exits before MIDI setup. Inspected stopped preparation, single output object/port snapshot, fallible naming/connection and start-after-connect. Done. |
| O-02 | Actual loaded fast Clock→TM→sink fixture through the production `play` path matches the independently driven seeded engine oracle for 24 samples; completed count24, empty drain, stopped rack. On-time and late deadline values pass. Inspected ordered same-thread sends and checked/rebased monotonic deadlines. Done. |
| O-03 | Failed off retains active note; successful off followed by failed on does not acquire the new note; failed first on marks used channel. Literal ascending cleanup continues after failure and repeated finish does not resend. Actual runner tests retain primary plus cleanup failures, stop/discard, cancellation during successful send and fresh-session replay. Inspected successful-send state commit before fallible trace write, cancellation checks within drains, and unconditional stop/discard/finish on returned errors. Done. |
| O-04 | Both example and integration feature configurations, native gates and fresh compatibility/CLI controls below pass. Authorized source/dependency scope, README and original fixture checked. Done. |
| O-05 | Refreshed host inventory exists, but CRC did not select/connect an endpoint. No receiver routing, musical or physical stop observation is available. Open; Duncan must select the destination and supply the design§5 witness. This is not endpoint unavailability or a deferral. |

## Fresh commands and results

All executed from `.worktrees/codex-oxurack-module-behavior-installation` at the
source above; required gates exit0:

- `cargo test --locked -p oxurack --example turing-machine-midi`: 11 passed.
- Same command with `--no-default-features`: 11 passed.
- `cargo test --locked -p oxurack --test midi_output`: 2 passed.
- Same command with `--no-default-features`: 2 passed.
- Host `make test`: full workspace passed, including facade28, core438,
  rack-enabled TM134 and RT150; three existing RT timing/scheduling ignores.
- `make lint`: workspace Clippy and formatting passed.
- `cargo clippy --locked -p oxurack --all-features --all-targets -- -D warnings`: passed.
- `RUSTDOCFLAGS='-D warnings' make docs`: passed.
- `cargo build --locked -p oxurack --example turing-machine-midi`: passed.
- `git diff --check`: passed.

Two fresh built-executable processes with the original RON and default2,000
samples return0, emit identical177-byte/13-row traces and match accepted SHA-256
`98e109812e40f52ebdd9ae419b8a04938eee84608462158ee10a068760f5dc7b`.
Samples0 returns0 with empty stdout. Missing and malformed temporary patches
return1 with stderr and no stdout; malformed patch plus index999 also returns1
at patch parsing. Both selectors together and listing plus patch return2 with
no stdout. Host `--list-outputs` returns0 and prints `0|MIDI Bus 1` and
`1|nanoKEY2 CTRL`. Original patch plus index999 returns1, out-of-range diagnostic
and empty stdout. These inventory/negative controls open no connection.

CC's restricted CoreMIDI initialization failures remain in its unchanged report;
CRC used host execution for the relevant test gate and inventory, without
rerunning that known restriction or relabeling its failures. The three ignored
RT tests remain unverified. Existing `block0.1.6` future-incompatibility advisory
is present; no new Clippy/rustdoc failure. No hardware timing or silence guarantee.

## Scope, provenance and bubble-up

Inspected the complete seven-path source delta from accepted Slice03 `2942c59`:
Cargo.lock, README.md, umbrella Cargo.toml, example, private playback/oracle
helpers and MIDI integration test. No core/module/RT/wire production or literal
fixture change. Lock delta adds only already-locked ctrlc/midir umbrella dev
edges; no package/version changes. The oracle extraction retains the separately
driven engine and independent byte transitions; it does not call playback logic.
Inspected CC planning delta, its canonical report/ledger and all five dispositions.
Prompt hash matches `4e5be5ac8afcc1f60d01e553d90b290f45453b27a71b9c75c25d9b109d959f0e`;
CC report hash matches `783d5d831e5bac001372b1cb5bba788d1bfae8d59ea4efcd36e2455b3be2f5f0`.

Software scope delivered without an architecture or acceptance amendment.
O-05 is the disclosed remaining scope, not a silent drop or mock pass.
Parents receive routine status updates only; all Arc10/project rows stay open.
After Duncan's named-endpoint/routing, finite-playback and interruption observation,
compare successful performance rows with the same-seed offline equivalent;
then CRC can complete the child review and assemble Arc10 for CDC composition
review. Future release timing, coverage, soak and broader musical gates retain
their existing owners. No endpoint was guessed, opened or configured by CRC.

Artifact inventory: CC changed the canonical report and ledger only; CRC adds
this verification record and routine plan/status/ledger updates. Temporary
command logs and CLI controls are not durable supporting artifacts. Issued prompt
and CC report preserved. What worked: a narrow sender seam exercises the actual
loaded runner while literal cleanup cases and an independent engine oracle prove
separate boundaries without another runtime or validation framework.

## Operator runbook added — 2026-10-10

At Duncan's request, added [operator-runbook.md](artifacts/operator-runbook.md)
as a slice supporting artifact, linked from slice-plan v1.2. Checked command
flags against the actual example and fixture; executed its 5,000-sample offline
reference: exit0, 34 rows, hash `918f4883252988093f737571cbb862aa29a33b737d0db026c98a7d7fea652d17`.
Live/interrupt/restart commands are instructions for Duncan, not executed
witness evidence. No source or verdict change; O-05 remains open.

## Operator command witness — 2026-10-11; musical observation pending

Duncan supplied the runbook terminal transcript from lappy at source
`65522f116a12a9ac96d35cfe25ed9c184f5bddcb`. It reports build success, current
outputs `0|MIDI Bus 1` / `1|nanoKEY2 CTRL`, and three live connections to the
explicitly selected `MIDI Bus 1`. Finite5,000 and restarted5,000 exit0, with
comparison exit0. The longer60,000 session was interrupted with Ctrl-C, exited0
and yielded four rows matching its offline reference prefix. These actual
connection/exit observations are Operator-attested; CRC did not run live playback.

CRC read all six actual trace files from Duncan's reported temporary directory
and reproduced the file comparisons. Fresh offline executions at unchanged
source for5,000 and60,000 samples independently reproduce both reference files.

| Preserved file(s) | Rows | SHA-256 |
| --- | --- | --- |
| offline-5000.trace; live-5000.trace; restarted-5000.trace | 34 each; byte-identical | `918f4883252988093f737571cbb862aa29a33b737d0db026c98a7d7fea652d17` |
| offline-60000.trace | 533 | `0577ebaf4fbdfd0e6034d1ccb9b11b97fa2d523e03bebf30d20e7a13b839c1cb` |
| interrupted.trace; expected-interrupted-prefix.trace | 4 each; byte-identical first four reference rows | `8e2ddb1a32f8cc3f13fe75ec047fd180738e0df2f1239bc353f0324bbfd9e4b9` |

The six byte-for-byte copies are preserved in
[operator-witness-2026-10-11](artifacts/operator-witness-2026-10-11/), superseding
reliance on temporary storage for this receipt. No extra checker or manifest.
Original directory: `/tmp/oxurack-midi-witness.65522f116a12a9ac96d35cfe25ed9c184f5bddcb`.
The interrupted trace ends at `751|80 40 00`, a note-off. It does not establish
that Ctrl-C caught a sounding note or that a receiving instrument rendered or
stopped the sound. Cleanup messages are excluded from performance traces by design.

**O-05 stays open** only for the missing receiver/preset/channel/routing and
Duncan's musical/normal-stop/interrupt-stop observations. The packet does not
report those observations; neither success exits nor byte equality supplies them.
CRC requested the missing details. No command/software failure identified, no
corrective implementation iteration, and no demand to repeat successful commands
without a concrete observation gap. No unavailable-endpoint claim or deferral.
O-01–O-04 remain accepted; Slice04/Arc10 stay open and Arc11 is not advanced.

Duncan then supplied a [receiver screenshot](artifacts/operator-witness-2026-10-11/receiver-setup.png).
Visible setup: Blue Cat's PatchWork with Kontakt8 in parallel chain2,
Electric Storm Deluxe (Melody), Dreamscapes (Melody)/36 Dreamscapes selected,
sound preset Drive To Space; Bloom in the post chain with Infinity displayed.
Zebra3 is also loaded in another parallel chain; its use in the witness is not
inferred. The screenshot establishes displayed receiver configuration, not
MIDI input selection/channel, audible rendering or absence of stuck notes.
It is preserved alongside the six traces. Routing and musical/stop observations
remain requested; O-05 is still open.

## Final Operator witness and acceptance — 2026-10-11

Duncan confirmed: “it sounded amazing” and “everything played out exactly as
expected”, then specified PatchWork/Kontakt were set to omni receive. This follows
the posted finite, Ctrl-C and fresh-session commands and the request for musical,
normal-stop and interrupt-stop observations. Taken together, Duncan confirms
audible playback and expected completion/interruption/restart behavior through
MIDI Bus1 → PatchWork → Kontakt8 Electric Storm Deluxe (Melody), Dreamscapes;
Bloom is present in the post chain. Omni accepts fixture channel1. Exact input
menu settings are not visible in the image; actual routing is Operator-attested
by the named output, receiver setup, omni setting and audible result.

**O-05 done, Operator-witnessed; trace comparison reproduced by CRC.** CRC does
not relabel the auditory observation as its own reproduction. The interrupted
trace does not establish a sounding note at the instant of Ctrl-C; the witness
confirms expected stop behavior, while the actual-runner fault/cancellation tests
prove successful-send ownership and best-effort cleanup. No extra guarantee or
new gate is inferred from this first musical trial.

Final five-row disposition: O-01–O-05 done, zero deferred/no-op/open. Source
unchanged; no corrective iteration needed. Slice04 closes. Arc10 can now be
assembled for CDC's independent composition/design review; CRC does not close
that arc or issue CDC's Arc11 first slice. All project/release gates remain.
The original CC report remains in Git at planning `4f479c4`; the current report
adds a clearly attributed Operator/CRC receipt, as required by design§5.
Artifact inventory now includes the runbook plus six preserved trace files and
one receiver screenshot. No additional validation tool or source edit.
