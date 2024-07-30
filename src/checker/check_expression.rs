use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::types::Type;

impl Checker<'_> {
  pub fn check_expression(&mut self, expression: &ast::Expression) -> Result<Type, Diagnostic> {
    match expression {
      ast::Expression::Literal(literal) => self.check_literal_expression(literal),
      ast::Expression::Identifier(ident) => self.check_identifier(ident),
      ast::Expression::Call(call) => self.check_call_expression(call),
      ast::Expression::Binary(binary_expr) => self.check_binary_expression(binary_expr),
      ast::Expression::Require(require) => self.check_require_expression(require),
      ast::Expression::Unary(unary_expr) => self.check_unary_expression(unary_expr),
      _ => todo!("Oops, try go to: {:#?}", expression),
    }
  }
}
