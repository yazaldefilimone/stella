use super::Checker;
use crate::ast::ast;
use crate::diagnostics::{Diagnostic, TypeError};
use crate::types::Type;

impl Checker<'_> {
  pub fn check_variable_declaration(&mut self, declaration: &ast::VariableDeclaration) -> Result<Type, Diagnostic> {
    let text_name = declaration.name.lexeme();

    if self.ctx.defined_in_current_scope(text_name.as_str()) && declaration.local {
      let location = declaration.location.clone();
      let diagnostic = TypeError::RedeclaredInSameScope(text_name.to_string(), Some(location));
      return Err(self.create_diagnostic(diagnostic));
    }
    let mut left_t = self.check_t(&declaration.t);

    let mut right_t = Type::Unknown;

    if let Some(init) = &declaration.init {
      right_t = self.check_expression(init)?;
    }

    let location = declaration.location.clone();

    if !left_t.check_match(&right_t) {
      let diagnostic = TypeError::TypeMismatchAssignment(left_t.to_string(), right_t.to_string(), Some(location));
      return Err(self.create_diagnostic(diagnostic));
    }

    if left_t.check_is_can_replace(&right_t) {
      left_t = right_t;
    }

    self.ctx.set_variable_location(text_name.as_str(), location);

    if declaration.local {
      self.ctx.declare_variable(text_name.as_str(), left_t.clone());
    } else {
      self.ctx.declare_global_variable(text_name.as_str(), left_t.clone());
    }
    Ok(left_t)
  }
}
