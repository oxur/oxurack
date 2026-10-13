# Explicit rack lifetimes — acceptance ledger

Opened2026-10-12 before implementation. Six rows. Current CRC2026-10-12 review
at1242a60 accepts LF-05/LF-06; LF-01–LF-04 remain open for the remaining
iteration01 interaction observations. Active CC assignment is iteration02.
See crc-verification.md for fresh checks and retained historical evidence.
No slice, ancestor or historical acceptance is inferred.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LF-01 | Isolated replacement retires exact old membership/reservations and preserves the entire old owner on recoverable candidate failure | Same-channel repeats, valid[1,0], rejected[1,1], late valid spawner writes then Err; World/index/graph/messages/Scope/control comparison | correctness-grade | directive12 D1/D4; reproduced replacement leak | open | `tests/lifecycle.rs`: `repeated_real_patch_replacement_matches_fresh_structure_and_graph`, `candidate_sink_reservations_are_isolated_from_the_live_owner`, `late_spawner_failure_isolated_and_old_engine_continues` | CRC open: remaining F-01 resource/scalar control/normalized membership; see iteration02 and CRC review. |
| LF-02 | Installed types replay deterministically once per App, with current-lifetime idempotence and collision preflight | Ordered hook/resources and actual system runs; failed-candidate hooks separately; setter/getter/schedule availability after unload/reload | correctness-grade | directive12 D1 installation amendment | open | `tests/lifecycle.rs`: `install_recipes_keep_order_reinstall_and_collision_contract`; late failure records hook calls per App | CRC open: remaining F-02 ordered every-App/both-instance replay; see iteration02 and CRC review. |
| LF-03 | Narrow reset preserves unread events, engine/time/Scope/running/first fault and atomically appends bounded exact silence once | Literal note0→cleanup1, channel order, CC-only/released/unused,16 sinks80+48=128, prepare/room failure unchanged, repeat/no-allocation checks | correctness-grade | directive12 D2 | open | `oxurack-mod-midiout` tests: `silence_is_channel_ordered_fixed_capacity_and_idempotent`, `failed_silence_room_check_preserves_pending_and_dirty_state_for_retry`, `sixteen_sinks_fit_eighty_normal_plus_forty_eight_cleanup_events`; facade reset/restart test; fault preservation unit test | CRC open: remaining F-03 actual repeated reset/state/retrigger/full-buffer error; see iteration02 and CRC review. |
| LF-04 | Admission, unload, current-snapshot restart and fault recovery enforce explicit lifetime semantics | Running-first/no-I/O, pending count, failed snapshot/candidate, empty unload, stopped fresh0, fault recovery, phase/Scope and controlled Live no burst | correctness-grade | directive12 D1/D2 matrix | open | `tests/lifecycle.rs`: lifecycle precedence, pending rejection, live snapshot restart with Manual driver retained, unload/reload, late failure; `persistence.rs` getterless restart failure; `lifecycle.rs` fault retention/recovery | CRC open: remaining F-04 restart/phase/faulted-candidate/empty lifetime; see iteration02 and CRC review. |
| LF-05 | Public/documented failure and notification boundaries are honest and preserve compatibility | Typed lifecycle errors, Core Panic/SetBpm NotImplemented, candidate messages/PatchLoaded, loader production unchanged, hook/lifetime/event docs | correctness-grade | directive12 D3 honesty exception/D4 | done | reproduced: CRC2026-10-12 at1242a60, `crc-verification.md` Iteration01 independent review | CRC accepted this row; see the row-specific checks in the review. Whole slice/parent gates remain open. |
| LF-06 | Real edit/save/unload/reload/Scope musical session and affected native gates pass with inherited scope accounted for | Existing --lifecycle example both modes, saved fresh reference/Scope/trace, original hashes, core/MIDI/facade/workspace/lint/Clippy/docs/build; row walk | serious | playable-first directive08; directive12 D4; Arc05 carryover | done | reproduced: CRC2026-10-12 at1242a60, `crc-verification.md` Iteration01 independent review | CRC accepted this row; see the row-specific checks in the review. Whole slice/parent gates remain open. |

## Iteration01 CC evidence note

The corrective source commit is `1242a60fa3a997f95b29195de3b28f7e9a2864ff`.
It adds the missing lifecycle boundary interactions, exact candidate notification
payload and rejection checks, real MIDI reset/capacity cases, unsupported core
command non-mutation coverage, and example-dispatch/capture tests. The lifecycle
diagnostic now reports the saved retired boundary (300) and then the new
lifetime boundary (0). Final gates and reproducible trace receipts are recorded
in `closing-report.md` under “Iteration01 evidence and proposed disposition”.
CC proposes all six rows done; **every row remains open for CRC disposition**.
Slice05 remains held. No inherited Arc05 row, Arc12 parent row, or release gate
is closed by this note.

## CC iteration02 proposal — 2026-10-12

Source test-only commit `03825e5077b7cd621185ae70b47020da321706c4` adds the
four remaining lifecycle interaction observations and the 128-event pending
buffer oracle. It changes only the authorized test regions in
`crates/oxurack/src/lib.rs`, `crates/oxurack/src/lifecycle.rs`,
`crates/oxurack/tests/lifecycle.rs`, and
`crates/oxurack-mod-midiout/src/lib.rs`. No product defect was reproduced and
no production behavior changed. Exact row evidence and final commands are in
`closing-report.md` under “Iteration02 evidence and proposed disposition”.

| LF row | CC proposal | CRC status |
| --- | --- | --- |
| LF-01 | done | open pending independent disposition |
| LF-02 | done | open pending independent disposition |
| LF-03 | done | open pending independent disposition |
| LF-04 | done | open pending independent disposition |
| LF-05 | retain independently accepted done | accepted at1242a60 |
| LF-06 | retain independently accepted done | accepted at1242a60 |

All prescribed iteration02 gates pass. Restricted `make test` retains four
CoreMIDI initialization failures (146 pass, 3 existing ignored); its unchanged
authorized host retry passes (RT 150 pass, 0 fail, 3 ignored; full workspace
and doctests pass). LF-01–LF-04 remain **open** for CRC. Arc05 A-05-01/A-05-02,
Arc12 R-01–R-04, project/release gates and Slice04 acceptance remain open;
Slice05 stays held. This CC proposal is not CRC acceptance.

## CRC iteration01 disposition — 2026-10-12

Six rows accounted for:2 independently done,4 open. CC iteration01 note above is
retained historical proposed evidence. Fresh affected tests/native commands/host
retry and actual72-row example replay pass at1242a60. The remaining assertions
were prescribed in iteration01 and are collected in iteration02; no new feature
contract or implementation defect is claimed. Slice05/R-01–R-04 stay held/open.
