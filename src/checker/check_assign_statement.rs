use super::Checker;
use crate::{ast::ast::AssignStatement, diagnostics::Diagnostic, types::Type};

impl<'a> Checker<'a> {
  pub fn check_assign_statement(&mut self, assign: &AssignStatement) -> Result<Type, Diagnostic> {
    let right_type = self.check_expression(&assign.value)?;
    let value_location = assign.get_location();
    self._declaration(&assign.values, right_type, false, value_location)?;
    Ok(Type::Nil)
  }
}
