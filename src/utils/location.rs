#[derive(Debug, Clone, PartialEq)]
pub struct Location {
  pub start: Position,
  pub end: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}
