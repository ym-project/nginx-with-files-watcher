# ----------------------- #
# ---- Files-watcher ---- #
# ----------------------- #
FROM rust:1.97-alpine3.24 AS files-watcher-builder

# Add target architecture
RUN rustup target add x86_64-unknown-linux-musl
# Install upx. Upx reduces binary size
RUN apk add --no-cache upx

# Set work dir
WORKDIR /app

# Copy Cargo files
COPY files-watcher/Cargo.toml .
COPY files-watcher/Cargo.lock .

# Download dependencies
# Very important to pass --target. Without this flag cargo tries to download windows' or macos'
# dependencies.
RUN cargo fetch --locked --target x86_64-unknown-linux-musl

# Copy source files
COPY files-watcher/src src

# Build
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN upx --best --lzma target/x86_64-unknown-linux-musl/release/files-watcher


# --------------------- #
# ---- Final image ---- #
# --------------------- #
FROM nginx:1.31.4-alpine

# Copy binary
COPY --from=files-watcher-builder /app/target/x86_64-unknown-linux-musl/release/files-watcher /files-watcher

# Add sh script. Needs to rename script using number 40 because of original image already has
# scripts with names 10, 15, 20, 30. The number in the beginning needs to keep order of scripts
# execution.
# https://github.com/nginx/docker-nginx/blob/b8590bd36b4504b9b847fcf2e98a9111dcae85fa/mainline/alpine-slim/Dockerfile#L113-L116
COPY watch-files.sh /docker-entrypoint.d/40-watch-files.sh
