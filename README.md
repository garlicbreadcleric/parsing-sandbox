# Parsing sandbox

## Measurements

### Reading from memory

```
$ cargo bench

test tests::parse_byte_bench      ... bench:       6,300 ns/iter (+/- 606)
test tests::parse_byte_skip_bench ... bench:       4,756 ns/iter (+/- 362)
test tests::parse_char_bench      ... bench:       7,734 ns/iter (+/- 374)
test tests::parse_char_skip_bench ... bench:       6,457 ns/iter (+/- 335)
```

### Reading from file

```
$ rm -f input.txt && node create-file.js
$ cargo build --release
$ time ./target/release/parsing-sandbox bytes && \
    time ./target/release/parsing-sandbox bytes-skip && \
    time ./target/release/parsing-sandbox chars && \
    time ./target/release/parsing-sandbox chars-skip

./target/release/parsing-sandbox bytes  1.63s user 0.57s system 99% cpu 2.202 total
./target/release/parsing-sandbox bytes-skip  1.17s user 0.57s system 99% cpu 1.741 total
./target/release/parsing-sandbox chars  1.25s user 0.58s system 99% cpu 1.833 total
./target/release/parsing-sandbox chars-skip  1.21s user 0.46s system 99% cpu 1.672 total
```
