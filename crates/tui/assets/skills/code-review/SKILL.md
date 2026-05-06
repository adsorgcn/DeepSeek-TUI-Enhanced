---
name: code-review
description: Structured code review using batch analysis via RLM. Security-first, actionable feedback, leverages sub-agents for large changesets.
version: 1.0.0
metadata:
  short-description: Review code systematically. Security first, fix suggestions included.
---

::GENE{review_protocol|conf:confirmed|scope:global}
  T:review_order=security⇒correctness⇒performance⇒readability
  T:every_finding_includes_fix_suggestion
  T:severity_levels=critical|warning|info
  T:critical=security_vulnerabilities,data_loss_risks,auth_bypasses
  T:warning=error_handling_gaps,race_conditions,missing_validation
  T:info=naming,style,minor_optimization
  A:approving_without_reading_every_changed_file⇒forbidden
  A:nitpicking_style_when_security_issues_exist⇒fix_security_first
  A:reporting_problem_without_suggesting_fix⇒always_suggest

::GENE{batch_review|conf:confirmed|scope:global}
  T:changeset_>5_files⇒use_rlm_for_parallel_analysis
  T:rlm_task=classify_each_file_by_risk_level
  T:then_review=critical_files_first|skip_generated_files
  T:changeset_>15_files⇒spawn_subagent_per_subsystem
  A:reviewing_15_files_sequentially_in_parent⇒rlm_or_subagent

::GENE{review_output|conf:confirmed|scope:global}
  T:format=file:line|severity|finding|suggested_fix
  T:group_by_severity|critical_first
  T:end_with=summary_verdict(approve|request_changes|needs_discussion)
  T:if_approve⇒state_what_was_verified
  T:if_request_changes⇒list_blocking_items_only
  A:walls_of_text_feedback⇒structured_findings
  A:vague_feedback("looks good","consider improving")⇒specific_actionable

::ACTIVATE{code-review}
  ON:user_says(review,check,audit,look at this,PR)
  ON:git_diff_detected_in_workspace
