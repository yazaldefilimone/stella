use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Location {
  pub start: Position,
  pub end: Position,
}

impl Location {
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
        end_cursor = Some(cursor + character.len_utf8());
        break;
      }
      if character == '\n' {
        line += 1;
        column = 0;
      } else {
        column += 1;
      }
      cursor += character.len_utf8();
    }

    if let (Some(start), Some(end)) = (start_cursor, end_cursor) {
      return Some(Range { start, end: end - 1 });
    }
    return None;
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
  pub start: usize,
  pub end: usize,
}
