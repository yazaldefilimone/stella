use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_variable_declaration(&mut self, declaration: &ast::VariableDeclaration) -> Result<Type, Diagnostic> {
    let right_type = if let Some(initializer) = &declaration.initializer {
      self.check_expression(initializer)?
    } else {
      Type::Nil
    };
    self.declare_variables(&declaration.values, right_type, declaration.local, declaration.get_range())?;
    Ok(Type::Nil)
  }
}
