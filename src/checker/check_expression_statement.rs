use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::types::Type;

impl Checker {
  pub fn check_expression_statement(&mut self, expression: &ast::ExpressionStatement) -> Result<Type, Diagnostic> {
    match expression {
      ast::ExpressionStatement::LiteralExpression(literal) => self.check_literal_expression(literal),
      ast::ExpressionStatement::IdentifierExpression(identifier) => self.check_identifier_expression(identifier),
      _ => todo!("Implement more expression checks"),
    }
  }
}
