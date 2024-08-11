use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::{TableType, Type},
  utils::range::Range,
};

impl<'a> Checker<'a> {
  pub fn check_member_expression(&mut self, member: &ast::MemberExpression) -> Result<Type, Diagnostic> {
    let base_type = self.check_expression(&member.base)?;
    let base_range = member.base.get_range();

    match base_type {
      Type::Table(ref table_type) => self.check_member(table_type, &member.member),
      _ => Err(self.create_diagnostic(TypeError::ExpectedTable(base_type.to_string(), Some(base_range)))),
    }
  }

  fn check_member(&mut self, table: &TableType, member: &ast::Expression) -> Result<Type, Diagnostic> {
    match member {
      ast::Expression::Literal(literal) => {
        if let ast::LiteralExpression::String(string) = literal {
          self.check_identifier_member(&string.value, table, string.range.clone())
        } else {
          Err(self.create_member_error(member))
        }
      }
      ast::Expression::Call(call) => self.check_call_member(call, table),
      ast::Expression::Identifier(identifier) => {
        self.check_identifier_member(&identifier.name, table, identifier.range.clone())
      }
      _ => Err(self.create_member_error(member)),
    }
  }

  fn check_identifier_member(&mut self, name: &str, table: &TableType, range: Range) -> Result<Type, Diagnostic> {
    if let Some(member_type) = table.get_type(name) {
      Ok(member_type.clone())
    } else {
      Err(self.create_not_found_key_error(name, table, range))
    }
  }

  fn create_member_error(&mut self, member: &ast::Expression) -> Diagnostic {
    let range = member.get_range();
    match self.check_expression(member) {
      Ok(member_type) => {
        let diagnostic = TypeError::MismatchedKeyType(member_type.to_string(), Some(range));
        self.create_diagnostic(diagnostic)
      }
      Err(diagnostic) => diagnostic,
    }
  }

  fn create_not_found_key_error(&self, name: &str, table: &TableType, range: Range) -> Diagnostic {
    self.create_diagnostic(TypeError::KeyNotFoundInTable(name.to_string(), table.to_string(), Some(range)))
  }

  fn check_call_member(&mut self, call: &ast::CallExpression, table: &TableType) -> Result<Type, Diagnostic> {
    let name = call.name.lexeme();
    if let Some(member_type) = table.get_type(name) {
      self.check_call_type(member_type, &call.args, call.name.range.clone())
    } else {
      Err(self.create_not_found_key_error(name, table, call.get_range()))
    }
  }
}
