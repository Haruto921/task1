# Task Components & Requirements (Harbor / Terminus Edition 2)

## File Structure (non-milestone)
```
my-task-folder/
├── instruction.md          # Concise, human-styled instructions
├── task.toml               # Metadata (see below)
├── environment/            # Dockerfile / docker-compose.yaml + build files
├── solution/solve.sh       # Reference Oracle solution
├── tests/
│   ├── test.sh             # Entry point; runs pytest, writes reward file
│   └── test_outputs.py     # Python pytest state validation
├── [optional data/assets]
└── README.md               # Optional
```
Milestone tasks instead use `steps/milestone_N/` with per-milestone instruction/tests/solution.

## task.toml (v2.0) required fields
- `author_name`, `author_email` (anonymous ok)
- `difficulty` — tier from frontier-model pass rates (accuracy out of 5 attempts)
- `category` (task_type) — exactly one from the taxonomy
- `subcategories` — from: long_context, tool_specific, api_integration, db_interaction, ui_building (empty if none)
- `number_of_milestones` — 0 if none
- `codebase_size` — minimal (0-20), small (20+), large (200+) — env files agent operates on, NOT outputs
- `languages` — main language(s) of oracle/agent. Do NOT add Python just because verifier tests are Python.
- `tags` — 3-6 free-form keywords (tools/libraries/techniques)
- `expert_time_estimate_min`, `junior_time_estimate_min`
- Runtime limits: `[verifier] timeout_sec`, `[agent] timeout_sec`, `[environment] build_timeout_sec`, cpus, memory_mb, storage_mb, `allow_internet = false`

## instruction.md — 6 principles
Concise; well specified; interesting; NO answers/hints; unique; uses absolute paths.
Must be human-written (spelling mistakes ok — mimic real engineer prompting).

## environment/
- Install `tmux` and `asciinema` (required by agent runtime; missing = all runs fail with no verifier output).
- Pin ALL app dependency versions (pip/npm). Digest-pin every `FROM` and compose `image:` with `@sha256:<digest>`.
- Final runtime stage: canonical Terminal-Bench base image (non-canonical needs written justification).
- environment/ <= 100 MiB total, <= 50 MiB per file. Include `.dockerignore` for non-trivial envs.
- NEVER copy `solution/` or `tests/` in the Dockerfile. No privileged mode.
- Special container paths: `/logs/verifier/` (reward + logs), `/logs/agent/`, `/oracle/`, `/tests/`.
- Doc files (spec.md/README) must read like real system specs (API contracts, schemas, RFCs) — NOT
  step-by-step solutions, and must not be used to dodge instruction.md length limits.

## solution/solve.sh
Human-written, deterministic, self-contained, idempotent. Demonstrates the command sequence,
not just the answer. No hardcoded outputs. `set -euo pipefail`.

## tests/test.sh + test_outputs.py
- test.sh runs `python -m pytest --ctrf /logs/verifier/ctrf.json /tests/test_outputs.py -rA`,
  then writes `/logs/verifier/reward.txt` (1 pass / 0 fail) via an if/else block.
- No trailing `exit $?` after the reward block (check_test_sh enforces this exact shape).
- Harbor reads reward.txt, NOT the script exit code. ALWAYS write the reward file (even on failure).
- Test deps must be pre-installed in the image; test.sh must not install/download (allow_internet=false).
- Verifier assertions are ALWAYS Python pytest (use subprocess/file checks for non-Python tasks).
- Every requirement in the prompt maps to a test; test behavior, not implementation; docstring each test.

## Difficulty verification
```
stb harbor run -m @openai/gpt-5.5 -p <task-folder>
stb harbor run -m @anthropic/claude-opus-4-8 -p <task-folder>
```
Run each model >= 2-3 times (batch flag is -k / --n-attempts).

## Anti-cheating
Dynamic/computed values (not hardcoded answers); validation depth; answers not derivable from
tests; can't be guessed. Rejected if solution copyable from assertions, output format reveals
answer, hardcoded values pass, or agent can guess.

## Rubric
Authored in the Terminus submission UI. Must include >= 3 distinct negative-reward criteria
(e.g., -1). Allowed values: +/-1, 2, 3, 5 (never 4).

## CI checks (must pass)
check_pinned_images, check_sanctioned_base_images, check_build_context_size, pinned_dependencies,
typos, tests_or_solution_in_image, check_dockerfile_references, check_test_sh,
check_task_absolute_path, ruff, validate_task_fields.

## CI checks (warn)
check_dockerignore, check_dockerfile_hygiene, check_offline_tests, check_apt_usage,
check_reproducible_builds, check_layer_volatility, check_no_build_tools_in_runtime,
check_file_extraction, check_heredoc_usage, check_recursive_permissions.

## LLMaJ checks (must pass)
behavior_in_task_description, behavior_in_tests, informative_test_docstrings,
anti_cheating_measures, hardcoded_solution, file_reference_mentioned, structured_data_schema.

## No canary strings required in Terminus Edition 2.
