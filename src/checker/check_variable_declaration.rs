use super::Checker;
use crate::ast::ast;
use crate::diagnostics::{Diagnostic, TypeError};
use crate::types::Type;

impl Checker<'_> {
  pub fn check_variable_declaration(&mut self, declaration: &ast::VariableDeclaration) -> Result<Type, Diagnostic> {
    let text_name = declaration.name.lexeme();

    if self.ctx.defined_in_current_scope(text_name) && declaration.local {
      let location = declaration.location.clone();
      let diagnostic = TypeError::RedeclaredInSameScope(text_name.to_string(), Some(location));
      return Err(self.create_diagnostic(diagnostic));
    }
    let mut left_type = self.check_t(&declaration.var_type);

    let mut right_type = Type::Unknown;

    if let Some(initializer) = &declaration.initializer {
      right_type = self.check_expression(initializer)?;
    }

    let location = declaration.location.clone();

    if !left_type.check_match(&right_type) {
      let location = if declaration.initializer.is_some() {
        declaration.initializer.as_ref().unwrap().get_location()
      } else {
        declaration.location.clone()
      };

      let diagnostic = TypeError::TypeMismatchAssignment(left_type.to_string(), right_type.to_string(), Some(location));
      return Err(self.create_diagnostic(diagnostic));
    }

    if left_type.can_replace(&right_type) {
      left_type = right_type;
    }

    let location = declaration.location.clone();

    self.ctx.set_variable_location(text_name, location);

    if declaration.local {
      self.ctx.declare_variable(text_name, left_type.clone());
    } else {
      self.ctx.declare_global_variable(text_name, left_type.clone());
    }
    Ok(left_type)
  }
}
