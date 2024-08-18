use super::{type_utils::CheckResult, Checker};
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::{TableType, Type},
  utils::range::Range,
};

impl<'a> Checker<'a> {
  // todo: improve this... :(
  pub fn check_member_expression(&mut self, member: &ast::MemberExpression) -> CheckResult<Option<Type>> {
    let base_type = self.check_expression(&member.base)?.unwrap();
    let base_type = self.check_type(base_type)?;
    let base_range = member.base.get_range();
    match base_type {
      Type::Table(ref table_type) => self.check_identifier_member(table_type, &member.identifier),
      _ => Err(self.create_diagnostic(TypeError::ExpectedTable(base_type.to_string(), Some(base_range)))),
    }
  }
  fn check_identifier_member(&mut self, table: &TableType, identifier: &ast::Identifier) -> CheckResult<Option<Type>> {
    let name = identifier.name.as_str();
    if let Some(member_type) = table.get_type(name) {
      Ok(Some(member_type.clone()))
    } else {
      Err(self.create_not_found_key_error(name, table, identifier.range.clone()))
    }
  }

  // fn create_member_error(&mut self, member: &ast::Expression) -> Diagnostic {
  //   let range = member.get_range();
  //   match self.check_expression(member) {
  //     Ok(member_type) => {
  //       let diagnostic = TypeError::MismatchedKeyType(member_type.unwrap_or(Type::Nil).to_string(), Some(range));
  //       self.create_diagnostic(diagnostic)
  //     }
  //     Err(diagnostic) => diagnostic,
  //   }
  // }

  fn create_not_found_key_error(&self, name: &str, table: &TableType, range: Range) -> Diagnostic {
    self.create_diagnostic(TypeError::KeyNotFoundInTable(name.to_string(), table.to_string(), Some(range)))
  }

  // fn check_call_member(&mut self, call: &ast::CallExpression, table: &TableType) -> CheckResult<Option<Type>> {
  //   let name = call.name.lexeme();
  //   if let Some(member_type) = table.get_type(name) {
  //     self.check_call_type(member_type, &call.args, call.name.range.clone())
  //   } else {
  //     Err(self.create_not_found_key_error(name, table, call.get_range()))
  //   }
  // }
}
