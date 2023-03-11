build:
    cargo build --release

check:
    cargo clippy -- -D warnings
    cargo fmt --check

test:
    cargo test

prepare-bench-data:
    node generate-md.js

bench-memory: build
    cargo bench

bench-files: build prepare-bench-data
    #!/usr/bin/env bash
    set -euxo pipefail
    hyperfine \
        './target/release/parsing-sandbox chars seq utf32' \
        './target/release/parsing-sandbox bytes seq utf32' \
        './target/release/parsing-sandbox vector128 seq utf32' \
        './target/release/parsing-sandbox vector256 seq utf32' \
        './target/release/parsing-sandbox vector128portable seq utf32' \
        './target/release/parsing-sandbox chars seq utf16' \
        './target/release/parsing-sandbox bytes seq utf16' \
        './target/release/parsing-sandbox vector128portable seq utf16' \
        './target/release/parsing-sandbox chars par utf32' \
        './target/release/parsing-sandbox bytes par utf32' \
        './target/release/parsing-sandbox vector128 par utf32' \
        './target/release/parsing-sandbox vector256 par utf32' \
        './target/release/parsing-sandbox vector128portable par utf32' \
        './target/release/parsing-sandbox chars par utf16' \
        './target/release/parsing-sandbox bytes par utf16' \
        './target/release/parsing-sandbox vector128portable par utf16' \

bench: bench-memory bench-files
