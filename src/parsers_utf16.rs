//! Parsers that produce offsets based on UTF-16 code points (LSP-compatible).

use std::simd::{u8x16, SimdPartialEq};

use crate::types::*;
use crate::utf8::*;

pub struct Utf16Parser<'a> {
  input: &'a str,
  offset: usize,
  line: usize,
  character: usize,
  character_offset: usize,
  range_start: Option<Position>,
  ranges: Vec<Range>,
}

impl<'a> Utf16Parser<'a> {
  pub fn new(input: &str) -> Utf16Parser {
    Utf16Parser { input, offset: 0, line: 0, character: 0, character_offset: 0, range_start: None, ranges: vec![] }
  }

  pub fn parse_chars(&mut self) -> &[Range] {
    let mut line = 0;
    let mut character = 0;

    for char in self.input.chars() {
      let previous_position = Position { line, character, offset: self.offset };

      character += char.len_utf16();
      self.offset += char.len_utf8();

      match (char, self.range_start) {
        (']', Some(start)) => {
          self.ranges.push(Range { start, end: Position { line, character, offset: self.offset } });
          self.range_start = None;
        }
        ('[', None) => {
          self.range_start = Some(previous_position);
        }
        ('\n', _) => {
          line += 1;
          character = 0;
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

    let max_offset = (self.offset + limit).min(bytes.len());

    while self.offset < max_offset {
      let byte = bytes[self.offset];

      let character_width = get_character_width(byte);
      self.offset += character_width;

      if character_width == 1 {
        match (byte, self.range_start) {
          (b'\n', _) => {
            self.line += 1;
            self.character_offset = self.offset;
            self.character = 0;
          }
          (b'[', None) => {
            self.character += unsafe { simdutf::count_utf16_from_utf8(&bytes[self.character_offset..self.offset - 1]) };
            self.character_offset = self.offset - 1;
            self.range_start = Some(Position { line: self.line, character: self.character, offset: self.offset - 1 });
          }
          (b']', Some(start)) => {
            self.character += unsafe { simdutf::count_utf16_from_utf8(&bytes[self.character_offset..self.offset]) };
            self.character_offset = self.offset;
            self
              .ranges
              .push(Range { start, end: Position { line: self.line, character: self.character, offset: self.offset } });
            self.range_start = None;
          }
          _ => {}
        }
      }
    }
  }

  pub fn parse_v128_portable(&mut self) -> &[Range] {
    let bytes = self.input.as_bytes();

    while self.offset + 15 < bytes.len() {
      let bytes_vec = u8x16::from_slice(&bytes[self.offset..]);

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
        self.offset += 16;
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
      let ranges1 = Utf16Parser::new(s.as_str()).parse_chars().to_vec();
      let ranges2 = Utf16Parser::new(s.as_str()).parse_bytes().to_vec();
      let ranges3 = Utf16Parser::new(s.as_str()).parse_v128_portable().to_vec();

      assert_eq!(ranges1.len(), ranges2.len());
      assert_eq!(ranges2.len(), ranges3.len());

      for i in 0..ranges1.len() {
        assert_eq!(ranges1[i], ranges2[i]);
        assert_eq!(ranges2[i], ranges3[i]);
      }
    }
  }

  #[test]
  pub fn parse_small_ascii_test() {
    let ranges1 = Utf16Parser::new(SHORT_ASCII_INPUT).parse_chars().to_vec();
    let ranges2 = Utf16Parser::new(SHORT_ASCII_INPUT).parse_bytes().to_vec();
    let ranges3 = Utf16Parser::new(SHORT_ASCII_INPUT).parse_v128_portable().to_vec();

    for ranges in vec![ranges1, ranges2, ranges3] {
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
    let ranges1 = Utf16Parser::new(SHORT_UNICODE_INPUT).parse_chars().to_vec();
    let ranges2 = Utf16Parser::new(SHORT_UNICODE_INPUT).parse_bytes().to_vec();
    let ranges3 = Utf16Parser::new(SHORT_UNICODE_INPUT).parse_v128_portable().to_vec();

    for ranges in vec![ranges1, ranges2, ranges3] {
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
    let ranges1 = Utf16Parser::new(LONG_ASCII_INPUT).parse_chars().to_vec();
    let ranges2 = Utf16Parser::new(LONG_ASCII_INPUT).parse_bytes().to_vec();
    let ranges3 = Utf16Parser::new(LONG_ASCII_INPUT).parse_v128_portable().to_vec();

    for ranges in vec![ranges1, ranges2, ranges3] {
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
    let ranges1 = Utf16Parser::new(LONG_UNICODE_INPUT).parse_chars().to_vec();
    let ranges2 = Utf16Parser::new(LONG_UNICODE_INPUT).parse_bytes().to_vec();
    let ranges3 = Utf16Parser::new(LONG_UNICODE_INPUT).parse_v128_portable().to_vec();

    for ranges in vec![ranges1, ranges2, ranges3] {
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
    let ranges1 = Utf16Parser::new(SHORT_MULTILINE_INPUT).parse_chars().to_vec();
    let ranges2 = Utf16Parser::new(SHORT_MULTILINE_INPUT).parse_bytes().to_vec();
    let ranges3 = Utf16Parser::new(SHORT_MULTILINE_INPUT).parse_v128_portable().to_vec();

    assert_eq!(ranges1.len(), ranges2.len());
    assert_eq!(ranges2.len(), ranges3.len());

    for i in 0..ranges1.len() {
      assert_eq!(ranges1[i], ranges2[i]);
      assert_eq!(ranges2[i], ranges3[i]);
    }
  }

  #[test]
  pub fn long_multiline_test() {
    let ranges1 = Utf16Parser::new(LONG_MULTILINE_INPUT).parse_chars().to_vec();
    let ranges2 = Utf16Parser::new(LONG_MULTILINE_INPUT).parse_bytes().to_vec();
    let ranges3 = Utf16Parser::new(LONG_MULTILINE_INPUT).parse_v128_portable().to_vec();

    assert_eq!(ranges1.len(), ranges2.len());
    assert_eq!(ranges2.len(), ranges3.len());

    for i in 0..ranges1.len() {
      assert_eq!(ranges1[i], ranges2[i]);
      assert_eq!(ranges2[i], ranges3[i]);
    }
  }

  #[bench]
  pub fn parse_chars_bench(b: &mut Bencher) {
    b.iter(|| {
      Utf16Parser::new(BENCHMARK_INPUT).parse_chars();
    })
  }

  #[bench]
  pub fn parse_bytes_bench(b: &mut Bencher) {
    b.iter(|| {
      Utf16Parser::new(BENCHMARK_INPUT).parse_bytes();
    })
  }

  #[bench]
  pub fn parse_v128_portable_bench(b: &mut Bencher) {
    b.iter(|| {
      Utf16Parser::new(BENCHMARK_INPUT).parse_v128_portable();
    })
  }
}
