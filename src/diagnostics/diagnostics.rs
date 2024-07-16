//! Contains type checking errors, warnings and related structures

use crate::utils::location::Location;
use highlight_error::highlight_error;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
  pub location: Location,
  pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Warning {
  pub location: Location,
  pub message: String,
}

pub struct Diagnostic {
  pub errors: Vec<Error>,
  pub warnings: Vec<Warning>,
}

impl Diagnostic {
  pub fn new() -> Diagnostic {
    Diagnostic { errors: vec![], warnings: vec![] }
  }

  pub fn add_error(&mut self, message: &str, location: Location) {
    self.errors.push(Error { message: message.to_string(), location });
  }

  pub fn add_warning(&mut self, message: &str, location: Location) {
    self.warnings.push(Warning { message: message.to_string(), location });
  }

  pub fn display(&self, source: &str) {
    for error in &self.errors {
      self.display_message("Error", &error.message, &error.location, source);
    }
    for warning in &self.warnings {
      self.display_message("Warning", &warning.message, &warning.location, source);
    }
  }

  fn display_message(&self, kind: &str, message: &str, location: &Location, source: &str) {
    println!("{}: {} at:", kind, message);
    let report = format!("{}", highlight_error(location.start.line, location.end.line, source));
    println!("{}", report);
    println!("");

    // let lines: Vec<&str> = source.split('\n').collect();
    // let start = &location.start;
    // let end = &location.end;

    // if start.line <= lines.len() && end.line <= lines.len() {
    //   println!(
    //     "{}: {} at column: {} and line: {}",
    //     kind, message, start.column, start.line
    //   );

    //   if start.line == end.line {
    //     let line = lines[start.line - 1];
    //     let report = format!("{}", highlight_error(start.line, end.line, line));
    //     println!("{}", report);
    //     let mut indicator = String::new();
    //     for _ in 0..start.column - 1 {
    //       indicator.push(' ');
    //     }
    //     for _ in start.column..=end.column - 1 {
    //       indicator.push('^');
    //     }
    //     println!("{}", indicator);
    //   } else {
    //     // If the error spans multiple lines
    //     for (i, line) in lines.iter().enumerate().take(end.line).skip(start.line - 1) {
    //       println!("{}", line);
    //       let mut indicator = String::new();
    //       if i == start.line - 1 {
    //         for _ in 0..start.column - 1 {
    //           indicator.push(' ');
    //         }
    //         for _ in start.column..line.len() {
    //           indicator.push('^');
    //         }
    //       } else if i == end.line - 1 {
    //         for _ in 0..end.column {
    //           indicator.push('^');
    //         }
    //       } else {
    //         for _ in line.chars() {
    //           indicator.push('^');
    //         }
    //       }
    //       let report = format!("{}", highlight_error(start.line, end.line, &indicator));
    //       println!("{}", report);
    //     }
    //   }
    // }
  }
}
