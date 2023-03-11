build:
    cargo build --release

check:
    cargo clippy -- -D warnings
    cargo fmt --check

test:
    cargo test

bench-memory: build
    cargo bench

bench-files: build
    #!/usr/bin/env bash
    set -euxo pipefail
    hyperfine \
        './target/release/parsing-sandbox chars seq char' \
        './target/release/parsing-sandbox bytes seq char' \
        './target/release/parsing-sandbox vector128 seq char' \
        './target/release/parsing-sandbox vector256 seq char' \
        './target/release/parsing-sandbox vector128portable seq char' \
        './target/release/parsing-sandbox chars seq utf16' \
        './target/release/parsing-sandbox bytes seq utf16' \
        './target/release/parsing-sandbox vector128portable seq utf16' \
        './target/release/parsing-sandbox chars par char' \
        './target/release/parsing-sandbox bytes par char' \
        './target/release/parsing-sandbox vector128 par char' \
        './target/release/parsing-sandbox vector256 par char' \
        './target/release/parsing-sandbox vector128portable par char' \
        './target/release/parsing-sandbox chars par utf16' \
        './target/release/parsing-sandbox bytes par utf16' \
        './target/release/parsing-sandbox vector128portable par utf16' \

bench: bench-memory bench-files
