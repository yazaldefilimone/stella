use super::{type_utils::CheckResult, Checker};
use crate::{ast::ast, types::Type};

impl<'a> Checker<'a> {
  pub fn check_empty_statement(&mut self, _empty: &ast::EmptyStatement) -> CheckResult<Option<Type>> {
    Ok(None)
  }
}
