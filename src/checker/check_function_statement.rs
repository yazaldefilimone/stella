use super::Checker;
use crate::{
  ast::{ast, tokens::Token},
  diagnostics::Diagnostic,
  types::{replace_type, Type},
};

impl<'a> Checker<'a> {
  pub fn check_function_statement(&mut self, function: &ast::FunctionStatement) -> Result<Type, Diagnostic> {
    let function_name = function.name.lexeme();
    let mut return_type = self.check_optional_type(&function.return_type)?;

    // declare function placeholder
    let anonymous_function = self.ctx.create_anonymous_function();
    let scope_idx = self.ctx.declare_variable(function_name, anonymous_function, None);

    self.ctx.enter_scope();

    let params = self.declare_function_params(&function.arguments)?;

    self.ctx.declare_return_param_type(return_type.clone());

    let function_type = Type::new_function(params, return_type.clone());
    self.ctx.redeclare_variable(function_name, function_type, Some(scope_idx));

    let last_type = self.check_statement(&function.body)?;

    // check unused variables in current scope
    self.check_used_variable_in_current_scope();

    if last_type.can_replace(&return_type) {
      return_type = replace_type(&return_type, &last_type);
    }

    self.ctx.leave_scope();

    Ok(return_type)
  }

  pub fn declare_function_params(&mut self, arguments: &[(Token, Option<Type>)]) -> Result<Vec<Type>, Diagnostic> {
    let mut params = Vec::with_capacity(arguments.len());
    for (param, ty) in arguments.iter() {
      let arg_type = if let Some(ty) = ty { ty.clone() } else { Type::Unknown };
      let lexeme = param.lexeme();
      self.ctx.declare_variable(lexeme, arg_type.clone(), None);
      self.ctx.declare_variable_range(lexeme, param.range.clone());
      params.push(arg_type);
    }
    Ok(params)
  }
}
