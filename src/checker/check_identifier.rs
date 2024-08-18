use super::type_utils::CheckResult;
use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::diagnostics::TypeError;
use crate::types;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_identifier(&mut self, ident: &ast::Identifier) -> CheckResult<Option<Type>> {
    let (defined, scope_pointer) = self.ctx.defined_in_any_scope(&ident.name);
    if !defined {
      let diagnostic = TypeError::UndeclaredVariable(ident.name.to_string(), Some(ident.range.clone()));
      // I think we should return unknown and save the error in diagnostics... is it ok? :(
      // return Err(self.create_diagnostic(diagnostic));
      self.diagnostics.add(self.create_diagnostic(diagnostic));
      return Ok(Some(Type::Unknown));
    }
    self.ctx.use_variable(&ident.name, Some(scope_pointer));
    let tyy = self.ctx.get_variable(&ident.name, Some(scope_pointer)).unwrap();
    Ok(Some(self.check_type(tyy.clone())?))
  }
  pub fn check_type_identifier(&mut self, ident: &types::IdentifierType) -> Result<types::Type, Diagnostic> {
    let tty = self.ctx.get_type(ident.name.as_str()).cloned().ok_or_else(|| {
      self.create_diagnostic(TypeError::UndeclaredType(ident.name.to_string(), Some(ident.range.clone())))
    })?;
    return Ok(self.check_type(tty)?);
  }
}
