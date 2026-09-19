# Arc09 CRC operational and composition readiness

Date: 2026-09-18. Role: Sofie/CRC, arc-close assembler.
Source candidate: `642313a62db437ee2bf0b3d5b97fcdcffa16782a`.
Planning predecessor: `cc35c4b955cb04a3e1ec841745f42d0969be43b2`.
CRC slice-close and arc-assembly commit: `2a0411b`.

## Evidentiary role and verdict

This record supplies the Three-Contributor arc-level CRC evidence required by
the project workflow. CRC independently accepted each CC slice, but CRC is the
author of this arc assembly. Its arc-scale composition result is therefore
**operational readiness awaiting independent CDC verification**. It is not the
missing design/composition gate, does not close E-01–E-04, and does not authorize
arc10.

I inspected the final source diff across all three slices, all slice plans,
ledgers, CC reports, CRC records, arc plan, arc ledger, project successor map
and current status. The final source and planning predecessors were clean.

## Reproduced operational evidence

On the final source candidate, CRC independently ran:

- core, umbrella and no-default package suites;
- focused `module_installation`, `sample_pipeline`, core seeded-context and
  umbrella seeded-context integrations;
- workspace all-features and no-default-features tests with host MIDI access;
- all-features/all-targets Clippy with `-D warnings`, formatting and both
  strict rustdoc configurations;
- host `make check`, source diff checks and the inherited structural planning
  verifier;
- a separate implementation-independent calculation of the six FNV vectors.

Every required command passed. The workspace matrices retain 3 pre-existing
ignored RT tests; `block 0.1.6` emits its existing future-incompatibility notice.
CC's restricted MIDI/DNS failures and successful host retries remain in each
slice packet. No physical MIDI, real-time scheduling, coverage, benchmark,
soak or musical acceptance is inferred.

## Composition row readiness

| Row | CRC operational evidence | Strength / CDC re-entry |
| --- | --- | --- |
| E-01 | Final `module_installation` tests load two instances, execute each once per fixed update, preserve state and hooks across reinstall, and reject a conflicting kind before mutation. Core/umbrella package runs preserve schema/spawn/setter/getter behavior. | Reproduced by CRC at slice level; CRC-attested composition. CDC reruns or stronger equivalent on `642313a`. |
| E-02 | Final `sample_pipeline` tests compare exact manual/live 1 ms traces, shared phase indices, merge visibility, stateful latency, stopped edges, discarded overstep and lexical ordering under spawn permutations. | Reproduced by CRC at slice level; CRC-attested composition. CDC verifies the final integrated candidate. |
| E-03 | Final core/umbrella seeded tests prove pre-construction context from real patches, fixed vectors, legacy fallback, failure isolation, changed inputs, reorder stability and two fresh child-process traces. | Reproduced by CRC at slice level; CRC-attested composition. CDC independently checks the fixed trace and seed handoff. |
| E-04 | Three slice ledgers contain 6/6 done rows each; prompt history and evidence packets are preserved. Final feature matrices, strict docs and scope/successor comparison pass with inherited gates retained. | CRC assembly complete. CDC checks the 3-slice/18-row denominator, silent-drop diff and project bubble-up. |

## Silent-drop and plan-change review

Specified and delivered arc scope match. Installation, explicit deterministic
sampling and pre-construction seeded replay are present on one source candidate.
TM/MIDI, Clock/LFO/Scope, live persistence/runtime operations, remaining module
catalog, interfaces and release validation were explicitly assigned outside
arc09 before implementation and remain there. No new plan change is requested.

## CDC handoff

CDC should review these project-relative paths:

- `arc09-executable-rack/closing-report.md`
- `arc09-executable-rack/crc-verification.md`
- `arc09-executable-rack/ledger.md`

CDC records its independent verdict in
`arc09-executable-rack/cdc-verification.md`. If composition passes, CDC closes
E-01–E-04, updates the project bubble-up, closes arc09, and prepares arc10 plus
its first slice under the existing roadmap. Required changes return as a new
project-level `cdc-directiveNN.md` through the Operator.

## CDC review06 disposition addendum — 2026-09-18

The operational-readiness verdict above applies only to source `642313a` and was
not accepted at the independent composition gate. [CDC review](cdc-verification.md)
reproduced E-01/E-03 but left E-02/E-04 open after its phase-stop probes exposed
incorrect partial return and time accounting. Passing standard matrices did not
cover that interaction.

[Directive06](../cdc-directive06.md) now governs the bounded
[Slice04 open set](slice04-sample-stop-accounting/slice-plan.md). The arc contains
four planned slices/24 child rows; the prior three/18 retain their accepted
candidate evidence while M-01–M-06 are open. CRC must independently reproduce the
repair and reassemble E-01–E-04 before another CDC review. This addendum does not
accept new source or authorize arc10.

## Repaired composition reassembly — 2026-09-18

CRC independently reviewed source
`37deeace90e7fe2dff633ada9d58ef9051577c3d` and CC evidence
`7efed01357a9332b5b00556daefcf8a0c1464d3a`. The diff from `642313a` changes
only `crates/oxurack/src/lib.rs`: a bounded production loop repair and focused
private tests. [Slice04 verification](slice04-sample-stop-accounting/crc-verification.md)
closes M-01–M-06 with 0 corrective passes.

CRC reproduced the nine stop-accounting tests in default and no-default modes,
the umbrella and core suites, both full host workspace matrices, all-targets
Clippy, format, both strict doc gates, host `make check`, dependency observation,
source scope/diff checks and the current planning checker with three negative
controls. Exact phase/count/time/state and seeded resume observations match
directive06. The pre-existing three ignored RT tests and `block 0.1.6` notice
remain; no coverage, physical MIDI/timing or musical acceptance is inferred.

| Row | CRC repaired-candidate result |
| --- | --- |
| E-01 | Ready: installation/reinstallation, two-instance execution, state/accessor preservation and typed conflict regressions pass unchanged on `37deeace`. |
| E-02 | Ready: manual/live phase traces and ordering pass; mid-sample stops now finish the latched sample and return/count/advance only completed work, with stopped no-op and final-state continuation proven. |
| E-03 | Ready: construction seed/fresh-process tests pass; Slice04 additionally proves stopped resume preserves owned seed/state and equals an uninterrupted reference. |
| E-04 | Ready: four slice ledgers contain 24/24 CRC-accepted rows; scope, iteration counts, full gates, inherited gaps and successor routing reconcile. |

These are CRC reassembly results, not CDC acceptance. CDC must independently
review all four rows on `37deeace`. Arc09 and Arc10 remain held until that verdict.
