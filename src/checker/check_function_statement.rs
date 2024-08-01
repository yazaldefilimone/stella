use super::Checker;
use crate::{ast::ast, diagnostics::Diagnostic, types::Type};

impl<'a> Checker<'a> {
  // function name, arguments, return type, body
  pub fn check_function_statement(&mut self, function: &ast::FunctionStatement) -> Result<Type, Diagnostic> {
    self.ctx.enter_scope();
    let name = function.name.lexeme();
    let mut return_type = self.check_t(&function.return_type);

    let mut params = vec![];
    for (param, ty) in function.arguments.iter() {
      let arg_type = self.check_t(ty);
      self.ctx.declare_variable(param.lexeme(), arg_type.clone());
      self.ctx.set_variable_location(param.lexeme(), param.location.clone());
      params.push(arg_type);
    }

    self.ctx.declare_return_param_type(return_type.clone());

    let last_type = self.check_statement(&function.body)?;

    if last_type.can_replace(&return_type) {
      return_type = last_type;
    }
    self.ctx.leave_scope();
    self.ctx.declare_function(name, params, return_type.clone());

    Ok(return_type)
  }
}
