# DeepSeek-TUI Enhanced

## What is inherited from upstream

Everything. The full DeepSeek-TUI runtime is unchanged:

- TUI interface and rendering
- Shell execution, file operations, git integration
- MCP protocol client
- RLM (sandboxed Python REPL with sub-LLM helpers)
- Sub-agent system (agent_spawn, agent_wait, agent_result)
- Session save/restore and workspace rollback
- LSP diagnostics
- User memory system
- Task queue
- HTTP/SSE runtime API
- All configuration, modes (Plan/Agent/YOLO), and keybindings

No upstream Rust source is modified except `crates/tui/src/skills/system.rs`
(the skill installer, extended to install all enhanced skills on first launch).

## What this fork adds

### 5 structured behavioral skills (auto-installed on first launch)

| Skill | Purpose | Leverages |
|-------|---------|-----------|
| `session-guardian` | Context budget enforcement, auto-delegation triggers, compaction rules | Context tracking, `/compact` |
| `coordinator` | Task decomposition into parallel sub-agent workstreams | `agent_spawn`, `agent_wait`, RLM |
| `imprint` | Behavioral profiling via `.dna.md`, enhances user memory | Memory system |
| `code-review` | Batch code review with severity classification | RLM (`llm_query_batched`) |
| `project-init` | Bootstrap `.deepseek/instructions.md` from interview | Project instructions |

### SOUL.md template

Drop-in behavioral definition file for `.deepseek/instructions.md`.
Pre-built blocks for Python, TypeScript, Rust, and Go projects.
Located in `templates/SOUL.md`.

### Token comparison benchmark

Concrete word and token counts comparing natural language vs. structured
behavioral definitions. Located in `benchmarks/token-comparison.md`.

## How to verify enhanced skills are installed

After first launch, check your skills directory:

```bash
ls ~/.deepseek/skills/
```

You should see:

```
skill-creator/
session-guardian/
coordinator/
imprint/
code-review/
project-init/
```

If you only see `skill-creator/`, the enhanced installer hasn't run yet.
Delete `~/.deepseek/skills/.system-installed-version` and restart to
trigger a fresh install.

## Limitations

### Structured definitions are prompt-level, not runtime-enforced

The `::GENE{}`, `::ACTIVATE{}`, `T:`, and `A:` syntax is read by the model
as structured text in the skill body. The skill parser extracts `name`,
`description`, and `body` from YAML frontmatter, then passes the body to
the model as-is.

This means:
- The model CAN interpret and follow the behavioral rules
- The model is NOT forced to follow them by runtime code
- Compliance depends on the model's instruction-following capability

This is not a deficiency for the current use case. DeepSeek V4's
instruction-following is strong enough that well-structured behavioral
definitions produce consistently better results than equivalent natural
language. But it is not "protocol-level enforcement" and should not be
described as such.

### Token reduction is approximate

The 35-45% figure comes from a specific comparison of context management
rules (see `benchmarks/token-comparison.md`). Actual reduction varies by
content type. Behavioral rules (do X, don't do Y) compress well.
Procedural instructions with many steps compress less.

## Roadmap

### Near-term
- More benchmark samples across different skill types
- Additional skills for common workflows (deploy, test, debug)

### Medium-term
- GENE parser: extract structured data from `::GENE{}` blocks at load time
- Activation registry: map `::ACTIVATE{}` triggers to session lifecycle hooks
- Skill compliance scoring: measure how well the model follows GENE rules

### Long-term
- Runtime enforcement: before_tool_call / after_tool_call hooks that
  check GENE anti-patterns before allowing tool execution
- Cross-session behavioral DNA: `.dna.md` profiles that evolve across
  projects and sessions

---

## Distribution ownership

| Channel | Owner | Status |
|---------|-------|--------|
| GitHub releases | This fork (adsorgcn) | Active, v0.1.0 published |
| npm (`deepseek-tui-enhanced`) | This fork | Package metadata ready, not yet published to registry |
| Docker (GHCR) | This fork | Workflow configured, not yet publishing images |
| Website (`website/`) | This fork | Included in repo, links point to fork |
| crates.io (`deepseek-tui-cli`, `deepseek-tui`) | Upstream (Hmbown) | Not published by this fork |
| Homebrew tap | Upstream (Hmbown) | Not maintained by this fork |
| Scoop | Neither | Not available |

## Runtime modification boundary

This fork modifies exactly **one** Rust source file from upstream:

| File | Change | Reason |
|------|--------|--------|
| `crates/tui/src/skills/system.rs` | Replaced single-skill installer with multi-skill installer | Auto-install 6 system skills on first launch |

All other Rust source changes are **reference updates only** (URLs, UA strings,
help text pointing to this fork instead of upstream). No runtime logic,
protocol handling, tool execution, or TUI rendering code is modified.

Everything else in the Enhanced layer is additive:
- `crates/tui/assets/skills/` — 5 new skill files
- `templates/` — SOUL.md template
- `benchmarks/` — token comparison data
- `ENHANCED.md`, `NOTICE.md` — governance and attribution docs

## Upstream sync strategy

This fork tracks upstream `Hmbown/DeepSeek-TUI` main branch.

**Sync method:** Selective manual merge.

**When to sync:**
- Upstream publishes a new stable release
- Upstream fixes a security vulnerability
- Upstream adds features that benefit Enhanced users

**What to watch during sync:**
- `crates/tui/src/skills/system.rs` — our only modified Rust file.
  If upstream changes this file, manual conflict resolution is needed.
- `crates/tui/assets/skills/` — if upstream adds new bundled skills,
  we add them to our `SYSTEM_SKILLS` array.
- Skill format changes — if upstream changes how SKILL.md is parsed
  (`crates/tui/src/skills/mod.rs`), verify our skills still load.

**What never syncs:**
- Upstream's npm package identity (we have our own)
- Upstream's release workflow (we have our own)
- Upstream's SECURITY.md contacts (we have our own boundary)

