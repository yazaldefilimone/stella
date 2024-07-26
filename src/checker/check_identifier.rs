use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::diagnostics::TypeError;
use crate::types::Type;

impl Checker {
  pub fn check_identifier(&mut self, ident: &ast::Identifier) -> Result<Type, Diagnostic> {
    let text_name = ident.name.clone();
    let (defined, scope_idx) = self.ctx.defined_in_any_scope(text_name.as_str());
    if !defined {
      let diagnostic = TypeError::UndeclaredVariable(text_name.to_string(), Some(ident.location.clone()));
      return Err(self.create_diagnostic(diagnostic));
    }
    self.ctx.set_unused_variable_in_scope(text_name.as_str(), scope_idx);
    let t = self.ctx.get_variable_in_scope(text_name.as_str(), scope_idx);
    Ok(t.unwrap().clone())
  }
}
