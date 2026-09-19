#!/usr/bin/env python3
"""Bounded Slice02 packet checks; structural evidence, not Rust acceptance."""

from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[4]
SLICE = Path("project01-mvp/arc10-turing-machine-midi/slice02-turing-machine-adapter")
ASSIGNMENT = "arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt.md"
FILES = [SLICE / name for name in ("slice-plan.md", "ledger.md", "cc-prompt.md")]
FILES += [SLICE.parent / name for name in ("arc-plan.md", "design.md", "ledger.md")]
FILES += [Path("project01-mvp/cdc-directive07.md")]

OUTPUT_MARKERS = (
    "`step`", "`note`, `note_valid`", "`velocity`, `velocity_valid`",
    "`gate`", "`scale_note`, `scale_note_valid`", "`pulse_1`…`pulse_6`",
    "`gate_1`…`gate_8`", "`div2`, `div4`", "`noise_cc`",
    "`register_bits`", "`length`", "`write_probability`",
)
ACTION_MARKERS = (
    "`with_seed(u64)`", "`tick()`", "`move_step()`", "`reset()`",
    "`set_write(f32)`", "`modulate_write(f32)`", "`set_length(usize)`",
    "`set_length_position(usize)`", "`set_scale(Scale)`",
    "`set_note_range(RangeInclusive<u8>)`", "`set_scale_output_scale(Scale)`",
)
SOURCE_PATHS = (
    "crates/oxurack-mod-turingmachine/Cargo.toml",
    "crates/oxurack-mod-turingmachine/README.md",
    "crates/oxurack-mod-turingmachine/src/lib.rs",
    "crates/oxurack-mod-turingmachine/src/engine.rs",
    "crates/oxurack-mod-turingmachine/src/quantizer.rs",
    "crates/oxurack-mod-turingmachine/src/rack.rs",
    "crates/oxurack/Cargo.toml",
    "crates/oxurack/tests/turing_machine.rs",
    "crates/oxurack/tests/fixtures/turing-machine.ron",
    "Cargo.lock",
)


def validate(texts):
    errors = []
    plan = texts[FILES[0]]
    ledger = texts[FILES[1]]
    prompt = texts[FILES[2]]
    if f"Current assignment:\n`{ASSIGNMENT}`" not in plan:
        errors.append("current assignment mismatch")
    if ASSIGNMENT not in prompt:
        errors.append("prompt identity mismatch")
    rows = [line.split("|") for line in ledger.splitlines()
            if re.match(r"^\| A-\d\d \|", line)]
    if [row[1].strip() for row in rows] != [f"A-{n:02}" for n in range(1, 7)]:
        errors.append("six-row denominator mismatch")
    if any(row[6].strip() not in ("open", "done") for row in rows):
        errors.append("invalid row status")
    for marker in OUTPUT_MARKERS + ACTION_MARKERS:
        if marker not in plan:
            errors.append(f"missing inventory mapping: {marker}")
    if "4 inputs plus 28 outputs" not in plan or "complete 32-port schema" not in plan:
        errors.append("port denominator mismatch")
    if "complete parameter schema" not in plan or "| `scale_output_root` |" not in plan:
        errors.append("parameter inventory incomplete")
    for path in SOURCE_PATHS:
        if path not in plan:
            errors.append(f"source allowlist entry missing: {path}")
    for path, body in texts.items():
        for target in re.findall(r"\[[^\]]+\]\(([^)]+)\)", body):
            if target.startswith(("https:", "http:", "#")):
                continue
            filename = target.split("#", 1)[0]
            if filename and not (ROOT / path.parent / filename).exists():
                errors.append(f"broken link: {path}: {target}")
    return errors


if __name__ == "__main__":
    texts = {path: (ROOT / path).read_text() for path in FILES}
    errors = validate(texts)
    if errors:
        raise SystemExit("\n".join(errors))
    print("Slice02 packet passed: assignment, six rows, mappings, denominators, links")
    for name in ("missing-row", "stale-assignment", "missing-mapping", "invalid-status"):
        changed = dict(texts)
        if name == "missing-row":
            changed[FILES[1]] = re.sub(r"^\| A-06 \|.*\n", "", changed[FILES[1]], flags=re.M)
        elif name == "stale-assignment":
            changed[FILES[0]] = changed[FILES[0]].replace(ASSIGNMENT, "wrong/cc-prompt.md")
        elif name == "missing-mapping":
            changed[FILES[0]] = changed[FILES[0]].replace("`noise_cc`", "noise_cc")
        else:
            changed[FILES[1]] = changed[FILES[1]].replace("| open |", "| invalid |", 1)
        assert validate(changed), f"negative control accepted: {name}"
        print(f"negative control rejected: {name}")
