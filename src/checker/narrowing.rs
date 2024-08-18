use super::{type_utils::CheckResult, Checker};
use crate::{
  ast::ast::{BinaryOperator, Expression, LiteralExpression},
  types::Type,
};
impl<'a> Checker<'a> {
  pub fn is_condition_narrowing(&self, condition: &Expression) -> bool {
    if let Expression::Binary(binary_expr) = condition {
      match binary_expr.operator {
        BinaryOperator::Equal => {
          return self.is_literal_narrowing(&*binary_expr.left);
        }
        _ => {}
      }
    }
    false
  }

  pub fn get_specified_type(&mut self, condition: &Expression) -> CheckResult<(Option<String>, Option<Type>)> {
    match condition {
      Expression::Binary(binary_expr) => match binary_expr.operator {
        BinaryOperator::Equal => {
          if let Some(string) = self.get_identifier_string(&*binary_expr.left) {
            let right_type = self.check_expression(&*binary_expr.right)?;
            return Ok((Some(string), right_type));
          }
        }
        _ => {}
      },
      Expression::Call(call_expr) => match &*call_expr.left {
        Expression::Identifier(identifier) => {
          let name = identifier.name.as_str();
          return Ok((Some(name.to_string()), None));
        }
        _ => {}
      },
      _ => {}
    }
    Ok((None, None))
  }

  fn check_narrowing(&mut self, expr: &Expression) -> CheckResult<Option<Type>> {
    match expr {
      Expression::Binary(binary_expr) => {
        match binary_expr.operator {
          BinaryOperator::Equal => {
            let left_type = self.check_expression(&*binary_expr.left)?.unwrap_or(Type::Nil);
            let left_type = self.check_type(left_type)?;
            let right_type = self.check_expression(&*binary_expr.right)?.unwrap_or(Type::Nil);
            let right_type = self.check_type(right_type)?;
            match left_type {
              Type::Union(union) => {
                for left_type in union.types {
                  if left_type.check_match(&right_type) {
                    return Ok(Some(left_type));
                  }
                }
              }
              Type::Option(option) => {
                return if right_type.is_nil() {
                  Ok(Some(Type::Nil))
                } else {
                  return Ok(Some(*option.inner_type));
                }
              }
              _ => {
                return Ok(Some(right_type));
              }
            }
          }
          _ => {}
        }
        Ok(None)
      }
      _ => Ok(None),
    }
  }

  fn is_literal_narrowing(&self, expr: &Expression) -> bool {
    match expr {
      Expression::Call(call_expr) => match &*call_expr.left {
        Expression::Identifier(identifier) => identifier.name.as_str() == "type",
        _ => false,
      },
      Expression::Identifier(identifier) => {
        return true;
      }
      Expression::Literal(literal) => matches!(literal, LiteralExpression::Nil(_)),
      _ => false,
    }
  }

  fn get_identifier_string(&self, expr: &Expression) -> Option<String> {
    if let Expression::Identifier(identifier) = expr {
      return Some(identifier.name.clone());
    }
    None
  }
}
