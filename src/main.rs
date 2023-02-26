#![feature(test)]
#![feature(portable_simd)]

use core::simd::Simd;
use std::{
  arch::x86_64::{
    _mm256_and_si256, _mm256_cmpeq_epi8, _mm256_loadu_si256, _mm256_or_si256, _mm256_set1_epi8,
    _mm_and_si128, _mm_cmpeq_epi8, _mm_loadu_si128, _mm_or_si128, _mm_set1_epi8,
  },
  simd::SimdUint,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
  pub line: usize,
  pub character: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
  pub start: Position,
  pub end: Position,
}

pub fn parse_chars(input: &str) -> Vec<Range> {
  let mut ranges = vec![];
  let mut range_start = None;
  let mut position = Position {
    line: 0,
    character: 0,
  };

  for char in input.chars() {
    match (char, range_start) {
      (']', Some(start)) => {
        ranges.push(Range {
          start,
          end: position,
        });
        range_start = None;
        position.character += 1;
      }
      ('[', None) => {
        range_start = Some(position);
        position.character += 1;
      }
      ('\n', _) => {
        position.line += 1;
        position.character = 0;
      }
      _ => {
        position.character += 1;
      }
    }
  }
  ranges
}

pub fn parse_chars_skip(input: &str) -> Vec<Range> {
  let mut ranges = vec![];
  let mut range_start = None;
  let mut position = Position {
    line: 0,
    character: 0,
  };

  let mut chars = input.chars();
  let chars = chars.by_ref();

  while let Some(next) = chars.next() {
    match (next, range_start) {
      ('[', None) => {
        range_start = Some(position);
        position.character += 1;
      }
      (']', Some(start)) => {
        ranges.push(Range {
          start,
          end: position,
        });
        position.character += 1;
      }
      ('\n', _) => {
        position.line += 1;
        position.character = 0;
      }
      _ => {
        position.character += 1;
      }
    }

    position.character += chars
      .take_while(|&c| c != '\n' && c != '[' && c != ']')
      .count();
  }

  ranges
}

pub fn parse_bytes(input: &str) -> Vec<Range> {
  let mut ranges = vec![];
  let mut range_start = None;
  let mut position = Position {
    line: 0,
    character: 0,
  };

  let bytes = input.as_bytes();
  let mut offset = 0;

  while offset < bytes.len() {
    let byte = bytes[offset];

    if byte < 0b1000_0000 {
      offset += 1;
      match (byte, range_start) {
        (10, _) => {
          position.line += 1;
          position.character = 0;
          continue;
        }
        (91, None) => {
          range_start = Some(position);
        }
        (93, Some(start)) => {
          ranges.push(Range {
            start,
            end: position,
          });
          range_start = None;
        }
        _ => {}
      }
    } else if byte < 0b1110_0000 {
      offset += 2;
    } else if byte < 0b1111_0000 {
      offset += 3;
    } else {
      offset += 4;
    }

    position.character += 1;
  }

  ranges
}

pub fn parse_bytes_skip(input: &str) -> Vec<Range> {
  let mut ranges = vec![];
  let mut range_start = None;
  let mut position = Position {
    line: 0,
    character: 0,
  };

  let mut bytes = input.bytes();
  let bytes = bytes.by_ref();

  while let Some(byte) = bytes.next() {
    if byte < 0b1000_0000 {
      match (byte, range_start) {
        (10, _) => {
          position.line += 1;
          position.character = 0;
          continue;
        }
        (91, None) => {
          range_start = Some(position);
        }
        (93, Some(start)) => {
          ranges.push(Range {
            start,
            end: position,
          });
          range_start = None;
        }
        _ => {}
      }
    } else if byte < 0b1110_0000 {
      let _ = bytes.skip(1);
    } else if byte < 0b1111_0000 {
      let _ = bytes.skip(2);
    } else {
      let _ = bytes.skip(3);
    }

    position.character += 1;
  }

  ranges
}

