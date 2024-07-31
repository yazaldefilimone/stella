use super::Checker;
use crate::ast::ast;
use crate::diagnostics::{Diagnostic, TypeError};
use crate::parser::parser::Parser;
use crate::types::Type;
use crate::utils::location::Location;

impl<'a> Checker<'a> {
  pub fn check_require_expression(&mut self, require: &ast::RequireExpression) -> Result<Type, Diagnostic> {
    let module_name = require.module_name.lexeme();
    let export_type = self.check_module(&module_name, require.location.clone())?;
    return Ok(export_type);
  }

  pub fn check_module(&mut self, module_name: &str, location: Location) -> Result<Type, Diagnostic> {
    let module_path = self.resolver.resolve(module_name);
    if module_path.is_err() {
      let diagnostic = TypeError::ModuleNotFound(module_name.to_string(), Some(location));
      return Err(self.create_diagnostic(diagnostic));
    }

    let module_path = module_path.unwrap();
    let path_name = module_path.to_str().unwrap();
    let content = self.loader.load_module_from_path(&module_path);

    if content.is_err() {
      let diagnostic = TypeError::ModuleNotFound(module_name.to_string(), Some(location));
      return Err(self.create_diagnostic(diagnostic));
    }

    let content = content.unwrap();
    let mut parser = Parser::new(&content, path_name);
    let program = parser.parse_program();
    let mut checker = Checker::new(path_name, &content);
    let _ = checker.check(&program)?;
    // let exports = checker.ctx.get_exports();
    // todo: improve this
    let return_module_type = checker.ctx.get_last_return();
    if return_module_type.is_none() {
      let diagnostic = TypeError::ModuleNotExported(module_name.to_string(), Some(location));
      return Err(self.create_diagnostic(diagnostic));
    }
    let return_module_type = return_module_type.unwrap();
    // println!("module:{} exports:{:#?}", module_name, exports);
    // let first_export = *exports.first().unwrap();
    return Ok(return_module_type.clone());
  }
}
