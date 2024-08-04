use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_type_declaration(&mut self, declaration: &ast::TypeDeclaration) -> Result<Type, Diagnostic> {
    if declaration.generis.len() > 0 {
      let init = declaration.initiizer.clone();
      let generics = declaration.generis.clone();
      let new_generics = Type::new_generic(declaration.name.lexeme(), generics, init, declaration.location.clone());
      self.ctx.declare_type(&declaration.name.lexeme(), new_generics);
      return Ok(Type::Nil);
    }
    self.ctx.declare_type(&declaration.name.lexeme(), declaration.initiizer.clone());
    Ok(Type::Nil)
  }
}
