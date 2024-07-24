use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::types::Type;

impl Checker {
  pub fn check_expression_statement(&mut self, expression: &ast::Expression) -> Result<Type, Diagnostic> {
    match expression {
      ast::Expression::LiteralExpression(literal) => self.check_literal_expression(literal),
      ast::Expression::Identifier(ident) => self.check_identifier(ident),
      _ => todo!("Implement more expression checks"),
    }
  }
}
