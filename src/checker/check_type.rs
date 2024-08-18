use std::collections::HashMap;

use super::{type_utils::CheckResult, Checker};
use crate::types::Type;

type GenericBinds = HashMap<String, Type>;

impl<'a> Checker<'a> {
  pub fn check_option_type(&mut self, ty: &Option<Type>, assume_nil: bool) -> CheckResult<Type> {
    match ty {
      Some(Type::Identifier(identifier)) => self.check_type_identifier(identifier),
      Some(Type::Generic(generic)) => self.check_generic_type(generic),
      Some(Type::GenericCall(generic_call)) => self.check_generic_call(generic_call),
      Some(t) => Ok(t.clone()),
      None => Ok(if assume_nil { Type::Nil } else { Type::Unknown }),
    }
  }

  pub fn check_type(&mut self, ty: Type) -> CheckResult<Type> {
    match ty {
      Type::Identifier(identifier) => self.check_type_identifier(&identifier),
      Type::Generic(generic) => self.check_generic_type(&generic),
      Type::GenericCall(generic_call) => self.check_generic_call(&generic_call),
      _ => Ok(ty),
    }
  }
}
