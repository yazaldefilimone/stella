use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Location {
  pub start: Position,
  pub end: Position,
  pub rage_start: usize,
  pub rage_end: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Range {
  pub start: usize,
  pub end: usize,
}

impl Location {
  pub fn new() -> Self {
    Location {
      start: Position { line: 0, column: 0 },
      end: Position { line: 0, column: 0 },
      rage_end: 0,
      rage_start: 0,
    }
  }
  // remove this function
  pub fn cursor_range(&self, raw: &str) -> Option<Range> {
    let mut cursor = 0;
    let mut start_cursor = None;
    let mut end_cursor = None;
    let mut line = 1;
    let mut column = 0;

    for character in raw.chars() {
      if line == self.start.line && column == self.start.column {
        start_cursor = Some(cursor);
      }

      if line == self.end.line && column == self.end.column {
        end_cursor = Some(cursor);
        break;
      }
      if character == '\n' {
        line += 1;
        column = 0;
      } else {
        column += 1;
      }
      cursor += 1
    }
    if let (Some(start), Some(end)) = (start_cursor, end_cursor) {
      Some(Range { start, end })
    } else {
      None
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}

pub fn get_middle_location(left: &Location, right: &Location) -> Location {
  let start = Position { line: left.start.line, column: left.start.column };
  let end = Position { line: right.end.line, column: right.end.column };
  Location { start, end, rage_start: left.rage_start, rage_end: right.rage_end }
}
