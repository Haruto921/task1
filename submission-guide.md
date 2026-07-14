# Platform Submission Guide (Terminus Edition 2)

End-to-end process for submitting a task on the Terminus Expert Platform (Terminus-2nd-Edition).
Read alongside `prompt-styling-guide.md` and `CLAUDE.md`.

> **Reminder:** always start from the downloaded **skeleton** (Regular / UI / Milestone) and edit it —
> its `Dockerfile`, `task.toml`, and `tests/test.sh` already carry the CI-compliant boilerplate
> (digest-pinned canonical base, tmux/asciinema, pytest + pytest-json-ctrf, reward/ctrf wiring).
> Building from scratch means re-deriving all of that by hand.

## Workflow

1. Download the correct skeleton (Regular / UI / Milestone).
2. Extract, rename folder to **kebab-case** (e.g. `fix-memory-leak-python`).
3. Write `instruction.md` + configure `task.toml`.
4. Set up `environment/Dockerfile`.
5. Create + test `solution/solve.sh`.
6. Write + verify `tests/`.
7. Run agents (oracle, then real models).
8. ZIP the inner files, **check the rubric checkbox**, keep **"Send to reviewer" unchecked**, submit.
9. Read CI feedback, iterate, edit the generated rubric.
10. When CI passes, submit to a reviewer (check "Send to Reviewer").

## task.toml (v2.0)

Fields: `[metadata]` (author_name, author_email, difficulty="unknown", category, subcategories,
number_of_milestones, codebase_size, languages, tags, expert/junior_time_estimate_min),
`[verifier].timeout_sec`, `[agent].timeout_sec`, `[environment]` (build_timeout_sec, cpus, memory_mb,
storage_mb, allow_internet). subcategories are snake_case: `long_context`, `tool_specific`,
`api_integration`, `db_interaction`, `ui_building`. `codebase_size` counts **all** env files
(minimal 0–20, small 20+, large 200+).

## Dockerfile requirements (CI-enforced)

- **Install `tmux` AND `asciinema`** — required by the agent runtime. Missing them makes **all agent
  runs fail silently** with no verifier output.
- **Bake ALL test dependencies into the image** (e.g. `pytest`, `pytest-json-ctrf`). `tests/test.sh`
  must **not** install anything or hit the network at runtime (the sandbox is offline).
- **Pin every package version.** **Digest-pin every `FROM`** with `@sha256:<digest>` (and any images
  in `docker-compose.yaml`).
- Prefer a **canonical Terminal-Bench base image** for the language. Non-canonical images need a
  brief **written justification** in the Dockerfile or README, or CI blocks it.
- `environment/` ≤ **100 MiB** total, no single file > **50 MiB**. Add a `.dockerignore` for
  non-trivial environments.
- **Never** copy `solution/` or `tests/` into the image.

## tests/ requirements

- `tests/test.sh` runs pytest and writes `/logs/verifier/reward.txt` (`1` pass / `0` fail).
- Emit a CTRF report: `python -m pytest --ctrf /logs/verifier/ctrf.json /tests/test_outputs.py -rA`
  (this is how the platform reads per-test results).
- pytest tests drive any non-Python system under test (don't shell out to another test framework).
- Tests must **fully cover the prompt**: every explicit requirement, implied behavior, and critical
  edge case maps to a test.
- Reference `tests/test.sh` skeleton:
  ```bash
  #!/bin/bash
  set -uo pipefail
  mkdir -p /logs/verifier
  if [ "$PWD" = "/" ]; then echo 0 > /logs/verifier/reward.txt; exit 0; fi
  python -m pytest --ctrf /logs/verifier/ctrf.json /tests/test_outputs.py -rA
  rc=$?
  if [ "$rc" -eq 0 ]; then echo 1 > /logs/verifier/reward.txt; else echo 0 > /logs/verifier/reward.txt; fi
  ```

## Local commands (stb)

> `stb harbor run` and `stb harbor tasks check` look similar but do different jobs:
> - **`stb harbor run -m <model>`** = an AI agent **attempts** your task -> measures difficulty / pass rate.
> - **`stb harbor tasks check <task-folder> -m <model>`** = automated **LLMaJ quality review** of the task
>   (behavior coverage, test docstrings, anti-cheating). It reviews the task; it does not solve it.
>
> (The Platform Submission Guide's "Run LLMaJ Checks Locally" step mistakenly shows `stb harbor run`;
> the correct LLMaJ command is `stb harbor tasks check`.)

```bash
stb harbor tasks start-env -p <task-folder> -i        # interactive env
stb harbor run -a oracle -p <task-folder>             # oracle — must PASS
stb harbor run -m @openai/gpt-5.5 -k 5 -p <task-folder>       # real agent, 5 attempts
# NOTE: -k/--n-attempts = number of runs; -n/--n-concurrent = parallelism (NOT run count).
stb harbor run -m @anthropic/claude-opus-4-8 -p <task-folder>
stb harbor tasks check <task-folder> -m openai/@openai/gpt-5.5   # LLMaJ QUALITY review (does NOT solve the task)
```
Acceptance target: **< 80% pass rate** across the real-agent runs (harder is better). The
"Failed to retrieve model info … isn't mapped yet" line is a harmless warning.

## ZIP + submit

- ZIP the **inner files**, not the containing folder. Non-milestone contents: `instruction.md`,
  `task.toml`, `environment/`, `solution/`, `tests/`.
- Platform: experts.Terminus-ai.com → Terminus-2nd-Edition → Submission node → upload ZIP →
  **check rubric checkbox**, **leave "Send to reviewer" unchecked** → Submit.
- After CI returns (~10–15 min): Revise → fix CI issues → edit the generated rubric for
  accuracy/completeness → re-zip if needed. When clean, check "Send to Reviewer" → Submit.
- Peer review: 1–7 business days.

## Common failures

- **ZIP wrong**: files nested in an extra folder → zip the files directly.
- **Missing files**: verify all five (instruction/task.toml/environment/solution/tests) are in.
- **CI fails though local passed**: usually unpinned deps or a non-digest-pinned base image; also
  check env ≤100 MiB and no file >50 MiB.
- **All agent runs fail silently**: missing `tmux`/`asciinema` in the image.
