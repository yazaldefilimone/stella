use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
  pub start: Position,
  pub end: Position,
}

impl Location {
  pub fn cursor_range(&self, raw: &str) -> Option<Range> {
    let mut cursor = 0;
    let mut line = 1;
    let mut start_cursor = None;
    let mut end_cursor = None;

    for (i, c) in raw.chars().enumerate() {
      if line == self.start.line && i - cursor == self.start.column - 1 {
        start_cursor = Some(cursor + (self.start.column - 1));
      }
      if line == self.end.line && i - cursor == self.end.column - 1 {
        end_cursor = Some(cursor + (self.end.column - 1));
        break;
      }

      if c == '\n' {
        line += 1;
        cursor = i + 1;
      }
    }

    match (start_cursor, end_cursor) {
      (Some(start), Some(end)) => Some(Range { start, end }),
      _ => None,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
  pub start: usize,
  pub end: usize,
}
