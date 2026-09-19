# Arc09 closing report — CRC assembly

Date: 2026-09-18. Workflow: Three-Contributor, Expedited Mode.
Candidate source: `642313a62db437ee2bf0b3d5b97fcdcffa16782a`.
Candidate planning predecessor: `cc35c4b955cb04a3e1ec841745f42d0969be43b2`.
CRC slice-close and arc-assembly commit: `2a0411b`.

## Status and capability verdict

CRC has assembled the arc09 close and demonstrated operational composition.
The arc is **ready for independent CDC verification, not yet closed**. E-01–E-04
remain open until CDC reviews this exact candidate and the CRC assembly in
[crc-verification.md](crc-verification.md). The required future
`cdc-verification.md` does not yet exist and is not inferred.

Arc09 promised an executable and reproducible rack substrate: module behavior
installed once per type and applied to all instances, explicit deterministic
sample/phase driving, and patch seed/name context available before module state
construction with replay across fresh processes. The accepted slices and the
integrated candidate deliver that capability. They do not implement the Turing
Machine adapter, MIDI-out, workbench modules, live persistence, final timing,
coverage or musical acceptance.

## Slice walk

The arc plan contains three slices; all three are present and independently
accepted by CRC.

| Slice | Outcome | Evidence |
| --- | --- | --- |
| slice01-module-behavior-installation | Delivered after two corrective passes: once-per-type installation, typed kind conflicts, live replacement preservation and strict documentation repair | [Slice ledger](slice01-module-behavior-installation/ledger.md), [CRC close](slice01-module-behavior-installation/crc-verification.md#iteration02-independent-close) |
| slice02-simulation-samples | Delivered after one proof-repair pass: zero-based 1 ms samples, five ordered phases, running gates, manual/live exclusivity and name-stable ordering | [Slice ledger](slice02-simulation-samples/ledger.md), [CRC close](slice02-simulation-samples/crc-verification.md#iteration01-independent-close--2026-09-18) |
| slice03-seeded-spawn-context | Delivered without correction: fixed FNV-1a derivation, pre-construction context, legacy dispatch compatibility and fresh-process replay | [Slice ledger](slice03-seeded-spawn-context/ledger.md), [CRC close](slice03-seeded-spawn-context/crc-verification.md) |

The slice denominator is 3/3. No slice is deferred, dropped or replaced.

## Composition check

CRC ran the final source as a single locked workspace. The umbrella module
installation suite proves two loaded instances share one installed behavior
system, reinstall does not duplicate it, state/accessors/hooks survive, and a
kind conflict cannot mutate the working registration. The sample-pipeline suite
then drives installed behavior through the explicit Produce → Propagate →
MergeInputs → Consume → Observe chain under manual and controlled live time,
with exact index/latency/gating/ordering observations. The seeded-context suite
loads real RON patches through core and umbrella APIs, constructs RNG state from
the patch seed/name before spawn returns, and compares two child processes with
a hard-coded six-line trace.

The combined package and workspace runs exercise all three surfaces together;
both all-features and no-default host matrices pass. Full Clippy, formatting,
strict rustdoc and `make check` pass. This demonstrates the arc composition at
its stated deterministic simulation boundary. It does not promote these tests
to hardware timing, physical MIDI or end-user acceptance.

## Composition ledger walk

| Row | CRC assembly result |
| --- | --- |
| E-01 | Ready: module installation/reinstallation, two-instance execution, accessors and conflict preservation reproduce on the final candidate. |
| E-02 | Ready: manual/live traces, five phases, gate edges, merge visibility, one-sample latency and name ordering reproduce after the ModuleId migration. |
| E-03 | Ready: real patch master seed reaches synchronous construction; two child processes match the fixed trace while seed/name/reorder controls distinguish the rejected behavior. |
| E-04 | Ready: all 3 slice ledgers and 18 child rows are closed; source scope, full gates, inherited gaps and successor routing are accounted for below. |

These are CRC assembly results. The [arc ledger](ledger.md) remains open because
CDC must independently reproduce composition before assigning final statuses.

## Accumulated plan changes

Arc-plan v1.1 recorded slice01 closure and opened the already designed sample
slice. V1.2 recorded slice02 closure after test-strength repair and opened the
already designed seed slice. V1.3 records the final slice close and composition
gate. None changed the CDC architecture, three-slice breakdown, acceptance
scope or successor sequence. The iterations repaired implementation evidence
within approved contracts.

## Bubble-up to project01

Arc09 supplies the execution, sample and seed prerequisites that the maintained
project roadmap assigns before arc10. It confirms that a Turing Machine adapter
can receive a deterministic seed at construction and run on the explicit sample
pipeline once installed. It does not demonstrate TM behavior or MIDI output;
those remain arc10 capability, with the musical Clock pulled forward there as
already planned. Clock/LFO/Scope remains arc11.

No unanticipated dependency or scope correction requires a project-plan
structural amendment. The silent-drop diff retains live-world persistence and
runtime operations in arc12, remaining catalog work in arc13, CLI/REPL/controller
work in Arc07, and coverage provenance/hardening, physical MIDI/timing,
benchmarks, soak and musical acceptance in Arc08/project gates.

CDC must now review this closing report, [CRC verification](crc-verification.md)
and [ledger](ledger.md). Only a successful `cdc-verification.md` may close arc09
and authorize CDC to open arc10 and its first slice.

## CDC review06 addendum — 2026-09-18

CDC rejected the preceding candidate-specific ready verdict after independently
finding that a mid-sample stop lets the old manual outer loop return and advance
three requested samples although core completes one. E-01/E-03 were reproduced;
E-02/E-04 remain open. The earlier composition statement and 3/3 denominator are
historical for source `642313a`, not the current arc disposition.

[Directive06](../cdc-directive06.md) adds
[slice04-sample-stop-accounting](slice04-sample-stop-accounting/slice-plan.md).
Its six M rows begin open, giving a final planned denominator of four slices and
24 child rows. The three prior slices/18 rows retain their candidate-specific CRC
acceptance. Arc09 stays open until CC repairs the stop boundary, CRC independently
closes the new slice and refreshes this assembly, and CDC accepts the new candidate.

The Slice01 row above incorrectly says “two corrective passes.” The preserved
history is: the original prompt was held and never executed, iteration01 was the
first execution, and iteration02 was one corrective pass (1 of 5). This addendum
corrects the accounting without rewriting the original report. Slice04 starts
with zero executed corrective refinements. Arc10 remains held.

## Repaired four-slice assembly — CRC, 2026-09-18

CRC independently accepted [Slice04](slice04-sample-stop-accounting/crc-verification.md)
at source `37deeace90e7fe2dff633ada9d58ef9051577c3d`. Its loop-head running check
and post-schedule completed counter repair the exact CDC failure: Produce,
Consume and Observe stops each finish one five-phase sample, return/count one
and advance Fixed/Virtual 1 ms; later stops, stopped no-ops, seeded resume and
false-then-true continuation also reproduce.

The current child denominator is 4/4 slices and 24/24 CRC-accepted rows. Slice01
used one corrective pass; Slice02 used one proof-repair pass; Slice03 and Slice04
used none. Both host workspace feature matrices, Clippy, format, strict docs,
native Make, dependency observation, diff and current planning checks pass on
the repaired candidate. Three RT tests remain ignored, and the retained coverage,
physical MIDI/timing, benchmark, soak and musical gates remain outside this arc.

CRC now judges E-01–E-04 operationally ready on `37deeace`. The arc remains open:
CDC must independently review this repaired candidate and assign the arc rows.
The historical CDC review of `642313a` is not reused as final acceptance, and
Arc10 remains held.
