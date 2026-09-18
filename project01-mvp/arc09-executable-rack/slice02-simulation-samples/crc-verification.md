# CRC verification — slice02, initial assignment

2026-09-17. Three-Contributor Workflow, Expedited Mode. Reviewer: Sofie/CRC,
independent of CC. Assignment reviewed:
`arc09-executable-rack/slice02-simulation-samples/cc-prompt.md`. Source
`8a73d33ad17cfad849db34d10b672fa4d894a79a` on
`codex/oxurack-module-behavior-installation`; planning evidence
`80ddf0ee6ce3f8cf5d3a32a814b3f15a5aa3c8c4` and
`85ea14d3f42a6b636634650a17bdea76be4cb73a`. Both worktrees were clean
at intake. This is a **partial review, not slice acceptance**.

## Independent checks and row walk

I inspected the complete source path diff from accepted `6a31c0f` to
`8a73d33`, the planning diff from `bc64132` to `85ea14d`, the new fixture,
the CC closing report and its 22-row validation table. All eight source paths
are within the allowed list. Independently reran at the stated source HEAD:
`cargo test --locked -p oxurack --test sample_pipeline` (4/4), the same with
`--no-default-features` (4/4), `cargo test --locked -p oxurack-core` (411 unit,
5 integration, 2 doctests), full workspace all-features/all-targets Clippy
with `-D warnings`, `cargo fmt --all -- --check`, and
`git diff --check 6a31c0f..HEAD`; all exited zero. CC's raw host/restricted
gate logs remain available, but I did not promote their attestation into an
independent rerun of the entire matrix because the slice is returning for
material evidence repairs. Full final-candidate gates remain required.

| Row | CRC finding | Disposition |
| --- | --- | --- |
| S-01 | `SampleTime` begins at None/0, publishes 0/1/2, and finalizes to 1/2/3; manual and controlled live 1+1+1 and 1+2 traces match. | Reproduced for reviewed candidate; final acceptance awaits repaired candidate. |
| S-02 | RON fixture and observed tuples match 1→0→0, 2→1→0, 3→2→1; phase markers and a disconnected hold are asserted. | Reproduced for reviewed candidate; final acceptance awaits repaired candidate. |
| S-03 | Bare Update core default/false and fixed running latch tests pass; rack starts stopped and manual stopped calls preserve sample count. The fixed preinserted-false and stopped-live paths deserve explicit checks in the corrective pass. | Partially reproduced; open. |
| S-04 | The transition test verifies overstep becomes zero but never performs the next live `step()`. It cannot reject a later catch-up sample. The integration negative also never measures the post-stop live trace. | Open: missing distinguishing transition oracle. |
| S-05 | The LastWins test permutes spawn order and produces `b` as winner, but never proves `ModuleId("b") < ModuleId("a")` (or chooses another opposing pair). The topological test checks names but likewise lacks an opposing numeric-ID order. The former numeric-order implementation may satisfy these tests. | Open: ordering policy is not discriminated. |
| S-06 | Source scope and principal Rust gates are sound. CC placed its first-class `closing-report.md` in `artifacts/` despite the prompt and framework requiring the slice root. The green verifier is `slice01/.../verify-planning.py`, hard-coded to slice01 assignment and ledger; it does not validate the slice02 close set. CC's `artifacts/test-evidence.md` still ends with a future-tense promise to extend it. | Open: canonical close placement and current-slice evidence repair. |

No ledger row is marked `done` on this partial review; opening and proposed
closing denominators are both six. No source defect is established by the
missing oracles. Preserve the existing CC report as prior-attempt evidence;
CC should move/update its own first-class report at the next return. The
restricted MIDI/DNS outcomes and three ignored RT timing tests retain their
existing status. This review does not claim physical MIDI, coverage, or arc
composition acceptance.

## Bubble-up check and next action

The delivered code appears within arc09's approved sample/phase scope, with
no discovered architecture or scope change requiring an arc-plan amendment.
The silent-drop diff is presently incomplete because S-04/S-05 claims lack
tests that distinguish the rejected behavior, and S-06 has a misplaced close
record. Arc09/slice03 stays held. CRC issued the next preserved assignment,
`cc-prompt-iteration01.md`, for bounded tests and evidence repair under the
existing contract. Source changes beyond that assignment require prior CRC
review; design changes return to CDC and the Operator.

## What worked

The real RON fixture, exact per-sample tuples, controlled live grouping, and
separate restricted/host logs made the sample behavior reproducible. The
review also exposed why a green predecessor-specific planning checker and
same-result ordering test are weaker than current-slice acceptance evidence.

