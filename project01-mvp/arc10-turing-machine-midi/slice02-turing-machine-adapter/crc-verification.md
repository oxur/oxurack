# CRC verification — Slice02 Turing Machine adapter

## Iteration00 review — returned for correction, 2026-10-10

Reviewer: Sofie/CRC, independent of CC. Three-Contributor Workflow;
Duncan renewed Expedited Mode on 2026-10-10. Reviewed assignment:
`arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt.md`.

Clean candidate source: `9ba55dd0a4c8aa7d67fe26c30f878d89ead34aa2` on
`codex/oxurack-module-behavior-installation`. CC planning evidence:
`516afcd` and clean final planning HEAD
`2eb5ac138fedf21d4185d1cc2f1dd3df84c4972d`. No later Slice02 review or Slice03
assignment exists at intake. Clock remains accepted at `a19cfd1`.

CRC reloaded collaboration-framework 2.6.0, the prompt-authoring guide and
template, testing discipline and Rust guidelines 2.0.0. CRC read the complete
adapter and integration test, the engine/getter diff, parameter registry,
sample scheduling/merge boundaries, initial assignment and slice plan, Arc10
design, ledger and CC evidence. The initial prompt does contain concrete Rust
examples and explicit behavioral oracles; this return concerns missing proof
in the implementation packet, not a missing authoring guide.

## Reproduction and scope

Exact commands, working directories, exit codes and raw output are in
[`artifacts/crc-review01/results.tsv`](artifacts/crc-review01/results.tsv).
CRC independently reproduced:

- standalone TM: 112 unit tests, 0 failures/ignored; 0 doctests;
- rack-feature TM: 120 unit tests, 0 failures/ignored; 0 doctests;
- umbrella TM: 5 integration tests, 0 failures/ignored;
- default normal dependency tree without Bevy/core;
- full workspace/all-feature/all-target Clippy with denied warnings,
  formatting and source diff check;
- current planning predicate and its four existing negative controls.

The first checker invocation used the wrong CRC working directory and exited 2.
Its raw receipt is retained; the correctly rooted retry exited 0. This is a CRC
harness error, not a CC packet failure. The independent scope audit confirms
exactly the ten initial allowlisted paths changed from `a19cfd1` to `9ba55dd`.
Standalone musical implementation changes are read-only getters and tests.

Full workspace tests, rustdoc, Make and dependency freshness were **not rerun
by CRC in this partial review**: the behavioral proof defects below already
prevent acceptance. CC's previous host retries remain attested, with their
restricted MIDI/DNS failures and three ignored RT tests preserved. The corrected
candidate must return the complete mandatory matrix before final CRC review.

## Findings

### F-01 — strict parameter and failure-preservation matrix is incomplete (A-02)

In `rack.rs::tests`, schema checks assert names/counts and only two merge
policies; they omit the full directions/kinds/policies and parameter defaults.
The validation loop invokes private `set_parameter`, while the registration
loop merely calls getters without checking their values. It does not execute
every registered setter or prove every valid value reads back as actual state.
Spawn coverage is limited to unknown keys, one malformed Scale and one valid
boundary configuration. Unknown-key precedence has no invalid known key.
Missing negatives include infinities, signed/extreme values, invalid selector
members inside the numeric range, both endpoint order errors, and malformed
secondary scales at spawn and setter boundaries.

The invalid-setter snapshot contains only register bits, step count, a Boolean
pending-presence flag and `step`. It omits pending **contents**, engine settings,
base/modulation, edge histories and the other held ports. It therefore cannot
support the report's complete no-mutation claim. Require a deep observation and
continued seeded output comparison, not additional calls without assertions.

### F-02 — direct publication and held-snapshot oracles are incomplete (A-03)

`optional_output(None)` is tested, but the real publication path is not exercised
with absent note/velocity/scale-note. The standalone integration oracle checks
odd snapshot samples only; even samples are counted but their step/held values
are not compared. A wrong implementation clearing a scalar on idle samples
could pass. Add a complete publication-helper/pending snapshot case and compare
all idle samples against independently determined held values.

### F-03 — action/modulation and lifecycle claims exceed exercised cases (A-04)

`reset_move_clock_priority_and_modulation_are_explicit` raises all three gates,
then immediately lowers them. It never holds the losing edges high on a later
sample, never exercises move over clock, and never checks move's count/divider
behavior or reset's retained settings/RNG and held publication. Modulation is
only +0.3 repeated; negative/zero/clamped/summed inputs and a base edit under
held modulation are absent.

