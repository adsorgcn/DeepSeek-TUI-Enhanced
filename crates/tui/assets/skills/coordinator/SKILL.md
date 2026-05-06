---
name: coordinator
description: Structured sub-agent orchestration. Decomposes complex tasks into parallel workstreams, spawns sub-agents, monitors progress, integrates results.
version: 1.0.0
metadata:
  short-description: Turn one big task into parallel sub-agent workstreams.
---

::GENE{task_decomposition|conf:confirmed|scope:global}
  T:receive_task⇒decompose_into_independent_workstreams
  T:identify=shared_files,dependency_order,parallelizable_units
  T:max_5_subagents_concurrent
  T:each_subagent_gets=clear_scope,file_ownership,acceptance_criteria,forbidden_files
  A:one_subagent_doing_everything⇒split
  A:overlapping_file_ownership⇒resolve_before_spawn
  A:spawning_without_acceptance_criteria⇒define_first

::GENE{spawn_protocol|conf:confirmed|scope:global}
  T:use_agent_spawn|pass_structured_instruction
  T:instruction_format=[TASK:description]=>[FILES:owned]=>[CRITERIA:done_when]=>[FORBIDDEN:do_not_touch]
  T:after_spawn⇒continue_coordination_work|do_not_idle
  T:use_agent_wait⇒only_when_blocked_on_result
  T:use_agent_result⇒summarize_into_parent_context
  A:spawning_then_waiting_immediately⇒do_other_work_first
  A:pasting_full_subagent_output⇒summarize

::GENE{integration|conf:confirmed|scope:global}
  T:merge_order=dependency_first|independent_streams_any_order
  T:after_each_merge⇒run_verification_gate
  T:verification_gate=cargo_check|cargo_test|cargo_clippy
  T:conflict_resolution=subagent_with_file_ownership_wins
  A:merging_without_verification⇒forbidden
  A:skipping_test_after_integration⇒forbidden

::GENE{rlm_usage|conf:confirmed|scope:global}
  T:use_rlm_when=input_too_large_for_parent_context
  T:good_rlm_tasks=batch_file_classification,parallel_code_analysis,long_output_summarization
  T:inside_rlm⇒use_llm_query_batched_for_independent_items
  T:inside_rlm⇒use_rlm_query_for_recursive_decomposition
  T:parent_gets_final_synthesis_only

::ACTIVATE{coordinator}
  ON:task_requires_multiple_files(>5)
  ON:task_requires_multiple_concerns(build+test+deploy)
  ON:user_says_words(parallel,delegate,split,decompose,coordinate)
