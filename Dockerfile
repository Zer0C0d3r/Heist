# ---- Build Dependencies Stage ----
FROM rust:1.77-slim-bullseye as deps
WORKDIR /app
# Copy Cargo.toml first
COPY Cargo.toml ./
# Copy Cargo.lock if it exists (using wildcard to avoid error)
COPY Cargo.lock* ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# ---- Build Stage ----
FROM rust:1.77-slim-bullseye as builder
WORKDIR /app
# Copy dependencies from deps stage
COPY --from=deps /app/target target
COPY --from=deps /app/Cargo.* ./
# Now copy the source code
COPY . .
# Build the application
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bullseye-slim
ARG HEIST_VERSION=unknown
ARG USERNAME=heist
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Add metadata
LABEL maintainer="Zer0C0d3r" \
      org.opencontainers.image.version=$HEIST_VERSION \
      org.opencontainers.image.title="Heist" \
      org.opencontainers.image.description="Shell History Analyzer" \
      org.opencontainers.image.url="https://github.com/Zer0C0d3r/Heist" \
      org.opencontainers.image.licenses="MIT"

# Install runtime dependencies and create non-root user
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    bash \
    zsh \
    fish \
    ca-certificates && \
    groupadd --gid $USER_GID $USERNAME && \
    useradd --uid $USER_UID --gid $USER_GID -m $USERNAME && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set up application
WORKDIR /app
COPY --from=builder /app/target/release/heist /usr/local/bin/heist
COPY --chown=$USERNAME:$USERNAME README.md install.sh ./

# Use non-root user
USER $USERNAME

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD ["heist", "--version"]

# Set entrypoint and default command
ENTRYPOINT ["heist"]
CMD ["--help"]

# Volume for shell history
VOLUME ["/home/$USERNAME"]

# Usage examples as comments:
# Build:
#   docker build --build-arg HEIST_VERSION=$(git describe --tags --always) -t heist:latest .
# Run:
#   docker run --rm -it -v ~/.bash_history:/home/heist/.bash_history heist
# Build multi-arch:
#   docker buildx build --platform linux/amd64,linux/arm64 -t heist:latest .
