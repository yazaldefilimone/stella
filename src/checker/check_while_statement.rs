use super::Checker;
use crate::ast::ast;

impl<'a> Checker<'a> {
  pub fn check_while_statement(&mut self, while_: &ast::WhileStatement) {
    // Empty statements don't change the type context
  }
}
