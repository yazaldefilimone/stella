use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::types::Type;

impl Checker {
  pub fn check_block_statement(&mut self, block: &ast::BlockStatement) -> Result<Type, Diagnostic> {
    let mut last_t = Type::Nil;
    for statement in &block.body {
      match self.check_statement(statement) {
        Ok(ty) => last_t = ty,
        Err(diag) => self.diagnostics.add(diag),
      }
    }
    Ok(last_t)
  }
}
