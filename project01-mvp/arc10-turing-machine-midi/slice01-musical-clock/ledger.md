# Slice01 musical Clock — ledger

Opened 2026-09-18 before CC; six rows. All start open. CC proposes; CRC accepts.

| ID | Criterion | Verify | Significance | Origin | Status | Evidence | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| C-01 | Standalone validated clock emits exact integer/fractional sample pulses | Defaults, 120/4, 137/4 golden rising lists, width/low gap boundaries, invalid and canonical BPM cases | correctness-grade | Arc10 T-01; timing design | open | — | One call is one 1 ms sample |
| C-02 | Edits and pause/resume preserve the specified state transitions | Equal no-op; changed while high forces one low then fresh epoch; multiple edits/back-to-original; invalid no mutation; stopped state and resume continuation | correctness-grade | TM edge prerequisite; Arc09 stop lesson | open | — | Pending restart survives idempotent edit |
| C-03 | Installed Clock composes with real patch/cable/Consume edges | Two instances, twice-install and post-progress reinstall, manual/grouped Live, reordered spawn, exact per-sample input/output and edge counts | correctness-grade | Arc09 substrate; TM prerequisite | open | — | No device or TM claim |
| C-04 | Schema, spawn and runtime parameter access agree and fail cleanly | All setters/getters, aggregate override validation, unknown/type/nonfinite/range/overlap errors, pre-spawn entity count and post-invalid state preservation | correctness-grade | API/parameter contract | open | — | Patch BPM explicitly separate |
| C-05 | Packaging, docs and full gates pass with standalone compatibility | Default/rack feature builds and dependency trees, umbrella feature modes, workspace matrices, Clippy/fmt/strict docs, make check/outdated and lock audit | serious | Quality contract | open | — | No dependency upgrades or waived global gates |
| C-06 | Assignment, source scope and durable evidence support independent review | Intake/readback, exact file diff and revisions, all six report rows, current structural checker positive/negative controls, bubble-up | serious | Framework | open | — | CRC review required |
