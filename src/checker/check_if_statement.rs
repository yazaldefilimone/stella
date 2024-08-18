use super::{type_utils::CheckResult, Checker};
use crate::{ast::ast, types::Type};
impl<'a> Checker<'a> {
  pub fn check_if_statement(&mut self, if_stmt: &ast::IfStatement) -> CheckResult<Option<Type>> {
    let condition_type = self.check_expression(&if_stmt.condition)?.unwrap_or(Type::Nil);
    if !condition_type.check_match(&Type::Boolean) {
      let condition_range = if_stmt.condition.get_range();
      return Err(self.create_type_mismatch(Type::new_boolean(), condition_type, condition_range));
    }

    self.enter_scope();
    let narrowing = self.is_condition_narrowing(&if_stmt.condition);
    if narrowing {
      let condition_narrowing = self.get_specified_type(&if_stmt.condition);
      match condition_narrowing {
        Ok((Some(name), Some(narrowing_type))) => {
          self.ctx.declare_variable(name.as_str(), narrowing_type, None);
        }
        _ => {}
      }
    }

    let then_type = self.check_statement(&if_stmt.then_body)?;

    self.leave_scope();

    let mut result_types = vec![];

    if let Some(then_type) = then_type {
      result_types.push(then_type);
    }

    for else_if_branch in &if_stmt.else_if_branches {
      let else_if_condition_type = self.check_expression(&else_if_branch.condition)?.unwrap_or(Type::Nil);
      if !else_if_condition_type.check_match(&Type::Boolean) {
        let condition_range = else_if_branch.condition.get_range();
        return Err(self.create_type_mismatch(Type::new_boolean(), else_if_condition_type, condition_range));
      }
      self.enter_scope();

      if self.is_condition_narrowing(&else_if_branch.condition) {
        let condition_narrowing = self.get_specified_type(&else_if_branch.condition);
        match condition_narrowing {
          Ok((Some(name), Some(narrowing_type))) => {
            self.ctx.declare_variable(name.as_str(), narrowing_type, None);
          }
          _ => {}
        }
      }

      if let Some(else_if_type) = self.check_statement(&else_if_branch.then_branch)? {
        result_types.push(else_if_type);
      }

      self.leave_scope();
    }

    if let Some(else_body) = &if_stmt.else_body {
      self.enter_scope();
      if let Some(else_type) = self.check_statement(else_body)? {
        result_types.push(else_type);
      }
      self.leave_scope();
    }
    // println!("result {:#?}", result_types);
    if result_types.len() > 1 {
      return Ok(Some(Type::new_union(result_types)));
    }

    if let Some(single_type) = result_types.into_iter().next() {
      return Ok(Some(single_type));
    }

    Ok(None)
  }
}
