# Live persistence — acceptance ledger

Opened2026-10-11. Five rows open; CC evidence is proposed-done until CRC acceptance.
Binding details and exact oracles: sibling cc-prompt-iteration01.md and slice-plan.md;
the initial cc-prompt.md governs its expressly retained sections.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LP-01 | Public parameter read/write operates on the live instance with correct validation and notification provenance | Real LFO edit between samples, previous/new DirectApi once; rejected/no-patch/unknown cases; Command provenance retained | correctness-grade | Live persistence prerequisite | open | CC iteration01 proposed done: TM configured-base getter and facade DirectApi/invalid-write oracles pass; timing, held modulation, and prior error/provenance cases pass; CRC pending | Existing setter semantics retained |
| LP-02 | Snapshot contains current active configuration and complete getter values with correct topology/metadata/header | Actual ECS edits, omitted defaults, active-handle restriction, cable order/tie oracle, missing getter and disabled-cable rejection | correctness-grade | Arc02 World-to-patch | open | CC iteration01 proposed done: core suite passes; snapshot/save/fresh-load preserves `.75,.25` cable order over two rounds; logical CableOrder and fallback schedule tests pass; CRC pending | No cached parameter fallback |
| LP-03 | Snapshot/save does not consume or reset running-rack state and failures propagate honestly | Before/after time/Scope/pending MIDI plus continuation control, failed path and destination untouched on extraction failure | correctness-grade | Sample/MIDI ownership | open | CC iteration01 proposed done: snapshot/save at samples 200 and 252 preserves running/sample/Scope/pending MIDI state; one-time drain and continuation match control; failure cases pass; CRC pending | Synchronous between samples |
| LP-04 | Saved seeded patch replays as its edited configuration in fresh racks | Two fresh saved-file runs versus independently edited reference; original differs; no claim of saved engine continuation | correctness-grade | Arc02 deterministic round trip | open | CC iteration01 proposed done: saved `.25` configuration matches independent reference; duplicate cable order survives two save/reload rounds; fresh-process traces compare as required; CRC pending | Configuration restart |
| LP-05 | Runnable edit/save example and docs expose the feature; affected checks pass | Execute new example then existing trace/workbench runner; default/no-default tests, workspace/lint/docs checks | serious | Playable-first delivery | open | CC iteration01 proposed done: example and fresh-process recipe pass; default/no-default suites, host `make test`, lint, strict Clippy/docs pass; restricted CoreMIDI failure retained with host retry; CRC pending | No endpoint gate/new validator |

## CDC conflict disposition — 2026-10-11

[Directive11](../../cdc-directive11.md) independently reproduces the replay failure
at a3633bc and confirms that the passing tie test currently asserts the wrong
replay result. LP-02/LP-04 remain unfulfilled; all five statuses stay open pending
CRC's independent full review. Correct base getter/event readback affects LP-01;
logical cable order affects LP-02/LP-04; recheck LP-03 preservation and complete
LP-05 gates after both repairs. Keep prior CC outcomes as attestation. No current
row is accepted or relaxed; CRC issued the preserved iteration01 assignment below.

## CRC correction issuance — 2026-10-11

Current assignment: `arc12-live-rack-operations/slice01-live-persistence/cc-prompt-iteration01.md`,
correction1/5, source a3633bc, author planning intake37d41f6. CRC reproduced the
LP-04 failed reference comparison (exit101, one failure), and the passing LP-02
diagnostic (exit0, one pass) that explicitly asserts the incorrect reversal.
The prompt repairs configured readback/events, logical ordering and discriminating
tests, and completes the original LP-03 snapshot/save preservation proof at200/252.
All five statuses stay open; initial CC evidence remains attestation and the
initial failure records remain historical. Native/fresh-process gates must
complete on the corrected source before CRC reviews. Subsequent slices held.

## CC iteration01 return — 2026-10-11

Source commit `fabfa1cb4beab5fd79f05b88d4bbec34208a1cac` implements the issued
corrections on the nine authorized source paths. The complete command inventory,
focused evidence, fresh-process recipe outcomes, and environment-limited `make
test` first run plus exact successful host retry are preserved in the sibling
closing report. LP-01–LP-05 are each proposed done by CC, not accepted. All rows
remain `open` until CRC independently reviews the corrected source and evidence.
The initial discrepancy report and CRC correction history above remain intact.
