use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::Type,
};

impl<'a> Checker<'a> {
  pub fn check_call_expression(&mut self, call_expr: &ast::CallExpression) -> Result<Type, Diagnostic> {
    let name = call_expr.name.lexeme();
    let (defined, scope_idx) = self.ctx.defined_in_any_scope(name);

    if !defined {
      let diagnostic = TypeError::UndeclaredVariable(name.to_string(), Some(call_expr.name.location.clone()));
      return Err(self.create_diagnostic(diagnostic));
    }

    if let Some(call_t) = self.ctx.get_function_in_scope(name, scope_idx) {
      // todo: improve this
      let return_t = *call_t.return_type.clone();
      self.check_call_arguments(&call_expr.args, &call_t.params.to_vec())?;
      return Ok(return_t);
    }

    let diagnostic = TypeError::UndeclaredVariable(name.to_string(), Some(call_expr.name.location.clone()));
    Err(self.create_diagnostic(diagnostic))
  }

  pub fn check_call_arguments(&mut self, args_call: &ast::Expression, params_tt: &[Type]) -> Result<(), Diagnostic> {
    if let ast::Expression::Grouped(ast::GroupedExpression { expressions, location }) = args_call {
      if expressions.len() != params_tt.len() {
        let diagnostic = TypeError::FunctionArityMismatch(params_tt.len(), expressions.len(), Some(location.clone()));
        return Err(self.create_diagnostic(diagnostic));
      }

      for (arg_expr, param_t) in expressions.iter().zip(params_tt.iter()) {
        let arg_t = self.check_expression(arg_expr)?;
        if !arg_t.check_match(&param_t) {
          let location = arg_expr.get_location();
          let diagnostic = TypeError::MismatchedTypes(param_t.to_string(), arg_t.to_string(), Some(location.clone()));
          return Err(self.create_diagnostic(diagnostic));
        }
      }
      return Ok(());
    }

    let arg_t = self.check_expression(args_call)?;

    if params_tt.len() != 1 {
      let location = args_call.get_location();
      let diagnostic = TypeError::FunctionArityMismatch(params_tt.len(), 1, Some(location));
      return Err(self.create_diagnostic(diagnostic));
    }

    let param_tt = params_tt.first().unwrap();

    if !arg_t.check_match(&param_tt) {
      let diagnostic = TypeError::MismatchedTypes(param_tt.to_string(), arg_t.to_string(), None);
      return Err(self.create_diagnostic(diagnostic));
    }

    Ok(())
  }
}
