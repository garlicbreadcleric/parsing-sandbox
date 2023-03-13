use std::io::{BufRead, BufReader, Read};
use std::simd::{u8x16, SimdPartialEq};

use crate::types::{Position, Range};
use crate::utf8::get_character_width;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LspPosition {
  line: usize,
  character: usize,
}

impl LspPosition {
  pub const fn eq_position(&self, pos: Position) -> bool {
    self.line == pos.line && self.character == pos.character
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LspRange {
  start: LspPosition,
  end: LspPosition,
}

impl LspRange {
  pub const fn eq_range(&self, range: Range) -> bool {
    self.start.eq_position(range.start) && self.end.eq_position(range.end)
  }
}

pub struct Utf16BufParser {
  line: usize,
  character: usize,
  character_offset: usize,
  offset: usize,
  range_start: Option<LspPosition>,
  ranges: Vec<LspRange>,
}

impl Utf16BufParser {
  #[must_use]
  pub const fn new() -> Self {
    Self { line: 0, character: 0, character_offset: 0, offset: 0, range_start: None, ranges: vec![] }
  }

  fn parse_line(&mut self, line: &str) {
    let bytes = line.as_bytes();

    while self.offset + 15 < bytes.len() {
      let bytes_vec = u8x16::from_slice(&bytes[self.offset..]);

      let lookup = if self.range_start.is_some() {
        // Lookup: ']'
        bytes_vec.simd_eq(u8x16::splat(b']'))
      } else {
        // Lookup: '['
        bytes_vec.simd_eq(u8x16::splat(b'['))
      };

      if lookup.any() {
        self.parse_bytes(bytes, 16);
      } else {
        self.offset += 16;
      }
    }

    self.parse_bytes(bytes, line.len());

    self.line += 1;
    self.character = 0;
    self.character_offset = 0;
    self.offset = 0;
  }

  fn parse_bytes(&mut self, bytes: &[u8], count: usize) {
    let max_offset = (self.offset + count).min(bytes.len());

    while self.offset < max_offset {
      let byte = bytes[self.offset];

      let character_width = get_character_width(byte);
      self.offset += character_width;

      if character_width == 1 {
        match (byte, self.range_start) {
          (b'[', None) => {
            self.character += unsafe { simdutf::count_utf16_from_utf8(&bytes[self.character_offset..self.offset - 1]) };
            self.character_offset = self.offset - 1;
            self.range_start = Some(LspPosition { line: self.line, character: self.character });
          }
          (b']', Some(start)) => {
            self.character += unsafe { simdutf::count_utf16_from_utf8(&bytes[self.character_offset..self.offset]) };
            self.character_offset = self.offset;
            self.ranges.push(LspRange { start, end: LspPosition { line: self.line, character: self.character } });
            self.range_start = None;
          }
          _ => {}
        }
      }
    }
  }

  pub fn parse_from_reader<R: Read>(&mut self, reader: &mut BufReader<R>) -> &[LspRange] {
    let mut buf = String::new();
    while reader.has_data_left().unwrap_or(false) {
      if reader.read_line(&mut buf).is_err() {
        break;
      }
      self.parse_line(&buf);
      buf.clear();
    }

    &self.ranges
  }
}

#[cfg(test)]
mod tests {
  use std::fs::File;
  use std::io::BufReader;

  use super::*;
  use crate::utf16_parser::Utf16Parser;

  #[test]
  pub fn short_multiline_test() {
    let ranges1 =
      Utf16Parser::new(std::fs::read_to_string("input/input-0.txt").unwrap().as_str()).parse_chars().to_vec();

    let ranges2 = {
      let file = File::open("input/input-0.txt").unwrap();
      let mut reader = BufReader::new(file);
      let mut parser = Utf16BufParser::new();
      parser.parse_from_reader(&mut reader).to_vec()
    };

    assert_eq!(ranges1.len(), ranges2.len());

    for i in 0..ranges1.len() {
      assert!(ranges2[i].eq_range(ranges1[i]));
    }
  }
}
