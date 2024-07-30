use crate::ast::ast;
use crate::diagnostics::TypeError;
use crate::{diagnostics::Diagnostic, types::Type};

use super::Checker;

impl Checker<'_> {
  pub fn check_unary_expression(&mut self, unary_expr: &ast::UnaryExpression) -> Result<Type, Diagnostic> {
    let operand_t = self.check_expression(&unary_expr.operand)?;
    self.check_unary_operator(&operand_t, &unary_expr)?;
    Ok(operand_t)
  }

  fn check_unary_operator(&mut self, t: &Type, unary: &ast::UnaryExpression) -> Result<Type, Diagnostic> {
    match unary.operator {
      ast::UnaryOperator::Negate => {
        if t.check_match(&Type::Number) {
          return Ok(Type::Number);
        }

        let diagnostic = TypeError::UnsupportedOperator(
          t.to_string(),
          t.to_string(),
          unary.operator.to_str().to_owned(),
          Some(unary.get_operator_location()),
        );

        Err(self.create_diagnostic(diagnostic))
      }

      ast::UnaryOperator::Not => {
        if t.check_match(&Type::Boolean) {
          return Ok(Type::Boolean);
        }
        Err(self.create_diagnostic(TypeError::UnsupportedOperator(
          t.to_string(),
          t.to_string(),
          unary.operator.to_str().to_owned(),
          Some(unary.get_operator_location()),
        )))
      }
      _ => todo!("Implement more unary operators"),
    }
  }
}
