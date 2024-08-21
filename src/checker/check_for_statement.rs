use super::{type_utils::CheckResult, Checker};
use crate::{ast::ast, types::Type};

impl<'a> Checker<'a> {
  pub fn check_for_statement(&mut self, for_: &ast::ForStatement) -> CheckResult<Option<Type>> {
    let init_value = for_.init.initializer.first().unwrap();
    let init_variable = for_.init.variables.first().unwrap();
    if for_.init.initializer.len() != 1 || for_.init.variables.len() != 1 {
      // todo: implement this
      panic!("Invalid for statement");
      // let diagnostic =
      //   TypeError::MismatchedTypes(init_variable.to_string(), init_value.to_string(), Some(init_value.get_range()));
      // return Err(diagnostic);
    }

    let init_type = self.check_expression(&init_value)?.unwrap_or(Type::Nil);
    if !init_type.check_match(&Type::Number) {
      let init_range = for_.init.get_range();
      let diagnostic = self.create_type_mismatch(Type::Number, init_type.to_owned(), init_range);
      return Err(diagnostic);
    }

    self.assign_variables(init_variable, Some(init_value))?;

    let init_type = self.check_expression(&init_value)?.unwrap_or(Type::Nil);
    let limit_type = self.check_expression(&for_.limit)?.unwrap_or(Type::Nil);

    let step_type = match &for_.step {
      Some(step) => Some(self.check_expression(step)?.unwrap_or(Type::Nil)),
      None => None,
    };

    let body_type = self.check_statement(&for_.body)?;

    if !init_type.check_match(&Type::Number) {
      let init_range = for_.init.get_range();
      let diagnostic = self.create_type_mismatch(Type::Number, init_type.to_owned(), init_range);
      return Err(diagnostic);
    }

    if !limit_type.check_match(&Type::Number) {
      let limit_range = for_.limit.get_range();
      let diagnostic = self.create_type_mismatch(Type::Number, limit_type.to_owned(), limit_range);
      return Err(diagnostic);
    }

    if let Some(step_type) = step_type {
      if !step_type.check_match(&Type::Number) {
        let step_range = for_.step.as_ref().unwrap().get_range();
        let diagnostic = self.create_type_mismatch(Type::Number, step_type.to_owned(), step_range);
        return Err(diagnostic);
      }
    }

    Ok(body_type)
  }
}
