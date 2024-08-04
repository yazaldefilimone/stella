#![allow(dead_code)]

mod format;
mod format_awesome;
pub mod report;

use crate::utils::highlight_text_with_gray;
use crate::utils::location::Location;
use report::report_error;
use std::fmt::{self, Debug};

use format::{
  format_function_arity_mismatch, format_mismatched_types, format_missing_variable_declaration,
  format_module_not_exported, format_module_not_found, format_redeclared_in_same_scope,
  format_type_mismatch_assignment, format_undeclared_type, format_undeclared_variable, format_unsupported_operator,
  format_warning_shadow_warning, format_warning_unused_variable,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticLevel {
  Info,
  Warning,
  Error,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Diagnostic {
  pub level: DiagnosticLevel,
  pub message: String,
  pub location: Option<Location>,
}

impl Debug for Diagnostic {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}: {}", self.level, self.message)
  }
}

impl Diagnostic {
  pub fn new(level: DiagnosticLevel, message: String, location: Option<Location>) -> Self {
    Diagnostic { level, message, location }
  }

  pub fn emit(&self, raw: &str, file_name: &str) {
    if let Some(mut location) = self.location.clone() {
      let warning = self.level == DiagnosticLevel::Warning;
      report_error(&self.message, &mut location, raw, file_name, warning);
    } else {
      // ignore if location is not provided
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
    match diagnostic.level {
      DiagnosticLevel::Error => self.error_count += 1,
      DiagnosticLevel::Warning => self.warning_count += 1,
      _ => {}
    }
    self.diagnostics.push(diagnostic);
  }

  pub fn emit_all(&self, raw: &str, file_name: &str) {
    for diagnostic in &self.diagnostics {
      diagnostic.emit(raw, file_name);
    }
    let message = format!("done. {} errors, {} warnings", self.error_count, self.warning_count);

    println!("{}", highlight_text_with_gray(&message));

    if self.error_count > 0 {
      std::process::exit(1)
    }
  }

  pub fn emit_warnings(&self, raw: &str, file_name: &str) {
    for diagnostic in &self.diagnostics {
      match diagnostic.level {
        DiagnosticLevel::Warning => {
          diagnostic.emit(raw, file_name);
        }
        _ => {}
      }
    }
  }
}

#[derive(Debug, Clone)]
pub enum TypeWarning {
  UnusedVariable(String, Option<Location>),
  ShadowedVariable(String, Option<Location>),
}

impl From<TypeWarning> for Diagnostic {
  fn from(warning: TypeWarning) -> Self {
    let (message, location) = match warning {
      TypeWarning::UnusedVariable(name, loc) => (format_warning_unused_variable(&name), loc),
      TypeWarning::ShadowedVariable(name, loc) => (format_warning_shadow_warning(&name), loc),
    };
    Diagnostic::new(DiagnosticLevel::Warning, message, location)
  }
}

#[derive(Debug, Clone)]
pub enum TypeError {
  MismatchedTypes(String, String, Option<Location>),
  UndeclaredVariable(String, Option<Location>),
  UndeclaredType(String, Option<Location>),
  ModuleNotFound(String, Option<Location>),
  ModuleNotExported(String, Option<Location>),
  TypeMismatchAssignment(String, String, Option<Location>),
  RedeclaredInSameScope(String, Option<Location>),
  FunctionArityMismatch(usize, usize, Option<Location>),
  UnsupportedOperator(String, String, String, Option<Location>),
  MissingVariableDeclaration(Option<Location>),
}

impl From<TypeError> for Diagnostic {
  fn from(error: TypeError) -> Self {
    let (message, location) = match error {
      TypeError::MismatchedTypes(expected, found, loc) => (format_mismatched_types(&expected, &found), loc),
      TypeError::UndeclaredVariable(name, loc) => (format_undeclared_variable(&name), loc),
      TypeError::FunctionArityMismatch(expected, found, loc) => (format_function_arity_mismatch(expected, found), loc),
      TypeError::UnsupportedOperator(left, right, op, loc) => (format_unsupported_operator(&left, &right, &op), loc),
      TypeError::RedeclaredInSameScope(name, loc) => (format_redeclared_in_same_scope(&name), loc),
      TypeError::ModuleNotFound(name, loc) => (format_module_not_found(&name), loc),
      TypeError::ModuleNotExported(name, loc) => (format_module_not_exported(&name), loc),
      TypeError::MissingVariableDeclaration(loc) => (format_missing_variable_declaration(), loc),
      TypeError::TypeMismatchAssignment(expected, found, loc) => {
        (format_type_mismatch_assignment(&expected, &found), loc)
      }
      TypeError::UndeclaredType(name, loc) => (format_undeclared_type(&name), loc),
    };

    Diagnostic::new(DiagnosticLevel::Error, message, location)
  }
}
