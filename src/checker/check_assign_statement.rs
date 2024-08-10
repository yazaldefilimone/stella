use super::Checker;
use crate::{ast::ast::AssignStatement, diagnostics::Diagnostic, types::Type};

impl<'a> Checker<'a> {
  pub fn check_assign_statement(&mut self, assign: &AssignStatement) -> Result<Type, Diagnostic> {
    let right_type = self.check_expression(&assign.value)?;
    let value_range = assign.get_range();
    self.declare_variables(&assign.values, right_type, false, value_range)?;
    Ok(Type::Nil)
  }
}
