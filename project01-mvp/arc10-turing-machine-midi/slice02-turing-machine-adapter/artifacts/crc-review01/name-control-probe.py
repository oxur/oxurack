#!/usr/bin/env python3
"""Demonstrate the rename oracle's false positive using real candidate traces.

This probes the assertion, not a defect in the candidate's seed derivation.
"""
import json
import os
from pathlib import Path
import subprocess

SOURCE = Path(__file__).resolve().parents[6] / "codex-oxurack-module-behavior-installation"
build = subprocess.run(
    ["cargo", "test", "--locked", "-p", "oxurack", "--test", "turing_machine",
     "--no-run", "--message-format=json"],
    cwd=SOURCE, text=True, capture_output=True, check=True,
)
executables = [record["executable"] for line in build.stdout.splitlines()
               if (record := json.loads(line)).get("reason") == "compiler-artifact"
               and record.get("executable") and record["target"]["name"] == "turing_machine"]
assert len(executables) == 1, executables
child = subprocess.run(
    [executables[0], "--exact", "fresh_process_replays_canonical_trace_twice", "--nocapture"],
    env={**os.environ, "OXURACK_TURING_MACHINE_CHILD": "1"},
    text=True, capture_output=True, check=True,
)
body = child.stdout.split("TRACE_BEGIN\n", 1)[1].split("TRACE_END\n", 1)[0]
baseline = {}
for line in body.splitlines():
    name, sample, values = line.split("|", 2)
    baseline.setdefault(name, []).append((int(sample), values))
assert len(baseline) == 2 and all(len(rows) == 24 for rows in baseline.values())

# A counterexample: rename only the identity, preserving every generated value.
counterexample = {("tm_renamed" if name == "tm_primary" else name): rows
                  for name, rows in baseline.items()}
assert counterexample != baseline  # The current whole-Trace inequality passes.
assert counterexample["tm_renamed"] == baseline["tm_primary"]  # Content did not change.
print("candidate source: 9ba55dd0a4c8aa7d67fe26c30f878d89ead34aa2")
print("canonical population: 2 named instances x 24 samples x 28 output values")
print("current whole-Trace rename predicate: PASSES with unchanged sample/output content")
print("required corresponding-row content predicate: REJECTS that counterexample")
print("scope: oracle false positive; no production seed defect inferred")
