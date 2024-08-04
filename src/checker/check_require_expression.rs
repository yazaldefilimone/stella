use std::path::PathBuf;

use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  parser::parser::Parser,
  types::Type,
  utils::location::Location,
};

impl<'a> Checker<'a> {
  pub fn check_require_expression(&mut self, require: &ast::RequireExpression) -> Result<Type, Diagnostic> {
    let module_name = require.module_name.lexeme();
    self.check_module(module_name, require.location.clone())
  }

  pub fn check_module(&mut self, module_name: &str, location: Location) -> Result<Type, Diagnostic> {
    let module_path = self.resolve_module_path(module_name, &location)?;
    let content = self.load_module_content(&module_path, &location)?;
    let return_module_type = self.analyze_module_content(module_name, &module_path, &content, location)?;
    Ok(return_module_type)
  }

  fn resolve_module_path(&mut self, module_name: &str, location: &Location) -> Result<std::path::PathBuf, Diagnostic> {
    self.resolver.resolve(module_name).map_err(|_| {
      let diagnostic = TypeError::ModuleNotFound(module_name.to_string(), Some(location.clone()));
      self.create_diagnostic(diagnostic)
    })
  }

  fn load_module_content(&mut self, module_path: &PathBuf, location: &Location) -> Result<String, Diagnostic> {
    self.loader.load_module_from_path(module_path).map_err(|_| {
      let diagnostic = TypeError::ModuleNotFound(module_path.to_string_lossy().to_string(), Some(location.clone()));
      self.create_diagnostic(diagnostic)
    })
  }

  fn analyze_module_content(
    &mut self,
    module_name: &str,
    module_path: &PathBuf,
    content: &str,
    location: Location,
  ) -> Result<Type, Diagnostic> {
    let path_name = module_path.to_str().unwrap();

    let mut parser = Parser::new(content, path_name);

    let program = parser.parse_program();

    let mut checker = Checker::new(path_name, content);

    let _ = checker.check(&program)?;

    let return_module_type = checker.ctx.get_last_return().ok_or_else(|| {
      let diagnostic = TypeError::ModuleNotExported(module_name.to_string(), Some(location.clone()));
      self.create_diagnostic(diagnostic)
    })?;

    Ok(return_module_type.clone())
  }
}
