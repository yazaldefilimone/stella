use super::Checker;
use crate::{
  ast::ast,
  diagnostics::Diagnostic,
  types::{replace_type, Type},
};

impl<'a> Checker<'a> {
  pub fn check_function_statement(&mut self, function: &ast::FunctionStatement) -> Result<Type, Diagnostic> {
    let function_name = function.name.lexeme();
    let mut return_type = self.check_type(&function.return_type);

    // declare function placeholder
    let scope_idx = self.ctx.declare_function_placeholder(function_name);

    self.ctx.enter_scope();
    let mut params = vec![];
    for (param, ty) in function.arguments.iter() {
      let arg_type = self.check_type(ty);
      self.ctx.declare_variable(param.lexeme(), arg_type.clone());
      self.ctx.set_variable_location(param.lexeme(), param.location.clone());
      params.push(arg_type);
    }

    self.ctx.declare_return_param_type(return_type.clone());

    self.ctx.update_function_placeholder(function_name, params, return_type.clone(), scope_idx);

    let last_type = self.check_statement(&function.body)?;

    // check unused variables in current scope
    self.check_used_variable_in_current_scope();

    if last_type.can_replace(&return_type) {
      return_type = replace_type(&return_type, &last_type);
    }
    self.ctx.leave_scope();
    // self.ctx.declare_function(function_name, params, return_type.clone());

    Ok(return_type)
  }
}
