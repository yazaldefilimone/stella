use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::{replace_type, Type},
  utils::range::Range,
};

impl<'a> Checker<'a> {
  pub fn check_return_statement(&mut self, return_stmt: &ast::ReturnStatement) -> Result<Type, Diagnostic> {
    let return_types = self.check_multiple_return(&return_stmt.values)?;
    let mut grup_return_type = Type::new_grup(return_types);

    if let Some(expected_t) = self.ctx.get_return_param_type() {
      if !self.validate_return_type(&grup_return_type, &expected_t, &return_stmt.range)? {
        return Ok(Type::Unknown);
      }

      if grup_return_type.can_replace(&expected_t) {
        grup_return_type = replace_type(&grup_return_type, &expected_t);
      }
    }

    if self.ctx.is_global_scope() {
      self.ctx.set_last_return(grup_return_type.clone());
    }

    Ok(grup_return_type)
  }

  pub fn check_multiple_return(&mut self, values: &[ast::Expression]) -> Result<Vec<Type>, Diagnostic> {
    values.iter().map(|value| self.check_expression(value)).collect()
  }

  fn validate_return_type(&self, return_t: &Type, expected_t: &Type, range: &Range) -> Result<bool, Diagnostic> {
    if return_t.is_grup() && expected_t.is_grup() && !return_t.same_grup_length(expected_t) {
      let diagnostic = TypeError::MismatchedTypes(expected_t.to_string(), return_t.to_string(), Some(range.clone()));
      return Err(self.create_diagnostic(diagnostic));
    }

    if !return_t.check_match(expected_t) {
      let diagnostic = TypeError::MismatchedTypes(expected_t.to_string(), return_t.to_string(), Some(range.clone()));
      return Err(self.create_diagnostic(diagnostic));
    }

    Ok(true)
  }
}
