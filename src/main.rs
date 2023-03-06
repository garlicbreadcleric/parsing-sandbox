use std::process::exit;

use parsing_sandbox::parsers::Parser;

pub fn main() {
  let args: Vec<_> = std::env::args().collect();

  let input = std::fs::read_to_string("input.txt").unwrap();
  let input = input.as_str();

  let mut parser = Parser::new(input);

  let ranges = match args.get(1).map(|s| s.as_str()) {
    Some("chars") => parser.parse_chars(),
    Some("bytes") => parser.parse_bytes(),
    Some("vector128") => parser.parse_v128(),
    Some("vector256") => parser.parse_v256(),
    Some("vector128portable") => parser.parse_v128_portable(),
    _ => {
      eprintln!("Expected parsing mode (one of: 'chars', 'bytes', 'vector128', 'vector256', 'vector128portable').");
      exit(1)
    }
  };

  println!("Parsed {} ranges.", ranges.len());
}
