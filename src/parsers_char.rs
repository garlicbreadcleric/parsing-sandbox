//! Parsers that produce character-based offsets.

use std::arch::x86_64::*;
use std::simd::{u8x16, Simd, SimdPartialEq, SimdUint};

use crate::types::*;
use crate::utf8::*;

pub struct CharParser<'a> {
  input: &'a str,
  position: Position,
  range_start: Option<Position>,
  ranges: Vec<Range>,
}

impl<'a> CharParser<'a> {
  pub fn new(input: &str) -> CharParser {
    CharParser { input, position: Position::default(), range_start: None, ranges: vec![] }
  }

  pub fn parse_chars(&mut self) -> &[Range] {
    for char in self.input.chars() {
      let previous_position = self.position;

      self.position.character += 1;
      self.position.offset += char.len_utf8();

      match (char, self.range_start) {
        (']', Some(start)) => {
          self.ranges.push(Range { start, end: self.position });
          self.range_start = None;
        }
        ('[', None) => {
          self.range_start = Some(previous_position);
        }
        ('\n', _) => {
          self.position.line += 1;
          self.position.character = 0;
        }
        _ => {}
      }
    }
    &self.ranges
  }

  pub fn parse_bytes(&mut self) -> &[Range] {
    self.parse_bytes_limited(self.input.len());
    &self.ranges
  }

  pub fn parse_bytes_limited(&mut self, limit: usize) {
    let bytes = self.input.as_bytes();

    let max_offset = (self.position.offset + limit).min(bytes.len());

    while self.position.offset < max_offset {
      let byte = bytes[self.position.offset];

      let previous_position = self.position;

      if is_continuation_byte(byte) {
        self.position.offset += 1;
        continue;
      }

      let character_width = get_character_width(byte);

      self.position.offset += character_width;
      self.position.character += 1;

      if character_width == 1 {
        match (byte, self.range_start) {
          (b'\n', _) => {
            self.position.line += 1;
            self.position.character = 0;
          }
          (b'[', None) => {
            self.range_start = Some(previous_position);
          }
          (b']', Some(start)) => {
            self.ranges.push(Range { start, end: self.position });
            self.range_start = None;
          }
          _ => {}
        }
      }
    }
  }

  pub fn parse_v128(&mut self) -> &[Range] {
    let bytes = self.input.as_bytes();

    while self.position.offset + 15 < bytes.len() {
      let bytes_vec = unsafe { _mm_loadu_si128((bytes[self.position.offset..].as_ptr()).cast()) };

      let lookup: Simd<u8, 16> = match self.range_start {
        Some(_) => {
          // Lookup: ']', '\n'
          let eq_93 = unsafe { _mm_cmpeq_epi8(bytes_vec, _mm_set1_epi8(b']' as i8)) };
          let eq_10 = unsafe { _mm_cmpeq_epi8(bytes_vec, _mm_set1_epi8(b'\n' as i8)) };

          unsafe { _mm_or_si128(eq_93, eq_10) }
        }
        None => {
          // Lookup: '[', '\n'
          let eq_91 = unsafe { _mm_cmpeq_epi8(bytes_vec, _mm_set1_epi8(b'[' as i8)) };
          let eq_10 = unsafe { _mm_cmpeq_epi8(bytes_vec, _mm_set1_epi8(b'\n' as i8)) };

          unsafe { _mm_or_si128(eq_91, eq_10) }
        }
      }
      .into();

      if lookup.reduce_or() != 0 {
        self.parse_bytes_limited(16);
      } else {
        self.position.character += count_utf8_characters_v128(bytes_vec);
        self.position.offset += 16;
      }
    }

    self.parse_bytes();

    &self.ranges
  }

