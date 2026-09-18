# CDC directive05 — bounded gate repairs and corrective assignment

2026-09-17. Sofie/CDC to CRC through Duncan. Responds to every decision in
[crc-escalation05.md](crc-escalation05.md); predecessor
[cdc-directive04.md](cdc-directive04.md). Inspected clean planning `69140854cd13cffb3531f4a75b39249e8a04aa88`,
project v3.3 / arc09 v1.0 / slice01 v1.3 and clean implementation candidate
`d43078ea9c71bba10035699c9914bef3891f1d83` in
`.worktrees/codex-oxurack-module-behavior-installation`, branch
`codex/oxurack-module-behavior-installation`. Main remains `a7ce6e6`.
This return updates project v3.4 and slice01 v1.4. It authorizes repair work;
it neither closes the slice nor upgrades CRC's three open rows.

## Independent inspection and decisions

CDC read CRC's actual review, ledger, assignment record, source tests and verifier.
CDC reran `CARGO_NET_OFFLINE=true cargo clippy --locked --workspace --all-features
--all-targets -- -D warnings` at the clean source candidate: exit 101, reproducing
both reported diagnostics. [Raw Clippy output](arc09-executable-rack/slice01-module-behavior-installation/artifacts/cdc05-clippy.log).
The planning verifier independently exited 1 at its obsolete current-assignment
assertion; [raw result](arc09-executable-rack/slice01-module-behavior-installation/artifacts/cdc05-verifier.log). Its later link checks
therefore did not run. CDC did not rerun CRC's successful behavioral matrix or
make/host checks and does not adopt those as CDC reproduction.

| Request | Disposition | Rationale and boundary |
| --- | --- | --- |
| D1 source allowance | Approved | Two localized test-code normalizations make the existing full Clippy gate reachable; no separate prerequisite slice or gate waiver is needed. Exact edits below. |
| D2 current verifier | Approved | Evolve the existing checker to validate explicit assignment metadata and preserved prompts; do not freeze it to another numbered prompt. Original checks and recorded failures survive. CC implements; CRC inspects and reproduces. |
| D3 combined pass | Approved | CRC may issue one guide-authored iteration02 covering I-02, I-05 and I-06, then independently review the new source/planning candidates and regressions. Arc09/slice02 remains held until full slice acceptance. |

Authority: Duncan's existing engineering-planning remit, this Operator-relayed
scope escalation and the retained quality gates. These are bounded additions to
test/checker scope, with no product behavior, API, scheduler, seed, MIDI or target
change. Directive03's architecture and directive04's authoring discipline remain.

## D1 — exact source edits and proof

1. `crates/oxurack-graph/src/lib.rs`, test helper `find_port`, candidate line 1036:
   combine the nested Some/port-name conditions using the compiler-suggested let
   chain. Retain first-match return, child iteration and the not-found panic.
   This is the sole non-comment exception to that file's prior doc-only allowance.
   No unrelated graph cleanup. The crate already uses edition 2024; no manifest
   or toolchain change is authorized.
2. `crates/oxurack-core/src/bridge.rs`, test
   `test_midi_output_queue_push_and_drain`, candidate line 481: replace the full
   drain/collect expression with `std::mem::take(&mut queue.commands)`. Preserve
   both commands, result length two and empty-source assertion. It moves the Vec
   and replaces it with its empty default; allocation capacity is not this test's
   contract. Production queue draining/capacity policy must remain unchanged.

The original eight source paths remain with their prior bounds, plus bridge.rs
for only this test edit. For this corrective source commit, expected paths are
`crates/oxurack-graph/src/lib.rs`, `crates/oxurack-core/src/bridge.rs`,
`crates/oxurack/src/lib.rs` (test module only for I-02), and
`crates/oxurack/tests/module_installation.rs` (only if its misleading test is
renamed/adjusted or preserved fixture assertions need correction). CRC must list
the exact chosen files in the prompt. Do not replay already committed doc/API
changes or expand production code. Changes beyond these corrections require review.

Applied guidance: Rust `guides/15-cargo/07-lints-and-formatters.md` CG-L-01
(SHOULD) explains why all-targets covers tests; the required command is not
replaceable by default-target or targeted Clippy. Rust
`guides/04-ownership-borrowing.md` OB-19 (SHOULD) supports the Vec ownership
transfer; AP-01 (AVOID) keeps warning policy in the command, not a new library
attribute. Testing's `01-testing-discipline.md` and `03-validation-gates.md`
require meaningful behavioral proof and separate gate evidence. CRC must load
these actual installed chapters when applying them; no lint suppression,
`cargo clippy --fix` sweep, dependency change or weakened checks.

## I-02 — a feasible after-edit reinstall test

CRC correctly identified the missing temporal oracle. CDC additionally inspected
`Oxurack::into_app(self)`: it consumes the rack. Moving the existing reinstall
call below it is impossible without changing the test arrangement. Do not add a
public app accessor or reconstruction API just for this test.

Recommended implementation: add a focused test and minimal counter fixture inside
`crates/oxurack/src/lib.rs`'s existing `#[cfg(test)] mod tests`, which can access
`rack.app` while retaining the rack. Reuse the existing two-instance RON file
through the public `install`/`load_patch` path; the private access only controls
schedules and observes state. Use a test module with the same fixture kind and
increment parameter, real registered system/setter/getter and state totals. A
small test-local fixture is preferable to production hooks or broad test plumbing.
The existing integration cases continue to exercise the public consumption path.

