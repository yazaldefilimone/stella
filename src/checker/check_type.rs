use super::Checker;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_t(&mut self, t: &Option<Type>) -> Type {
    match t {
      Some(t) => t.clone(),
      None => Type::Unknown,
    }
  }
}
