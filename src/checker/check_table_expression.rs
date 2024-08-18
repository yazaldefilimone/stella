use std::collections::{BTreeMap, HashSet};

use super::{type_utils::CheckResult, Checker};
use crate::{ast::ast, diagnostics::TypeError, types::Type};

impl<'a> Checker<'a> {
  pub fn check_table_expression(&mut self, table_expr: &ast::TableExpression) -> CheckResult<Option<Type>> {
    let mut array_elements = HashSet::new();
    let mut map_elements = BTreeMap::new();

    for (key_expr, value_expr) in &table_expr.values {
      if let Some(value_expr) = value_expr {
        let value_type = self.check_expression(value_expr)?.unwrap();
        let key_str = self.extract_table_key(key_expr)?;
        map_elements.insert(key_str, value_type);
      } else {
        let array_element_type = self.check_expression(key_expr)?.unwrap();
        array_elements.insert(array_element_type);
      }
    }

    let table_type = Type::new_table(
      if array_elements.is_empty() { None } else { Some(array_elements) },
      if map_elements.is_empty() { None } else { Some(map_elements) },
    );

    Ok(Some(table_type))
  }

  fn extract_table_key(&mut self, key_expr: &ast::Expression) -> CheckResult<String> {
    match key_expr {
      ast::Expression::Identifier(identifier) => Ok(identifier.name.clone()),
      ast::Expression::Literal(literal) => match literal {
        ast::LiteralExpression::String(string) => Ok(string.value.clone()),
        _ => self.create_invalid_literal_key_error(key_expr),
      },
      _ => self.create_invalid_literal_key_error(key_expr),
    }
  }

  fn create_invalid_literal_key_error(&mut self, key_expr: &ast::Expression) -> CheckResult<String> {
    let range = key_expr.get_range();
    let expr_type = self.check_expression(key_expr)?.unwrap();
    let diagnostic = TypeError::MismatchedKeyType(expr_type.to_string(), Some(range));
    Err(self.create_diagnostic(diagnostic))
  }
}
