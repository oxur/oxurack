"""Current planning structure; never behavioral or independent implementation acceptance."""
import csv
from pathlib import Path
import re
import subprocess

HERE = Path(__file__).resolve().parent
PROJECT = HERE.parents[2]
WORKTREE = PROJECT.parent
BASELINE = "f0cf3a5172e02c717c7f924c84156f5fe0405733"


def check(value, message):
    if not value:
        raise RuntimeError(message)


def git(*args):
    return subprocess.check_output(["git", *args], cwd=WORKTREE)


def rows(path):
    result = {}
    for line in path.read_text().splitlines():
        if re.match(r"^\| (?:S|A|R|P|E|I)-[0-9-]+ \|", line):
            cells = [x.strip() for x in line.split("|")[1:-1]]
            check(cells[0] not in result, f"Duplicate ledger row: {path}:{cells[0]}")
            result[cells[0]] = cells
    return result


def check_current_assignment(plan_text: str, slice_dir: Path) -> Path:
    """Validate the explicit current prompt field and its history row."""
    fields = [
        line for line in plan_text.splitlines()
        if line.startswith("Current assignment:")
    ]
    check(len(fields) == 1, "Current assignment field count mismatch")

    match = re.fullmatch(r"Current assignment: `([^`]+)`", fields[0])
    check(match is not None, "Malformed current assignment field")
    relative = match.group(1)
    expected_prefix = "arc09-executable-rack/slice01-module-behavior-installation/"
    expected = (
        relative == expected_prefix + "cc-prompt.md"
        or re.fullmatch(
            re.escape(expected_prefix) + r"cc-prompt-iteration[0-9]{2}\.md",
            relative,
        )
        is not None
    )
    check(expected, "Non-canonical current assignment path")

    relative_path = Path(relative)
    check(not relative_path.is_absolute(), "Current assignment must be relative")
    check(".." not in relative_path.parts, "Current assignment escapes its project")

    project_dir = slice_dir.parent.parent
    resolved_slice = slice_dir.resolve()
    resolved_prompt = (project_dir / relative_path).resolve()
    try:
        resolved_prompt.relative_to(resolved_slice)
    except ValueError as error:
        raise RuntimeError("Current assignment is outside this slice") from error
    check(resolved_prompt.is_file(), "Current assignment prompt is missing")

    for name in ("cc-prompt.md", "cc-prompt-iteration01.md"):
        check((slice_dir / name).is_file(), f"Missing preserved prompt: {name}")

    matching_history = []
    for line in plan_text.splitlines():
        if not line.startswith("|"):
            continue
        cells = [cell.strip() for cell in line.split("|")[1:-1]]
        if len(cells) >= 2 and cells[1] == relative_path.name:
            matching_history.append(line)
    check(len(matching_history) == 1, "Current assignment history mismatch")
    return resolved_prompt


def main():
    with (PROJECT / "artifacts/migration-manifest.tsv").open() as stream:
        imported = list(csv.DictReader(stream, delimiter="\t"))
    preserved = {}
    for row in imported:
        path = WORKTREE / row["destination_path"]
        original = git("show", BASELINE + ":" + row["destination_path"])
        current = path.read_bytes()
        check(current == original if row["action"] == "move-unchanged" else current.startswith(original), f"Changed imported content: {path}")
        preserved[path] = original.decode()
    check(len(imported) == 23, "Migration denominator changed")
    with (PROJECT / "arc08-validation-and-release/slice01-restart-baseline/artifacts/carryover-register.tsv").open() as stream:
        inherited = list(csv.DictReader(stream, delimiter="\t"))
    with (PROJECT / "artifacts/cdc-successors03.tsv").open() as stream:
        successors = list(csv.DictReader(stream, delimiter="\t"))
    expected = {(r["source_ledger"], r["row_id"]) for r in inherited}
    actual = {(r["source_ledger"], r["row_id"]) for r in successors}
    check(len(inherited) == len(successors) == len(actual) == 24 and actual == expected, "24-row successor mismatch")
    for r in successors:
        row = rows(PROJECT / r["source_ledger"])[r["row_id"]]
        check(row[1] == r["original_criterion"] and row[5] == "deferred", f"Criterion/disposition drift: {r}")
        check(r["successors"] and r["owner"] and r["reentry"], f"Unowned successor: {r}")
    for n, expected_children in [(1,0),(2,0),(3,2),(4,3),(5,0),(6,1)]:
        arc = next(PROJECT.glob(f"arc{n:02}-*"))
        children = list(arc.glob("slice*"))
        check(len(children) == expected_children, f"Historical child mismatch: {arc}")
        for name in ["closing-report.md", "crc-verification.md", "cdc-verification.md"]:
            check((arc / name).is_file(), f"Missing arc close record: {arc}/{name}")
        check("closed, carried over" in (arc / "cdc-verification.md").read_text(), f"Missing actual CDC verdict: {arc}")
        for child in children:
            check((child / "crc-verification.md").is_file(), f"Missing child verdict: {child}")
            check(all(row[5] == "deferred" for row in rows(child / "ledger.md").values()), f"Child upgraded: {child}")
    check(len(rows(PROJECT / "ledger.md")) == 7, "Project row accounting mismatch")
    check(len(rows(HERE.parent / "ledger.md")) == 6, "New slice denominator mismatch")
    check(len(rows(HERE.parents[1] / "ledger.md")) == 4, "New arc denominator mismatch")
    for name in ["slice-plan.md", "ledger.md", "cc-prompt.md"]:
        check((HERE.parent / name).is_file(), f"Incomplete open set: {name}")
    check((PROJECT / "cdc-directive03.md").is_file(), "Missing CDC return")
    current_assignment = check_current_assignment(
        (HERE.parent / "slice-plan.md").read_text(), HERE.parent
    )
    count = 0
    for doc in [WORKTREE / "AGENTS.md", WORKTREE / "README.md", *PROJECT.rglob("*.md"), *(WORKTREE / "status").glob("*.md")]:
        text = doc.read_text()
        if doc in preserved:
            text = text[len(preserved[doc]):]
        for target in re.findall(r"\[[^\]]*\]\(([^)]+)\)", text):
            if "://" in target or target.startswith("#"):
                continue
            check((doc.parent / target.split("#")[0]).exists(), f"Broken link: {doc}: {target}")
            count += 1
    print("PASS: 23 imported documents preserved; 24 exact deferred rows assigned successors")
    print("PASS: six arc CDC records; child denominator 0/0/2/3/0/1; no behavior upgraded")
    print(f"PASS: project/arc09/slice01 rows 7/4/6; complete open set and directive03; {count} local links")
    print(f"PASS: current assignment metadata and history resolve to {current_assignment}")
    print("Structural self-check only; arc09 implementation and independent CRC acceptance remain pending")


if __name__ == "__main__":
    main()
