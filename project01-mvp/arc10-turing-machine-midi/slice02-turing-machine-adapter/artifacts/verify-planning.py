#!/usr/bin/env python3
"""Bounded Slice02 packet checks; structural evidence, not Rust acceptance."""

from pathlib import Path
import argparse
import re

ROOT = Path(__file__).resolve().parents[4]
SLICE = Path("project01-mvp/arc10-turing-machine-midi/slice02-turing-machine-adapter")
ASSIGNMENT = "arc10-turing-machine-midi/slice02-turing-machine-adapter/cc-prompt-iteration01.md"
FILES = [SLICE / name for name in ("slice-plan.md", "ledger.md", "cc-prompt-iteration01.md")]
FILES += [SLICE.parent / name for name in ("arc-plan.md", "design.md", "ledger.md")]
FILES += [Path("project01-mvp/cdc-directive07.md")]
CLOSE = SLICE / "closing-report.md"
FILES += [CLOSE]

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


def validate(texts, require_return=False):
    errors = []
    plan = texts[FILES[0]]
    ledger = texts[FILES[1]]
    prompt = texts[FILES[2]]
    if f"Current assignment:\n`{ASSIGNMENT}`" not in plan:
        errors.append("current assignment mismatch")
    if ASSIGNMENT not in prompt:
        errors.append("prompt identity mismatch")
    close = texts.get(CLOSE, "")
    if not close.strip():
        errors.append("missing canonical closing report")
    if require_return:
        if ASSIGNMENT not in close:
            errors.append("return assignment mismatch")
        if "## Bubble-up to arc" not in close:
            errors.append("missing return bubble-up disposition")
        if not re.search(r"^Source commit: `[0-9a-f]{40}`\.?$", close, flags=re.M):
            errors.append("missing full source provenance hash")
        close_rows = re.findall(r"^\| (A-\d\d) \|", close, flags=re.M)
        if close_rows != [f"A-{n:02}" for n in range(1, 7)]:
            errors.append("current return six-row walk mismatch")
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
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--require-return", action="store_true",
                        help="also require iteration01 return identity, provenance, rows and bubble-up")
    args = parser.parse_args()
    texts = {path: (ROOT / path).read_text() if (ROOT / path).is_file() else ""
             for path in FILES}
    errors = validate(texts, args.require_return)
    if errors:
        raise SystemExit("\n".join(errors))
    print("Slice02 packet passed: assignment, six rows, mappings, denominators, links, canonical close")
    if args.require_return:
        print("iteration01 return passed: identity, source hash, six rows, bubble-up (structural only)")
    controls = ["missing-row", "stale-assignment", "missing-mapping", "invalid-status", "missing-close"]
    if args.require_return:
        controls += ["stale-return", "missing-bubble-up", "missing-return-row", "missing-source-hash"]
    for name in controls:
        changed = dict(texts)
        if name == "missing-row":
            changed[FILES[1]] = re.sub(r"^\| A-06 \|.*\n", "", changed[FILES[1]], flags=re.M)
        elif name == "stale-assignment":
            changed[FILES[0]] = changed[FILES[0]].replace(ASSIGNMENT, "wrong/cc-prompt.md")
        elif name == "missing-mapping":
            changed[FILES[0]] = changed[FILES[0]].replace("`noise_cc`", "noise_cc")
        elif name == "invalid-status":
            changed[FILES[1]] = re.sub(r"\| (open|done) \|", "| invalid |", changed[FILES[1]], count=1)
        elif name == "missing-close":
            changed[CLOSE] = ""
        elif name == "stale-return":
            changed[CLOSE] = changed[CLOSE].replace(ASSIGNMENT, "wrong/cc-prompt.md")
        elif name == "missing-bubble-up":
            changed[CLOSE] = changed[CLOSE].replace("## Bubble-up to arc", "## Remaining work")
        elif name == "missing-return-row":
            changed[CLOSE] = re.sub(r"^\| A-06 \|.*\n", "", changed[CLOSE], flags=re.M)
        elif name == "missing-source-hash":
            changed[CLOSE] = re.sub(r"\b[0-9a-f]{40}\b", "short-hash", changed[CLOSE])
        assert validate(changed, args.require_return), f"negative control accepted: {name}"
        print(f"negative control rejected: {name}")
