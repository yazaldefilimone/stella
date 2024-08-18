use super::type_utils::CheckResult;
use super::Checker;
use crate::ast::ast;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_type_declaration(&mut self, declaration: &ast::TypeDeclaration) -> CheckResult<Option<Type>> {
    if declaration.generis.len() > 0 {
      let init = declaration.initiizer.clone();
      let generics = declaration.generis.clone();
      let new_generics = Type::new_generic(declaration.name.lexeme(), generics, init, declaration.get_range());
      self.ctx.declare_type(&declaration.name.lexeme(), new_generics);
      return Ok(None);
    }
    self.ctx.declare_type(&declaration.name.lexeme(), declaration.initiizer.clone());
    Ok(None)
  }
}
