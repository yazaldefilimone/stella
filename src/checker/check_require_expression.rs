use std::path::PathBuf;

use super::{type_utils::CheckResult, Checker};
use crate::{ast::ast, diagnostics::TypeError, parser::parser::Parser, types::Type, utils::range::Range};

impl<'a> Checker<'a> {
  pub fn check_require_expression(&mut self, require: &ast::RequireExpression) -> CheckResult<Option<Type>> {
    let name = require.module_name.lexeme();
    self.check_module(name, require.range.clone())
  }

  pub fn check_module(&mut self, name: &str, range: Range) -> CheckResult<Option<Type>> {
    let path = self.resolve_path(name, &range)?;
    let content = self.load_module(&path, &range)?;
    let return_module_type = self.analyze_module(name, &path, &content, range)?;
    Ok(return_module_type)
  }

  fn resolve_path(&mut self, name: &str, range: &Range) -> CheckResult<std::path::PathBuf> {
    self.resolver.resolve(name).map_err(|_| {
      let diagnostic = TypeError::ModuleNotFound(name.to_string(), Some(range.clone()));
      self.create_diagnostic(diagnostic)
    })
  }

  fn load_module(&mut self, path: &PathBuf, range: &Range) -> CheckResult<String> {
    self.loader.load_module_from_path(path).map_err(|_| {
      let diagnostic = TypeError::ModuleNotFound(path.to_string_lossy().to_string(), Some(range.clone()));
      self.create_diagnostic(diagnostic)
    })
  }

  fn analyze_module(&mut self, name: &str, path: &PathBuf, content: &str, range: Range) -> CheckResult<Option<Type>> {
    let path_name = path.to_str().unwrap();

    let mut parser = Parser::new(content, path_name);

    let program = parser.parse_program();

    let mut checker = Checker::new(path_name, content);

    let _ = checker.check(&program)?;

    let return_module_type = checker.ctx.get_last_return().ok_or_else(|| {
      let diagnostic = TypeError::ModuleNotExported(name.to_string(), Some(range.clone()));
      self.create_diagnostic(diagnostic)
    })?;

    Ok(Some(return_module_type.clone()))
  }
}
