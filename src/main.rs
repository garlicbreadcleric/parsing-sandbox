use std::process::exit;

use rayon::prelude::*;

use parsing_sandbox::utf16_parser::*;
use parsing_sandbox::utf32_parser::*;

#[derive(Copy, Clone)]
pub enum ParserName {
  Chars,
  Bytes,
  Vector128,
  Vector256,
  Vector128Portable,
}

#[derive(Copy, Clone)]
pub enum ModeName {
  Seq,
  Par,
}

#[derive(Copy, Clone)]
pub enum OutputName {
  Utf32,
  Utf16,
}

fn parse_utf32(input: &str, parser_name: ParserName) -> usize {
  let mut parser = Utf32Parser::new(input);
  match parser_name {
    ParserName::Chars => parser.parse_chars(),
    ParserName::Bytes => parser.parse_bytes(),
    ParserName::Vector128 => parser.parse_v128(),
    ParserName::Vector256 => parser.parse_v256(),
    ParserName::Vector128Portable => parser.parse_v128_portable(),
  }
  .len()
}

fn parse_utf16(input: &str, parser_name: ParserName) -> usize {
  let mut parser = Utf16Parser::new(input);
  match parser_name {
    ParserName::Chars => parser.parse_chars(),
    ParserName::Bytes => parser.parse_bytes(),
    ParserName::Vector128Portable => parser.parse_v128_portable(),
    _ => todo!(),
  }
  .len()
}

pub fn main() {
  let args: Vec<_> = std::env::args().collect();

  let parser_name = match args.get(1).map(|s| s.as_str()) {
    Some("chars") => ParserName::Chars,
    Some("bytes") => ParserName::Bytes,
    Some("vector128") => ParserName::Vector128,
    Some("vector256") => ParserName::Vector256,
    Some("vector128portable") => ParserName::Vector128Portable,
    _ => {
      eprintln!("Expected first argument to be parser name (one of: 'chars', 'bytes', 'vector128', 'vector256', 'vector128portable').");
      exit(1);
    }
  };

  let mode_name = match args.get(2).map(|s| s.as_str()) {
    Some("seq") => ModeName::Seq,
    Some("par") => ModeName::Par,
    _ => {
      eprintln!("Expected second argument to be mode name (one of: 'seq', 'par').");
      exit(1);
    }
  };

  let output_name = match args.get(3).map(|s| s.as_str()) {
    Some("utf32") => OutputName::Utf32,
    Some("utf16") => OutputName::Utf16,
    _ => {
      eprintln!("Expected third argument to be output name (one of: 'utf32', 'utf16').");
      exit(1);
    }
  };

  let sum = match mode_name {
    ModeName::Seq => {
      let mut sum = 0;
      for i in 0..100 {
        let input = std::fs::read(format!("input/input-{}.txt", i)).unwrap();
        let input = simdutf8::basic::from_utf8(&input).unwrap();

        match output_name {
          OutputName::Utf32 => sum += parse_utf32(input, parser_name),
          OutputName::Utf16 => sum += parse_utf16(input, parser_name),
        }
      }
      sum
    }
    ModeName::Par => (0..100)
      .into_par_iter()
      .map(|i| {
        let input = std::fs::read(format!("input/input-{}.txt", i)).unwrap();
        let input = simdutf8::compat::from_utf8(&input).unwrap();

        match output_name {
          OutputName::Utf32 => parse_utf32(input, parser_name),
          OutputName::Utf16 => parse_utf16(input, parser_name),
        }
      })
      .sum(),
  };

  println!("Parsed {} ranges.", sum);
}
