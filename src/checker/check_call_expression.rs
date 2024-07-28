use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::Type,
};

impl Checker<'_> {
  pub fn check_call_expression(&mut self, call_expr: &ast::CallExpression) -> Result<Type, Diagnostic> {
    let name = call_expr.name.lexeme();

    if !self.ctx.defined_in_current_scope(name.as_str()) {
      let diagnostic = TypeError::UndeclaredVariable(name.to_string(), Some(call_expr.name.location.clone()));
      return Err(self.create_diagnostic(diagnostic));
    }

    if let Some(call_t) = self.ctx.get_function(name.as_str()) {
      // todo: improve this
      let return_t = *call_t.return_type.clone();
      self.check_call_arguments(&call_expr.args, &call_t.params.to_vec())?;
      return Ok(return_t);
    }

    let diagnostic = TypeError::UndeclaredVariable(name.to_string(), Some(call_expr.name.location.clone()));
    Err(self.create_diagnostic(diagnostic))
  }

  pub fn check_call_arguments(&mut self, args_call: &ast::Expression, call_tt: &[Type]) -> Result<(), Diagnostic> {
    if let ast::Expression::GroupedExpression(ast::GroupedExpression { expression, location }) = args_call {
      if expression.len() != call_tt.len() {
        let diagnostic = TypeError::FunctionArityMismatch(call_tt.len(), expression.len(), Some(location.clone()));
        return Err(self.create_diagnostic(diagnostic));
      }

      for (arg_expr, param_t) in expression.iter().zip(call_tt.iter()) {
        let arg_t = self.check_expression(arg_expr)?;
        if !arg_t.check_match(param_t) {
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
      let diagnostic = TypeError::TypeMismatchAssignment(first_call_t.to_string(), arg_t.to_string(), None);
      return Err(self.create_diagnostic(diagnostic));
    }

    Ok(())
  }
}
