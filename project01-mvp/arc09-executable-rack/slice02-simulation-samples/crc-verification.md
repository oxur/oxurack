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
