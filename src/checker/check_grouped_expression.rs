use super::Checker;
use crate::{ast::ast, diagnostics::Diagnostic, types::Type};

impl<'a> Checker<'a> {
  pub fn check_grouped_expression(&mut self, grup_expr: &ast::GroupedExpression) -> Result<Type, Diagnostic> {
    let mut last_type = Type::Nil;
    for expression in &grup_expr.expressions {
      last_type = self.check_expression(expression)?;
    }
    Ok(last_type)
  }
}
