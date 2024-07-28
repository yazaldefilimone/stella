use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::Type,
  utils::location::Location,
};

impl Checker<'_> {
  pub fn check_binary_expression(&mut self, binary_expr: &ast::BinaryExpression) -> Result<Type, Diagnostic> {
    let left_t = self.check_expression(&binary_expr.left)?;
    let right_t = self.check_expression(&binary_expr.right)?;

    if left_t.suport_operator(&binary_expr.operator) {
      if right_t.suport_operator(&binary_expr.operator) {
        return Ok(left_t.get_operator_type(&left_t, &right_t, &binary_expr.operator));
      }
    }

    Err(self.create_diagnostic(TypeError::UnsupportedOperator(
      left_t.to_string(),
      right_t.to_string(),
      binary_expr.operator.clone(),
      Some(binary_expr.location.clone()),
    )))
  }

  pub fn check_add_expression(&mut self, left_t: Type, right_t: Type, loc: Location) -> Result<Type, Diagnostic> {
    if left_t.suport_operator(&ast::BinaryOperator::Add) && right_t.suport_operator(&ast::BinaryOperator::Add) {
      return Ok(Type::Number);
    }
    Err(self.create_diagnostic(TypeError::UnsupportedOperator(
      left_t.to_string(),
      right_t.to_string(),
      ast::BinaryOperator::Add,
      Some(loc),
    )))
  }
}
