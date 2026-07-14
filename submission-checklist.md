# Submission Checklist (Terminus Edition 2)

Run through this before every submission. Companion to `submission-guide.md`.

## Task design

- [ ] Problem statement clear and unambiguous
- [ ] All requirements explicitly stated; output files named in the instructions
- [ ] Absolute paths only (`/app/...`)
- [ ] Data schemas fully specified
- [ ] **Difficulty target: worst model < 80% pass rate** (else rejected)

## Required files (non-milestone)

- [ ] `task.toml` — complete v2.0 config, all sections
- [ ] `instruction.md` — clear, human-written (not LLM-style)
- [ ] `environment/Dockerfile` — builds; app deps pinned; **every `FROM` digest-pinned `@sha256:`**;
      base canonical or non-canonical-with-justification; `environment/` ≤100 MiB, no file >50 MiB
- [ ] `solution/solve.sh` — deterministic, human-written, implements logic (not a hardcoded answer)
- [ ] `tests/test.sh` — runs pytest with **pre-installed** verifier deps; writes `/logs/verifier/reward.txt`
- [ ] `tests/test_outputs.py` — pytest tests with docstrings

## Rubric

- [ ] Generated via the submission UI, then edited for accuracy
- [ ] **At least three criteria with negative rewards** (e.g. `-1`)

## Quality

- [ ] Every requirement has a corresponding test; full coverage (explicit + implicit + edge cases)
- [ ] Tests written in Python / pytest; check **behavior, not implementation**
- [ ] Anti-cheating: no hints/answers exposed; tests run after the agent and can't be read to cheat

## Automated checks (all must pass)

Oracle: `stb harbor run -a oracle -p <task-folder>` → PASSES

CI: `stb harbor tasks check <task-folder> -m openai/@openai/gpt-5.5`
- [ ] pinned_dependencies, check_pinned_images, check_sanctioned_base_images, check_build_context_size
- [ ] typos, tests_or_solution_in_image, check_dockerfile_references, check_test_sh
- [ ] check_task_absolute_path, check_privileged_containers, ruff, check_task_sizes, validate_task_fields
- [ ] Warnings too (fix unless reviewer-approved): check_dockerignore, check_dockerfile_hygiene,
      check_offline_tests, check_apt_usage, check_reproducible_builds, check_layer_volatility,
      check_no_build_tools_in_runtime, check_file_extraction, check_heredoc_usage,
      check_recursive_permissions

LLMaJ:
- [ ] behavior_in_task_description, behavior_in_tests, informative_test_docstrings,
      anti_cheating_measures, structured_data_schema, hardcoded_solution, file_reference_mentioned

## Real-agent difficulty

Run 2–3× each: `stb harbor run -m @openai/gpt-5.5 -p <task-folder>` and
`stb harbor run -m @anthropic/claude-opus-4-8 -p <task-folder>`.

| Difficulty | Rule |
|---|---|
| Hard | accuracy ≤ 20% on best OR worst model |
| Medium | 20% < accuracy ≤ 60% on worst model |
| Easy | 60% < accuracy ≤ 80% on worst model |

- Worst-model pass rate: ____%  · Best-model pass rate: ____%
- **Worst model above 80% → NOT accepted.**

## Self-check

- Understandable to a first-time reader? Ambiguous requirements? Could an agent cheat? Do tests
  verify real behavior? Is the solution deterministic (seeded, no randomness)?

## Submit

- [ ] ZIP the **files** (not the folder); all required files included
- [ ] Upload to Terminus-2nd-Edition; metadata filled; rubric checkbox checked; "Send to reviewer"
      unchecked for the first pass