  pub fn parse_v256(&mut self) -> &[Range] {
    let bytes = self.input.as_bytes();

    while self.position.offset + 31 < bytes.len() {
      let bytes_vec = unsafe { _mm256_loadu_si256((bytes[self.position.offset..].as_ptr()).cast()) };

      let lookup: Simd<u8, 32> = match self.range_start {
        Some(_) => {
          // Lookup: ']', '\n'
          let eq_93 = unsafe { _mm256_cmpeq_epi8(bytes_vec, _mm256_set1_epi8(b']' as i8)) };
          let eq_10 = unsafe { _mm256_cmpeq_epi8(bytes_vec, _mm256_set1_epi8(b'\n' as i8)) };

          unsafe { _mm256_or_si256(eq_93, eq_10) }
        }
        None => {
          // Lookup: '[', '\n'
          let eq_91 = unsafe { _mm256_cmpeq_epi8(bytes_vec, _mm256_set1_epi8(b'[' as i8)) };
          let eq_10 = unsafe { _mm256_cmpeq_epi8(bytes_vec, _mm256_set1_epi8(b'\n' as i8)) };

          unsafe { _mm256_or_si256(eq_91, eq_10) }
        }
      }
      .into();

      if lookup.reduce_or() != 0 {
        self.parse_bytes_limited(32);
      } else {
        self.position.character += count_utf8_characters_v256(bytes_vec);
        self.position.offset += 32;
      }
    }

    self.parse_bytes();

    &self.ranges
  }

  pub fn parse_v128_portable(&mut self) -> &[Range] {
    let bytes = self.input.as_bytes();

    while self.position.offset + 15 < bytes.len() {
      let bytes_vec = u8x16::from_slice(&bytes[self.position.offset..]);

      let lookup = match self.range_start {
        Some(_) => {
          // Lookup: ']', '\n'
          let eq_93 = bytes_vec.simd_eq(u8x16::splat(b']'));
          let eq_10 = bytes_vec.simd_eq(u8x16::splat(b'\n'));

          eq_93 | eq_10
        }
        None => {
          // Lookup: '[', '\n'

          let eq_91 = bytes_vec.simd_eq(u8x16::splat(b'['));
          let eq_10 = bytes_vec.simd_eq(u8x16::splat(b'\n'));

          eq_91 | eq_10
        }
      };

      if lookup.any() {
        self.parse_bytes_limited(16);
      } else {
        self.position.character += count_utf8_characters_v128_portable(bytes_vec);
        self.position.offset += 16;
      }
    }

    self.parse_bytes();

    &self.ranges
  }
}

#[cfg(test)]
pub mod tests {
  extern crate test;
  use test::bench::Bencher;

  use proptest::prelude::*;

  use super::*;
  use crate::tests::test_data::*;

  proptest! {
    #[test]
    fn parse_property_test(s in "[0-9a-zA-Zа-яА-Я\\[\\]]{300}") {
      let ranges1 = CharParser::new(s.as_str()).parse_chars().to_vec();
      let ranges2 = CharParser::new(s.as_str()).parse_bytes().to_vec();
      let ranges3 = CharParser::new(s.as_str()).parse_v128().to_vec();
      let ranges4 = CharParser::new(s.as_str()).parse_v256().to_vec();
      let ranges5 = CharParser::new(s.as_str()).parse_v128_portable().to_vec();

      assert_eq!(ranges1.len(), ranges2.len());
      assert_eq!(ranges2.len(), ranges3.len());
      assert_eq!(ranges3.len(), ranges4.len());
      assert_eq!(ranges4.len(), ranges5.len());

      for i in 0..ranges1.len() {
        assert_eq!(ranges1[i], ranges2[i], "ranges1[i] == ranges2[i]");
        assert_eq!(ranges2[i], ranges3[i], "ranges2[i] == ranges3[i]");
        assert_eq!(ranges3[i], ranges4[i], "ranges3[i] == ranges4[i]");
        assert_eq!(ranges3[i], ranges5[i], "ranges4[i] == ranges5[i]");
      }
    }
  }

