use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_variable_declaration(&mut self, declaration: &ast::VariableDeclaration) -> Result<Type, Diagnostic> {
    let mut right_type = Type::Nil;

    if let Some(initializer) = &declaration.initializer {
      right_type = self.check_expression(initializer)?;
    }

    self._declaration(&declaration.values, right_type, declaration.local, declaration.get_location())?;

    let location = declaration.get_location();

    let location = declaration.get_location();

    Ok(Type::Nil)
  }
}