`stop_pending_resume_and_reinstall_preserve_progress` stops after one sample
and ends immediately after checking the resumed `step` gate. There is no full
trace/SampleTime comparison with an uninterrupted reference, no later sample
proving exactly-once publication, and no high reset/move pause case. Reinstall
is invoked after progress, but no continuing trace proves preserved state/RNG
or exactly one system execution. The approved APIs suffice for these checks;
no public test hook is needed.

### F-04 — instance-name negative control is a false positive (A-05)

`changed_seed_and_instance_name_change_trace` compares the complete BTreeMap
after renaming `tm_primary` to `tm_renamed`. Different keys make `assert_ne!`
pass even if all generated sample values are identical. CRC's
[`name-control-probe.py`](artifacts/crc-review01/name-control-probe.py) obtains
the actual two-instance, 24-sample, 28-output child trace and demonstrates that
predicate accepts an identity-only rename with unchanged content. Its
[`receipt`](artifacts/crc-review01/10-name-control-probe.log) distinguishes an
oracle defect from a production seed defect; CRC does **not** claim the actual
seed derivation is wrong.

Compare baseline primary **row contents** with renamed primary row contents,
and independently compare the renamed rows against the standalone engine using
the renamed context. Preserve equality for the unchanged secondary instance.
The fixture contains an unrelated module in both compared runs; reversal
perturbs allocation but does not prove adding/removing an unrelated instance
preserves the named TM traces. Add that independent perturbation control.

### F-05 — evidence and current-return checks need bounded repair (A-06)

The CC row walk/test evidence describes the incomplete checks above as passed.
Retain iteration00 bytes and receipts, then account for each repaired assertion
by test name and command in iteration01; distinguish absence of proof from an
observed production failure. The canonical close lacks an explicit slice-to-arc
bubble-up disposition. Add it with the next return. The existing checker is
structural only and omits closing-report presence/assignment and bubble-up;
extend that same predicate with current-return checks and negative controls,
without advertising it as behavioral acceptance.

## Row disposition and evidence retention

| Row | CRC disposition | Independently established / remaining gap |
| --- | --- | --- |
| A-01 | open | Opt-in default tree, unchanged engine algorithm, context path and existing standalone oracle reproduce; final candidate regression gate remains. |
| A-02 | open | Implementation appears to validate before mutation; F-01 leaves the complete registered API and preservation contract unproved. |
| A-03 | open | Complete 12-tick odd-sample oracle reproduces; F-02 leaves absent/held scalar publication unproved. |
| A-04 | open | Basic clock latency and reset/+0.3 cases pass; F-03 leaves required conflicts, modulation and continuing lifecycle unproved. |
| A-05 | open | Fresh-process byte replay and Manual/Live/reversal equality reproduce; F-04 invalidates the rename negative and leaves independent unrelated insertion unproved. |
| A-06 | open | Ten-path source scope and focused quality gates reproduce; corrected evidence and full candidate gates remain. |

No row is closed by this partial review. The independently reproduced subset
above may be retained if CC changes tests only and the relevant production bytes,
dependencies and fixtures remain identical. CC's older attestations are not
promoted. Final review must inspect the new diff and rerun repaired/affected
checks; retention is a CRC decision, not permission to skip assigned gates.

Verdict: Slice02 stays open; issue `cc-prompt-iteration01.md` as corrective pass
1/5. Source production behavior is not repaired by CRC. Slice03 remains held.

## Bubble-up to arc

These are proof/evidence defects within the existing adapter contract. The
source inspected shows no demonstrated need to change Arc10 architecture,
acceptance criteria, slice order or mapping. No design escalation is required.
The correction preserves TM-first, followed by MIDI-output/runtime/endpoint
acceptance; Arc11 LFO/Scope and retained project quality/hardware gates stay
assigned. The current status header is refreshed because it still described
the already-accepted Clock correction as active.

## Iteration01 review — accepted, 2026-10-10

Sofie/CRC independently reviewed the current `cc-prompt-iteration01.md` return:
source `b2ffb16d342d9a6cc800bab2713813d9646fb505`, CC evidence
`61671a3eb487cbc433f721d59d645ffd60ebfb19`; planning intake `e70620b`.
Three contributors and Expedited Mode remain active. Directive08 is acknowledged
and does not amend this issued correction's criteria or gates.

