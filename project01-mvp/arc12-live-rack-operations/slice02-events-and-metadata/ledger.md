# Live events and metadata — acceptance ledger

Opened2026-10-12 before implementation. Five rows open; CC evidence remains
proposed done until CRC independent acceptance. Detailed binding contract in
sibling cc-prompt.md; plan in slice-plan.md. No ancestor/legacy acceptance inferred.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| M-01 | Public label read/edit selects only the active rack/module/one cable with defined empty/tag/note semantics | Raw readback on all targets, clear/equal, typed tags/notes, parallel ordinal/Entity and missing/stale selector cases | correctness-grade | Arc12 Slice02; Arc03 labels | open | — | Occurrence is view-relative; no public ECS handle |
| M-02 | Every successful edit emits once with exact previous/new and actual provenance; failures emit nothing | Real shared helper/explicit command/direct facade calls; mixed operation-boundary consumer; typed failures and before/after state | correctness-grade | Arc03 notification tier; arc provenance | open | — | No implicit command queue execution/global MessageId order claim |
| M-03 | Edits inspect immediately and persist/reload without altering schema or musical configuration | Fresh graph labels/default mode; raw public read both modes; snapshot/save/fresh load all fields, tag ordering and note order | correctness-grade | Arc03 literate patching; Slice01 save | open | — | Empty entity labels absent; rack raw defaults explicit |
| M-04 | Read/edit/rejected edit preserves running-rack state and sole MIDI ownership | Actual fixture at200/pending252; time/transport/Scope records/admission before-after; one drain event251; unchanged continuation control | correctness-grade | Arc12 synchronous owner contract | open | — | No tick/drain/engine reset or topology/order change |
| M-05 | Runnable annotation/replay/docs and native feature gates pass; inherited obligations retain explicit accounting | Execute example and two fresh traces, original pinned hashes; native core/facade/workspace/lint/docs; six Arc03 inherited rows walked | serious | Playable-first policy; Arc03 carryover | open | — | Lifecycle/removal and final composition remain Slice04/Arc12; no endpoint gate |
