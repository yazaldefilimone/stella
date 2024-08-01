use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::{replace_type, Type},
};

impl<'a> Checker<'a> {
  pub fn check_return_statement(&mut self, return_stmt: &ast::ReturnStatement) -> Result<Type, Diagnostic> {
    // suport single return statement, todo: support multiple return statements
    let multiple_return = self.check_multiple_return(&return_stmt.values)?;
    let mut grup_return_type = Type::new_grup(multiple_return);

    let expected_return_t = self.ctx.get_return_param_type();

    if let Some(expected_t) = expected_return_t {
      if grup_return_type.is_grup() && expected_t.is_grup() && !grup_return_type.same_grup_length(expected_t) {
        let localtion = Some(return_stmt.location.clone());
        let diagnostic = TypeError::MismatchedTypes(expected_t.to_string(), grup_return_type.to_string(), localtion);
        return Err(self.create_diagnostic(diagnostic));
      }

      if !grup_return_type.check_match(expected_t) {
        let localtion = Some(return_stmt.location.clone());
        let diagnostic = TypeError::MismatchedTypes(expected_t.to_string(), grup_return_type.to_string(), localtion);
        return Err(self.create_diagnostic(diagnostic));
      }

      if grup_return_type.can_replace(expected_t) {
        grup_return_type = replace_type(&grup_return_type, &expected_t);
      }
    }

    if self.ctx.is_global_scope() {
      self.ctx.set_last_return(grup_return_type.clone());
    }
    Ok(grup_return_type)
  }

  pub fn check_multiple_return(&mut self, values: &Vec<ast::Expression>) -> Result<Vec<Type>, Diagnostic> {
    Ok(values.iter().map(|value| self.check_expression(value)).collect::<Result<Vec<Type>, Diagnostic>>()?)
  }
}
