use super::{type_utils::CheckResult, Checker};
use crate::{
  ast::{ast, tokens::Token},
  types::Type,
};

impl<'a> Checker<'a> {
  pub fn check_function_statement(&mut self, function: &ast::FunctionStatement) -> CheckResult<Option<Type>> {
    let function_name = function.name.lexeme();
    let mut return_type = self.check_option_type(&function.return_type, false)?;

    // declare function placeholder
    let anonymous_function = self.ctx.create_anonymous_function();
    let scope_pointer = self.ctx.declare_variable(function_name, anonymous_function, None);

    self.enter_scope();
    let params = self.declare_function_params(&function.arguments)?;

    self.ctx.declare_return_param_type(return_type.clone());

    let function_type = Type::new_function(params.clone(), return_type.clone());

    self.ctx.redeclare_variable(function_name, function_type, Some(scope_pointer));

    let last_type = self.check_statement(&function.body)?;

    if let Some(last_type) = last_type {
      if return_type.can_replace(&last_type) {
        return_type = last_type;
      }
    }

    let function_type = Type::new_function(params, return_type);

    self.ctx.redeclare_variable(function_name, function_type.clone(), Some(scope_pointer));

    // println!("{}", function_type);
    self.leave_scope();
    Ok(None)
  }

  pub fn declare_function_params(&mut self, arguments: &Vec<ast::Variable>) -> CheckResult<Vec<Type>> {
    let params = arguments.iter().map::<CheckResult<Type>, _>(|arg| {
      let arg_type = if let Some(ty) = &arg.ty { ty.clone() } else { Type::Unknown };
      let lexeme = arg.name.lexeme();

      let arg_type = self.check_type(arg_type.clone())?;

      self.ctx.declare_variable(lexeme, arg_type.clone(), None);
      self.ctx.declare_variable_range(lexeme, arg.name.range.clone(), None);

      Ok(arg_type)
    });

    let params = params.collect::<CheckResult<Vec<Type>>>()?;

    return Ok(params);
  }

  pub fn check_variadic_type(&mut self, token: &Token, inner_type: Type) -> Type {
    return if token.is_triple_dot() { Type::new_variadic(inner_type) } else { inner_type };
  }
}
