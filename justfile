fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --all-targets -- -D warnings

test entity="":
    cd sword-tests && cargo test {{entity}}

test-log entity="":
    cargo test {{entity}} -- --nocapture

example NAME="basic_web_server":
    cd examples && cargo run --example {{NAME}}

release LEVEL="patch":
    cargo release {{LEVEL}} --workspace --no-confirm --no-publish --execute

build:
    cargo build --workspace --all-targets

build-release:
    cargo build --workspace --release --all-targets
