# Project Terminus — Edition 2

Persistent project context for building Terminal-Bench-style evaluation tasks. Filip is a
Terminus expert whose job is to **author** tasks (environment + oracle solution + tests +
rubric), *not* to solve them as an agent.

> Note: This is **not** the Mercor "Imperium / CLI Comparisons" workflow. Do **not** use the
> `mercor-eval-workflow` skill here — that project solves tasks; this one creates them.

> **Writing style:** when writing text *as Filip* (task `instruction.md`, Slack, email, review
> replies), follow `writing-style.md` in this folder. It does not apply to code/specs/test docstrings.

---

## What the project is

Terminus builds a high-quality dataset in the style of **Terminal-Bench**: multi-step tasks an
AI agent completes entirely through a terminal/CLI (compile a repo, train a classifier, stand up
a server, etc.). The expert creates each task plus an **Oracle solution** and **tests** that
verify correctness.

Tasks should be hard: target a **~100% pass rate is the ceiling to avoid** — i.e. SOTA models
(GPT-5.2, Claude Opus 4.6) should *not* reliably solve them. Difficulty is assigned *after*
submission based on model accuracy:

| Rating | Model accuracy |
|--------|----------------|
| Easy   | > 80% |
| Medium | 21–80% |
| Hard   | ≤ 20% |

A "trivial" result means iterate to make it harder (aim for at least "easy").

---

## Acceptance bar (all must hold)

- All Python unit tests pass **at least once** across all 10 agent runs.
- Claude and ChatGPT agents **cannot** both pass at 100% accuracy.
- The **Oracle solution passes**.
- A **NOP** (no-op agent) does **not** pass.

---

## Required task file structure

Everything lives under a task folder. Zip the **contents**, not the top-level folder.

```
<task-folder>/
├── instruction.md          # Self-contained task prompt for the agent (natural, human tone)
├── task.toml               # Config + metadata
├── environment/            # Environment definition ONLY (keeps tests/solution out of image)
│   ├── Dockerfile          # Fully sets up env; no privileged mode; pin all deps
│   └── docker-compose.yaml # Orchestration; references Dockerfile image; no privileged containers
├── solution/
│   └── solve.sh            # Oracle: step-by-step shell script that reliably completes the task
├── tests/
│   ├── test.sh             # Entry point; installs TEST deps here (not in image); writes reward to /logs/verifier/
│   └── test_outputs.py     # Deterministic pytest unit tests over final env state
└── milestones.md           # ONLY if the task has milestones (goes in zip root)
```

Rubric is **not** a file in the zip — it's generated and edited in the submission UI.

Key rules:
- `environment/` folder isolates the image build so `task.toml`, `instruction.md`, and test
  files are never copied into the container.
- Dockerfile must **not** copy `solution/solve.sh`, `tests/test.sh`, or `test_outputs.py`.
- **Test dependencies install at test time** (in `tests/test.sh`), never during image build.
- Pin every required dependency version in the Dockerfile.
- `tests/test.sh` must contain a `uv init` / `uv venv` (or the task.toml carries global/
  system-wide keywords).
- Use **absolute paths** in instructions, never relative.
- Every file must be **< 1 MB**. No files outside `tasks/`.

---

## instruction.md — prompting style (VERY IMPORTANT)

Full rules + anti-patterns: **`docs/prompt-styling-guide.md`** (read it before writing any
`instruction.md`). Quick version:

- Write like a **real engineer** talking to a coding agent (Claude Code / Cursor), not like an
  LLM. Must **not** read as LLM-generated (no verbose/repetitive/overly-polite "GPT-style").
- **1 sentence to at most 3 short paragraphs.** No emojis, little/no markdown, no headers/bullets.
- **Vary tone/structure across tasks** — don't reuse the same template every time.
- Give the **WHAT** (requirements), never the **HOW** (hints/answers/stepwise). Especially milestones.
- Absolute paths, no canary string. Must be unique vs Terminal-Bench 2/3 and Terminus Edition 1.
- Spec/env files hold the technical contract (RFC-style, declarative) — **not** a how-to, and not a
  place to offload the goal to dodge the length limit.

---

## task.toml metadata (Edition 2 additions)

- `subcategories` / subtypes — **required field**; if none of the 5 apply, leave **blank**.
- `codebase_size` — `large` (200+ files; if forked from OSS, record repo name + commit),
  `small` (20+ files/scripts), or `minimal` (0–20 files).
- `number_of_milestones` — **required**; set to `0` if none.
- Must list required output filenames that the tests check for.
- Must specify any data files implied by the tests.

### The 5 subtypes

- **Long Context** — files ≥ 50k tokens; relies on semantic understanding (papers, 10-Ks, ISO
  standards, contracts, huge schemas, chat/email logs).
- **Tool Specific** — SDK/API tools models underperform on (Blender, FFmpeg, ImageMagick,
  Graphviz, MLflow, WandB, Prefect, QGIS, GIMP, …).
- **API Integration** — build/interact with/debug APIs; **source code included** in env; APIs
  **mocked inside Docker** (no external deps); agent uses terminal only (no MCP). Flask, Rails,
  Django, Express, etc.
