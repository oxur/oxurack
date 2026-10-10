# Slice04 CRC verification — software accepted; witness pending

2026-10-10, Sofie/CRC; Three-Contributor Workflow, Expedited Mode.
Independent of CC implementation; no CRC source changes.
Assignment: `arc10-turing-machine-midi/slice04-output-runtime/cc-prompt.md`.
Source: `65522f116a12a9ac96d35cfe25ed9c184f5bddcb`, clean on the assigned
implementation branch; includes `829feb99279b11b3f46a11e513aba577eacd6fd1`.
Planning intake: clean `514d599c21dd25c07ee24e18d4e0850ac9af354d`.
Authority: directive08, Arc10 design v1.2 §§4–5 and the issued prompt.
Framework 2.6.0 and Rust 2.0.0 applied to review.

## Verdict and five-row walk

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
