# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

See ./README.md

## Build Commands

`make help` provides the full list of commands, but here's a summary:

```bash
make build            # Debug build
make test             # Run all tests
make lint             # Clippy + format check
make format           # Auto-format with rustfmt
make check            # Build + lint + test
make check-all        # Build + lint + coverage
make coverage         # Text coverage report (cargo llvm-cov)
make docs             # Generate API docs -> target/doc/oxurack/index.html
```

Running a single test:

```bash
cargo test test_name
cargo test module::tests::specific_test
```

## Architecture

See `.worktrees/planning/project01-mvp/artifacts/0005-oxurack-system-architecture.md`.

## Planning

Planning artifacts live on branch `planning`, worktree `.worktrees/planning`,
under `project01-mvp/`, per the Operator's 2026-09-17 migration request.
The branch preserves filtered documentation history instead of starting empty.
Start with its `project-plan.md`, this repository's README, and Git history;
legacy documents are archaeology, not current assignments. Supporting artifacts
belong at the closest project/arc/slice level in `artifacts/`.

The current project uses the Three-Contributor Workflow (CDC, CRC, CC) and
Expedited Mode. Follow the active assignment for role boundaries, source and
planning worktrees, evidence, and explicit-path commits. CC implementation and
self-review remain proposed until the assigned reviewer accepts. Preserve
Operator gates. Do not merge `planning` into `main`.

## Writing Code

**For Rust Code Quality:**

1. **`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills/rust-guidelines/SKILL.md`** - Rust coding guidance
2. **`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills/rust-guidelines/guides/`** - Rust topic guides
3. **`/Users/oubiwann/Library/Application Support/ChatGPT-Profiles/personal/codex-home/skills/testing/SKILL.md`** - Testing guidance
4. **This file (CLAUDE.md)** - Project-specific conventions only

The installed skill paths may differ on another host; resolve the local installed
skill root when necessary and follow the active task's required reading manifest.
