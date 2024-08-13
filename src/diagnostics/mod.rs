#![allow(dead_code)]

mod format;
mod format_awesome;
pub mod report;

use crate::utils::highlight_text_with_gray;
use crate::utils::range::Range;
use report::report_error;
use std::fmt::{self, Debug};

use format::{
  format_cannot_index_non_array, format_expected_function, format_expected_table, format_function_arity_mismatch,
  format_generic_call_arity_mismatch, format_key_not_found_in_table, format_mismatched_accessor_type,
  format_mismatched_key_type, format_mismatched_types, format_missing_variable_declaration, format_module_not_exported,
  format_module_not_found, format_no_field, format_optional_call_arity_mismatch, format_redeclared_in_same_scope,
  format_type_mismatch_assignment, format_undeclared_type, format_undeclared_variable, format_unsupported_operator,
  format_warning_redundant_type, format_warning_shadow_warning, format_warning_unused_variable,
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
  pub range: Option<Range>,
}

impl Debug for Diagnostic {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}: {}", self.level, self.message)
  }
}

impl Diagnostic {
  pub fn new(level: DiagnosticLevel, message: String, range: Option<Range>) -> Self {
    Diagnostic { level, message, range }
  }

  pub fn emit(&self, raw: &str, file_name: &str) {
    if let Some(mut range) = self.range.clone() {
      let warning = self.level == DiagnosticLevel::Warning;
      report_error(&self.message, &mut range, raw, file_name, warning);
    } else {
      // ignore if range is not provided
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
  UnusedVariable(String, Option<Range>),
  ShadowedVariable(String, Option<Range>),
  RedundantType(String, String, Option<Range>),
}

impl From<TypeWarning> for Diagnostic {
  fn from(warning: TypeWarning) -> Self {
    let (message, range) = match warning {
      TypeWarning::UnusedVariable(name, loc) => (format_warning_unused_variable(&name), loc),
      TypeWarning::ShadowedVariable(name, loc) => (format_warning_shadow_warning(&name), loc),
      TypeWarning::RedundantType(name, type_name, loc) => (format_warning_redundant_type(&name, &type_name), loc),
    };
    Diagnostic::new(DiagnosticLevel::Warning, message, range)
  }
}

#[derive(Debug, Clone)]
pub enum TypeError {
  MismatchedTypes(String, String, Option<Range>),
  UndeclaredVariable(String, Option<Range>),
  ExpectedFunction(String, Option<Range>),
  UndeclaredType(String, Option<Range>),
  ModuleNotFound(String, Option<Range>),
  ModuleNotExported(String, Option<Range>),
  TypeMismatchAssignment(String, String, Option<Range>),
  RedeclaredInSameScope(String, Option<Range>),
  FunctionArityMismatch(usize, usize, Option<Range>),
  UnsupportedOperator(String, String, String, Option<Range>),
  MissingVariableDeclaration(Option<Range>),
  ExpectedTable(String, Option<Range>),
  MismatchedKeyType(String, Option<Range>),
  NoField(String, String, Option<Range>),
  CantIndexNonArray(String, Option<Range>),
  KeyNotFoundInTable(String, String, Option<Range>),
  MismatchedAccessorType(String, Option<Range>),
  GenericCallArityMismatch(usize, usize, Option<Range>),
  OptionalCallArityMismatch(usize, Option<Range>),
}
impl From<TypeError> for Diagnostic {
  fn from(error: TypeError) -> Self {
    let (message, range) = match error {
      TypeError::MismatchedTypes(expected, found, rg) => (format_mismatched_types(&expected, &found), rg),
      TypeError::UndeclaredVariable(name, rg) => (format_undeclared_variable(&name), rg),
      TypeError::FunctionArityMismatch(expected, found, rg) => (format_function_arity_mismatch(expected, found), rg),
      TypeError::UnsupportedOperator(left, right, op, rg) => (format_unsupported_operator(&left, &right, &op), rg),
      TypeError::RedeclaredInSameScope(name, rg) => (format_redeclared_in_same_scope(&name), rg),
      TypeError::ModuleNotFound(name, rg) => (format_module_not_found(&name), rg),
      TypeError::ModuleNotExported(name, rg) => (format_module_not_exported(&name), rg),
      TypeError::MissingVariableDeclaration(rg) => (format_missing_variable_declaration(), rg),
      TypeError::TypeMismatchAssignment(expected, found, rg) => {
        (format_type_mismatch_assignment(&expected, &found), rg)
      }
      TypeError::UndeclaredType(name, rg) => (format_undeclared_type(&name), rg),
      TypeError::ExpectedFunction(name, rg) => (format_expected_function(&name), rg),
      TypeError::NoField(base, member, rg) => (format_no_field(&base, &member), rg),
      TypeError::MismatchedKeyType(key, rg) => (format_mismatched_key_type(&key), rg),
      TypeError::ExpectedTable(message, rg) => (format_expected_table(&message), rg),
      TypeError::CantIndexNonArray(type_name, rg) => (format_cannot_index_non_array(&type_name), rg),
      TypeError::KeyNotFoundInTable(key, table, rg) => (format_key_not_found_in_table(&key, &table), rg),
      TypeError::MismatchedAccessorType(index, rg) => (format_mismatched_accessor_type(&index), rg),
      TypeError::GenericCallArityMismatch(expected, found, rg) => {
        (format_generic_call_arity_mismatch(expected, found), rg)
      }
      TypeError::OptionalCallArityMismatch(found, rg) => (format_optional_call_arity_mismatch(found), rg),
    };

    Diagnostic::new(DiagnosticLevel::Error, message, range)
  }
}
