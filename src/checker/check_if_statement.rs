use super::Checker;
use crate::ast::ast;

impl Checker {
  pub fn check_if_statement(&mut self, if_: &ast::IfStatement) {
    // Empty statements don't change the type context
  }
}
