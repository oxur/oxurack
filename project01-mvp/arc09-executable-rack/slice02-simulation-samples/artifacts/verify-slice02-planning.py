#!/usr/bin/env python3
"""Validate the current slice02 planning close set.

This checker is deliberately limited to current-slice planning structure. It
does not inspect Rust behavior and it cannot establish CRC independence or
slice acceptance.
"""

from __future__ import annotations

import argparse
import re
import shutil
import tempfile
from pathlib import Path


ASSIGNMENT = "arc09-executable-rack/slice02-simulation-samples/cc-prompt-iteration01.md"
ROW_IDS = {f"S-{number:02d}" for number in range(1, 7)}


def check(condition: bool, message: str) -> None:
    if not condition:
        raise RuntimeError(message)


def ledger_rows(text: str) -> dict[str, list[str]]:
    result: dict[str, list[str]] = {}
    for line in text.splitlines():
        if not re.match(r"^\| S-[0-9]+ \|", line):
            continue
        cells = [cell.strip() for cell in line.split("|")[1:-1]]
        check(cells[0] not in result, f"duplicate ledger row: {cells[0]}")
        result[cells[0]] = cells
    return result


def check_local_links(slice_dir: Path) -> int:
    checked = 0
    for document in slice_dir.rglob("*.md"):
        if "logs" in document.parts:
            continue
        for target in re.findall(r"\[[^\]]*\]\(([^)]+)\)", document.read_text()):
            if "://" in target or target.startswith("#") or target.startswith("mailto:"):
                continue
            path = target.split("#", 1)[0]
            if not path:
                continue
            check(
                (document.parent / path).resolve().exists(),
                f"broken local link: {document}: {target}",
            )
            checked += 1
    return checked


def validate(slice_dir: Path) -> int:
    slice_dir = slice_dir.resolve()
    plan = slice_dir / "slice-plan.md"
    ledger = slice_dir / "ledger.md"
    close = slice_dir / "closing-report.md"
    artifacts = slice_dir / "artifacts"

    check(plan.is_file(), "missing slice-plan.md")
    check(ledger.is_file(), "missing ledger.md")
    check((slice_dir / "cc-prompt.md").is_file(), "missing preserved cc-prompt.md")
    check(
        (slice_dir / "cc-prompt-iteration01.md").is_file(),
        "missing preserved cc-prompt-iteration01.md",
    )
    check(close.is_file(), "missing canonical root closing-report.md")
    check(artifacts.is_dir(), "missing artifacts directory")
    check((artifacts / "test-evidence.md").is_file(), "missing intake/test evidence")
    check((artifacts / "validation-results.tsv").is_file(), "missing validation matrix")
    check((artifacts / "verify-slice02-planning.py").is_file(), "missing current-slice checker")
    logs = sorted((artifacts / "logs").glob("*.log"))
    check(logs, "missing raw validation logs")

    plan_text = plan.read_text()
    assignments = [
        line for line in plan_text.splitlines() if line.startswith("Current assignment:")
    ]
    check(assignments == [f"Current assignment: `{ASSIGNMENT}`"], "stale or malformed current assignment")
    history = [
        line
        for line in plan_text.splitlines()
        if line.startswith("|")
        and len(line.split("|")) > 2
        and line.split("|")[2].strip() == "cc-prompt-iteration01.md"
    ]
    check(len(history) == 1, "current assignment history row mismatch")

    rows = ledger_rows(ledger.read_text())
    check(set(rows) == ROW_IDS, f"ledger IDs are not exactly {sorted(ROW_IDS)}")
    check(all(row[5] == "open" for row in rows.values()), "ledger status was upgraded")

    close_text = close.read_text()
    for required in ("iteration01", "S-01", "S-06", "proposed-done", "Bubble-up to arc09"):
        check(required in close_text, f"canonical close report missing: {required}")
    for required in (
        "artifacts/test-evidence.md",
        "artifacts/validation-results.tsv",
        "artifacts/logs/",
    ):
        check(required in close_text, f"close report inventory missing: {required}")

    links = check_local_links(slice_dir)
    print(f"PASS: current assignment, preserved prompts, six open ledger rows and canonical root close")
    print(f"PASS: artifact inventory has {len(logs)} raw logs and {links} checked local links")
    print("Structural self-check only; behavior, CRC reproduction and slice acceptance remain pending")
    return 0


def expected_failure(slice_dir: Path, name: str, mutate) -> int:
    with tempfile.TemporaryDirectory(prefix=f"slice02-{name}-") as directory:
        case = Path(directory) / "slice02"
        shutil.copytree(slice_dir, case)
        mutate(case)
        try:
            validate(case)
        except (OSError, RuntimeError) as error:
            print(f"{name}: exit=1 ({error})")
            return 1
        print(f"{name}: exit=0 (unexpected pass)")
        return 0


def run_negative_cases(slice_dir: Path) -> int:
    valid = validate(slice_dir)
    missing_close = expected_failure(
        slice_dir,
        "missing-root-close",
        lambda case: (case / "closing-report.md").unlink(),
    )

    def stale_assignment(case: Path) -> None:
        plan = case / "slice-plan.md"
        text = plan.read_text().replace(
            f"Current assignment: `{ASSIGNMENT}`",
            "Current assignment: `arc09-executable-rack/slice02-simulation-samples/cc-prompt.md`",
        )
        plan.write_text(text)

    stale = expected_failure(slice_dir, "stale-assignment", stale_assignment)
    print(f"valid_exit={valid}")
    print(f"missing_close_exit={missing_close}")
    print(f"stale_assignment_exit={stale}")
    return 0 if valid == 0 and missing_close == 1 and stale == 1 else 1


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parent.parent)
    parser.add_argument(
        "--negative-cases",
        action="store_true",
        help="run the valid predicate and its missing-close/stale-assignment controls",
    )
    args = parser.parse_args()
    try:
        return run_negative_cases(args.root) if args.negative_cases else validate(args.root)
    except (OSError, RuntimeError) as error:
        print(f"FAIL: {error}")
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
