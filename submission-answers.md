# Submission form answers — wheelhouse-resolver-flask (Node.js)

Prepared answers for the Terminus-2 submission form. Text fields are written in Filip's voice.
The system populates Summary / Quality check / Agent review / Test Quality Report / Difficulty
results after you submit, so leave those blank on the first pass.

---

## SHORT VERSIONS (1-2 sentences each, recommended)

**Difficulty Explanation**
The version rules do not follow PEP 440 (pre-releases are eligible, a post-release sorts below its base, epochs are ignored) and they sit once each in a long spec, so you have to read it carefully and get every one right. On top of that it is Node with no libraries, the index fails with 503 on purpose, and the lockfile plus POSIX build script must match byte for byte, so one small mistake gives a wrong plan.

**Solution Explanation**
Read the three real dependency roots, then query the index over HTTP with retry and resolve the graph with backtracking, using my own order key instead of any default (epoch dropped, pre-releases kept, post below base). Then write the lockfile in the exact format with the vowel based hash order and the trailing count line, and emit a plain POSIX build script that downloads each artifact and checks its sha256.

**Verification Explanation**
The tests run the planner fresh against the live index, so a hand written lockfile can not pass, and they check every resolved version, the hash order, and the exact lockfile format against the index. They also confirm the build script is POSIX, then run it and verify every artifact downloads and every sha256 matches.

---

## Difficulty Explanation
(Describe why your task is challenging for humans and agents to solve.)

The rules are not standard. Version ordering does not follow PEP 440. Pre-releases are eligible, a
post-release sorts below its base release, and epochs are ignored. On top of that there is PEP 503
name normalization, a yanked-but-pinned exception, a vowel based order for the artifact hashes, a
trailing count line, extras, constraint union, and backtracking when a greedy pick paints you into a
corner.

All of this lives in one long spec. Each rule is stated once, so you have to read it carefully and
not skim. The planner is written in Node with no third party packages, so version parsing, TOML and
JSON are all done by hand. The index also returns 503 on purpose, so retry is required. The final
lockfile has to match exactly, byte for byte, and the build script must be portable POSIX shell. One
wrong rule gives a wrong plan, so partial understanding is not enough.

---

## Solution Explanation
(Describe your high-level approach and key insights.)

Read the three real dependency roots from the monorepo and ignore the dev-requirements decoy. For
each package, get its versions, dependencies and hashes from the index over HTTP, with retry on the
503 responses.

The key insight is the ordering. It is not PEP 440, so I do not trust any default. I parse each
version myself and build an order key: epoch dropped, pre-releases kept, post below the base.
Selection takes the highest eligible version, but with backtracking, because the highest choice for
one package can make another package impossible.

After resolution I write the lockfile in the exact format, with the vowel based hash order and the
trailing count line. Then I emit the build script in plain POSIX shell that downloads each artifact
and checks its sha256. Everything is deterministic, the same input always gives the same output.

---

## Verification Explanation
(Explain how your tests verify correctness.)

The tests run the planner fresh against the live index, so a hand written lockfile can not pass.
They check the resolved version of every package against the expected answer. They check the hash
order for each package against the index, including the vowel rule. They check the lockfile format:
the two header lines, the sorted package blocks, the trailing count line, LF endings and no trailing
spaces.

For the build script they check it starts with /bin/sh, parses under dash, and uses no bash only
syntax. Then they run it and confirm every artifact downloads and every sha256 matches. All values
come from the index at test time, nothing is hard coded to be guessable.

---

## Prompt Check (checkbox)
CHECK the box. The instruction is one short prompt, not a list of 20+ requirements. It reads like a
real engineer asking for the tool. It points to the spec but does not reveal the rules or the
answer.

## Generate your Rubric(s) (checkbox)
CHECK "Click here to generate rubric(s)". Let the system generate it during CI. Use the rubric draft
I made as a reference when you edit it in the revision step.

## Send to reviewer? (checkbox)
LEAVE UNCHECKED on the first submission. This runs the CI, the agent difficulty runs and the quality
checks without going to a human reviewer. Read the results first. If difficulty and quality are
passing, resubmit with this checked.

## Does this task use an approved canonical base image?
YES. The base is the digest-pinned python:3.13-slim-bookworm from the canonical set.

## Did you use a Task Inspiration from the Task Gallery?
YES. Task id 5e84c036-462e-46ff-a6b5-4052b72fe07d (the wheelhouse resolver inspiration). Implemented
in Node.js, which is allowed since the inspiration language is not binding.

## How long did it take you to complete this submission?
Put your real build time in minutes. Set your own honest number.

---

## Comments for Reviewer (optional — for when you send to reviewer)
I built this from the wheelhouse resolver inspiration. I wrote it in Node.js instead of Python.
Python build/dependency tasks are very hard to push to HARD, so I moved to a non-Python language,
which targets MEDIUM. The verifier is still Python pytest and runs the Node CLI. The difficulty
comes from the non-standard ordering rules and the byte-exact output, all implemented with no third
party packages. Thanks for the review.
