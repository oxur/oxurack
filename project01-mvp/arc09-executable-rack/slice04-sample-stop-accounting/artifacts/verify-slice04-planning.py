#!/usr/bin/env python3
"""Verify Slice04 planning structure and exercise bounded negative controls."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path


SLICE = Path("project01-mvp/arc09-executable-rack/slice04-sample-stop-accounting")
ARC = Path("project01-mvp/arc09-executable-rack")
PROJECT = Path("project01-mvp/project-plan.md")
STATUS = Path("status/project01-mvp.md")
ASSIGNMENT = "arc09-executable-rack/slice04-sample-stop-accounting/cc-prompt.md"
ROW_IDS = tuple(f"M-0{number}" for number in range(1, 7))
LINK_RE = re.compile(r"\[[^\]]+\]\(([^)]+)\)")


def repository_root() -> Path:
    result = subprocess.run(
        ["git", "rev-parse", "--show-toplevel"],
        check=True,
        capture_output=True,
        text=True,
    )
    return Path(result.stdout.strip())


def read(root: Path, relative: Path) -> str:
    path = root / relative
    if not path.is_file():
        raise ValueError(f"missing required file: {relative}")
    return path.read_text(encoding="utf-8")


def markdown_links(root: Path, relative: Path, text: str) -> list[str]:
    failures: list[str] = []
    for target in LINK_RE.findall(text):
        target = target.strip()
        if target.startswith(("http://", "https://", "mailto:", "#")):
            continue
        target_path = target.split("#", 1)[0]
        if not target_path:
            continue
        resolved = (root / relative.parent / target_path).resolve()
        if not resolved.exists():
            failures.append(f"broken link in {relative}: {target}")
    return failures


def validate_texts(texts: dict[Path, str]) -> list[str]:
    failures: list[str] = []
    plan = texts[SLICE / "slice-plan.md"]
    ledger = texts[SLICE / "ledger.md"]
    prompt = texts[SLICE / "cc-prompt.md"]
    arc_plan = texts[ARC / "arc-plan.md"]
    arc_ledger = texts[ARC / "ledger.md"]
    project = texts[PROJECT]
    status = texts[STATUS]

    if f"Current assignment: `{ASSIGNMENT}`" not in plan:
        failures.append("slice plan lacks the exact current assignment")
    if ASSIGNMENT not in prompt:
        failures.append("CC prompt lacks the exact current assignment")
    if "slice04-sample-stop-accounting" not in arc_plan:
        failures.append("arc plan lacks Slice04")
    if "four-slice/24-row denominator" not in arc_ledger:
        failures.append("arc ledger lacks the four-slice/24-row denominator")

    ledger_rows = re.findall(r"^\| (M-\d{2}) \|.*?\| (open|done) \|", ledger, re.MULTILINE)
    actual_ids = tuple(row_id for row_id, _ in ledger_rows)
    if actual_ids != ROW_IDS:
        failures.append(f"ledger IDs are {actual_ids!r}, expected {ROW_IDS!r}")

    status_by_row = {row_id: row_status for row_id, row_status in ledger_rows}
    plan_is_open = "Status: open" in plan
    if plan_is_open:
        wrong = [row_id for row_id in ROW_IDS if status_by_row.get(row_id) != "open"]
        if wrong:
            failures.append(f"open slice has non-open rows: {', '.join(wrong)}")
        for relative, body in ((PROJECT, project), (STATUS, status)):
            if ASSIGNMENT not in body:
                failures.append(f"open assignment missing from {relative}")
    else:
        wrong = [row_id for row_id in ROW_IDS if status_by_row.get(row_id) != "done"]
        if wrong:
            failures.append(f"closed slice has non-done rows: {', '.join(wrong)}")
        if ARC / "slice04-sample-stop-accounting/crc-verification.md" not in texts:
            failures.append("closed slice lacks CRC verification")

    report_path = SLICE / "closing-report.md"
    if report_path in texts:
        report = texts[report_path]
        if "proposed-done" not in report:
            failures.append("CC closing report lacks proposed-done status")
        for row_id in ROW_IDS:
            if row_id not in report:
                failures.append(f"CC closing report lacks {row_id}")
        for evidence_path in (
            SLICE / "artifacts/test-evidence.md",
            SLICE / "artifacts/validation-results.tsv",
        ):
            if evidence_path not in texts:
                failures.append(f"CC closing report exists without {evidence_path}")

    return failures


def load_texts(root: Path) -> dict[Path, str]:
    required = (
        SLICE / "slice-plan.md",
        SLICE / "ledger.md",
        SLICE / "cc-prompt.md",
        SLICE / "artifacts/verify-slice04-planning.py",
        ARC / "arc-plan.md",
        ARC / "ledger.md",
        PROJECT,
        STATUS,
    )
    texts = {relative: read(root, relative) for relative in required}
    for optional in (
        SLICE / "closing-report.md",
        SLICE / "crc-verification.md",
        SLICE / "artifacts/test-evidence.md",
        SLICE / "artifacts/validation-results.tsv",
    ):
        if (root / optional).is_file():
            texts[optional] = read(root, optional)
    return texts


def validate(root: Path, texts: dict[Path, str]) -> list[str]:
    failures = validate_texts(texts)
    linked_files = (
        SLICE / "slice-plan.md",
        SLICE / "ledger.md",
        SLICE / "cc-prompt.md",
        ARC / "arc-plan.md",
        ARC / "ledger.md",
        SLICE / "closing-report.md",
    )
    for relative in linked_files:
        if relative in texts:
            failures.extend(markdown_links(root, relative, texts[relative]))
    return failures


def run_negative_cases(texts: dict[Path, str]) -> list[str]:
    failures: list[str] = []
    cases: list[tuple[str, dict[Path, str]]] = []

    missing_row = dict(texts)
    missing_row[SLICE / "ledger.md"] = re.sub(
        r"^\| M-06 \|.*\n", "", missing_row[SLICE / "ledger.md"], flags=re.MULTILINE
    )
    cases.append(("missing-row", missing_row))

    stale_assignment = dict(texts)
    stale_assignment[SLICE / "slice-plan.md"] = stale_assignment[
        SLICE / "slice-plan.md"
    ].replace(ASSIGNMENT, "arc09-executable-rack/slice03-seeded-spawn-context/cc-prompt.md")
    cases.append(("stale-assignment", stale_assignment))

    invalid_status = dict(texts)
    replacement = "done" if "Status: open" in texts[SLICE / "slice-plan.md"] else "open"
    current = "open" if replacement == "done" else "done"
    invalid_status[SLICE / "ledger.md"] = re.sub(
        rf"(^\| M-01 \|.*?\| ){current}( \|)",
        rf"\g<1>{replacement}\g<2>",
        invalid_status[SLICE / "ledger.md"],
        count=1,
        flags=re.MULTILINE,
    )
    cases.append(("invalid-row-status", invalid_status))

    for name, mutated in cases:
        if validate_texts(mutated):
            print(f"negative control passed: {name}")
        else:
            failures.append(f"negative control was not rejected: {name}")
    return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--negative-cases", action="store_true")
    args = parser.parse_args()

    root = repository_root()
    texts = load_texts(root)
    failures = validate(root, texts)
    if failures:
        for failure in failures:
            print(f"FAIL: {failure}", file=sys.stderr)
        return 1
    print("baseline planning verification passed")

    if args.negative_cases:
        negative_failures = run_negative_cases(texts)
        if negative_failures:
            for failure in negative_failures:
                print(f"FAIL: {failure}", file=sys.stderr)
            return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
