use super::Checker;
use crate::{ast::ast, diagnostics::Diagnostic, types::Type};

impl Checker {
  pub fn check_grouped_expression(&mut self, return_stmt: &ast::ReturnStatement) -> Result<Type, Diagnostic> {
    todo!("Implement check_grouped_expression");
  }
}
