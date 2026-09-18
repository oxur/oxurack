# CRC escalation 05 — complete slice01's unchanged quality gate

2026-09-17. Sofie/CRC to CDC through Duncan, Three-Contributor Workflow,
Expedited Mode. Project01 MVP / arc09 executable rack / slice01 module behavior
installation. Predecessors: `cdc-directive04.md` and the executed
`arc09-executable-rack/slice01-module-behavior-installation/cc-prompt-iteration01.md`.
Source candidate: clean `codex/oxurack-module-behavior-installation` at
`d43078ea9c71bba10035699c9914bef3891f1d83`, based on clean main
`a7ce6e61536ca96b6aeb2227ee06e430ed8df162`. Planning intake:
`2da485ebad1c685b847bcebf954921dd0aecedfa`. Governing project plan v3.2,
arc09 plan v1.0, slice01 plan v1.2, six I rows. This packet requests a bounded
scope/checker decision, not acceptance or a replacement design.

## Independent finding

The [CRC slice review](arc09-executable-rack/slice01-module-behavior-installation/crc-verification.md)
reproduced I-01, I-03 and I-04. I-02 still needs a test that reinstalls **after**
editing loaded live state; I-06 needs ten invalid `cwd` cells and its dirty-tree
provenance corrected, plus an explicit arc bubble-up. Those two repairs fit the
existing implementation/evidence scope and can be included in the next CC
iteration without changing public behavior.

I-05 cannot pass under the current source/checker scope:

1. The required `cargo clippy --locked --workspace --all-features --all-targets -- -D warnings`
   exits 101 on `clippy::collapsible_if` in a test helper at
   `crates/oxurack-graph/src/lib.rs:1036` and `clippy::drain_collect` in a test
   at `crates/oxurack-core/src/bridge.rs:481`. CRC reproduced both warnings;
   the same constructs are present at the pinned source baseline. The graph
   path is currently **doc-comment-only** under slice01's eight-path allowlist;
   bridge is not in it. Passing targeted clippy and `make check` does not satisfy
   the separately required all-targets command. No lint suppression or target
   reduction is appropriate.
2. `artifacts/verify-planning.py:70` still requires the original held
   `cc-prompt.md` to be the current assignment. Both CC and CRC ran it and got
   `Assignment mismatch` after the authorized iteration01 assignment. Its
   other structural checks did not execute past that assertion. The file was
   retained as historical evidence during CRC authoring; silently calling this
   a pass would hide a failed required check.

CRC also reproduced the module behavior tests, workspace feature matrices,
strict docs, format, targeted clippy, and host `make check` as passing. A
restricted `make check` run failed four MIDI initialization tests and the
host-access rerun passed. Restricted `cargo outdated` failed DNS; the host
retry passed without upgrading dependencies. These access failures and reruns
are accounted for, not offered as gate waivers. The 95% coverage and future
timing/MIDI acceptance obligations remain with their existing successors.

## Decision requested from CDC

**D1 — exact source allowance for the unchanged full Clippy gate.** Recommend
authorizing only two test-code repairs in a corrective iteration: collapse the
already diagnosed nested `if` in graph `lib.rs`'s `find_port` test helper, and
replace `queue.commands.drain(..).collect()` with `std::mem::take` in the core
`bridge.rs` test while retaining the empty-queue assertions. Amend slice01's
allowlist so graph `lib.rs` permits that exact test-helper edit in addition to
its already delivered doc fixes, and add `crates/oxurack-core/src/bridge.rs`
for that exact test-only edit. Keep the required all-targets Clippy command,
all other gates and the original eight production/doc paths unchanged. CDC
may instead assign these fixes to a named prerequisite slice, but must state
how I-05 and arc09 advancement remain gated until the command passes; a waiver
or out-of-scope CC edit is not recommended.

**D2 — current structural verifier.** Recommend authorizing a narrow update
to the existing `verify-planning.py`: require the preserved initial prompt and
the current iteration01 (or later recorded) assignment, then execute all its
original carryover/ledger/link checks. Retain its historical source in Git and
the recorded failing attempts; do not weaken denominators or promote the old
failure. Recommend CC make the checker edit in the corrective packet, with CRC
independently inspecting its diff and reproducing the complete check. A separate current-state
verifier is possible but duplicates the existing check; a permanent failure
disposition without a passing successor is not recommended.

**D3 — next CC pass and evidence.** Confirm CRC may issue a single detailed
`cc-prompt-iteration02.md` combining the existing-authority I-02 test order and
I-06 provenance/bubble-up corrections with D1/D2's approved scope. Preserve
the existing source behavior and all failed/retried raw results. This would be
the first **executed corrective** pass (1 of 5) if CC runs it; the unexecuted
initial prompt did not consume a pass. Require a new source/planning candidate,
complete gate matrix and independent CRC review. If CDC chooses a different
owner or sequence, state the exact re-entry condition and assignment path.

No API, behavior, scheduler, seed, timing, MIDI or coverage target amendment is
requested. The expected effect is to make the existing gate genuinely pass and
to strengthen the evidence for the behavior already implemented. The test and
provenance repairs can proceed under current design, but CC relay, slice close
and arc09/slice02 planning are held while D1/D2 are unresolved. CDC owns the
scope/checker disposition; Duncan relays the response as
`cdc-directive05.md` at this project level. CRC will acknowledge it in the
planning record before issuing CC's corrective prompt.
