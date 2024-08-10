use super::Checker;
use crate::{
  ast::ast,
  diagnostics::Diagnostic,
  types::{replace_type, Type},
};

impl<'a> Checker<'a> {
  pub fn check_function_expression(&mut self, function: &ast::FunctionExpression) -> Result<Type, Diagnostic> {
    let mut return_type = self.check_optional_type(&function.return_type)?;
    self.ctx.enter_scope();

    let params = self.declare_function_params(&function.arguments)?;

    self.ctx.declare_return_param_type(return_type.clone());

    let last_type = self.check_statement(&function.body)?;

    if !return_type.check_match(&last_type) {
      let range = function.range_return_type.clone().unwrap_or(function.range.clone());
      let diagnostic = self.create_type_mismatch(return_type, last_type, range);
      return Err(diagnostic);
    }
    if return_type.can_replace(&last_type) {
      return_type = replace_type(&return_type, &last_type);
    }

    self.check_used_variable_in_current_scope();
    self.ctx.leave_scope();

    let function_type = Type::new_function(params, return_type);
    Ok(function_type)
  }
}
