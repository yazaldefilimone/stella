#![allow(dead_code, unused_variables)]

pub mod binding;
pub mod check_assign_statement;
pub mod check_binary_expression;
pub mod check_block_statement;
pub mod check_call_expression;
pub mod check_empty_statement;
pub mod check_expression;
pub mod check_for_statement;
pub mod check_function_expression;
pub mod check_function_statement;
pub mod check_generic;
pub mod check_grouped_expression;
pub mod check_identifier;
pub mod check_if_statement;
pub mod check_index_expression;
pub mod check_literal_expression;
pub mod check_member_expression;
pub mod check_repeat_statement;
pub mod check_require_expression;
pub mod check_return_statement;
pub mod check_shadowing;
pub mod check_statement;
pub mod check_stdlib;
pub mod check_table_expression;
pub mod check_type;
pub mod check_type_declaration;
pub mod check_unary_expression;
pub mod check_unused_variables;
pub mod check_variable_declaration;
pub mod check_while_statement;
pub mod declare_variables;
pub mod type_utils;

use crate::ast::ast;
use crate::context::context::Context;
use crate::diagnostics::{Diagnostic, DiagnosticManager, TypeError};
use crate::modules::loader::Loader;
use crate::modules::resolver::Resolver;
use crate::types::Type;
use crate::utils::range::Range;

pub struct Checker<'a> {
  pub ctx: Context,
  pub file_name: String,
  pub diagnostics: DiagnosticManager,
  pub loader: Loader,
  pub resolver: Resolver,
  pub expect: Option<Type>,
  pub raw: &'a str,
}

impl<'a> Checker<'a> {
  pub fn new(file_name: &str, raw: &'a str) -> Checker<'a> {
    let ctx = Context::new();
    let loader = Loader::new();
    let mut resolver = Resolver::new();
    resolver.add_search_path(file_name);
    let diagnostics = DiagnosticManager::new();
    Checker { ctx, file_name: file_name.to_string(), diagnostics, loader, resolver, raw, expect: None }
  }

  pub fn check(&mut self, program: &ast::Program) -> Result<Type, Diagnostic> {
    let mut last_t = Type::Nil;
    for statement in &program.statements {
      match self.check_statement(statement) {
        Ok(ty) => last_t = ty,
        Err(diag) => self.diagnostics.add(diag),
      }
    }

    self.show_diagnostics();
    return Ok(last_t);
  }

  // pub fn check_break_statement(&mut self, break_: &ast::BreakStatement) {
  //   // Empty statements don't change the type context
  // }

  pub fn create_diagnostic(&self, error: TypeError) -> Diagnostic {
    error.into()
  }

  pub fn show_diagnostics(&mut self) {
    self.check_unused_variables();
    self.diagnostics.emit_all(self.raw, &self.file_name);
  }

  pub fn create_type_mismatch(&self, expected: Type, found: Type, range: Range) -> Diagnostic {
    let diagnostic = TypeError::MismatchedTypes(expected.to_string(), found.to_string(), Some(range));
    self.create_diagnostic(diagnostic)
  }

  pub fn create_redeclaration(&self, lexeme: &str, range: Range) -> Diagnostic {
    let diagnostic = TypeError::RedeclaredInSameScope(lexeme.to_string(), Some(range));
    self.create_diagnostic(diagnostic)
  }

  pub fn create_function_arity_mismatch(&self, expected: usize, found: usize, range: Range) -> Diagnostic {
    let diagnostic = TypeError::FunctionArityMismatch(expected, found, Some(range));
    self.create_diagnostic(diagnostic)
  }
}
