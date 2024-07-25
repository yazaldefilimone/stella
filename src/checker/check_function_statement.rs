use super::Checker;
use crate::{ast::ast, diagnostics::Diagnostic, types::Type};

impl Checker {
  // function name, arguments, return type, body
  pub fn check_function_statement(&mut self, function: &ast::FunctionStatement) -> Result<Type, Diagnostic> {
    self.ctx.enter_scope();
    let name = function.name.lexeme();
    let mut return_t = self.check_t(&function.return_t);

    let mut params = vec![];
    for (param, t) in function.arguments.iter() {
      let arg_t = self.check_t(t);
      self.ctx.declare_variable(param.lexeme().as_str(), arg_t.clone());
      params.push(arg_t);
    }

    self.ctx.declare_return_variable_type(return_t.clone());

    let last_t = self.check_statement(&function.body)?;

    if last_t.check_is_can_replace(&return_t) {
      return_t = last_t;
    }
    self.ctx.leave_scope();
    self.ctx.declare_function(name.as_str(), params, return_t.clone());

    Ok(return_t)
  }
}
