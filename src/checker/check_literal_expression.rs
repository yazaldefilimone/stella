use super::type_utils::CheckResult;
use super::Checker;
use crate::ast::ast;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_literal_expression(&mut self, literal: &ast::LiteralExpression) -> CheckResult<Option<Type>> {
    match literal {
      ast::LiteralExpression::Number(_) => Ok(Some(Type::Number)),
      ast::LiteralExpression::String(_) => Ok(Some(Type::String)),
      ast::LiteralExpression::Boolean(_) => Ok(Some(Type::Boolean)),
      ast::LiteralExpression::Nil(_) => Ok(Some(Type::Nil)),
    }
  }
}
