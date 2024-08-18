use super::{type_utils::CheckResult, Checker};
use crate::{ast::ast, types::Type};

impl<'a> Checker<'a> {
  pub fn check_while_statement(&mut self, while_: &ast::WhileStatement) -> CheckResult<Option<Type>> {
    self.enter_scope();
    let condition_type = self.check_expression(&while_.condition)?.unwrap_or(Type::Nil);
    if !condition_type.check_match(&Type::Boolean) {
      let condition_range = while_.condition.get_range();
      return Err(self.create_type_mismatch(Type::new_boolean(), condition_type, condition_range));
    }

    if self.is_condition_narrowing(&while_.condition) {
      let narrowing = self.get_specified_type(&while_.condition)?;
      match narrowing {
        (Some(narrowing), Some(narrowing_range)) => {
          self.ctx.declare_variable(narrowing.as_str(), narrowing_range, None);
        }
        _ => {}
      }
    }

    let last_t = self.check_statement(&while_.body)?;

    self.leave_scope();

    Ok(last_t)
  }
}
