# Skill Token Comparison

Concrete comparison: the `session-guardian` skill's context management rules
expressed in natural language vs. structured behavioral definitions.

## Natural language (91 words, ~120 tokens)

```
When the context window reaches 40%, start delegating file reads to sub-agents.
When it reaches 60%, suggest compaction to the user. When it reaches 75%,
force compaction before the next tool call. Never let context hit 90%. Don't
read files one by one in the parent session, spawn a sub-agent instead. Don't
paste full logs into the parent, summarize them. After compacting, verify that
task definitions, file paths, and decisions survived. Batch tool calls, fire
at least 3 per turn instead of one at a time. Maximum 3 sequential turns on
the same topic before delegating.
```

## Structured behavioral definition (58 words, ~70 tokens)

```
::GENE{context_budget|conf:confirmed|scope:global|priority:P0}
  T:track_context_usage_every_turn
  T:at_40pct⇒start_delegating_reads_to_subagents
  T:at_60pct⇒suggest_/compact_to_user
  T:at_75pct⇒force_compact_before_next_tool_call
  T:never_let_context_hit_90pct
  T:batch_tool_calls|min:3_per_turn_when_possible
  A:reading_files_one_by_one_in_parent⇒spawn_subagent
  A:sequential_turns_on_same_topic>3⇒delegate_immediately
  A:pasting_full_logs_into_parent⇒summarize_or_rlm
```

## Result

| Metric | Natural language | Structured | Reduction |
|--------|-----------------|------------|-----------|
| Words | 91 | 58 | 36% |
| Approx tokens | ~120 | ~70 | 42% |
| Ambiguity | "start delegating" (when? how?) | `at_40pct⇒` (exact trigger) | N/A |
| Anti-patterns | implicit (buried in prose) | explicit (`A:` prefix, scannable) | N/A |

The token reduction varies by skill and content density. For behavioral rules
(do X, don't do Y), structured definitions typically reduce token count by
35-45% while eliminating interpretation ambiguity.
