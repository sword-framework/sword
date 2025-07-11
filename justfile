fmt:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --all-targets -- -D warnings

test:
    cargo test --workspace

test-log:
    cargo test --workspace -- --nocapture

example NAME="basic_web_server":
    cd examples && cargo run --example {{NAME}}

release LEVEL="patch":
    cargo release {{LEVEL}} --workspace --no-confirm --no-publish --execute
