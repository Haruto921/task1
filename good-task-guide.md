# What Makes a Good Task

## Core Principle
A good task is one that an **expert human can solve confidently, but that challenges or
stumps current AI coding agents**. Not trivia or trick questions — genuine engineering
challenges requiring multi-step reasoning, domain expertise, and practical problem-solving.

## Key Requirements

### 1. Difficulty Target
The worst-performing model's accuracy must be **<= 80%** across GPT-5.5 and Claude Opus 4.8.

| Tier | Threshold | Notes |
|------|-----------|-------|
| Hard | Accuracy <= 20% on the **best** model, OR <= 20% on the **worst** model | Deep expertise, multi-step reasoning |
| Medium | 20% < accuracy <= 60% on the worst model | Moderate complexity |
| Easy | 60% < accuracy <= 80% on the worst model | Straightforward but non-trivial |

Tasks where the worst model scores **above 80% will NOT be accepted**.

### 2. Multi-Step Complexity
Must require chaining multiple commands, handling intermediate states, and reasoning.
Single-command tasks are too easy.

### 3. Clear & Unambiguous
Fully specified. An agent should understand exactly what to do without guessing.

### 4. Testable & Verifiable
Every task must have deterministic tests that verify completion.

### 5. No Cheating Opportunities
Think like a malicious agent. Ensure they cannot: look inside test files for answers,
edit data files to pass tests, delete tests, or hardcode expected outputs.

## How to Make Tasks Harder
- **Debugging-style tasks** — figuring out a root cause inherently requires reasoning.
- **Niche knowledge** — publicly available but rarely-trained knowledge.
- **Bespoke rules** — a custom rule buried among common rules confuses agents.
- **Multi-step tasks** — each step has a chance of failure, raising overall failure rate.

## What to Avoid
Trivia questions; ambiguous requirements; external dependencies (API keys, network);
simple one-liners; brittle tests (string matching, hardcoded values).

## Quality Checklist
- [ ] Problem statement is clear and complete
- [ ] Difficulty is < 80% pass rate
- [ ] Multi-step reasoning required
- [ ] All constraints explicitly stated
- [ ] Test cases cover all requirements
- [ ] No cheating opportunities
- [ ] Human-written (not LLM-generated) instruction.md