  #[test]
  pub fn parse_gibberish_test() {
    let gibberish = "АaaAa0AAAaА0aАAАaAaАAAAaaaAA0aa]aaaaaaАaaA0AAa]00AaA]]aaА0aA]АaА]АaA00]a0А]]0a0АА]0AaaАa0]aaАA0AА0A0AAAAaАAАААAAaА]]a0]aaA]0A0aAaAaAaaaaА0a0A]]A0a0a]aА0AaAAaa]]AaA0AААAa]]AAaА0AA]0АaAa0AAАААaA]]AAaАA0A0А00a0aaAААA0a0AАaA]aАa0A]0a0AАaAa0aА]0АAAa]А]AА]]AaA0AaA0000aaАa]AaAaA]aAAAА]aAA[]AAaaAaa0Aaaaa]E]";

    let ranges1 = CharParser::new(gibberish).parse_chars().to_vec();
    let ranges2 = CharParser::new(gibberish).parse_bytes().to_vec();
    let ranges3 = CharParser::new(gibberish).parse_v128().to_vec();
    let ranges4 = CharParser::new(gibberish).parse_v256().to_vec();
    let ranges5 = CharParser::new(gibberish).parse_v128_portable().to_vec();

    assert_eq!(ranges1.len(), ranges2.len());
    assert_eq!(ranges2.len(), ranges3.len());
    assert_eq!(ranges3.len(), ranges4.len());
    assert_eq!(ranges4.len(), ranges5.len());

    for i in 0..ranges1.len() {
      assert_eq!(ranges1[i], ranges2[i], "ranges1[i] == ranges2[i]");
      assert_eq!(ranges2[i], ranges3[i], "ranges2[i] == ranges3[i]");
      assert_eq!(ranges3[i], ranges4[i], "ranges3[i] == ranges4[i]");
      assert_eq!(ranges3[i], ranges5[i], "ranges4[i] == ranges5[i]");
    }
  }

  #[test]
  pub fn parse_small_ascii_test() {
    let ranges1 = CharParser::new(SHORT_ASCII_INPUT).parse_chars().to_vec();
    let ranges2 = CharParser::new(SHORT_ASCII_INPUT).parse_bytes().to_vec();
    let ranges3 = CharParser::new(SHORT_ASCII_INPUT).parse_v128().to_vec();
    let ranges4 = CharParser::new(SHORT_ASCII_INPUT).parse_v256().to_vec();
    let ranges5 = CharParser::new(SHORT_ASCII_INPUT).parse_v128_portable().to_vec();

    for ranges in vec![ranges1, ranges2, ranges3, ranges4, ranges5] {
      assert_eq!(ranges.len(), 1);
      assert_eq!(
        ranges[0],
        Range {
          start: Position { line: 0, character: 4, offset: 4 },
          end: Position { line: 0, character: 9, offset: 9 }
        }
      )
    }
  }

  #[test]
  pub fn parse_small_unicode_test() {
    let ranges1 = CharParser::new(SHORT_UNICODE_INPUT).parse_chars().to_vec();
    let ranges2 = CharParser::new(SHORT_UNICODE_INPUT).parse_bytes().to_vec();
    let ranges3 = CharParser::new(SHORT_UNICODE_INPUT).parse_v128().to_vec();
    let ranges4 = CharParser::new(SHORT_UNICODE_INPUT).parse_v256().to_vec();
    let ranges5 = CharParser::new(SHORT_UNICODE_INPUT).parse_v128_portable().to_vec();

    for ranges in vec![ranges1, ranges2, ranges3, ranges4, ranges5] {
      assert_eq!(ranges.len(), 1);
      assert_eq!(
        ranges[0],
        Range {
          start: Position { line: 0, character: 4, offset: 7 },
          end: Position { line: 0, character: 9, offset: 15 }
        }
      )
    }
  }

  #[test]
  pub fn medium_ascii_test() {
    let ranges1 = CharParser::new(LONG_ASCII_INPUT).parse_chars().to_vec();
    let ranges2 = CharParser::new(LONG_ASCII_INPUT).parse_bytes().to_vec();
    let ranges3 = CharParser::new(LONG_ASCII_INPUT).parse_v128().to_vec();
    let ranges4 = CharParser::new(LONG_ASCII_INPUT).parse_v256().to_vec();
    let ranges5 = CharParser::new(LONG_ASCII_INPUT).parse_v128_portable().to_vec();

    for ranges in vec![ranges1, ranges2, ranges3, ranges4, ranges5] {
      assert_eq!(ranges.len(), 1);
      assert_eq!(
        ranges[0],
        Range {
          start: Position { line: 0, character: 42, offset: 42 },
          end: Position { line: 0, character: 55, offset: 55 }
        }
      )
    }
  }

