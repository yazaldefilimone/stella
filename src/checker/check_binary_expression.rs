use super::{type_utils::CheckResult, Checker};
use crate::{ast::ast, diagnostics::TypeError, types::Type};

impl<'a> Checker<'a> {
  pub fn check_binary_expression(&mut self, binary_expr: &ast::BinaryExpression) -> CheckResult<Option<Type>> {
    let left_type = self.check_expression(&binary_expr.left)?.unwrap();
    let right_type = self.check_expression(&binary_expr.right)?.unwrap();
    if left_type.supports_operator(&binary_expr.operator) && right_type.supports_operator(&binary_expr.operator) {
      let result_type = left_type.get_operator_result_type(&right_type, &binary_expr.operator);
      return Ok(Some(result_type));
    }

    let range = binary_expr.get_range();
    let diagnostic = TypeError::UnsupportedOperator(
      left_type.to_string(),
      binary_expr.operator.to_owned(),
      right_type.to_string(),
      Some(range),
    );

    Err(self.create_diagnostic(diagnostic))
  }
}
