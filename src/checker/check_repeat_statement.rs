use super::Checker;
use crate::ast::ast;

impl<'a> Checker<'a> {
  pub fn check_repeat_statement(&mut self, repeat: &ast::RepeatStatement) {
    // Empty statements don't change the type context
  }
}
