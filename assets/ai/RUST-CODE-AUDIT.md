# Rust Code Audit

Please prepare for a full Rust audit of this project.

## Preparation

Read CLAUDE.md, then read every file it references for the Rust skill:
assets/ai/ai-rust/skills/claude/SKILL.md, the relevant guides in
assets/ai/ai-rust/guides/*.md, and assets/ai/CLAUDE-CODE-COVERAGE.md.
Also read the architecture doc at
docs/design/01-draft/0005-oxurack-system-architecture.md.

## Scope

All Rust code in crates/, including test modules and integration tests.
No files excluded. Treat this as handing the codebase off to a senior Rust
reviewer who will ship it to users next week.

## Output

A single Markdown report at workbench/cc-rust-audit.md, structured:

1. Executive summary — 3-5 sentences. What's solid? What's the most
   important cluster of issues?
2. Findings, grouped in this order: correctness/soundness, API design
   & invariants, error handling, concurrency & RT safety, testing,
   performance, idioms/style. Within each category, highest-severity
   first.
3. Per finding: file:line, what's wrong, why it's wrong, concrete fix
   (with code snippet if non-obvious). Every finding cites at least
   one specific file:line. No category-wide generalities.
4. "Things I looked for and did not find" section at the end — at least
   five checks you ran that came back clean. This disciplines against
   padding the report with filler.

## Stance

- Do not soft-pedal. A real bug reads as "fix this," not "consider this"
  or "nice-to-have." The only exception is genuine open design questions
  where there is a real tradeoff — label those explicitly as "open
  question."
- The current state of the code is not evidence it is correct. Compilation
  and passing tests mean only that the compiler and the existing tests
  are satisfied. Look for what the tests do not cover.
- Do not produce generic Rust advice. "Prefer `?` over `unwrap`" is
  worthless; "bridge.rs:127 unwraps where `?` would propagate the error
  cleanly" is actionable. Every recommendation must land on a specific
  line.
- Specifically hunt for: silently-dropped Results, panics on library
  paths that a user could hit, unsoundness around Send/Sync or lifetime
  assumptions in the RT layer, test doubles that diverge from production
  code paths, wildcard patterns that suppress compile-time variant checks,
  and assertions that accept ranges where exact values are computable.

## Do not modify code

The audit is diagnosis only. A follow-up round will apply the fixes.
