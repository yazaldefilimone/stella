use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
  pub start: Position,
  pub end: Position,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}