pub fn parse_bytes_simd128(input: &str) -> Vec<Range> {
  let mut ranges = vec![];
  let mut range_start = None;
  let bytes = input.as_bytes();
  let mut offset = 0usize;
  let mut position = Position {
    line: 0,
    character: 0,
  };

  while offset < bytes.len() - 15 {
    let cv = unsafe { _mm_loadu_si128((bytes[offset..].as_ptr()).cast()) };

    // http://0x80.pl/articles/simd-byte-lookup.html
    let lookup: Simd<u8, 16> = match range_start {
      Some(_) => {
        // Lookup: ']', '\n'
        let eq_93 = unsafe { _mm_cmpeq_epi8(cv, _mm_set1_epi8(b']' as i8)) };
        let eq_10 = unsafe { _mm_cmpeq_epi8(cv, _mm_set1_epi8(b'\n' as i8)) };

        unsafe { _mm_or_si128(eq_93, eq_10) }
      }
      None => {
        // Lookup: '[', '\n'
        let eq_91 = unsafe { _mm_cmpeq_epi8(cv, _mm_set1_epi8(b'[' as i8)) };
        let eq_10 = unsafe { _mm_cmpeq_epi8(cv, _mm_set1_epi8(b'\n' as i8)) };

        unsafe { _mm_or_si128(eq_91, eq_10) }
      }
    }
    .into();
    if lookup.reduce_or() != 0 {
      // Process bytes one at a time.
      let mut i = 0;
      while i < 16 {
        let byte = bytes[offset + i];

        if byte < 0b1000_0000 {
          i += 1;
          match (byte, range_start) {
            (10, _) => {
              position.line += 1;
              position.character = 0;
              continue;
            }
            (91, None) => {
              range_start = Some(position);
            }
            (93, Some(start)) => {
              ranges.push(Range {
                start,
                end: position,
              });
              range_start = None;
            }
            _ => {}
          }
        } else if byte < 0b1110_0000 {
          i += 2;
        } else if byte < 0b1111_0000 {
          i += 3;
        } else {
          i += 4;
        }

        position.character += 1;
      }
    } else {
      let mask: Simd<u8, 16> = unsafe {
        _mm_cmpeq_epi8(
          _mm_and_si128(cv, _mm_set1_epi8(0b1100_0000u8 as i8)),
          _mm_set1_epi8(0b1000_0000u8 as i8),
        )
      }
      .into();
      let continuation_bytes = mask.reduce_sum() / 255;
      let code_points = 16 - continuation_bytes;
      position.character += code_points as usize;
    }
    offset += 16;
  }

  ranges
}

pub fn parse_bytes_simd256(input: &str) -> Vec<Range> {
  let mut ranges = vec![];
  let mut range_start = None;
  let bytes = input.as_bytes();
  let mut offset = 0usize;
  let mut position = Position {
    line: 0,
    character: 0,
  };

  while offset < bytes.len() - 15 {
    let cv = unsafe { _mm256_loadu_si256((bytes[offset..].as_ptr()).cast()) };

    // http://0x80.pl/articles/simd-byte-lookup.html
    let lookup: Simd<u8, 32> = match range_start {
      Some(_) => {
        // Lookup: ']', '\n'
        let eq_93 = unsafe { _mm256_cmpeq_epi8(cv, _mm256_set1_epi8(b']' as i8)) };
        let eq_10 = unsafe { _mm256_cmpeq_epi8(cv, _mm256_set1_epi8(b'\n' as i8)) };

        unsafe { _mm256_or_si256(eq_93, eq_10) }
      }
      None => {
        // Lookup: '[', '\n'
        let eq_91 = unsafe { _mm256_cmpeq_epi8(cv, _mm256_set1_epi8(b'[' as i8)) };
        let eq_10 = unsafe { _mm256_cmpeq_epi8(cv, _mm256_set1_epi8(b'\n' as i8)) };

        unsafe { _mm256_or_si256(eq_91, eq_10) }
      }
    }
    .into();
    if lookup.reduce_or() != 0 {
      // Process bytes one at a time.
      let mut i = 0;
      while i < 16 {
        let byte = bytes[offset + i];

        if byte < 0b1000_0000 {
          i += 1;
          match (byte, range_start) {
            (10, _) => {
              position.line += 1;
              position.character = 0;
              continue;
            }
            (91, None) => {
              range_start = Some(position);
            }
            (93, Some(start)) => {
              ranges.push(Range {
                start,
                end: position,
              });
              range_start = None;
            }
            _ => {}
          }
        } else if byte < 0b1110_0000 {
          i += 2;
        } else if byte < 0b1111_0000 {
          i += 3;
        } else {
          i += 4;
        }

        position.character += 1;
      }
    } else {
      let mask: Simd<u8, 32> = unsafe {
        _mm256_cmpeq_epi8(
          _mm256_and_si256(cv, _mm256_set1_epi8(0b1100_0000u8 as i8)),
          _mm256_set1_epi8(0b1000_0000u8 as i8),
        )
      }
      .into();
      let continuation_bytes = mask.reduce_sum() / 255;
      let code_points = 16 - continuation_bytes;
      position.character += code_points as usize;
    }
    offset += 16;
  }

  ranges
}

fn main() {
  let args: Vec<_> = std::env::args().collect();
  let input = std::fs::read_to_string("input.txt").unwrap();
  let input = input.as_str();

  let ranges = match args[1].as_str() {
    "chars" => parse_chars(input),
    "chars-skip" => parse_chars_skip(input),
    "bytes" => parse_bytes(input),
    "bytes-skip" => parse_bytes_skip(input),
    "bytes-simd128" => parse_bytes_simd128(input),
    "bytes-simd256" => parse_bytes_simd256(input),
    _ => panic!("Unknown parser: {}", args[1]),
  };

  if ranges.len() % 2 == 0 {
    print!("");
  }
}

