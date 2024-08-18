use super::{type_utils::CheckResult, Checker};
use crate::{ast::ast, types::Type};

impl<'a> Checker<'a> {
  pub fn check_grouped_expression(&mut self, grup_expr: &ast::GroupedExpression) -> CheckResult<Option<Type>> {
    let mut last_type = None;
    for expression in &grup_expr.expressions {
      last_type = self.check_expression(expression)?;
    }
    Ok(last_type)
  }
}
