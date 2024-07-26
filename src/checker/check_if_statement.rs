use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::Type,
};

impl Checker {
  pub fn check_if_statement(&mut self, if_: &ast::IfStatement) -> Result<Type, Diagnostic> {
    let condition_t = self.check_expression(&if_.condition)?;
    if condition_t.check_match(&Type::Boolean) {
      let diagnostic = TypeError::TypeMismatchAssignment(
        condition_t.to_string(),
        Type::Boolean.to_string(),
        Some(if_.location.clone()),
      );
      return Err(self.create_diagnostic(diagnostic));
    }
    self.check_statement(&if_.body)?;
    if let Some(else_body) = &if_.else_body {
      self.check_statement(else_body)?;
    }
    Ok(Type::Nil)
  }
}
