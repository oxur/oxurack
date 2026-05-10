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

See `./docs/design/01-draft/0005-oxurack-system-architecture.md`

## Writing Code

**For Rust Code Quality:**

1. **`assets/ai/ai-rust/skills/claude/SKILL.md`** - Advanced Rust programming skill (**use this**)
2. **`assets/ai/ai-rust/guides/*.md`** - Comprehensive Rust guidelines referenced by the skill
3. **`assets/ai/CLAUDE-CODE-COVERAGE.md`** - Comprehensive test coverage guide
4. **This file (CLAUDE.md)** - Project-specific conventions only

**Important:** Depending upon the systesm, `assets/ai/ai-rust` may be a symlink; if so, you will need to look in `assets/ai/ai-rust/` (note the final slash). Depending upon the computer you are running on, the actual dir may be at `~/lab/oxur/ai-rust`, `~/lab/oxur/ai-rust-skill`, etc. If, after performing the above checks, you determine that `assets/ai/ai-rust` does not exist on the file system in any form, ask permission to clone it:

```bash
git clone https://github.com/oxur/ai-rust assets/ai/ai-rust
```
