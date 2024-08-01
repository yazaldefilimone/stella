use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::Type,
};

impl<'a> Checker<'a> {
  pub fn check_if_statement(&mut self, if_: &ast::IfStatement) -> Result<Type, Diagnostic> {
    let condition_t = self.check_expression(&if_.condition)?;
    if !condition_t.check_match(&Type::Boolean) {
      let left_location = if_.condition.get_location();
      let diagnostic =
        TypeError::MismatchedTypes(Type::new_boolean().to_string(), condition_t.to_string(), Some(left_location));

      return Err(self.create_diagnostic(diagnostic));
    }

    let body_t = self.check_statement(&if_.then_body)?;
    if let Some(else_body) = &if_.else_body {
      let else_body_t = self.check_statement(else_body)?;

      let union_t = Type::new_union(vec![body_t, else_body_t]);

      return Ok(union_t);
    }
    return Ok(body_t);
  }
}
