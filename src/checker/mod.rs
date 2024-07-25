#![allow(dead_code, unused_variables)]

pub mod check_assign_statement;
pub mod check_binary_expression;
pub mod check_block_statement;
pub mod check_call_expression;
pub mod check_empty_statement;
pub mod check_expression;
pub mod check_for_statement;
pub mod check_function_statement;
pub mod check_grouped_expression;
pub mod check_identifier;
pub mod check_if_statement;
pub mod check_literal_expression;
pub mod check_repeat_statement;
pub mod check_return_statement;
pub mod check_statement;
pub mod check_type;
pub mod check_type_statement;
pub mod check_variable_declaration;
pub mod check_while_statement;

use crate::ast::ast;
use crate::context::context::Context;
use crate::diagnostics::{Diagnostic, DiagnosticManager, TypeError, TypeWarning};
use crate::types::Type;

pub struct Checker {
  pub ctx: Context,
  pub diagnostics: DiagnosticManager,
}

impl Checker {
  pub fn new() -> Checker {
    Checker { ctx: Context::new(), diagnostics: DiagnosticManager::new() }
  }
  pub fn check(&mut self, program: &ast::Program) -> Result<Type, Diagnostic> {
    let mut last_t = Type::Nil;
    for statement in &program.statements {
      match self.check_statement(statement) {
        Ok(ty) => last_t = ty,
        Err(diag) => self.diagnostics.add(diag),
      }
    }

    for unused_variable in self.ctx.check_unused_variables() {
      let unused_variable_location = self.ctx.get_variable_location(&unused_variable);
      if unused_variable_location.is_none() {
        continue;
      }
      let report = TypeWarning::UnusedVariable(unused_variable.clone(), unused_variable_location);
      self.diagnostics.add(report.into());
    }

    return Ok(last_t);
  }

  pub fn check_break_statement(&mut self, break_: &ast::BreakStatement) {
    // Empty statements don't change the type context
  }

  pub fn create_diagnostic(&self, error: TypeError) -> Diagnostic {
    error.into()
  }
}
