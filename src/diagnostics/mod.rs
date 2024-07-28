#![allow(dead_code)]

use crate::ast::ast::BinaryOperator;
use crate::utils::location::Location;
use crate::utils::{
  get_full_path, highlight_text_with_cyan, highlight_text_with_gray, highlight_text_with_red,
  highlight_text_with_white, highlight_text_with_yellow,
};
use std::fmt::Debug;

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

#[derive(Clone, PartialEq, Eq)]
pub struct Diagnostic {
  pub level: DiagnosticLevel,
  pub message: String,
  pub location: Option<Location>,
}

impl Debug for Diagnostic {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    <Option<Location> as Debug>::fmt(&self.location, f)
  }
}

impl Diagnostic {
  pub fn emit(&self, raw: &str, file_name: &str) {
    match self.level {
      DiagnosticLevel::Info | DiagnosticLevel::Warning | DiagnosticLevel::Error => {
        let mut location = self.location.clone().unwrap_or(Location::new());
        report_error(&self.message, &mut location, raw, file_name)
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct DiagnosticManager {
  pub error_count: usize,
  pub warning_count: usize,
  pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticManager {
  pub fn new() -> Self {
    DiagnosticManager { error_count: 0, warning_count: 0, diagnostics: vec![] }
  }

  pub fn add(&mut self, diagnostic: Diagnostic) {
    if diagnostic.level == DiagnosticLevel::Error {
      self.error_count += 1;
    }
    if diagnostic.level == DiagnosticLevel::Warning {
      self.warning_count += 1;
    }
    self.diagnostics.push(diagnostic);
  }

  pub fn emit_all(&self, raw: &str, file_name: &str) {
    for diagnostic in &self.diagnostics {
      diagnostic.emit(raw, file_name);
    }

    let message = format!(
      "Done. with {} errors and {} warnings",
      self.error_count, self.warning_count
    );

    println!("{}", highlight_text_with_gray(message.as_str()));
    if self.error_count > 0 {
      std::process::exit(1);
    }
  }
}

#[derive(Debug, Clone)]
pub enum TypeWarning {
  UnusedVariable(String, Option<Location>),
}

impl From<TypeWarning> for Diagnostic {
  fn from(warning: TypeWarning) -> Self {
    let level = DiagnosticLevel::Warning;
    let red_type_warning = highlight_text_with_yellow("WARNING");
    let (message, location) = match warning {
      TypeWarning::UnusedVariable(name, location) => {
        (format!("{}: unused variable '{}'", red_type_warning, name), location)
      }
    };
    Diagnostic { level, message, location }
  }
}

#[derive(Debug, Clone)]
pub enum TypeError {
  MismatchedTypes(String, String, Option<Location>),
  UndeclaredVariable(String, Option<Location>),
  ModuleNotFound(String, Option<Location>),
  ModuleNotExported(String, Option<Location>),
  TypeMismatchAssignment(String, String, Option<Location>),
  RedeclaredInSameScope(String, Option<Location>),
  InvalidAssignment(String, Option<Location>),
  FunctionArityMismatch(usize, usize, Option<Location>),
  UnsupportedOperator(String, String, BinaryOperator, Option<Location>),
}

impl From<TypeError> for Diagnostic {
  fn from(error: TypeError) -> Self {
    let level = DiagnosticLevel::Error;
    let red_type_error = highlight_text_with_red("ERROR");
    let (message, location) = match error {
      TypeError::TypeMismatchAssignment(expected, found, location) => (
        format!(
          "{}: type '{}' is not assignable to type '{}'",
          red_type_error, expected, found
        ),
        location,
      ),
      TypeError::MismatchedTypes(expected, found, location) => (
        format!(
          "{}: expected type '{}', but found type '{}'",
          red_type_error, expected, found
        ),
        location,
      ),
      TypeError::UndeclaredVariable(name, location) => (
        format!("{}: connot find a value '{}' in this scope", red_type_error, name),
        location,
      ),
      TypeError::InvalidAssignment(name, location) => (
        format!("{}: invalid assignment to '{}'", red_type_error, name),
        location,
      ),
      TypeError::FunctionArityMismatch(expected, found, location) => (
        format!(
          "{}: call expected {} arguments, but found {}",
          red_type_error, expected, found
        ),
        location,
      ),
      TypeError::UnsupportedOperator(left, right, operator, location) => (
        format!(
          "{}: unsupported operator '{}' for types '{}' and '{}'",
          red_type_error,
          operator.to_string(),
          left,
          right
        ),
        location,
      ),
      TypeError::RedeclaredInSameScope(name, location) => (
        format!("{}: '{}' is already declared in this scope.", red_type_error, name),
        location,
      ),
      TypeError::ModuleNotFound(name, location) => (
        format!(
          "{}: unresolved module, can't find module file: '{}.lua'",
          red_type_error, name
        ),
        location,
      ),
      TypeError::ModuleNotExported(name, location) => (
        format!("{}: module '{}' doesn't export anything", red_type_error, name),
        location,
      ),
    };

    Diagnostic { level, message, location }
  }
}

pub fn report_error(message: &str, location: &mut Location, raw: &str, file_name: &str) {
  let range = location.cursor_range(raw);

  if range.is_none() {
    println!("{}", highlight_text_with_red(message));
    return;
  }
  let range = range.unwrap();
  let line_highlight = highlight_text_with_yellow(format!("{}:{}", location.start.line, location.end.column).as_str());

  println!("");
  // todo: improve this
  if message.matches("WARNING").count() > 0 {
    println!("{}", code_highlighter::highlight_warning(range.start, range.end, raw));
  } else {
    println!("{}", code_highlighter::highlight_error(range.start, range.end, raw));
  }
  println!("");
  println!("{}", highlight_text_with_white(message));
  println!("");
  let absolute_path = get_full_path(file_name);
  let abs_path_highlight = highlight_text_with_cyan(absolute_path.as_str());
  println!(
    "{} {}:{}",
    highlight_text_with_gray("At"),
    abs_path_highlight,
    line_highlight
  );
}

pub fn report_and_exit(message: &str, location: &mut Location, raw: &str, file_name: &str) -> ! {
  report_error(message, location, raw, file_name);
  std::process::exit(1);
}
