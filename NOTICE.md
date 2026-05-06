# NOTICE

This project is a fork of [Hmbown/DeepSeek-TUI](https://github.com/Hmbown/DeepSeek-TUI).

## Attribution

**Original runtime, TUI, tool suite, and architecture:**
Copyright (c) 2024-2025 DeepSeek CLI Contributors (Hmbown and contributors).
Licensed under the MIT License. See [LICENSE](LICENSE).

**Enhanced behavioral skill layer (this fork's additions):**
- `crates/tui/assets/skills/session-guardian/`
- `crates/tui/assets/skills/coordinator/`
- `crates/tui/assets/skills/imprint/`
- `crates/tui/assets/skills/code-review/`
- `crates/tui/assets/skills/project-init/`
- `crates/tui/src/skills/system.rs` (multi-skill installer)
- `templates/`
- `benchmarks/`

Added by adsorgcn. Licensed under the same MIT License.

## What This Fork Changes

This fork does not modify the original runtime, TUI rendering, tool
registry, protocol layer, or any Rust source outside of the skill
installer (`system.rs`). All changes are additive:

1. Five structured behavioral skills auto-installed on first launch.
2. SOUL.md template for project behavioral definitions.
3. Token comparison benchmark.
4. Updated README with fork-specific install instructions.

The original DeepSeek-TUI functionality is fully preserved.
