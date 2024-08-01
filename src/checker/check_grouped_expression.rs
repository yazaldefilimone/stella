use super::Checker;
use crate::{ast::ast, diagnostics::Diagnostic, types::Type};

impl<'a> Checker<'a> {
  pub fn check_grouped_expression(&mut self, grup_exper: &ast::GroupedExpression) -> Result<Type, Diagnostic> {
    let location = grup_exper.get_location();
    let mut last_t = Type::Nil;
    for expression in &grup_exper.expressions {
      last_t = self.check_expression(expression)?;
    }
    Ok(last_t)
  }
}
