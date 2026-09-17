"""Verify the document migration against the pinned implementation history."""

import csv
import hashlib
import json
from pathlib import Path
import re
import subprocess

ARTIFACTS = Path(__file__).resolve().parent
PROJECT = ARTIFACTS.parent
WORKTREE = PROJECT.parent


def git(*args):
    return subprocess.check_output(["git", *args], cwd=WORKTREE)


def docs_tree(commit):
    entries = {}
    for entry in git("ls-tree", "-r", "-z", commit).split(b"\0"):
        if not entry:
            continue
        metadata, path = entry.split(b"\t", 1)
        name = path.decode()
        if name.startswith("docs/") and name.endswith(".md") and Path(name).name != "index.md":
            entries[name] = metadata.decode()
    return entries


def signature(entries):
    return tuple(sorted(entries.items()))


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def main():
    baseline = json.loads((ARTIFACTS / "migration-baseline.json").read_text())
    source, filtered = baseline["source_ref"], baseline["filtered_ref"]
    with (ARTIFACTS / "migration-manifest.tsv").open() as stream:
        rows = list(csv.DictReader(stream, delimiter="\t"))
    require(len(rows) == 23, "Expected 23 migrated documents")
    require({r["source_path"] for r in rows} == set(docs_tree(source)), "Source coverage mismatch")
    require(len({r["destination_path"] for r in rows}) == 23, "Duplicate destination")
    unchanged = renamed = 0
    for row in rows:
        old = git("show", source + ":" + row["source_path"])
        target = WORKTREE / row["destination_path"]
        new = target.read_bytes()
        require(hashlib.sha256(old).hexdigest() == row["source_sha256"], "Source hash mismatch: " + str(target))
        require(hashlib.sha256(new).hexdigest() == row["destination_sha256"], "Destination hash mismatch: " + str(target))
        if row["action"] == "move-unchanged":
            require(old == new, "Changed historical artifact: " + str(target))
            unchanged += 1
        else:
            require(row["action"] == "rename-plan", "Unknown operation")
            suffix = (f'\n\n## Version History\n\n'
                      f'- {baseline["date"]} — Planning migration: renamed `{Path(row["source_path"]).name}` '
                      f'(formerly `{row["source_path"]}`) to `{target.name}` in `{target.parent.relative_to(PROJECT)}`. '
                      'Original content retained as historical planning; this rename does not establish '
                      'current implementation or acceptance status. See the project plan and migration report for the current baseline.\n')
            require(new == old + suffix.encode(), "Unexpected plan edit: " + str(target))
            renamed += 1
        # A real rename commit must make the filtered import reachable by --follow.
        history = git("log", "--follow", "--format=%H", "HEAD", "--", row["destination_path"]).decode().splitlines()
        require(bool(set(history) & set(git("rev-list", filtered).decode().splitlines())), "Document ancestry not followed: " + str(target))
    print(f"PASS: {len(rows)} documents; {unchanged} byte-identical artifacts; {renamed} plans with rename-note-only additions")

    original_commits = git("rev-list", source).decode().splitlines()
    filtered_commits = git("rev-list", filtered).decode().splitlines()
    require(len(original_commits) == baseline["source_commit_count"], "Original ancestry changed")
    require(len(filtered_commits) == baseline["filtered_commit_count"], "Filtered ancestry changed")
    historical_states = set()
    for commit in filtered_commits:
        entries = docs_tree(commit)
        all_paths = set(git("ls-tree", "-r", "--name-only", commit).decode().splitlines())
        require(all_paths == set(entries), "Non-document file in filtered history: " + commit)
        historical_states.add(signature(entries))
    for commit in original_commits:
        state = signature(docs_tree(commit))
        require(not state or state in historical_states, "Lost historical document state: " + commit)
    print(f"PASS: all nonempty document states across {len(original_commits)} source commits preserved in {len(filtered_commits)} filtered commits; no implementation files or index in filtered history")

    matched = 0
    fmt = "%an%x00%ae%x00%aI%x00%cn%x00%ce%x00%cI%x00%B"
    for line in (ARTIFACTS / "history-commit-map.txt").read_text().splitlines()[1:]:
        old, new = line.split()
        if set(new) == {"0"}:
            continue
        require(docs_tree(old) == docs_tree(new), "Commit tree projection mismatch")
        require(git("show", "-s", "--format=" + fmt, old) == git("show", "-s", "--format=" + fmt, new), "Author/date/message changed")
        matched += 1
    require(matched == len(filtered_commits), "Incomplete commit map")
    result = subprocess.run(["git", "merge-base", source, filtered], cwd=WORKTREE, capture_output=True)
    require(result.returncode == 1, "Unexpected shared implementation ancestry or merge-base failure")
    print(f"PASS: {matched} retained commits preserve authors, committers, dates, messages and document blobs; planning ancestry is separate")

    historical_paths = {WORKTREE / r["destination_path"] for r in rows}
    checked_links = 0
    for doc in [WORKTREE / "README.md", *PROJECT.rglob("*.md")]:
        if doc in historical_paths:
            continue
        for target in re.findall(r"\[[^\]]*\]\(([^)]+)\)", doc.read_text()):
            if "://" in target or target.startswith("#"):
                continue
            require((doc.parent / target.split("#")[0]).exists(), f"Broken new link in {doc}: {target}")
            checked_links += 1
    for directory in PROJECT.glob("arc*/slice*"):
        require(any((WORKTREE / r["destination_path"]).is_relative_to(directory) for r in rows), "Slice without migrated document: " + str(directory))
    require(len(list(PROJECT.glob("arc*"))) == 8, "Arc count mismatch")
    require(len(list(PROJECT.glob("arc*/slice*"))) == 6, "Slice count mismatch")
    require(not list(WORKTREE.glob("docs/**/*.md")), "Old documents remain in planning worktree")
    print(f"PASS: {checked_links} new local links resolve; 8 arcs and 6 document-backed slices; no legacy docs left at old planning paths")
    print("SELF-CHECKED structural migration only; no implementation tests or independent acceptance claimed")


if __name__ == "__main__":
    main()
