#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Position {
  pub line: usize,
  pub character: usize,
  pub offset: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Range {
  pub start: Position,
  pub end: Position,
}
