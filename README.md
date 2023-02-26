# Parsing sandbox

## Measurements

### Reading from memory

```
$ cargo bench

test tests::parse_byte_bench         ... bench:       7,118 ns/iter (+/- 1,094)
test tests::parse_byte_simd128_bench ... bench:       5,252 ns/iter (+/- 1,013)
test tests::parse_byte_simd256_bench ... bench:      15,708 ns/iter (+/- 1,644)
test tests::parse_byte_skip_bench    ... bench:       7,152 ns/iter (+/- 316)
test tests::parse_char_bench         ... bench:       9,545 ns/iter (+/- 312)
test tests::parse_char_skip_bench    ... bench:       6,896 ns/iter (+/- 447)
```

### Reading from file

```
$ rm -f input.txt && node create-file.js
$ cargo build --release
$ time ./target/release/parsing-sandbox bytes && \
    time ./target/release/parsing-sandbox bytes-skip && \
    time ./target/release/parsing-sandbox bytes-simd128 && \
    time ./target/release/parsing-sandbox bytes-simd256 && \
    time ./target/release/parsing-sandbox chars && \
    time ./target/release/parsing-sandbox chars-skip

./target/release/parsing-sandbox bytes  1.87s user 0.79s system 94% cpu 2.807 total
./target/release/parsing-sandbox bytes-skip  1.77s user 0.70s system 97% cpu 2.530 total
./target/release/parsing-sandbox bytes-simd128  1.02s user 0.68s system 99% cpu 1.709 total
./target/release/parsing-sandbox bytes-simd256  2.92s user 0.73s system 99% cpu 3.674 total
./target/release/parsing-sandbox chars  1.21s user 0.69s system 99% cpu 1.921 total
./target/release/parsing-sandbox chars-skip  1.45s user 0.53s system 99% cpu 1.995 total
```
