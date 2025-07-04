fmt:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --all-targets -- -D warnings

test:
    cargo test --workspace

release LEVEL="patch":
    cargo release {{LEVEL}} --workspace --no-confirm
