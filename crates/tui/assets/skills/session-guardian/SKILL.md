---
name: session-guardian
description: Prevents session crashes by enforcing context budgets, auto-delegation to sub-agents, and compaction triggers. Activates every session.
version: 1.0.0
metadata:
  short-description: Keep sessions alive. Never crash from context bloat.
---

::GENE{context_budget|conf:confirmed|scope:global|priority:P0}
  T:track_context_usage_every_turn
  T:at_40pct⇒start_delegating_reads_to_subagents
  T:at_60pct⇒suggest_/compact_to_user
  T:at_75pct⇒force_compact_before_next_tool_call
  T:never_let_context_hit_90pct
  A:reading_files_one_by_one_in_parent⇒spawn_subagent
  A:sequential_turns_on_same_topic>3⇒delegate_immediately
  A:pasting_full_logs_into_parent⇒summarize_or_rlm

::GENE{delegation_protocol|conf:confirmed|scope:global}
  T:parent_is_coordinator_not_worker
  T:batch_tool_calls|min:3_per_turn_when_possible
  T:use_agent_spawn_for=read_only_investigation,single_file_edits,test_runs,grep_tasks
  T:use_rlm_for=batch_classification,parallel_analysis,long_output_summarization
  T:subagent_results⇒summarize_into_parent|never_paste_full
  A:parent_doing_grunt_work⇒spawn
  A:firing_one_read_file_and_waiting⇒batch_3+_tool_calls

::GENE{compaction_rules|conf:confirmed|scope:global}
  T:after_compact⇒verify_critical_context_survived
  T:keep=task_definition,file_paths,decisions_made,current_plan
  T:discard=intermediate_tool_outputs,superseded_drafts,verbose_error_logs
  T:session_save⇒only_after_compact|never_save_bloated_state

::ACTIVATE{session-guardian}
  ON:session_start
  ON:every_turn(check context %)
  ON:before_session_save
