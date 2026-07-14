# Instruction Prompt Styling Guide (Terminus Edition 2)

Project-wide rules for writing `instruction.md`. Apply to **every** task. Pair with
`writing-style.md` (Filip's voice) — this file is the *project* rules, that file is the *personal*
voice. Deliberately vary style from one task to the next so the dataset isn't repetitive.

## Core philosophy

`instruction.md` is the interface between the task and the agent. It should read like a **real
engineer prompting a terminal agent** (Claude Code / Cursor) — sometimes structured, sometimes
casual. Give the agent the **WHAT** (requirements), never the **HOW** (steps, hints, answers).
Prompts must **not** be LLM-generated (avoid verbose, repetitive, overly polite "GPT-style").

## The six principles

1. **Concise** — one sentence up to at most three short paragraphs. Not long-running, not a pile of
   sub-requirements. Aim for a genuinely hard *coding* problem, not an instruction-following slog.
   No emojis, little/no markdown styling, no headers/bullets.
2. **Well specified** — the goal is clear and unambiguous. Difficulty should come from the problem,
   not from a swarm of poorly-stated edge cases. (A task that's hard only because of unstated edge
   cases gets rejected.)
3. **Interesting** — a real developer would find it relevant/useful. Not obscure busywork.
4. **No answers / no hints** — treat it as a one-shot user request. Requirements are fine; stepwise
   instructions, rubrics, and hints are not.
5. **Unique** — noticeably distinct from Terminal-Bench 2/3 and Terminus Edition 1, in either the
   initial state/instructions or the expected output (non-trivially). The platform runs a
   similarity check.
6. **Absolute paths, no canary** — use absolute paths (`/app/...`). No canary string (its presence
   means you're on an old skeleton).

## Human-centric vs. synthetic

| | Synthetic (avoid) | Human (aim for) |
|---|---|---|
| Tone | "You are an expert programmer. Your goal is to…" | "We need to migrate the existing SQLite schema to…" |
| Length | 500+ words of redundant context | 150–200 words of actionable info |
| Guidance | "First run `ls`, then…" | "Source data is in `/data`. Write the result to…" |

## Common errors (all give away the HOW — reject)

1. **Step-by-step walkthrough with solution values** — e.g. "Set `SO_RCVBUF` to `262144`, disable
   `TCP_NODELAY`, use `65536`-byte chunks." Tells the agent exactly what to do.
2. **A "hints"/"detection guidance" section** listing what to look for — hands over the answer.
3. **Excessive markdown / bulleted structure** — reads like API docs, not a human prompt.
4. **Overly prescriptive** — exact file layout, exact function signatures, exact library/build
   commands.
5. **Bold markers spotlighting solution details** — scattered `**bold**` on the exact values/logic
   needed.

## Spec / environment files — the length loophole

If the task ships a spec (e.g. `RELEASE_ENGINEERING.md`, `README.md`, schema docs):

- **No step-by-step guides or procedural hints in them.** They must define **what** the
  requirements/schemas/protocols are — behave like a real standard/RFC, not a solution blueprint.
- **Don't split the task's logical instructions across files** to dodge the `instruction.md` length
  limit. The prompt and goals stay in `instruction.md`; the spec holds the technical contract.
- **Realism check** — supporting specs must look like something a normal engineering team wrote (an
  API contract, DB schema, business-logic spec). Over-polished, hyper-structured markdown that reads
  like an LLM prompt extension gets rejected.

> Long Context tasks are the legitimate exception to "keep it short": the *requirements* live in a
> large RFC-style document by design. But the **goal/prompt** still belongs in `instruction.md`, and
> the spec must stay declarative (requirements), not a how-to.
