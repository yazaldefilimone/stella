#![allow(dead_code)]
use std::collections::HashMap;

use crate::utils::location::Location;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticLevel {
  Info,
  Warning,
  Error,
}

#[derive(Debug, Clone)]
pub enum TypeErrorKind {
  MismatchedTypes,
  UndeclaredVariable,
  InvalidAssignment,
  FunctionArityMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiagnosticSource {
  TypeChecker,
  Lexer,
  Parser,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
  pub message: String,
  pub level: DiagnosticLevel,
}

impl Diagnostic {
  pub fn emit(&self) {
    match self.level {
      DiagnosticLevel::Info => println!("Info: {}", self.message),
      DiagnosticLevel::Warning => println!("Warning: {}", self.message),
      DiagnosticLevel::Error => eprintln!("Error: {}", self.message),
    }
  }
}

#[derive(Debug, Clone)]
pub struct DiagnosticManager {
  pub error_count: usize,
  pub diagnostics: HashMap<DiagnosticSource, Vec<Diagnostic>>,
}

impl DiagnosticManager {
  pub fn new() -> Self {
    DiagnosticManager { error_count: 0, diagnostics: HashMap::new() }
  }

  pub fn add(&mut self, source: DiagnosticSource, diagnostic: Diagnostic) {
    if diagnostic.level == DiagnosticLevel::Error {
      self.error_count += 1;
    }
    self.diagnostics.entry(source).or_default().push(diagnostic);
  }

  pub fn emit_all(&self) {
    for diagnostics in self.diagnostics.values() {
      for diagnostic in diagnostics {
        diagnostic.emit();
      }
    }
  }
}

pub enum TypeError {
  MismatchedTypes(String, String),
  UndeclaredVariable(String),
  InvalidAssignment(String),
  FunctionArityMismatch(String, usize, usize),
}

impl From<TypeError> for Diagnostic {
  fn from(error: TypeError) -> Self {
    let level = DiagnosticLevel::Error;
    let message = match error {
      TypeError::MismatchedTypes(expected, found) => {
        format!("Type error: expected type '{}', but found type '{}'", expected, found)
      }
      TypeError::UndeclaredVariable(name) => format!("Type error: undeclared variable '{}'", name),
      TypeError::InvalidAssignment(name) => format!("Type error: invalid assignment to '{}'", name),
      TypeError::FunctionArityMismatch(name, expected, found) => {
        format!(
          "Type error: function '{}' expected {} arguments, but found {}",
          name, expected, found
        )
      }
    };
    Diagnostic { message, level }
  }
}

pub fn report_error(message: &str, location: &mut Location, raw: &str) -> ! {
  println!("Error: {} at ", message);
  let range = location.cursor_range(raw).expect("Failed to get range");
  println!("{}", highlight_error::highlight_error(range.start, range.end, raw));
  std::process::exit(1);
}
