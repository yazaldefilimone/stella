use super::Checker;
use crate::ast::ast;
use crate::diagnostics::{Diagnostic, TypeError};
use crate::types::Type;

impl Checker {
  pub fn check_local_statement(&mut self, local: &ast::LocalStatement) -> Result<Type, Diagnostic> {
    let text_name = local.name.lexeme();
    let right_t = self.check_t(&local.type_);

    let left_t = if let Some(init) = &local.init {
      self.check_expression_statement(init).unwrap_or(Type::Unknown)
    } else {
      Type::Unknown
    };

    let location = local.location.clone();
    if !right_t.check_is_can_replace(&left_t) {
      let diagnostic = TypeError::MismatchedTypes(right_t.to_string(), left_t.to_string(), Some(location));
      return Err(self.create_diagnostic(diagnostic));
    }

    self.ctx.set_variable_location(text_name.as_str(), location);

    self.ctx.declare_variable(text_name.as_str(), right_t.clone());
    Ok(right_t)
  }
}
