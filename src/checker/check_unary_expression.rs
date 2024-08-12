use crate::ast::ast;
use crate::diagnostics::{Diagnostic, TypeError};
use crate::types::Type;

use super::Checker;

impl<'a> Checker<'a> {
  pub fn check_unary_expression(&mut self, unary_expr: &ast::UnaryExpression) -> Result<Type, Diagnostic> {
    let operand_type = self.check_expression(&unary_expr.operand)?;
    self.check_unary_operator(&operand_type, unary_expr)
  }

  fn check_unary_operator(&self, operand_t: &Type, unary_expr: &ast::UnaryExpression) -> Result<Type, Diagnostic> {
    match unary_expr.operator {
      ast::UnaryOperator::Negate => self.validate_unary_operator(operand_t, &Type::Number, unary_expr),
      ast::UnaryOperator::Not => self.validate_unary_operator(operand_t, &Type::Boolean, unary_expr),
      ast::UnaryOperator::Hash => self.validate_unary_operator(operand_t, &Type::String, unary_expr),
    }
  }

  fn validate_unary_operator(
    &self,
    operand_t: &Type,
    expected_t: &Type,
    unary_expr: &ast::UnaryExpression,
  ) -> Result<Type, Diagnostic> {
    if operand_t.check_match(expected_t) {
      return Ok(expected_t.clone());
    }

    let diagnostic = TypeError::UnsupportedOperator(
      operand_t.to_string(),
      operand_t.to_string(),
      unary_expr.operator.to_str().to_owned(),
      Some(unary_expr.get_operator_range()),
    );

    Err(self.create_diagnostic(diagnostic))
  }
}
