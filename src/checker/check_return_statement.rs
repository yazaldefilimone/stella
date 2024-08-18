use super::{type_utils::CheckResult, Checker};
use crate::{ast::ast, diagnostics::TypeError, types::Type, utils::range::Range};

impl<'a> Checker<'a> {
  pub fn check_return_statement(&mut self, return_stmt: &ast::ReturnStatement) -> CheckResult<Option<Type>> {
    let return_types = self.check_multiple_return(&return_stmt.values)?;
    let mut grup_return_type = Type::new_group(return_types);

    if let Some(expected_t) = self.ctx.get_return_param_type() {
      if !self.validate_return_type(&expected_t, &grup_return_type, &return_stmt.range)? {
        return Ok(None);
      }

      if grup_return_type.can_replace(&expected_t) {
        grup_return_type = expected_t.clone();
      }
    }

    if self.ctx.is_global_scope() {
      self.ctx.set_last_return(grup_return_type.clone());
    }
    Ok(Some(grup_return_type))
  }

  pub fn check_multiple_return(&mut self, values: &[ast::Expression]) -> CheckResult<Vec<Type>> {
    let values = values.iter().map(|value| self.check_expression(value)).collect::<Result<Vec<Option<Type>>, _>>()?;
    let types = values.iter().filter(|v| v.is_some()).map(|v| v.clone().unwrap()).collect::<Vec<Type>>();
    Ok(types)
  }

  fn validate_return_type(&self, expected_t: &Type, return_t: &Type, range: &Range) -> CheckResult<bool> {
    if return_t.is_group() && expected_t.is_group() && !return_t.same_group_length(expected_t) {
      let diagnostic = TypeError::MismatchedTypes(expected_t.to_string(), return_t.to_string(), Some(range.clone()));
      return Err(self.create_diagnostic(diagnostic));
    }

    if !expected_t.check_match(return_t) {
      let diagnostic = TypeError::MismatchedTypes(expected_t.to_string(), return_t.to_string(), Some(range.clone()));
      return Err(self.create_diagnostic(diagnostic));
    }

    Ok(true)
  }
}
