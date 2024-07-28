use super::Checker;
use crate::ast::ast;

impl Checker<'_> {
  pub fn check_for_statement(&mut self, for_: &ast::ForStatement) {
    // Empty statements don't change the type context
  }
}
