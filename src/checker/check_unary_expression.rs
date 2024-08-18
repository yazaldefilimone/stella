use crate::ast::ast;
use crate::diagnostics::TypeError;
use crate::types::Type;

use super::type_utils::CheckResult;
use super::Checker;

type Unary = ast::UnaryExpression;

impl<'a> Checker<'a> {
  pub fn check_unary_expression(&mut self, unary_expr: &Unary) -> CheckResult<Option<Type>> {
    let operand_type = self.check_expression(&unary_expr.operand)?.unwrap_or(Type::Nil);
    self.check_unary_operator(&operand_type, unary_expr)
  }

  fn check_unary_operator(&mut self, operand_t: &Type, unary_expr: &Unary) -> CheckResult<Option<Type>> {
    match unary_expr.operator {
      ast::UnaryOperator::Negate => self.validate_unary_operator(operand_t, unary_expr),
      ast::UnaryOperator::Not => self.validate_unary_operator(operand_t, unary_expr),
      ast::UnaryOperator::Hash => self.validate_unary_operator(operand_t, unary_expr),
    }
  }

  fn validate_unary_operator(&mut self, operand_t: &Type, unary_expr: &Unary) -> CheckResult<Option<Type>> {
    // let result_type = self.check_type(operand_t.clone())?;
    if operand_t.suport_unary_operator(&unary_expr.operator) {
      let result_type = operand_t.get_unary_operator_result_type(&unary_expr.operator);
      return Ok(Some(result_type));
    }

    let diagnostic = TypeError::UnsupportedUnaryOperator(
      unary_expr.operator.clone(),
      operand_t.to_string(),
      Some(unary_expr.get_operator_range()),
    );

    Err(self.create_diagnostic(diagnostic))
  }
}
