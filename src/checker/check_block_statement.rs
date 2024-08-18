use std::collections::HashSet;

use super::type_utils::CheckResult;
use super::Checker;
use crate::ast::ast;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_block_statement(&mut self, block: &ast::BlockStatement) -> CheckResult<Option<Type>> {
    let mut last_type = HashSet::new();
    for statement in &block.statements {
      match self.check_statement(statement) {
        Ok(Some(ty)) => {
          last_type.insert(ty);
        }
        Ok(None) => {}
        Err(diag) => self.diagnostics.add(diag),
      };
    }
    let array_type = self.create_type_based_array(last_type.into_iter().collect());
    Ok(array_type)
  }
}
