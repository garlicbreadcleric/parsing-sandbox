# Parsing sandbox

## Benchmarks

Measured on MacBook Pro 2018, 2.6 GHz 6-Core Intel Core i7. Some of the available SIMD instruction sets include SSE, SSE2, SSSE3, SSE4.1, SSE4.2, AVX1.0.

### Counting UTF-8 characters in a byte array

#### Smaller input

| native[^1]         | scalar               | vector128          | vector256            |
|--------------------|----------------------|--------------------|----------------------|
| 36 ns/iter (+/- 1) | 197 ns/iter (+/- 16) | 85 ns/iter (+/- 8) | 556 ns/iter (+/- 89) |

#### Larger input

| native              | scalar               | vector128            | vector256     |
|---------------------|----------------------|----------------------|---------------|
| 199 ns/iter (+/- 9) | 934 ns/iter (+/- 27) | 464 ns/iter (+/- 84) | 2,933 ns/iter |

### Parsing \[pairs of square brackets\]

#### From memory (smaller input)

| chars                   | bytes                  | vector128            | vector256              |
|-------------------------|------------------------|----------------------|------------------------|
| 1,220 ns/iter (+/- 100) | 1,005 ns/iter (+/- 53) | 977 ns/iter (+/- 41) | 1,393 ns/iter (+/- 59) |

#### From memory (larger input)

| chars                   | bytes                   | vector128               | vector256               |
|-------------------------|-------------------------|-------------------------|-------------------------|
| 6,702 ns/iter (+/- 821) | 4,308 ns/iter (+/- 185) | 2,123 ns/iter (+/- 105) | 5,676 ns/iter (+/- 259) |

#### From file

_to do_

## Discussion

- "Native" character counting is quite fast. I wonder if it does any vectorization under the hood as well? I should look into that and see if that's the case and if I can use the same techniques to increase parsing performance boost from vectorization.
- For some reason, using 256-bit registers turns out to be much slower than scalar byte matching. When I started to measure parsing I at first thought one of the reasons is that lookup hits happen more often (as there are less 32-byte slices without any characters that break vectorization, than such 16-byte slices), but character counting benchmarks hint that this is not true as they don't involve any such lookups at all.
- Using 128-bit registers causes a significant (2-4 times) increase in performance on inputs with greater average distance between lookup hits, but on inputs with very dense lookup hits performance is the same as with scalar byte matching (or even slightly worse). In other words, vectorization shines on inputs with longer lines that have less square brackets in them.
  - This means that the performance boost from vectorization might be less significant for an actual[^2] Markdown parser, as there will be more lookup hits (and more false-positives as well).
    - On the other hand, CommonMark is parsed [in two passes](https://spec.commonmark.org/0.30/#appendix-a-parsing-strategy) with different lookups and the first pass probably won't have a lot of lookups so I think vectorization will still give a significant boost there.

## Links

- [SIMDized check which bytes are in a set](http://0x80.pl/articles/simd-byte-lookup.html) — a blog post that describes some more advanced lookup techniques that I didn't have to use in this toy parser but might need when I try to parse a more complicated grammar with more symbols to look for.

[^1]: "Native" character counting refers to `.chars().count()`
