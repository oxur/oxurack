# Live persistence — acceptance ledger

Opened2026-10-11. Five rows open; CC evidence is proposed-done until CRC acceptance.
Binding details and exact oracles: sibling cc-prompt.md and slice-plan.md.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LP-01 | Public parameter read/write operates on the live instance with correct validation and notification provenance | Real LFO edit between samples, previous/new DirectApi once; rejected/no-patch/unknown cases; Command provenance retained | correctness-grade | Live persistence prerequisite | open | — | Existing setter semantics retained |
| LP-02 | Snapshot contains current active configuration and complete getter values with correct topology/metadata/header | Actual ECS edits, omitted defaults, active-handle restriction, cable order/tie oracle, missing getter and disabled-cable rejection | correctness-grade | Arc02 World-to-patch | open | — | No cached parameter fallback |
| LP-03 | Snapshot/save does not consume or reset running-rack state and failures propagate honestly | Before/after time/Scope/pending MIDI plus continuation control, failed path and destination untouched on extraction failure | correctness-grade | Sample/MIDI ownership | open | — | Synchronous between samples |
| LP-04 | Saved seeded patch replays as its edited configuration in fresh racks | Two fresh saved-file runs versus independently edited reference; original differs; no claim of saved engine continuation | correctness-grade | Arc02 deterministic round trip | open | — | Configuration restart |
| LP-05 | Runnable edit/save example and docs expose the feature; affected checks pass | Execute new example then existing trace/workbench runner; default/no-default tests, workspace/lint/docs checks | serious | Playable-first delivery | open | — | No endpoint gate/new validator |
