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
make docs             # Generate API docs -> target/doc/minilogue-xd/index.html
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

The Operator selected One-Contributor mode: Sofie handles design, implementation
and self-review; Duncan is Operator. Report self-checks as attested and preserve
any existing independent/Operator gates. Follow the collaboration-framework for
new planning and implementation work. Do not merge `planning` into `main`.

## Writing Code

**For Rust Code Quality:**

1. **`assets/ai/rust/SKILL.md`** - Advanced Rust programming skill (**use this**)
2. **`assets/ai/rust/guides/*`** - Comprehensive Rust guidelines referenced by the skill
3. **`assets/ai/CLAUDE-CODE-COVERAGE.md`** - Comprehensive test coverage guide
4. **This file (CLAUDE.md)** - Project-specific conventions only

**Important:** Depending upon the system, `assets/ai/rust` may be a symlink; if so, you will need to look in `assets/ai/rust/` (note the final slash). If, after performing the above checks, you determine that `assets/ai/ai-rust` does not exist on the file system in any form, ask the user for help.
