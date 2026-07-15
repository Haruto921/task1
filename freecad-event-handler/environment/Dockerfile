FROM rust:1.85-slim@sha256:9f841bbe9e7d8e37ceb96ed907265a3a0df7f44e3737d0b100e7907a679acb36 AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

COPY Cargo.toml .
COPY src ./src

RUN cargo build --release

# Runtime stage
FROM public.ecr.aws/docker/library/debian:bookworm-slim@sha256:4724b8cc51e33e398f0e2e15e18d5ec2851ff0c2280647e1310bc1642182655d

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        tmux \
        asciinema \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m appuser

COPY --from=builder /build/target/release/event_handler /usr/local/bin/event_handler

WORKDIR /app

USER appuser

LABEL org.opencontainers.image.source="https://github.com/terminus-bench/freecad-event-handler"
LABEL org.opencontainers.image.description="Rust GUI event handler simulation task"
LABEL org.opencontainers.image.licenses="MIT"
