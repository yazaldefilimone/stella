use super::Checker;
use crate::ast::ast;
use crate::diagnostics::{Diagnostic, TypeError};
use crate::types::Type;

impl Checker {
  pub fn check_assign_statement(&mut self, assign: &ast::AssignStatement) -> Result<Type, Diagnostic> {
    let right_t = self.check_expression_statement(&assign.value).unwrap();
    let lexema = assign.name.lexeme();
    if !self.ctx.is_defined(lexema.as_str()) {
      return Err(self.create_diagnostic(TypeError::UndeclaredVariable(
        lexema.to_string(),
        // TODO: hei :), use name location or a value location?
        Some(assign.name.location.clone()),
      )));
    }
    let left_t = self.ctx.get_variable(lexema.as_str()).unwrap().clone();
    if left_t.check_match(&right_t) {
      let location = Some(assign.location.clone());
      let diagnostic = TypeError::TypeMismatchAssignment(left_t.to_string(), right_t.to_string(), location);
      return Err(self.create_diagnostic(diagnostic));
    }
    if right_t.check_is_can_replace(&left_t) {
      self.ctx.declare_variable(lexema.as_str(), right_t.clone());
      return Ok(right_t);
    }
    Ok(left_t)
  }
}
