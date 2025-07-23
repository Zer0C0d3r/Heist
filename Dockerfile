# ---- Build Stage ----
FROM rust:1.77 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/heist /usr/local/bin/heist
COPY README.md /app/README.md
COPY install.sh /app/install.sh

# Optional: add bash, zsh, fish for shell analytics
RUN apt-get update && apt-get install -y bash zsh fish && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["heist"]
CMD ["--help"]

# Usage:
# docker build -t heist .
# docker run --rm heist
