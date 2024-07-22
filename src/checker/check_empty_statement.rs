use super::Checker;
use crate::ast::ast;

impl Checker {
  pub fn check_empty_statement(&mut self, _empty: &ast::EmptyStatement) {
    // Empty statements don't change the type context
  }
}
