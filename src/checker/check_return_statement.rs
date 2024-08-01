use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::Type,
};

impl<'a> Checker<'a> {
  pub fn check_return_statement(&mut self, return_stmt: &ast::ReturnStatement) -> Result<Type, Diagnostic> {
    let mut return_t = Type::Nil;
    // suport single return statement, todo: support multiple return statements
    let first_return = return_stmt.values.first();
    if let Some(return_value) = first_return {
      return_t = self.check_expression(return_value)?;
    }

    let expected_return_t = self.ctx.get_return_param_type();
    if let Some(expected_t) = expected_return_t {
      if !return_t.check_match(expected_t) {
        let localtion = Some(return_stmt.location.clone());
        let diagnostic = TypeError::MismatchedTypes(expected_t.to_string(), return_t.to_string(), localtion);
        return Err(self.create_diagnostic(diagnostic));
      }

      if return_t.can_replace(expected_t) {
        return_t = expected_t.clone();
      }
    }

    if self.ctx.is_global_scope() {
      self.ctx.set_last_return(return_t.clone());
    }

    Ok(return_t)
  }
}
