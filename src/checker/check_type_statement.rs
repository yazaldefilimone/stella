use super::Checker;
use crate::ast::ast;
// use crate::diagnostics::{Diagnostic, TypeError};
// use crate::types::Type;

impl Checker<'_> {
  pub fn check_type_statement(&mut self, local: &ast::VariableDeclaration) {
    // type .. = type
  }
}
