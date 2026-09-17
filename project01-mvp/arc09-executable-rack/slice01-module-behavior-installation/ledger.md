# Slice01 — implementation acceptance ledger

Opened 2026-09-17 before CC. Six rows; all open. CC evidence is attested and
proposed-done; CRC independently checks the exact source/planning candidate.
Deferred/no-op dispositions need a concrete reason and re-entry, never a weaker test.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| I-01 | Legacy module implementations compile; behavior hook is invoked once per type per rack | Fixture with real Produce system; two instances and two racks; count both hook registrations and actual per-instance work | correctness-grade | CDC directive03 / arc09 contract | open | — | CRC verification pending |
| I-02 | Repeated installation is idempotent and kind conflicts leave the original rack intact | Reinstall after state/parameter changes; different Rust type with same kind fails before mutation; verify original spawner, metadata, setters/getters and system behavior still operate | correctness-grade | CDC directive03 / arc09 contract | open | — | CRC verification pending |
| I-03 | Loaded instances execute on the supplied fixed schedule with graph both on and off | Actual RON load of two instances; direct Update does not run their system and FixedUpdate does; exact per-instance counts, no install/spawn multiplication | correctness-grade | CDC directive03 / arc09 contract | open | — | CRC verification pending |
| I-04 | Public API and existing docs pass strict rustdoc without behavior changes in doc-only files | Both doc commands exit 0; inspect three bounded legacy doc-only diffs and docs for try_install/hook/legacy panic contract | serious | CDC directive03 / arc09 contract | open | — | CRC verification pending |
| I-05 | Required runnable tests, lint, format and documentation gates pass; unavailable gates remain explicit blockers | Run all listed gates preserving failures/reruns; no silent skip, changed dependencies, new ignored tests or weakened coverage target | correctness-grade | CDC directive03 / arc09 contract | open | — | CRC verification pending |
| I-06 | Source/planning scope and complete evidence support independent acceptance | Inspect explicit commits, six-row closing walk, artifact inventory and bubble-up; preserve original carryovers and pending sample/seed/MIDI contracts | serious | CDC directive03 / arc09 contract | open | — | CRC verification pending |
