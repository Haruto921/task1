# Dockerfile & Image Best Practices (Terminus Edition 2)

Canonical reference for Dockerfile/image CI checks. Images must be reproducible, cacheable,
lazy-pull friendly, auditable, complete (offline), siloed (no solution/test leak), and resourced.

## CI enforcement

**Block by default:**
- `check_pinned_images` — every `FROM` must be digest-pinned `@sha256:<digest>` (tags allowed for
  readability, digest is the source of truth; never `latest`).
- `check_sanctioned_base_images` — the **final runtime** base must be canonical or non-canonical
  with a credible written justification (Dockerfile comment or README).
- `check_build_context_size` — `environment/` ≤ 100 MiB total, no file > 50 MiB.

**Warn by default** (fix unless reviewer-approved): dockerignore, dockerfile_hygiene, offline_tests,
apt_usage, reproducible_builds, layer_volatility, no_build_tools_in_runtime, file_extraction,
heredoc_usage, recursive_permissions.

## Canonical base images (digest-pinned; use the exact reference)

- **Python** (all 3.10–3.13): `public.ecr.aws/docker/library/python:3.13-slim-bookworm@sha256:01f42367a0a94ad4bc17111776fd66e3500c1d87c15bbd6055b7371d39c124fb`
- **Node** (18/20/22/24): `public.ecr.aws/docker/library/node:22-bookworm-slim@sha256:f3a68cf41a855d227d1b0ab832bed9749469ef38cf4f58182fb8c893bc462383`
- **Go** (1.21–1.26): `public.ecr.aws/docker/library/golang:1.24-bookworm@sha256:1a6d4452c65dea36aac2e2d606b01b4a029ec90cc1ae53890540ce6173ea77ac`
- **Rust** (1.75–1.95): `public.ecr.aws/docker/library/rust:1.85-slim@sha256:9f841bbe9e7d8e37ceb96ed907265a3a0df7f44e3737d0b100e7907a679acb36`
- **Java JDK** (17/21): `public.ecr.aws/docker/library/eclipse-temurin:21-jdk-jammy@sha256:25d1276565738d3c805e632a4542c3a7598866ef967f4def6544c15de3a74b14`
- **C/C++ (GCC 12–15)**: `public.ecr.aws/docker/library/gcc:13-bookworm@sha256:930f2ebe239275fa67226654cb79273ea34eee672ae61c8a39f689c37fb7ac5c`
- **Ruby** (3.2–3.4): `public.ecr.aws/docker/library/ruby:3.3-slim-bookworm@sha256:e76733e94b3a5893e4a141024ef3a583dc10781dc24becebf74f9c9f9a33e3df`
- **Maven**: `public.ecr.aws/docker/library/maven:3.9.9-eclipse-temurin-21@sha256:3a4ab3276a087bf276f79cae96b1af04f53731bec53fb2e651aca79e4b10211e`
- **Debian**: `public.ecr.aws/docker/library/debian:bookworm-slim@sha256:4724b8cc51e33e398f0e2e15e18d5ec2851ff0c2280647e1310bc1642182655d`
- **Ubuntu**: `public.ecr.aws/docker/library/ubuntu:24.04@sha256:0d39fcc8335d6d74d5502f6df2d30119ff4790ebbb60b364818d5112d9e3e932`

Canonical bases already provide the harness utilities (`tmux`, `asciinema`), CA certs, locale, and a
pinned package-manager baseline. Non-canonical needs a real justification or CI blocks it.

## The 15 rules (condensed)

1. **Pin every `FROM` by digest.** No floating tags.
2. **Use a sanctioned runtime base** for the final stage (builder stages may use toolchain images).
3. **Reproducible builds** — pin all deps (lockfiles); pin+checksum downloaded binaries (no bare
   `curl | sh`); avoid embedding time/paths/usernames/random output.
4. **Layer least→most volatile** — OS/runtime deps before source; copy manifests before source;
   install deps before copying frequently-edited files. Avoid `COPY . .` unless minimal + strict
   `.dockerignore`.
5. **One apt transaction per stage** — no `apt-get upgrade`; `--no-install-recommends`; `rm -rf
   /var/lib/apt/lists/*` in the same layer; no build tools in runtime unless the task needs them.
6. **Keep build tools out of runtime** — multi-stage builds when compiling artifacts.
7. **Images contain all deps (offline)** — `tmux`+`asciinema` installed; all downloads at build
   time; `test.sh` must not use curl/wget/pip/npm/etc.; `allow_internet = false`; Oracle passes
   offline.
8. **Silo verifier assets** — never COPY `solution/` or hidden tests into the runtime image; don't
   store ground truth in agent-writable paths.
9. **`.dockerignore` + narrow COPY** — required for non-trivial tasks; prefer `COPY src src` over
   `COPY . .`; exclude `.git`, caches, venvs, build outputs, `solution/`, `tests/`.
10. **Store files as files, not heredocs/opaque archives** *(in the Dockerfile)* — `COPY foo.py`,
    not `RUN cat > foo.py <<EOF`; extract archives at build time. (Oracle `solve.sh` writing a
    source file via heredoc is a separate, allowed pattern — this rule is about the image build.)
11. **Metadata-preserving COPY** — `COPY --chmod=` / `--chown=` per file; no `chmod -R /app`.
12. **Image hygiene** — no `.git`/secrets/caches/unused tools; clean up in the same layer.
13. **Lazy-pull locality** — startup-critical files (entrypoints, instructions, small scripts) in
    earlier/smaller layers; large optional assets later or mounted.
14. **Declare resources in `task.toml` `[environment]`** — realistic cpus/memory_mb/storage_mb,
    bounded build/agent/verifier timeouts; Oracle must pass within limits.
15. **Audit labels** — `LABEL org.opencontainers.image.*` (source, revision, created, version,
    licenses) where applicable.

## Recommended `.dockerignore`

```
.git
.gitignore
**/__pycache__/
**/*.pyc
**/.pytest_cache/
**/.mypy_cache/
**/.ruff_cache/
**/node_modules/
**/target/
**/dist/
**/build/
**/.venv/
**/venv/
.env
*.log
solution/
tests/
```
