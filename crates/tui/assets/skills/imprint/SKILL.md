---
name: imprint
description: Learns how you work through conversation, builds a structured behavioral profile (.dna.md), applies it every session. Replaces plain-text memory bullets with portable behavioral DNA.
version: 1.0.0
metadata:
  short-description: Your habits, imprinted on AI. Survives across sessions and tools.
---

## Rule 1: Never Expose Internal Concepts

Never say "DNA", "gene", "behavioral pattern", "encode", "mutation", "confidence level" to the user.

Say instead:
- "Let me get to know how you work"
- "Saved a quick note so I remember next time"
- "I'll get better at this the more we work together"

## Rule 2: One Question at a Time

When learning about the user, ask exactly ONE question per message. Wait for the answer.

## First Run

Check if `.dna.md` exists in the workspace root or `~/.deepseek/`. If not, this is a first run.

Before doing ANY other work, start the onboarding:

1. Open casual: "Before we dive in, mind if I ask a couple things so I can work the way you like?"
2. Cover naturally: role, stack, work style, communication preferences, AI tools used
3. When you have enough (at least role + work style + one preference), create `.dna.md`
4. Say: "Saved some notes so things go smoother next time." Then move on to real work.

## .dna.md Format

```
::GENE{identity|conf:confirmed|scope:global}
  T:role=<their_role>
  T:stack=<their_tech_stack>
  T:experience_level=<junior|mid|senior|lead>

::GENE{communication|conf:<confirmed_or_tentative>|scope:global}
  T:<observed_preference_1>
  T:<observed_preference_2>
  A:<observed_anti_pattern>⇒<consequence>

::GENE{workflow|conf:<confirmed_or_tentative>|scope:global}
  T:<work_habit_1>
  T:<work_habit_2>
  A:<anti_pattern>⇒<consequence>
```

## Ongoing Learning

::GENE{observation|conf:confirmed|scope:global}
  T:watch_for_patterns_in_user_behavior
  T:user_corrects_output⇒record_as_confirmed_gene
  T:user_shows_preference_twice⇒promote_tentative_to_confirmed
  T:user_contradicts_existing_gene⇒update_or_remove
  T:update_.dna.md_silently|never_announce_updates
  A:asking_user_to_confirm_observations⇒just_record_silently
  A:showing_.dna.md_contents_unprompted⇒only_when_asked

## Activation

::ACTIVATE{imprint}
  ON:session_start(if .dna.md missing => onboard before work)
  ON:user_corrects_output(update gene)
  ON:user_repeats_preference(promote tentative=>confirmed)
  ON:user_asks_about_profile(show .dna.md)

## Portability

`.dna.md` is plain text. Works across DeepSeek-TUI, Cursor, Codex, Copilot, and any SKILL.md-compatible agent. Switch tools, the file comes with you.
