use std::collections::HashMap;

use super::Checker;
use crate::{diagnostics::Diagnostic, types::Type};

type GenericBinds = HashMap<String, Type>;

pub type CheckResult<T> = std::result::Result<T, Diagnostic>;

impl<'a> Checker<'a> {
  pub fn create_generic_table_type(&self, generics: &[String], inferred: &[Type]) -> GenericBinds {
    generics.iter().cloned().zip(inferred.iter().cloned()).collect()
  }
  pub fn create_generic_table(&self, types: &[Type], variables: &[String]) -> GenericBinds {
    variables.iter().cloned().zip(types.iter().cloned()).collect()
  }

  pub fn create_type_based_array(&self, types: Vec<Type>) -> Option<Type> {
    if types.len() == 0 {
      return None;
    }
    if types.len() == 1 {
      return Some(types.first().unwrap().clone());
    }

    if types.len() == 2 && types[0].is_nil() || types[1].is_nil() {
      let inner_type = types.iter().filter(|t| !t.is_nil()).next().unwrap().clone();
      return Some(Type::new_option(inner_type));
    }
    return Some(Type::new_union(types));
  }
}
