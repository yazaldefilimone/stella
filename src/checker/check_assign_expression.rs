use super::{type_utils::CheckResult, Checker};
use crate::{ast::ast::AssignExpression, types::Type};

impl<'a> Checker<'a> {
  pub fn check_assign_expression(&mut self, assign: &AssignExpression) -> CheckResult<Option<Type>> {
    let result = assign.left.iter().enumerate().map::<CheckResult<()>, _>(|(position, variable)| {
      let left_expression = variable;
      let right_expression = assign.right.get(position);
      self.assign_variables(left_expression, right_expression)?;
      return Ok(());
    });
    // assign don't return a type
    let _ = result.collect::<CheckResult<Vec<()>>>()?;
    return Ok(None);
  }
}
