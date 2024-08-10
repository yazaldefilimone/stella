use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::Type,
  utils::range::Range,
};

impl<'a> Checker<'a> {
  pub fn check_binary_expression(&mut self, binary_expr: &ast::BinaryExpression) -> Result<Type, Diagnostic> {
    let left_t = self.check_expression(&binary_expr.left)?;
    let right_t = self.check_expression(&binary_expr.right)?;

    if left_t.supports_operator(&binary_expr.operator) && right_t.supports_operator(&binary_expr.operator) {
      return Ok(left_t.get_operator_result_type(&right_t, &binary_expr.operator));
    }
    let range = binary_expr.get_range();

    let diagnostic = TypeError::UnsupportedOperator(
      left_t.to_string(),
      right_t.to_string(),
      binary_expr.operator.to_str().to_owned(),
      Some(range),
    );

    Err(self.create_diagnostic(diagnostic))
  }

  pub fn check_add_expression(&mut self, left_t: Type, right_t: Type, range: Range) -> Result<Type, Diagnostic> {
    if left_t.supports_operator(&ast::BinaryOperator::Add) && right_t.supports_operator(&ast::BinaryOperator::Add) {
      return Ok(Type::Number);
    }

    let dignostic = TypeError::UnsupportedOperator(
      left_t.to_string(),
      right_t.to_string(),
      ast::BinaryOperator::Add.to_str().to_owned(),
      Some(range),
    );

    Err(self.create_diagnostic(dignostic))
  }
}
