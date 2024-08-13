use std::collections::HashMap;

use super::Checker;
use crate::types::Type;

type GenericBinds = HashMap<String, Type>;

impl<'a> Checker<'a> {
  pub fn create_generic_table_type(&self, generics: &[String], inferred: &[Type]) -> GenericBinds {
    generics.iter().cloned().zip(inferred.iter().cloned()).collect()
  }
  pub fn create_generic_table(&self, types: &[Type], variables: &[String]) -> GenericBinds {
    variables.iter().cloned().zip(types.iter().cloned()).collect()
  }
}
