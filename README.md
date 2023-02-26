# Parsing sandbox

## Measurements

### Reading from memory

```
$ cargo bench

test tests::parse_byte_bench         ... bench:       4,677 ns/iter (+/- 465)
test tests::parse_byte_simd128_bench ... bench:       1,073 ns/iter (+/- 323)
test tests::parse_byte_simd256_bench ... bench:       8,096 ns/iter (+/- 2,402)
test tests::parse_byte_skip_bench    ... bench:       4,195 ns/iter (+/- 745)
test tests::parse_char_bench         ... bench:       4,284 ns/iter (+/- 669)
test tests::parse_char_skip_bench    ... bench:       4,507 ns/iter (+/- 353)
```

### Reading from file (208 MB)

```
$ cargo build --release
$ time ./target/release/parsing-sandbox bytes && \
    time ./target/release/parsing-sandbox bytes-skip && \
    time ./target/release/parsing-sandbox bytes-simd128 && \
    time ./target/release/parsing-sandbox bytes-simd256 && \
    time ./target/release/parsing-sandbox chars && \
    time ./target/release/parsing-sandbox chars-skip

./target/release/parsing-sandbox bytes  0.46s user 0.14s system 90% cpu 0.659 total
./target/release/parsing-sandbox bytes-skip  0.43s user 0.11s system 99% cpu 0.546 total
./target/release/parsing-sandbox bytes-simd128  0.24s user 0.11s system 99% cpu 0.354 total
./target/release/parsing-sandbox bytes-simd256  0.66s user 0.11s system 99% cpu 0.778 total
./target/release/parsing-sandbox chars  0.36s user 0.11s system 99% cpu 0.479 total
./target/release/parsing-sandbox chars-skip  0.45s user 0.11s system 99% cpu 0.563 total
```
