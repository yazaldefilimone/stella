use super::type_utils::CheckResult;
use super::Checker;
use crate::ast::ast;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_local_statement(&mut self, declaration: &ast::LocalStatement) -> CheckResult<Option<Type>> {
    let variables = &declaration.variables;
    let initializer = &declaration.initializer;

    let result = variables.iter().enumerate().map::<CheckResult<Option<Type>>, _>(|(position, variable)| {
      let lexeme = variable.name.lexeme();
      let range = variable.name.range.clone();
      let left_hand_side = &(lexeme, variable.ty.clone());
      let init_expression = initializer.get(position);
      let assign_type = match init_expression {
        Some(init_expression) => self.check_expression(init_expression)?.unwrap_or(Type::Nil),
        None => Type::Nil,
      };
      // declare is not return type ...
      self.declare_local_variable(left_hand_side, assign_type, range)?;

      return Ok(None);
    });
    let _ = result.collect::<CheckResult<Vec<_>>>()?;
    return Ok(None);
  }
}
