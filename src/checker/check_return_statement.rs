use super::Checker;
use crate::ast::ast;

impl Checker {
  pub fn check_return_statement(&mut self, return_: &ast::ReturnStatement) {
    // Empty statements don't change the type context
  }
}
