use super::Checker;
use crate::diagnostics::{Diagnostic, TypeError};
use crate::types::{GenericCallType, Type};

impl<'a> Checker<'a> {
  pub fn check_stdlib_type(&mut self, generic_call: &GenericCallType) -> Result<Option<Type>, Diagnostic> {
    if generic_call.name.as_str() == "optional" {
      let optional_type = self.check_optional_stdlib_type(&generic_call)?;
      return Ok(Some(optional_type));
    }

    if generic_call.name.as_str() == "union" {
      let union_type = self.check_union_stdlib_type(&generic_call)?;
      return Ok(Some(union_type));
    }

    Ok(None)
  }
  pub fn check_optional_stdlib_type(&mut self, call: &GenericCallType) -> Result<Type, Diagnostic> {
    if call.types.len() != 1 {
      let range = call.range.clone();
      let found = call.types.len();
      let diagnostic = TypeError::OptionalCallArityMismatch(found, Some(range));
      return Err(self.create_diagnostic(diagnostic));
    }
    let infer_type = self.check_type(call.types.first().unwrap().clone())?;
    let optional_type = Type::new_optional(infer_type);
    Ok(optional_type)
  }

  pub fn check_union_stdlib_type(&mut self, call: &GenericCallType) -> Result<Type, Diagnostic> {
    let types = call.types.iter().map(|ty| self.check_type(ty.clone())).collect::<Result<Vec<Type>, Diagnostic>>()?;
    let union_type = Type::new_union(types);
    Ok(union_type)
  }
}
