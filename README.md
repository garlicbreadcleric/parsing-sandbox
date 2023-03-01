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
  - Native character counting does have an optimized version (`do_count_chars`) with some binary magic, but it only seems to be used for very long strings. This is the code for counting short strings:

    ```rust
    fn char_count_general_case(s: &[u8]) -> usize {
        s.iter().filter(|&&byte| !super::validations::utf8_is_cont_byte(byte)).count()
    }
    ```

    So, it does exactly the same thing as my scalar counter, and yet is several times faster on benchmarks (even after I changed my `is_continuation_byte` implementation to match `utf8_is_cont_byte`) and so far I have no idea why.
  - I also tried changing `get_character_width` to match `utf8_char_width` (using a table of widths instead of comparisons), but that made scalar byte parser almost twice slower, so I reverted that.
- For some reason, using 256-bit registers turns out to be much slower than scalar byte matching. When I started to measure parsing I at first thought one of the reasons is that lookup hits happen more often[^2] (since a 32-byte slice is more likely to include a special character that forces the parser to switch to scalar mode, than a 16-byte slice), but character counting benchmarks hint that this is not true as they don't involve any such lookups at all.
- Using 128-bit registers causes a significant (2-4 times) increase in performance on inputs with greater average distance between lookup hits, but on inputs with very dense lookup hits performance is the same as with scalar byte matching (or even slightly worse). In other words, vectorization shines on inputs with longer lines that have less square brackets in them.
  - This means that the performance boost from vectorization might be less significant for an actual Markdown parser, as there will be more lookup hits (and more false-positives as well).
    - On the other hand, CommonMark is parsed [in two passes](https://spec.commonmark.org/0.30/#appendix-a-parsing-strategy) with different lookups and the first pass probably won't have a lot of lookup hits so I think vectorization will still give a significant boost there.

## Links

- [SIMDized check which bytes are in a set](http://0x80.pl/articles/simd-byte-lookup.html) — a blog post that describes some more advanced lookup techniques that I didn't have to use in this toy parser but might need when I try to parse a more complicated grammar with more symbols to look for.

[^1]: "Native" character counting refers to `.chars().count()`.

[^2]: "Lookup hit" occurs when there's either a square bracket or a newline in the next 16/32 bytes. Whenever lookup hit happens the parser switches to scalar mode to process these 16/32 bytes one by one, and then returns to vector mode.
