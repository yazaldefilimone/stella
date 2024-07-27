use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::Type,
};

impl Checker {
  pub fn check_call_expression(&mut self, call_expr: &ast::CallExpression) -> Result<Type, Diagnostic> {
    let name = call_expr.name.lexeme();
    if !self.ctx.defined_in_current_scope(name.as_str()) {
      return Err(self.create_diagnostic(TypeError::UndeclaredVariable(
        name.to_string(),
        Some(call_expr.name.location.clone()),
      )));
    }
    if let Some(call_t) = self.ctx.get_function(name.as_str()) {
      // todo: improve this
      let call_t = call_t.clone();
      self.check_call_arguments(&call_expr.args, &call_t.params.to_vec())?;
      return Ok(*call_t.return_type);
    }
    let diagnostic = TypeError::UndeclaredVariable(name.to_string(), Some(call_expr.name.location.clone()));
    Err(self.create_diagnostic(diagnostic))
  }

  pub fn check_call_arguments(&mut self, args_call: &ast::Expression, call_tt: &Vec<Type>) -> Result<(), Diagnostic> {
    if let ast::Expression::GroupedExpression(ast::GroupedExpression { expression, location }) = args_call {
      if expression.len() != call_tt.len() {
        return Err(self.create_diagnostic(TypeError::FunctionArityMismatch(
          call_tt.len(),
          expression.len(),
          Some(location.clone()),
        )));
      }
      for (arg_expr, param_t) in expression.iter().zip(call_tt.iter()) {
        let arg_t = self.check_expression(arg_expr)?;
        if !arg_t.check_match(&param_t) {
          self.diagnostics.add(
            TypeError::TypeMismatchAssignment(arg_t.to_string(), param_t.to_string(), Some(location.clone())).into(),
          );
        }
      }
      return Ok(());
    }
    let arg_t = self.check_expression(args_call)?;
    let first_call_t = call_tt.first().unwrap();
    if !arg_t.check_match(first_call_t) {
      // TODO: add location
      return Err(self.create_diagnostic(TypeError::TypeMismatchAssignment(
        first_call_t.to_string(),
        arg_t.to_string(),
        None,
      )));
    }
    return Ok(());
  }
}
