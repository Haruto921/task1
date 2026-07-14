# Difficulty Guidelines (Terminus Edition 2)

Difficulty is measured by **accuracy against two frontier models**, each run **5 times**:
- **GPT-5.5** with the Codex agent
- **Claude Opus 4.8** with the Claude Code agent

## Levels

| Level | Threshold | Character |
|---|---|---|
| **Hard** | accuracy ≤ 20% on the **best** model, **OR** ≤ 20% on the **worst** model | deep expertise, 10+ step reasoning, or niche knowledge |
| **Medium** | 20% < accuracy ≤ 60% on the **worst** model | moderate complexity, some domain knowledge |
| **Easy** | 60% < accuracy ≤ 80% on the **worst** model | straightforward but non-trivial |

- **Worst model sets the floor** for most tasks (if the weaker model solves it often, it's Easy).
- **Best model matters for Hard**: if even the strongest model scores ≤ 20%, it earns Hard.
- **> 80% on the worst model → NOT accepted** (too easy to be useful training signal).

## Acceptance by language (important)

- **Python tasks must be HARD to be accepted.**
- **Non-Python tasks (Go, Java, TypeScript, …) may be Medium or Hard**; Easy is accepted for
  non-Python only.
- Regardless of language, **worst model > 80% is rejected**.

> Implication: our resolver oracle is Python, so the task must reach **Hard (≤20%)**, not just <80%.

## Designing for Hard (≤ 20%)

Needs one or more of: deep domain expertise (knowledge LLMs haven't seen much), complex multi-step
reasoning (10+ sequential steps), subtle debugging (root-cause analysis required), niche
tools/languages. Techniques:

- **Bespoke rules buried in common patterns** — a non-obvious, load-bearing rule stated once, deep
  in a long document, among standard-looking content.
- Require understanding of **obscure documentation** / long context.
- Debugging tasks where the **root cause isn't obvious**.
- Domain-specific knowledge (blockchains, scientific computing, etc.).

## Designing for Medium / Easy

- **Medium (20–60% worst):** 5–10 steps, some domain knowledge, non-obvious solution; combine
  familiar concepts, add edge cases, config that's easy to miss.
- **Easy (60–80% worst):** still multi-step (3–5) and testable; a standard task with one or two
  tricky aspects.

## Fair vs unfair difficulty

- **Good (fair) failures:** agent missed a buried rule, mis-handled real complexity, missed an edge
  case.
- **Bad (unfair) failures — avoid:** impossible requirements, ambiguous instructions, time-dependent
  / random results, external/environment flakiness.

## Measuring

```bash
stb harbor run -a oracle -p <task-folder>                  # must PASS
stb harbor run -m @openai/gpt-5.5 -p <task-folder>         # 2-3x
stb harbor run -m @anthropic/claude-opus-4-8 -p <task-folder>
```
The **worst model's** pass rate determines Easy/Medium for most tasks; for Hard, a ≤20% on **either**
model qualifies. Distinguish good failures (real difficulty) from bad (ambiguity/environment).

## To make harder / easier

- **Harder:** more steps · hidden/buried requirements · niche knowledge · debugging scenarios · edge
  cases · larger codebase.
- **Easier:** fewer steps · more explicit requirements · common tech · more hints · simpler env.
