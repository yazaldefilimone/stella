use crate::{
  diagnostics::{Diagnostic, TypeWarning},
  types::Type,
  utils::range::Range,
};

use super::Checker;

impl<'a> Checker<'a> {
  pub fn check_shadowing(&mut self, lexeme: &str, local: bool, rg: &Range) -> Result<(), Diagnostic> {
    if local {
      self.check_local_shadowing(lexeme, rg)
    } else {
      self.check_global_redeclaration(lexeme, rg)
    }
  }

  fn check_local_shadowing(&mut self, lexeme: &str, rg: &Range) -> Result<(), Diagnostic> {
    if self.ctx.defined_in_current_scope(lexeme) {
      if self.ctx.is_local_declaration(lexeme) {
        return Err(self.create_redeclaration(lexeme, rg.clone()));
      } else {
        let diagnostic = TypeWarning::ShadowedVariable(lexeme.to_string(), Some(rg.clone()));
        self.diagnostics.add(diagnostic.into());
      }
    }
    Ok(())
  }

  fn check_global_redeclaration(&mut self, lexeme: &str, rg: &Range) -> Result<(), Diagnostic> {
    if let Some(previous_type) = self.ctx.get_variable(lexeme, Some(0)) {
      let current_type = self.ctx.get_variable(lexeme, None).unwrap_or_else(|| &Type::Unknown);
      if !previous_type.check_match(&current_type) {
        return Err(self.create_type_mismatch(previous_type.to_owned(), current_type.to_owned(), rg.clone()));
      }
    }
    Ok(())
  }
}
