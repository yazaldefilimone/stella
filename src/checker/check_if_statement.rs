use super::Checker;
use crate::{ast::ast, diagnostics::Diagnostic, types::Type};

impl<'a> Checker<'a> {
  pub fn check_if_statement(&mut self, if_stmt: &ast::IfStatement) -> Result<Type, Diagnostic> {
    let condition_type = self.check_expression(&if_stmt.condition)?;
    if !condition_type.check_match(&Type::Boolean) {
      let condition_location = if_stmt.condition.get_location();
      return Err(self.create_type_mismatch(Type::new_boolean(), condition_type, condition_location));
    }

    let then_type = self.check_statement(&if_stmt.then_body)?;
    if let Some(else_body) = &if_stmt.else_body {
      let else_type = self.check_statement(else_body)?;
      let union_type = Type::new_union(vec![then_type, else_type]);
      return Ok(union_type);
    }

    Ok(then_type)
  }
}
