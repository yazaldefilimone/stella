use super::{type_utils::CheckResult, Checker};
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
  // todo: improve this... :(
  pub fn check_index_expression(&mut self, table_expr: &ast::IndexExpression) -> CheckResult<Option<Type>> {
    let base_type = self.check_expression(&table_expr.base)?.unwrap_or(Type::Nil);
    let base_type = self.check_type(base_type)?;
    let base_range = table_expr.base.get_range();
    match base_type {
      Type::Table(ref table_type) => {
        let acc = self.extract_accessor(&table_expr.index)?;
        let tty = self.check_index_access(table_type, acc, table_expr.index.get_range())?;
        Ok(Some(tty))
      }
      _ => Err(self.create_diagnostic(TypeError::ExpectedTable(base_type.to_string(), Some(base_range)))),
    }
  }

  fn check_index_access(&self, table: &TableType, acc: Option<Accessor>, range: Range) -> CheckResult<Type> {
    match acc {
      Some(Accessor::String(name)) => self.check_index_access_string(table, &name, range),
      Some(Accessor::Number(index)) => self.check_index_access_number(table),
      // todo: return union type based on table values
      None => Ok(Type::Unknown),
    }
  }

  fn check_index_access_string(&self, table: &TableType, name: &str, range: Range) -> CheckResult<Type> {
    if let Some(value_type) = table.get_type(name) {
      Ok(value_type.clone())
    } else {
      Err(self.create_diagnostic(TypeError::KeyNotFoundInTable(name.to_string(), table.to_string(), Some(range))))
    }
  }

  fn check_index_access_number(&self, table: &TableType) -> CheckResult<Type> {
    let types = table.array.as_ref().map(|array| array.iter().cloned().collect::<Vec<_>>()).unwrap_or(vec![]);
    if types.is_empty() {
      return Ok(Type::Nil);
    }
    if types.len() == 1 {
      return Ok(types.first().unwrap().clone());
    }
    return Ok(Type::new_union(types));
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
    let expr_type = self.check_expression(key_expr)?.unwrap_or(Type::Nil);
    match expr_type {
      Type::String | Type::Number => Ok(()),
      _ => Err(
        self.create_diagnostic(TypeError::MismatchedAccessorType(expr_type.to_string(), Some(key_expr.get_range()))),
      ),
    }
  }
}
