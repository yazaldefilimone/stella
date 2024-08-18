use super::{type_utils::CheckResult, Checker};
use crate::{
  ast::ast,
  diagnostics::TypeError,
  types::{Type, VariadicType},
  utils::range::Range,
};

impl<'a> Checker<'a> {
  pub fn check_call_expression(&mut self, call_expr: &ast::CallExpression) -> CheckResult<Option<Type>> {
    let call_type = self.check_expression(&call_expr.left)?.unwrap();
    // let (defined, scope_pointer) = self.ctx.defined_in_any_scope(name);

    // if !defined {
    //   let diagnostic = TypeError::UndeclaredVariable(name.to_string(), Some(call_expr.name.range.clone()));
    //   return Err(self.create_diagnostic(diagnostic));
    // }

    // let call_type = match self.ctx.get_variable(name, Some(scope_pointer)) {
    //   Some(call_type) => call_type.clone(),
    //   None => {
    //     let diagnostic = TypeError::UndeclaredVariable(name.to_string(), Some(call_expr.name.range.clone()));
    //     return Err(self.create_diagnostic(diagnostic));
    //   }
    // };

    self.check_call_type(&call_type, &call_expr.args, call_expr.get_range())
  }

  pub fn check_call_type(&mut self, call: &Type, args: &ast::Expression, range: Range) -> CheckResult<Option<Type>> {
    match call {
      Type::Function(func_type) => {
        self.check_call_arguments(args, &func_type.params)?;
        Ok(Some(*func_type.return_type.clone()))
      }
      Type::Unknown => Ok(Some(Type::Unknown)),
      _ => Err(self.create_diagnostic(TypeError::ExpectedFunction(call.to_string(), Some(range)))),
    }
  }

  pub fn check_call_arguments(&mut self, args: &ast::Expression, params: &[Type]) -> CheckResult<()> {
    if let ast::Expression::Grouped(ast::GroupedExpression { expressions, range }) = args {
      let required_params = params.iter().filter(|p| !p.is_variadic()).count();
      let variadic_param = params.iter().find(|p| p.is_variadic());

      if expressions.len() < required_params || (variadic_param.is_none() && expressions.len() != params.len()) {
        let diagnostic = TypeError::FunctionArityMismatch(params.len(), expressions.len(), Some(range.clone()));
        return Err(self.create_diagnostic(diagnostic));
      }

      for (arg_expr, param_type) in expressions.iter().zip(params.iter()) {
        if param_type.is_variadic() {
          self.check_variadic_arguments(arg_expr, param_type)?;
        } else {
          self.check_single_argument(arg_expr, param_type)?;
        }
      }

      if expressions.len() > required_params {
        if let Some(variadic_type) = variadic_param {
          for arg_expr in &expressions[required_params..] {
            self.check_variadic_arguments(arg_expr, variadic_type)?;
          }
        }
      }
      return Ok(());
    }
    if params.len() != 1 {
      return Err(self.create_diagnostic(TypeError::FunctionArityMismatch(params.len(), 1, Some(args.get_range()))));
    }
    self.check_single_argument(args, params.first().unwrap())
  }

  fn check_single_argument(&mut self, arg: &ast::Expression, param_type: &Type) -> CheckResult<()> {
    let param_type_checked = self.check_type(param_type.clone())?;
    let arg_type = self.check_expression(arg)?.unwrap();
    if !arg_type.check_match(&param_type_checked) {
      return Err(self.create_diagnostic(TypeError::MismatchedTypes(
        param_type_checked.to_string(),
        arg_type.to_string(),
        Some(arg.get_range()),
      )));
    }

    Ok(())
  }

  fn check_variadic_arguments(&mut self, arg: &ast::Expression, param_type: &Type) -> CheckResult<()> {
    if let Type::Variadic(VariadicType { inner_type }) = param_type {
      let arg_type = self.check_expression(arg)?.unwrap();

      let inner_type = self.check_type(*inner_type.to_owned())?;
      if !arg_type.check_match(&inner_type) {
        let diagnostic =
          TypeError::MismatchedTypes(inner_type.to_string(), arg_type.to_string(), Some(arg.get_range()));
        return Err(self.create_diagnostic(diagnostic));
      }
      Ok(())
    } else {
      let diagnostic = TypeError::ExpectedVariadic(param_type.to_string(), Some(arg.get_range()));
      Err(self.create_diagnostic(diagnostic))
    }
  }
}
