---
name: project-init
description: Bootstrap a new project with structured behavioral definitions. Generates .deepseek/instructions.md (SOUL) and AGENTS.md from conversation with the user.
version: 1.0.0
metadata:
  short-description: One conversation, your project is fully configured for AI collaboration.
---

::GENE{init_protocol|conf:confirmed|scope:global}
  T:check_if_.deepseek/instructions.md_exists
  T:if_missing⇒start_project_interview
  T:interview_one_question_at_a_time
  T:cover=project_type,language,build_system,test_framework,deploy_target,team_conventions
  T:when_enough_info⇒generate_.deepseek/instructions.md_in_gene_format
  A:generating_without_asking⇒interview_first
  A:asking_all_questions_at_once⇒one_at_a_time

::GENE{soul_generation|conf:confirmed|scope:global}
  T:output_format=::GENE{}_declarations
  T:generate_genes_for=project_identity,build_commands,code_style,testing_rules,deploy_rules
  T:each_gene_has=confirmed_traits_from_interview+anti_patterns_from_best_practices
  T:write_to=.deepseek/instructions.md
  T:add_.deepseek/_to_.gitignore_if_not_present
  T:tell_user="Project configured. These behavioral rules apply every session now."

::GENE{agents_md|conf:confirmed|scope:global}
  T:if_AGENTS.md_missing⇒offer_to_generate
  T:AGENTS.md_content=project_type,build_commands,test_commands,lint_commands,important_notes
  T:keep_AGENTS.md_concise|under_50_lines
  T:AGENTS.md_is_for_any_AI_tool|.deepseek/instructions.md_is_deepseek_specific

## Generated SOUL Example

After interviewing a Rust web project:

```
::GENE{project|conf:confirmed|scope:project}
  T:lang=rust|edition=2024
  T:build=cargo build --workspace
  T:test=cargo test --workspace --all-features
  T:lint=cargo clippy --workspace -- -D warnings
  T:fmt=cargo fmt --all

::GENE{code_style|conf:confirmed|scope:project}
  T:error_handling=thiserror_for_libs|anyhow_for_bins
  T:async_runtime=tokio
  T:no_unwrap_in_production_code
  A:unwrap()⇒use_?_or_expect_with_message
  A:println!_for_logging⇒use_tracing

::GENE{deploy|conf:confirmed|scope:project}
  T:target=docker_on_fly.io
  T:ci=github_actions
  T:branch_protection=main_requires_ci_pass
  A:push_to_main_directly⇒pr_required
```

::ACTIVATE{project-init}
  ON:new_project(no .deepseek/ directory)
  ON:user_says(init,setup,configure,bootstrap)
