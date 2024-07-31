use super::Checker;
use crate::{ast::ast, diagnostics::Diagnostic, types::Type};

impl<'a> Checker<'a> {
  pub fn check_empty_statement(&mut self, _empty: &ast::EmptyStatement) -> Result<Type, Diagnostic> {
    Ok(Type::Nil)
  }
}
