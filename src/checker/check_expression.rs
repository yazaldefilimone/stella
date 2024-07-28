use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::types::Type;

impl Checker<'_> {
  pub fn check_expression(&mut self, expression: &ast::Expression) -> Result<Type, Diagnostic> {
    match expression {
      ast::Expression::LiteralExpression(literal) => self.check_literal_expression(literal),
      ast::Expression::Identifier(ident) => self.check_identifier(ident),
      ast::Expression::CallExpression(call) => self.check_call_expression(call),
      ast::Expression::BinaryExpression(binary_expr) => self.check_binary_expression(binary_expr),
      ast::Expression::RequireExpression(require) => self.check_require_expression(require),
      _ => todo!("Oops, try go to: {:#?}", expression),
    }
  }
}
