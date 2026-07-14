# Task Authoring Playbook (Terminus Edition 2) — hard-won lessons

Repeatable knowledge from building the wheelhouse-resolver task. Read this first on the next task.

## 1. The single biggest lesson: language choice beats everything
- **Python must be HARD (worst model <=20%). This is close to impossible**, confirmed by a senior
  expert in Slack and by ~7 of our own rounds all landing 80-100%. Do NOT spend days trying to make
  a Python task Hard, especially a build/dependency/algorithm task.
- **Non-Python only needs <=80% (Easy/Medium accepted).** This is a far lower bar.
- **The task description is "inspiration" only. You may implement in ANY language.** (Slack: the
  gallery tasks are generator-made; use whatever language you want.) The task.toml `languages` field
  is your choice.
- **The verifier is ALWAYS Python pytest**, even for non-Python tasks (pytest subprocesses the
  agent's binary/CLI). Do not add Python to `languages` just because tests are pytest.
- **Language difficulty ladder (roughly, for beating 80%):** obscure/less-trained = lower pass rate.
  Rust / OCaml / Ada / Lua are Jason's picks. Go / TypeScript / JavaScript are well-known and may
  still be >80%. Python is a lost cause for Hard.

## 2. Why fully-specified tasks can't be made Hard (the rewrite-escape)
- If the correct behaviour is fully derivable from the provided materials (spec + code + API), the
  model just implements or REWRITES it correctly. Every buried rule gets applied; every planted bug
  a rewrite erases. We proved this: buried bespoke rules, PEP440 inversions, long-context rule
  induction, and a 3-bug debugging task all scored 80-100%.
- Adding more rules/bugs past a point makes REWRITING more attractive, not harder.
- The only fair levers that survive: (a) a language the model is not fluent in (primary lever),
  (b) genuine niche knowledge that is NOT stated in the task, (c) real-toolchain failure debugging
  where correctness depends on real tool behaviour, not a spec you can reimplement from.

## 3. Verification reality in this sandbox
- Sandbox has **Python and Node only**. You can fully build+run+verify Python and JS/TS here.
- **Rust, Lua, Go, OCaml: not installed and NOT installable** (no root; apt blocked; rust/lua
  mirrors return 403/000). So for those you author "blind" and verify via the platform's oracle run.
  Blind Rust = high risk (compiler strictness). Blind Lua/JS = lower risk (interpreted, simple).
- Keep the reference resolver logic language-neutral so porting is a translation, not a redesign.

## 4. Platform mechanics that save your API key and time
- **Submitting runs the agent difficulty measurement SERVER-SIDE** ("Submit runs slower checks and
  agent runs"; difficulty results are "populated by the system when your code is run"). You do NOT
  need to burn your own key with `stb harbor run -m`. Submit with **"Send to reviewer" UNCHECKED**
  first to get CI + difficulty + quality results without going to a human.
- The prior key errors were a **max-usage quota cap**, not rate limiting. `-n 1` (concurrency) does
  NOT help a quota cap; it only spreads the same total tokens over more time.
- A long spec (50k+ tokens) makes every local agent trial very token-heavy (~1-2M tokens/trial).
  Prefer server-side measurement, or measure one model at `-k 2` first.
- Difficulty result timing: not documented exactly; expect ~15 min to ~1 hour depending on queue.

## 5. Static-check gotchas (fix BEFORE submitting)
- **No `docker-compose.yaml` in edition_2** — single container via Dockerfile only. Delete compose.
- **tests/test.sh reward block must be the EXACT multi-line canonical form and end the file:**
  ```
  rc=$?
  if [ "$rc" -eq 0 ]; then
      echo 1 > /logs/verifier/reward.txt
  else
      echo 0 > /logs/verifier/reward.txt
  fi
  ```
  A one-liner `if ...; then ...; fi` FAILS the check. No trailing `exit` after `fi`.
- **.dockerignore** must include `**/node_modules/`, `**/__pycache__/`, `**/*.pyc`, caches, `.git`,
  `solution/`, `tests/`.
- **instruction.md: state WHAT, not HOW.** No procedural coaching ("read it carefully and follow it
  exactly", "read the monorepo"). State inputs, outputs, constraints; name the spec as the
  authoritative specification of required behaviour. Keep it short, human, no hints, absolute paths.
- **Digest-pin the base**, use a sanctioned canonical base (e.g. python:3.13-slim-bookworm@sha256).
  You can `apt-get install` other runtimes (nodejs, lua5.4) on top of the Python base.
- **pip lockfile warning** (no requirements.lock with --require-hashes) is advisory only, not
  blocking. Inline `==` pins pass CI (just a warning).
- **long_context**: needs >=50k valid tokens of real doc-like content, authoritative, non-filler,
  not greppable. Our 62k spec passed. If you don't need it, drop the subcategory (it makes measuring
  expensive and adds no difficulty for strong models).

## 6. Anti-cheating pattern that works
- The verifier RUNS the agent's program fresh into a temp dir and checks THAT output, so a
  hand-written lockfile/answer cannot pass. All expected values are recomputed from the live
  service at test time, nothing guessable is hard-coded. Rubric: >=3 negative-reward criteria
  (values +/- 1,2,3,5; never 4).

## 7. Reusable assets from the resolver task (in wheelhouse-resolver-flask/)
- Flask offline index fixture (`environment/api/app.py` + `universe.json`) with deterministic
  2x503 retry behaviour and digest-sorted hashes. Reusable as a generic "metadata API" service.
- A nested monorepo fixture with a decoy dev-requirements.
- A 62k-token RFC-style spec of bespoke, non-standard rules (non-PEP440 ordering, vowel hash order,
  yank-pin, trailer, extras, backtracking) — a good long_context corpus template.
- Reference resolver in Python (`solution/`) and JS (`environment/wheelhouse/resolve.mjs`). Port
  target list: Lua (curl + hand-rolled JSON), Rust, OCaml.

## 8. Tooling notes for the build environment
- Background API dies between separate bash calls — start the API and run the client in the SAME
  call. Use `_FAIL_TIMES=0` copy of app.py for fast local runs (skip the retry sleeps).
- The Write/Edit tools TRUNCATE large files and can inject null bytes. For big files use bash
  heredocs (`cat > f <<'EOF'`), then check with `tr -cd '\000' | wc -c` and a compiler/`node --check`.
- Filip's voice for as-Filip text (instruction.md, Slack, form explanations): short declarative
  sentences, plain words, slightly non-native ok, no em-dashes, no emojis, get to the point. See
  writing-style.md.

## 9. Use Claude Code / WSL to verify non-Python languages (important)
- This Cowork sandbox has ONLY Python + Node and no root, so Rust/Lua/OCaml/Go can't be installed
  or run here. That forced blind authoring for those.
- **In Claude Code with WSL you get a real Linux shell with sudo.** Install any toolchain there
  (`sudo apt install lua5.4 luarocks rustc cargo golang ocaml`, or `rustup`) and run the full
  build -> oracle -> tests loop, exactly like we do for Python/JS. This removes the blind-authoring
  risk entirely.
- Practical flow for a non-Python task: author + VERIFY in WSL/Claude Code; the shipped Docker
  image still installs the same toolchain via apt on the sanctioned Python base and must stay
  offline at agent runtime (build-time apt is fine). WSL is for verifying; the image is what ships.
- With WSL, the reliably-hard languages (Rust, OCaml, Ada, Lua) are all on the table, not just the
  well-known-but-verifiable ones (JS/TS). Prefer the less-trained language to get under 80% (or to
  Medium) more reliably.
- Reminder: even with WSL you don't need to burn the API key — difficulty is still measured
  server-side on submit.