Binding oracle: totals start `(0,0)`; two direct FixedUpdate runs reach `(4,6)`
from increments 2/3. Change alpha through its registered setter to 4, confirm its
getter, release all world borrows, then call `rack.try_install::<Fixture>()`.
Immediately assert totals remain `(4,6)` and getters return 4/3. One further
FixedUpdate reaches `(8,9)` with one execution per instance, rejecting resets and
duplicate registration. Retain conflict preservation and independent-rack tests.
Name the original pre-edit reinstall test accurately if retaining it; it is
useful baseline coverage but cannot alone close I-02. Run the new library test
with default features and `--no-default-features` in addition to the existing
required focused integration tests and full matrix. CRC must provide the exact
filter/name in iteration02 after source investigation. This recipe is inspected
reasoning, not compiled code or a passed test.

## D2 — verifier contract

CC may edit only this slice's `artifacts/verify-planning.py` for the assignment
check and necessary output wording. CRC owns synchronizing the slice plan's
assignment/history metadata before relay. Use one explicit canonical field,
`Current assignment: ` followed by the backticked full project-relative prompt
path; retain separate prose for executed/reviewed/held historical assignments.
The checker reads that field, not the largest filename or a historical mention.

Require exactly one current field, a canonical initial/iteration prompt filename
inside this slice, an existing regular file and a matching assignment-history
entry. Require the initial cc-prompt.md and issued iteration01 still present;
when iteration02 is recorded current, require that file too. Reject missing,
duplicate/ambiguous, nonexistent, out-of-slice or history-mismatched assignments.
Do not hardcode iteration02 as eternally current. CRC may choose a compact parser
or helper after reading the actual Markdown; state its accepted field/history
format in the prompt. If symlink/path normalization is used, it must not permit
escape from the owning slice.

All pre-existing import-preservation, exact 24-row successor/criterion checks,
child denominator 0/0/2/3/0/1, closure records, ledger counts 7/4/6 and local links
must still execute. Preserve the earlier failed logs and the checker's historical
Git source. No exception bypass, swallowed assertion, weakened denominator or
claim that earlier failure passed. Capture full current success plus small
negative fixtures for missing/nonexistent/mismatched/ambiguous assignment records
and a valid alternate numbered assignment (with corresponding file/history) to
prove the check follows metadata. Exercise copies/temp fixtures, never damage the
real planning tree to demonstrate failure. Record method and outputs in named
iteration02 artifacts; no general Markdown framework or extra permanent test suite
is requested. CRC independently inspects the diff and reruns the complete checker.

## I-06 — honest provenance and bubble-up

Correct the ten invalid cwd cells using the actual worktree path and supporting
logs; disclose that these are metadata corrections, not historical reruns. Retain
baseline HEAD as HEAD-at-execution and add explicit dirty/uncommitted implementation
state. Do not substitute the later commit SHA as if it existed when commands ran.
Map that working tree to `d43078e` only to the extent the evidence supports byte
identity; otherwise label the historical mapping uncertain and use fresh clean-
candidate checks for acceptance. Preserve every original command, exit, failure,
retry and raw log. Record corrections and their basis in test-evidence.md rather
than silently overwriting provenance. New runs get accurate cwd, HEAD, dirty-state
and exact artifact names captured at execution, preferably on the committed source
candidate. Do not discard earlier validation rows when adding fresh results.

Closing-report.md must explicitly bubble up delivered capability, newly found
scope/checker gaps, and scope-as-specified versus delivered/deferred work. Existing
sample/seed/MIDI obligations and Q-02/Q-03 remain with their named successors.

## CRC assignment and re-entry

Acknowledge this directive in the project handoff history against actual revisions.
Apply the guide/template from directive04, investigate the above source details,
and record all six author-readiness results for the new assignment. Preserve all
issued prompts/directives and CRC's prior candidate verdict. Prepare, but do not
pretend this directive has already issued, the next CC path:

`arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration02.md`

List all required commands, including unchanged all-targets Clippy, full matrix,
strict docs, native make check, separate outdated result, new default/no-default
library test, full structural checker and bounded parser negatives. Retain access
failures and authorized host reruns; no coverage/hardware acceptance is inferred.
CC returns a new committed source candidate and scoped planning evidence. CRC
independently verifies I-02/I-05/I-06 and relevant I-01/I-03/I-04 regressions, with
candidate-specific dispositions. The historical three done rows stay anchored to
`d43078e`; this directive is not acceptance of their future candidate equivalents.

CC's planning allowance additionally includes this checker and individually named
new logs/negative-fixture evidence under the same artifacts home; old raw logs and
issued design/prompts are immutable. CRC may update plans, ledger notes and current
pointers within this decision; no acceptance statuses change until review. Use
explicit file lists, cached-diff inspection and separate source/planning scoped
commits. Preserve unrelated work. No push/merge/release.

Executed corrective count remains 0/5 until CC executes iteration02, then 1/5.
After complete independent slice acceptance CRC can advance to slice02 within the
approved arc09 design. Until then slice close and that advancement remain held.
Return the exact prompt path, planning/source commits, readiness result and any
remaining blocker to Duncan. Next project exchange is 06 unless reserved.
