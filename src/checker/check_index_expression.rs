use super::Checker;
use crate::{
  ast::ast,
  diagnostics::{Diagnostic, TypeError},
  types::{TableType, Type},
  utils::range::Range,
};

pub enum Accessor {
  String(String),
  Number(usize),
}

impl<'a> Checker<'a> {
  pub fn check_index_expression(&mut self, table_expr: &ast::IndexExpression) -> Result<Type, Diagnostic> {
    let base_type = self.check_expression(&table_expr.base)?;
    let base_range = table_expr.base.get_range();

    match base_type {
      Type::Table(ref table_type) => {
        let acc = self.extract_accessor(&table_expr.index)?;
        self.check_index_access(table_type, acc, table_expr.index.get_range())
      }
      _ => Err(self.create_diagnostic(TypeError::ExpectedTable(base_type.to_string(), Some(base_range)))),
    }
  }

  fn check_index_access(&self, table: &TableType, acc: Option<Accessor>, range: Range) -> Result<Type, Diagnostic> {
    match acc {
      Some(Accessor::String(name)) => self.check_index_access_string(table, &name, range),
      Some(Accessor::Number(index)) => self.check_index_access_number(table, index, range),
      // todo: return union type based on table values
      None => Ok(Type::Unknown),
    }
  }

  fn check_index_access_string(&self, table: &TableType, name: &str, range: Range) -> Result<Type, Diagnostic> {
    if let Some(value_type) = table.get_type(name) {
      Ok(value_type.clone())
    } else {
      Err(self.create_diagnostic(TypeError::KeyNotFoundInTable(name.to_string(), table.to_string(), Some(range))))
    }
  }

  fn check_index_access_number(&self, table: &TableType, index: usize, range: Range) -> Result<Type, Diagnostic> {
    if let Some(element_type) = table.get_array_type(index) {
      Ok(element_type.clone())
    } else {
      Err(self.create_diagnostic(TypeError::KeyNotFoundInTable(index.to_string(), table.to_string(), Some(range))))
    }
  }

  fn extract_accessor(&mut self, index_expr: &ast::Expression) -> Result<Option<Accessor>, Diagnostic> {
    match index_expr {
      ast::Expression::Literal(literal) => match literal {
        ast::LiteralExpression::String(string) => Ok(Some(Accessor::String(string.value.clone()))),
        ast::LiteralExpression::Number(number) => {
          let number = number.value.parse::<usize>();
          // todo: create diagnostic ??
          return if number.is_err() { Ok(None) } else { Ok(Some(Accessor::Number(number.unwrap()))) };
        }
        _ => self.handle_non_literal_index(index_expr),
      },
      _ => self.handle_non_literal_index(index_expr),
    }
  }

  fn handle_non_literal_index(&mut self, index_expr: &ast::Expression) -> Result<Option<Accessor>, Diagnostic> {
    self.check_expression_index(index_expr)?;
    Ok(None)
  }

  fn check_expression_index(&mut self, key_expr: &ast::Expression) -> Result<(), Diagnostic> {
    let expr_type = self.check_expression(key_expr)?;
    match expr_type {
      Type::String | Type::Number => Ok(()),
      _ => Err(
        self.create_diagnostic(TypeError::MismatchedAccessorType(expr_type.to_string(), Some(key_expr.get_range()))),
      ),
    }
  }
}