  #[test]
  pub fn medium_unicode_test() {
    let ranges1 = CharParser::new(LONG_UNICODE_INPUT).parse_chars().to_vec();
    let ranges2 = CharParser::new(LONG_UNICODE_INPUT).parse_bytes().to_vec();
    let ranges3 = CharParser::new(LONG_UNICODE_INPUT).parse_v128().to_vec();
    let ranges4 = CharParser::new(LONG_UNICODE_INPUT).parse_v256().to_vec();
    let ranges5 = CharParser::new(LONG_UNICODE_INPUT).parse_v128_portable().to_vec();

    for ranges in vec![ranges1, ranges2, ranges3, ranges4, ranges5] {
      assert_eq!(ranges.len(), 1);
      assert_eq!(
        ranges[0],
        Range {
          start: Position { line: 0, character: 36, offset: 66 },
          end: Position { line: 0, character: 49, offset: 88 }
        }
      )
    }
  }

  #[test]
  pub fn short_multiline_test() {
    let ranges1 = CharParser::new(SHORT_MULTILINE_INPUT).parse_chars().to_vec();
    let ranges2 = CharParser::new(SHORT_MULTILINE_INPUT).parse_bytes().to_vec();
    let ranges3 = CharParser::new(SHORT_MULTILINE_INPUT).parse_v128().to_vec();
    let ranges4 = CharParser::new(SHORT_MULTILINE_INPUT).parse_v256().to_vec();
    let ranges5 = CharParser::new(SHORT_MULTILINE_INPUT).parse_v128_portable().to_vec();

    assert_eq!(ranges1.len(), ranges2.len());
    assert_eq!(ranges2.len(), ranges3.len());
    assert_eq!(ranges3.len(), ranges4.len());
    assert_eq!(ranges4.len(), ranges5.len());

    for i in 0..ranges1.len() {
      assert_eq!(ranges1[i], ranges2[i]);
      assert_eq!(ranges2[i], ranges3[i]);
      assert_eq!(ranges3[i], ranges4[i]);
      assert_eq!(ranges4[i], ranges5[i]);
    }
  }

  #[test]
  pub fn long_multiline_test() {
    let ranges1 = CharParser::new(LONG_MULTILINE_INPUT).parse_chars().to_vec();
    let ranges2 = CharParser::new(LONG_MULTILINE_INPUT).parse_bytes().to_vec();
    let ranges3 = CharParser::new(LONG_MULTILINE_INPUT).parse_v128().to_vec();
    let ranges4 = CharParser::new(LONG_MULTILINE_INPUT).parse_v256().to_vec();
    let ranges5 = CharParser::new(LONG_MULTILINE_INPUT).parse_v128_portable().to_vec();

    assert_eq!(ranges1.len(), ranges2.len());
    assert_eq!(ranges2.len(), ranges3.len());
    assert_eq!(ranges3.len(), ranges4.len());
    assert_eq!(ranges4.len(), ranges5.len());

    for i in 0..ranges1.len() {
      assert_eq!(ranges1[i], ranges2[i]);
      assert_eq!(ranges2[i], ranges3[i]);
      assert_eq!(ranges3[i], ranges4[i]);
      assert_eq!(ranges4[i], ranges5[i]);
    }
  }

  #[bench]
  pub fn parse_chars_bench(b: &mut Bencher) {
    b.iter(|| {
      CharParser::new(BENCHMARK_INPUT).parse_chars();
    })
  }

  #[bench]
  pub fn parse_bytes_bench(b: &mut Bencher) {
    b.iter(|| {
      CharParser::new(BENCHMARK_INPUT).parse_bytes();
    })
  }

  #[bench]
  pub fn parse_v128_bench(b: &mut Bencher) {
    b.iter(|| {
      CharParser::new(BENCHMARK_INPUT).parse_v128();
    })
  }

  #[bench]
  pub fn parse_v256_bench(b: &mut Bencher) {
    b.iter(|| {
      CharParser::new(BENCHMARK_INPUT).parse_v256();
    })
  }

  #[bench]
  pub fn parse_v128_portable_bench(b: &mut Bencher) {
    b.iter(|| {
      CharParser::new(BENCHMARK_INPUT).parse_v128_portable();
    })
  }
}
