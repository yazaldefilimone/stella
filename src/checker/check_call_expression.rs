use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::Type,
  utils::range::Range,
};

impl<'a> Checker<'a> {
  pub fn check_call_expression(&mut self, call_expr: &ast::CallExpression) -> Result<Type, Diagnostic> {
    let name = call_expr.name.lexeme();
    let (defined, scope_pointer) = self.ctx.defined_in_any_scope(name);

    if !defined {
      return Err(
        self.create_diagnostic(TypeError::UndeclaredVariable(name.to_string(), Some(call_expr.name.range.clone()))),
      );
    }

    let call_type = match self.ctx.get_variable(name, Some(scope_pointer)) {
      Some(call_type) => call_type.clone(),
      None => {
        return Err(
          self.create_diagnostic(TypeError::UndeclaredVariable(name.to_string(), Some(call_expr.name.range.clone()))),
        );
      }
    };

    self.check_call_type(&call_type, &call_expr.args, call_expr.get_range())
  }

  pub fn check_call_type(
    &mut self,
    call_type: &Type,
    args: &ast::Expression,
    range: Range,
  ) -> Result<Type, Diagnostic> {
    match call_type {
      Type::Function(func_type) => {
        self.check_call_arguments(args, &func_type.params)?;
        Ok(*func_type.return_type.clone())
      }
      Type::Unknown => Ok(Type::Unknown),
      _ => Err(self.create_diagnostic(TypeError::ExpectedFunction(call_type.to_string(), Some(range)))),
    }
  }

  pub fn check_call_arguments(&mut self, args: &ast::Expression, params: &[Type]) -> Result<(), Diagnostic> {
    if let ast::Expression::Grouped(ast::GroupedExpression { expressions, range }) = args {
      if expressions.len() != params.len() {
        return Err(self.create_diagnostic(TypeError::FunctionArityMismatch(
          params.len(),
          expressions.len(),
          Some(range.clone()),
        )));
      }

      for (arg_expr, param_type) in expressions.iter().zip(params.iter()) {
        let inferred_type = self.check_expression(arg_expr)?;
        let param_type_checked = self.check_type(param_type.clone())?;

        if !inferred_type.check_match(&param_type_checked) {
          return Err(self.create_diagnostic(TypeError::MismatchedTypes(
            param_type_checked.to_string(),
            inferred_type.to_string(),
            Some(arg_expr.get_range()),
          )));
        }
      }
      Ok(())
    } else {
      self.check_single_argument(args, params)
    }
  }

  fn check_single_argument(&mut self, arg: &ast::Expression, params: &[Type]) -> Result<(), Diagnostic> {
    if params.len() != 1 {
      return Err(self.create_diagnostic(TypeError::FunctionArityMismatch(params.len(), 1, Some(arg.get_range()))));
    }

    let param_type = self.check_type(params.first().unwrap().clone())?;
    let arg_type = self.check_expression(arg)?;

    if !arg_type.check_match(&param_type) {
      return Err(self.create_diagnostic(TypeError::MismatchedTypes(
        param_type.to_string(),
        arg_type.to_string(),
        Some(arg.get_range()),
      )));
    }

    Ok(())
  }
}
