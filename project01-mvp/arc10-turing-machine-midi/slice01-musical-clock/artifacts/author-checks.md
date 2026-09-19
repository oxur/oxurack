# CDC author checks — 2026-09-18

Source 37deeace; planning intake f6c778e. This is design self-review, not
independent acceptance of Clock (which is not implemented).

The eight 137-BPM/4-PPQN rising indices were calculated from integer ceiling
`(k*60_000_000 + 548_000 - 1) // 548_000`, k=0..7:
0,110,219,329,438,548,657,767. Repeated rounded intervals cannot match.
Default120/1 gives period500; width10 covers0..9. At1000/30, period2 and width1
leaves one low; width2 violates integer-floor low-gap validation. At240/1,
fresh epoch after edit's low sample3 starts4 and next rise254.

Verified actual APIs: CorePlugin targets Update; CorePlugin::fixed targets
FixedUpdate. No custom-schedule constructor exists, so the prompt uses these
two real APIs. Oxurack::into_app consumes the wrapper and its World accessors
are private. Wrapper integration therefore retains it for reinstall/stop/resume;
parameter tests use core App/World inside the new adapter's unit tests.
apply_patch_to_world passes the full override map to spawn and emits messages;
it does not initialize state by invoking parameter setters.

Clock structural checker passes assignment, six rows/statuses, file/link checks
and rejects missing-row, stale-assignment and invalid-status controls. This does
not validate Rust, behavior or future CRC acceptance. No required production
source changed during authoring. Six readiness dispositions are in slice-plan.md.