- **DB Interaction** — real DB engines (SQLite, Postgres, Parquet, Arrow); avoid pure-CSV flat
  files as the majority.
- **UI Building** — verified with **Playwright**, not pytest.

### Categories

build-and-dependency-management, system-administration, data-processing, games,
software-engineering, machine-learning, debugging, security, scientific-computing.

---

## Diversity constraints for the dataset

- ≤ 50% of tasks Python.
- ≤ 10% use FastAPI (avoid oversaturation).
- ≤ 10% multi-container (avoid multi-container where possible).
- Want more **large** codebase_size tasks.

---

## Task design requirements

Multi-step (not one command) · fully testable & unambiguous · unique vs public Terminal-Bench and
other submissions · no privileged/root ops · standalone (no user input after start; all params via
files/flags/env/task.toml) · interacts with the environment through the terminal (data-structure/
algorithm puzzles OK only as a sub-component, not the focus).

---

## Rubrics

Structured, objective **binary** checks over the agent's trace. Award points for meaningful
required steps; penalize incorrect/unsafe/wasteful ones. Example:

```
Agent keeps parsing logic data-driven (offsets/struct formats), does not hardcode test values, +3
Agent includes basic sanity checks and reports errors instead of failing silently, +1
Agent uses destructive commands unrelated to the task (e.g. rm -rf /), -5
```

Generation: check **"generate rubric"** during submission for CI. **Uncheck it before sending to
a human reviewer** — leaving it checked overrides your edited rubric. Edit the generated rubric
for accuracy/completeness in the box where it appears.

---

## Milestones (optional)

Divide a complex task into sequential subtasks, each a prerequisite for the next. Agent must
signal completion before the framework validates and lets it progress. Needs: milestone
requirements in `instruction.md`, per-stage verifiers in `tests/` (deterministic pass/fail,
stable state between validations), rubric coverage, `milestones.md` in zip root, and
`number_of_milestones` in task.toml.

---

## Submission workflow

1. Go to the project website, install harbor (`pip install harbor`).
2. Claim a task idea from the **Task Gallery** (or bring your own).
3. Download the correct **skeleton** (Regular / UI / Milestone) and rename the folder.
4. Build instruction.md, oracle `solve.sh`, and Python tests; iterate until all CI/LLMaJ pass.
5. Zip the **underlying files** (not the top folder).
6. Upload on the Terminus Expert Platform → **Terminus-2nd-Edition** node → "Check Feedback"
   (fast static checks, ~1–2 min).
7. When green, check **generate rubric**, leave **"Send to Reviewer" unchecked**, click Submit
   (runs CI + agent runs).
8. After ~10–15 min it appears under "Tasks to be revised" → Revise. Review agent-run + LLM
   quality results; edit the generated rubric.
9. When satisfied, check **"Send to Reviewer"** and Submit for human peer review. Reviewer
   accepts or returns with comments (which can be disputed).

### Quality gates
- **LLMaJ** (non-deterministic): behavior-in-description ↔ behavior-in-tests parity, informative
  test docstrings, anti-cheating (no reading tests to pass), structured-data schema, pinned deps,
  no typos, no tests/solution in image, test deps installed at test time, no hardcoded solution,
  output filenames referenced in task.toml.
- **CI** (deterministic): test file references present in task.toml, required task.toml fields,
  no privileged containers, no files outside `tasks/`, files < 1 MB, Ruff lint clean,
  `tests/test.sh` sanity (uv init/venv), absolute paths, Dockerfile doesn't reference
  solve.sh/test.sh/test_outputs.py.

---

## Local commands (harbor)

```bash
pip install harbor

# enter the task container interactively to develop/verify the solution
harbor tasks start-env --path <task-folder> --interactive

# run the oracle (must reward 1.0) and NOP (must reward 0.0) — plain harbor is fine
harbor run --agent oracle --path <task-folder>
harbor run --agent nop --path <task-folder>

# run the real frontier agents for a local difficulty preview.
# IMPORTANT: use `stb harbor run` (Terminus toolbelt) — it wires up model routing.
# `uv run harbor run` does NOT map these model strings and fails.
stb harbor run -m @anthropic/claude-opus-4-8 -p <task-folder>
stb harbor run -m @openai/gpt-5.5           -p <task-folder>
```

**Current model strings (as of Jul 2026):** Claude = `@anthropic/claude-opus-4-8`,
GPT = `@openai/gpt-5.5`. No `openai/` prefix, no `-tbench` path. The old
`openai/@openai-tbench/...` / `@anthropic-tbench/claude-opus-4-6` strings are outdated and wrong.
The `Failed to retrieve model info ... isn't mapped yet` line is a **harmless warning** — ignore it;
the run still works. Reward `0.0` from these agents is the *good* outcome (task is hard).

---

## Resources

- Project website / Docs & Task Gallery:
  https://Terminus-ai.github.io/Terminus-ECTraining-stateful/portal
- Slack: `#terminus-2nd-edition-submission` (help/questions),
  `#terminus-2nd-edition-announcements` (announcements).
- Office hours: Mon / Wed / Fri.
- Expected effort: ~2.5 hours per task once past the initial learning curve (first 1–2 longer).
