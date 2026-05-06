# SOUL.md — Behavioral Definition for DeepSeek-TUI

> Place this file at `.deepseek/instructions.md` in your project root.
> DeepSeek-TUI loads it into the system prompt every session.

## Why structured definitions?

DeepSeek-TUI's system prompt has a finite context budget. Every instruction competes for space with conversation history, tool results, and skill metadata.

Natural language instructions are verbose:

```
When reviewing code, always check for security issues first, then
correctness, then performance. Don't nitpick style when there are
real bugs. Every finding should include a suggested fix. Never approve
code you haven't fully read. Group findings by severity.
```

Structured behavioral definitions say the same thing in fewer tokens:

```
::GENE{code_review|conf:confirmed|scope:project}
  T:review_order=security⇒correctness⇒performance⇒readability
  T:every_finding_includes_fix_suggestion
  A:nitpicking_style_when_bugs_exist⇒fix_bugs_first
  A:approving_without_full_read⇒forbidden
```

Same intent, ~60% fewer tokens. In a 1M-token context window under pressure, this matters.

## Template

Copy the blocks below into `.deepseek/instructions.md`, edit to match your project:

```
::GENE{project|conf:confirmed|scope:project}
  T:lang=<your_language>
  T:build=<build_command>
  T:test=<test_command>
  T:lint=<lint_command>

::GENE{style|conf:confirmed|scope:project}
  T:<your_convention_1>
  T:<your_convention_2>
  A:<bad_practice>⇒<what_to_do_instead>

::GENE{workflow|conf:confirmed|scope:project}
  T:commits=<conventional|semantic|free_form>
  T:branching=<trunk|gitflow|feature_branches>
  T:ci=<github_actions|gitlab_ci|none>
  A:<workflow_antipattern>⇒<correct_approach>
```

## Syntax

| Element | Meaning |
|---------|---------|
| `::GENE{name\|conf:confirmed}` | Behavioral gene. `confirmed` = always active. `tentative` = experimental. |
| `T:trait` | Positive trait. "Do this." |
| `A:pattern⇒consequence` | Anti-pattern. "Never do this, because this happens." |
| `scope:project` | Applies to current project only. |
| `scope:global` | Applies everywhere (use in `~/.deepseek/memory.md`). |

## Pre-built Blocks

### Python Project
```
::GENE{python|conf:confirmed|scope:project}
  T:version=3.12+
  T:package_manager=uv
  T:test=pytest
  T:lint=ruff check
  T:type_check=pyright
  A:requirements.txt⇒use_pyproject.toml
  A:print_debugging⇒use_logging_or_breakpoint
```

### TypeScript Project
```
::GENE{typescript|conf:confirmed|scope:project}
  T:runtime=node_22
  T:package_manager=pnpm
  T:test=vitest
  T:lint=eslint+prettier
  T:strict_mode=true
  A:any_type⇒define_proper_types
  A:console.log_in_production⇒structured_logging
```

### Rust Project
```
::GENE{rust|conf:confirmed|scope:project}
  T:edition=2024
  T:build=cargo build --workspace
  T:test=cargo test --workspace --all-features
  T:lint=cargo clippy -- -D warnings
  T:fmt=cargo fmt --all
  A:unwrap_in_lib_code⇒return_Result
  A:force_push_main⇒forbidden
```

### Go Project
```
::GENE{go|conf:confirmed|scope:project}
  T:version=1.23+
  T:build=go build ./...
  T:test=go test ./... -race
  T:lint=golangci-lint run
  A:naked_returns⇒explicit_return_values
  A:ignoring_errors⇒handle_or_wrap
```