CRC inspected all corrected adapter tests, the complete public integration
runner, getter-test diffs, the current intake/readback, six-row CC close and
command matrix. The four source paths are the exact authorized test paths;
all three production prefixes are byte-identical to `9ba55dd`. Manifests,
lockfile, fixture and other implementation paths are unchanged. The CC-only
planning delta preserves issued prompts, prior CRC records, checker and
iteration00 artifacts.

### Independent reproduction and retained evidence

Commands, exits and receipts are in [crc-review02/results.tsv](artifacts/crc-review02/results.tsv).
New reproduction at `b2ffb16`: standalone 112 tests; rack 129; public TM
integration five each in default/no-default modes; normal dependency tree
without Bevy/core; all-target/all-feature workspace Clippy; formatting; both
host workspace feature matrices; both strict rustdoc modes; host `make check`;
and host dependency observation. All pass. Both workspace runs retain 150
passing RT tests and three ignored timing tests; no focused test is ignored.
The all-feature workspace enables existing midi-io tests, giving 134 TM tests,
versus 129 in the focused rack-only run. Both current checker modes and their
same-predicate negatives pass; their scope is packet structure only.

One initial CRC scope audit incorrectly treated `32e6934..61671a3` as a CC-only
planning delta. That range includes CDC's intervening `9cbcd02`; the failure
and corrected CC-commit-only audit are preserved in receipts 12/13. It is a
CRC inspection-method error, not a source/evidence defect.

Retain the iteration00 production review and independent sample/seed findings
at their original `9ba55dd` revision: algorithm, adapter production bytes,
fixture and dependencies are unchanged, and no new finding invalidates those
methods. Repaired tests and downstream integration were freshly reproduced
rather than inferred from that retention. CC's restricted MIDI/DNS failures
remain in its matrix alongside host retries. CRC used host access for these
known environmental gates. Dependency observation reports available updates
without changing source or dependencies; it is not an upgrade assignment.

### Six-row acceptance

| Row | CRC disposition | Checked behavior |
| --- | --- | --- |
| A-01 | done; reproduced | Default dependency isolation, standalone regression and seeded real rack construction; protected production bytes unchanged. |
| A-02 | done; reproduced | All 32 port declarations/nine defaults and actual registered setter/getter readbacks; full wrong-variant/range/Scale spawn and setter rejection, attribution, no orphans, deep snapshots and forward standalone seeded continuation. |
| A-03 | done; reproduced | Actual Produce sentinel publication, absent optional values, Some(0) validity and idle holding; all 28 outputs at all 24 integration sample indices match a separately configured engine. |
| A-04 | done; reproduced | Scripted reset/move/clock conflicts and consumed held edges, isolated actions, positive/negative/zero/clamped modulation and real two-cable Sum; full pause/pending/reinstall continuation and sample accounting. Existing unchanged engine owns divider/reset behavior; public reference spans 12 ticks. |
| A-05 | done; reproduced | Exact fresh-child equality, corresponding changed seed/name contents and renamed standalone oracle, unchanged secondary, reversal plus independent observer insertion/removal. |
| A-06 | done; reproduced | Current assignment/readback/close, exact source and protected evidence boundaries, complete gate receipts, retained failures/ignored counts, bubble-up and artifact placement. |

F-01–F-05 are resolved. Six rows opened, six rows done; no deferred/no-op rows
or silent drops. The getter range observation is owned, and DC-05 is correctly
interpreted as Errors documentation in the corrective intake. CC's canonical
report remains its original proposed-done attestation; this record supplies
independent acceptance.

## Bubble-up to arc — accepted correction

Slice02 delivers the complete seeded rack adapter assigned by Arc10, including
all settings/actions and 28 published scalar outputs. Its supporting evidence
lives in the existing `artifacts/` home: intake, test evidence, matrices, logs,
structural checker, preserved iteration00 material and CRC review receipts.
Both issued prompts remain at their slice-root paths. No production repair,
missing capability or new architectural dependency emerged from correction.
No substantive arc-plan amendment is needed; record routine Slice02 acceptance
and advance under CDC's already approved directive08/design v1.2.

Verdict: **Slice02 closed at `b2ffb16`**. Repin that accepted predecessor and
open Slice03 MIDI-out plus runnable seeded trace. Arc10/project composition
rows remain open; endpoint witnessing, playback cleanup and release obligations
are not accepted here.
