use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::types::Type;

impl Checker {
  pub fn check_literal_expression(&mut self, literal: &ast::LiteralExpression) -> Result<Type, Diagnostic> {
    match literal {
      ast::LiteralExpression::NumberLiteral(_) => Ok(Type::Number),
      ast::LiteralExpression::StringLiteral(_) => Ok(Type::String),
      ast::LiteralExpression::BoolLiteral(_) => Ok(Type::Boolean),
      ast::LiteralExpression::NilLiteral => Ok(Type::Nil),
    }
  }
}
