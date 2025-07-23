# ---- Build Stage ----
FROM rust:1.77 as builder
ARG HEIST_VERSION=unknown
WORKDIR /app
COPY . .
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bullseye-slim
ARG HEIST_VERSION
LABEL maintainer="Zer0C0d3r"
LABEL org.opencontainers.image.version=$HEIST_VERSION
LABEL org.opencontainers.image.title="Heist"
WORKDIR /app
COPY --from=builder /app/target/release/heist /usr/local/bin/heist
COPY README.md /app/README.md
COPY install.sh /app/install.sh

# Optional: add bash, zsh, fish for shell analytics
RUN apt-get update && apt-get install -y bash zsh fish && rm -rf /var/lib/apt/lists/*

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 CMD ["heist", "--version"]

ENTRYPOINT ["heist"]
CMD ["--help"]

# Usage:
# docker build --build-arg HEIST_VERSION=$(git describe --tags --always) -t heist:latest .
# docker run --rm -it heist
# Multi-arch: docker buildx build --platform linux/amd64,linux/arm64 -t heist:latest .
