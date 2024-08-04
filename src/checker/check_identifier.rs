use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::diagnostics::TypeError;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_identifier(&mut self, ident: &ast::Identifier) -> Result<Type, Diagnostic> {
    let (defined, scope_idx) = self.ctx.defined_in_any_scope(&ident.name);
    if !defined {
      let diagnostic = TypeError::UndeclaredVariable(ident.name.to_string(), Some(ident.location.clone()));
      return Err(self.create_diagnostic(diagnostic));
    }
    self.ctx.use_variable_in_scope(&ident.name, scope_idx);
    let t = self.ctx.get_variable_in_scope(&ident.name, scope_idx);
    Ok(t.unwrap().clone())
  }
}