#[cfg(test)]
mod tests {
  extern crate test;
  use test::bench::Bencher;

  use super::*;

  static INPUT: &str = "
  # Grune, Dick, and Ceriel J. H. Jacobs. _Parsing Techniques: A Practical Guide_. 1990.

**Определение**: _Парсинг_ --- процесс структурирования линейного предствления в соответствии с некоторой грамматикой [@grune_parsingtechniques_en_1990, 1].


## Грамматики как средства генерации предложений

Два типа символов [@grune_parsingtechniques_en_1990, 13]:

- **Определение**: _Терминальный символ_ --- символ, буквально присутствующий в предложениях языка (например, `42`)
- **Определение**: _Нетерминальный символ_ --- символ, заменяющий собой некоторые части предложений (например, `сложение`)

**Определение**: _Начальный символ_ --- нетерминальный символ, с которого начинается генерация предложений (например, `выражение`)

**Определение**: _Грамматика с фразовой структурой_ --- набор $(V_N, V_T, R, S)$ такой, что:

1. $V_N$ и $V_T$ --- конечные множества символов;
2. $V_N \\cap V_T = \\empty$;
3. $R$ --- множество пар $(P, Q)$ таких, что
   i. $P \\in (V_N \\cup V_T)^+$;
   ii. $Q \\in (V_N \\cup V_T)^*$;
4. $S \\in V_N$ [@grune_parsingtechniques_en_1990, 14].

**Определение**: _Иерархия Хомского_ --- классификация формальных грамматик по количеству ограничений [@grune_parsingtechniques_en_1990, 19].

**Определение**: _Тип 0_ --- класс неограниченных грамматик с фазовой структурой (см. выше).

**Определение**: _Тип 1 (монотонные)_ --- класс грамматик, у которых не правил, левая часть которых состояла бы из большего числа символов, чем правая ($\\forall (P, Q) \\in R, 1 \\leq P \\leq Q$).

**Определение**: _Тип 1 (контекстно-зависимые, context-sensitive, CS)_ --- класс грамматик, все правила которых являются контекстно-зависимыми, то есть в них только один нетерминальный символ из левой части заменяется непустым наборов символов в правой части, а другие остаются в том же составе и порядке ($(P, Q) = (\\alpha A \\beta, \\alpha \\gamma \\beta), A \\in V = V_N \\cup V_T, \\alpha, \\beta \\in V^*, \\gamma \\in V^+$).

**Определение**: _Тип 2 (контекстно-независимые, context-free, CF)_ --- класс контекстно-зависимых грамматик с пустым контекстом ($\\forall (P, Q) \\in R, (P, Q) = (A, \\gamma), A \\in V, \\gamma \\in V^+$) [@grune_parsingtechniques_en_1990, 23].

**Определение**: Нетерминальный символ, язык которого содержит $\\epsilon$, называется _обнуляемым (nullable)_.";

  #[test]
  pub fn parser_test() {
    let ranges1 = parse_chars(INPUT);
    let ranges2 = parse_chars_skip(INPUT);
    let ranges3 = parse_bytes(INPUT);
    let ranges4 = parse_bytes_skip(INPUT);
    let ranges5 = parse_bytes_simd128(INPUT);
    let ranges6 = parse_bytes_simd256(INPUT);

    for i in 0..ranges2.len() {
      let r1 = ranges1[i];
      let r2 = ranges2[i];
      let r3 = ranges3[i];
      let r4 = ranges4[i];
      let r5 = ranges5[i];
      let r6 = ranges6[i];
      assert_eq!(r1, r2);
      assert_eq!(r2, r3);
      assert_eq!(r3, r4);
      assert_eq!(r4, r5);
      assert_eq!(r5, r6);
    }
  }

  #[bench]
  pub fn parse_char_bench(b: &mut Bencher) {
    b.iter(|| {
      parse_chars(INPUT);
    });
  }

  #[bench]
  pub fn parse_char_skip_bench(b: &mut Bencher) {
    b.iter(|| {
      parse_chars_skip(INPUT);
    });
  }

  #[bench]
  pub fn parse_byte_bench(b: &mut Bencher) {
    b.iter(|| {
      parse_bytes(INPUT);
    });
  }

  #[bench]
  pub fn parse_byte_skip_bench(b: &mut Bencher) {
    b.iter(|| {
      parse_bytes_skip(INPUT);
    });
  }

  #[bench]
  pub fn parse_byte_simd128_bench(b: &mut Bencher) {
    b.iter(|| {
      parse_bytes_simd128(INPUT);
    });
  }

  #[bench]
  pub fn parse_byte_simd256_bench(b: &mut Bencher) {
    b.iter(|| {
      parse_bytes_simd256(INPUT);
    });
  }
}
