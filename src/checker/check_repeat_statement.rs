use super::{type_utils::CheckResult, Checker};
use crate::{ast::ast, types::Type};

impl<'a> Checker<'a> {
  pub fn check_repeat_statement(&mut self, repeat: &ast::RepeatStatement) -> CheckResult<Option<Type>> {
    todo!()
  }
}
