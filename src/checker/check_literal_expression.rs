use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_literal_expression(&mut self, literal: &ast::LiteralExpression) -> Result<Type, Diagnostic> {
    match literal {
      ast::LiteralExpression::Number(_) => Ok(Type::Number),
      ast::LiteralExpression::String(_) => Ok(Type::String),
      ast::LiteralExpression::Boolean(_) => Ok(Type::Boolean),
      ast::LiteralExpression::Nil(_) => Ok(Type::Nil),
    }
  }
}
