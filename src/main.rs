use std::process::exit;

use rayon::prelude::*;

use parsing_sandbox::parsers::*;

#[derive(Copy, Clone, Debug)]
pub enum ParserName {
  Chars,
  Bytes,
  Vector128,
  Vector256,
  Vector128Portable,
}

#[derive(Copy, Clone, Debug)]
pub enum ModeName {
  Seq,
  Par,
}

fn parse(input: &str, parser_name: ParserName) -> usize {
  let mut parser = Parser::new(input);
  match parser_name {
    ParserName::Chars => parser.parse_chars(),
    ParserName::Bytes => parser.parse_bytes(),
    ParserName::Vector128 => parser.parse_v128(),
    ParserName::Vector256 => parser.parse_v256(),
    ParserName::Vector128Portable => parser.parse_v128_portable(),
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

  let sum = match mode_name {
    ModeName::Seq => {
      let mut sum = 0;
      for i in 0..100 {
        let input = std::fs::read_to_string(format!("input/input-{}.txt", i)).unwrap();
        let input = input.as_str();

        sum += parse(input, parser_name);
      }
      sum
    }
    ModeName::Par => (0..100)
      .into_par_iter()
      .map(|i| {
        let input = std::fs::read_to_string(format!("input/input-{}.txt", i)).unwrap();
        let input = input.as_str();

        parse(input, parser_name)
      })
      .sum(),
  };

  println!("Parsed {} ranges.", sum);
}
