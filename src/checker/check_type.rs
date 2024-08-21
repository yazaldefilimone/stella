use std::collections::HashMap;

use super::{type_utils::CheckResult, Checker};
use crate::types::Type;

type GenericBinds = HashMap<String, Type>;

impl<'a> Checker<'a> {
  pub fn check_option_type<'t>(&mut self, ty: &'t Option<Type>, assume_nil: bool) -> CheckResult<Type> {
    match ty {
      Some(Type::Alias(alias)) => self.check_type_alias(alias),
      Some(Type::Generic(generic)) => self.check_generic_type(generic),
      Some(Type::GenericCall(generic_call)) => self.check_generic_call(generic_call),
      Some(t) => Ok(t.to_owned()),
      None => Ok(if assume_nil { Type::Nil } else { Type::Unknown }),
    }
  }

  pub fn check_type<'t>(&mut self, ty: &'t Type) -> CheckResult<Type> {
    match ty {
      Type::Alias(alias) => self.check_type_alias(&alias),
      Type::Generic(generic) => self.check_generic_type(&generic),
      Type::GenericCall(generic_call) => self.check_generic_call(&generic_call),
      _ => Ok(ty.to_owned()),
    }
  }
}
