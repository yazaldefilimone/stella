use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::diagnostics::TypeError;
use crate::types;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_identifier(&mut self, ident: &ast::Identifier) -> Result<Type, Diagnostic> {
    let (defined, scope_pointer) = self.ctx.defined_in_any_scope(&ident.name);
    if !defined {
      let diagnostic = TypeError::UndeclaredVariable(ident.name.to_string(), Some(ident.range.clone()));
      return Err(self.create_diagnostic(diagnostic));
    }
    self.ctx.use_variable(&ident.name, Some(scope_pointer));
    let tyy = self.ctx.get_variable(&ident.name, Some(scope_pointer)).unwrap();
    Ok(tyy.clone())
  }
  pub fn check_type_identifier(&mut self, ident: &types::IdentifierType) -> Result<types::Type, Diagnostic> {
    self.ctx.get_type(ident.name.as_str()).cloned().ok_or_else(|| {
      self.create_diagnostic(TypeError::UndeclaredVariable(ident.name.to_string(), Some(ident.range.clone())))
    })
  }
}
