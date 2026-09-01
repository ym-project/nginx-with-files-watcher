# Cert watcher

Utility watches files changes and restart nginx

## Building

1. `cargo build --release` - build release version
2. `upx --best --lzma target/release/cert-watcher` - compress built binary

## Linting

- `cargo check`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