## Iteration01 independent close — 2026-09-18

Reviewer: Sofie/CRC, independent of CC. Assignment:
`arc09-executable-rack/slice02-simulation-samples/cc-prompt-iteration01.md`.
Candidate source `5c1ce2ccb5e0ba36ac9fd2605a047e0ff45928b2`, planning packet
`733327f8af80c0f49bf8459fe7527cf82931cb23` with provenance follow-ups
`e82637dbd98df1e95f67794deb1895372d4d183a` and `f2897af`.
Both worktrees were clean. I inspected the four-file iteration diff and
CC's canonical root [`closing-report.md`](closing-report.md), 53 raw logs,
validation matrix, intake evidence and current-slice checker.

Independent commands at the clean source revision exited zero:
`cargo test --locked -p oxurack-core` (412 unit, 5 integration, 2 doctests);
`cargo test --locked -p oxurack` (15 umbrella, 5 installation, 5 sample,
0 doctests); no-default sample and installation suites (5 each);
`cargo clippy --locked --workspace --all-features --all-targets -- -D warnings`;
`cargo fmt --all -- --check`; strict all-features workspace rustdoc and
no-default umbrella rustdoc; `git diff --check 8a73d33..HEAD`.
The full host `make check` passed, including its dependency, build, lint,
format and all-features workspace test gates. Independently ran
`cargo test --locked --workspace --no-default-features --quiet` with host MIDI
access; it passed, including 150 oxurack-rt tests and the three pre-existing
ignored timing tests. CC's restricted failures and successful host retries
remain separately recorded; no physical MIDI/timing or 95% coverage acceptance
is inferred.

Before changing the ledger, I ran
`python3 project01-mvp/arc09-executable-rack/slice02-simulation-samples/artifacts/verify-slice02-planning.py --negative-cases`:
valid exit 0, missing-root-close exit 1, stale-assignment exit 1, with 53
logs and 16 local links checked. That checker deliberately requires CC's six
rows to remain `open`; after CRC closes the ledger it is a historical
pre-acceptance predicate, not a recurring post-close health check. The prior
slice01 checker remains inherited structural evidence only.

| Row | Independent disposition |
| --- | --- |
| S-01 | **Done, reproduced.** The public RON fixture yields indices 0/1/2, completed 1/2/3 and identical manual, live 1+1+1 and live 1+2 traces at 1 ms. |
| S-02 | **Done, reproduced.** All five phase markers and shared clock pairs, immediate merged inputs, one-sample relay/sink latency and disconnected hold match the exact ledger trace. |
| S-03 | **Done, reproduced.** Bare Update and fixed core run by default; preinserted false gates both; Oxurack begins stopped, stopped live/manual calls preserve state, and restart is contiguous. |
| S-04 | **Done, reproduced.** Exact wrong-driver error and manual `step()` no-op hold. New controlled zero/one ms post-transition updates prove discarded overstep cannot create a catch-up sample; host tests pass. |
| S-05 | **Done, reproduced.** The fixtures select and assert a lexical/ModuleId opposition before using actual LastWins propagation and `compute_tick_order`, then require the lexical result under both spawn orders. Numeric-ID ordering would yield a different result. |
| S-06 | **Done, reproduced.** Four changed test files stay within the corrective allowlist; the root CC report and artifact inventory are present; full gate matrix, strict docs, current-slice checker and negative cases pass with restricted failures retained. |

Opening, CC return and CRC close denominators are each six; all six rows are
accepted `done`. The CC report's `[a,b]` phrase is legacy shorthand from its
initial fixture: the final test actually chooses two opposing `order_*` names
at runtime, as checked in source and above. This minor stale label does not
alter its stated winner or evidence. The checker stage limitation is explicit
above and does not weaken the behavioral review.

### Bubble-up to arc09

The slice delivers the approved arc09 sample/phase piece at the expected source
and tests. Its only iteration finding was weak initial proof, repaired without
production or design change. The silent-drop comparison retains seed handoff
for slice03, TM/MIDI for arc10, Clock/LFO/Scope for arc11, persistence for its
named successor, and quality/hardware gates for their recorded owners. No
arc-plan scope, architecture or sequence amendment is needed. Slice02 closes;
CRC may open the already planned slice03 under CDC execution-contract §3.

### What worked

Controlled no-sleep time stimuli and an asserted opposing ID/name pair made
the disputed behaviors distinguishable. Preserving the first CC report and
restricted gate attempts kept the refinement's provenance inspectable.
