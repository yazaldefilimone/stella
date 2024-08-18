use super::type_utils::CheckResult;
use super::Checker;
use crate::diagnostics::{Diagnostic, TypeError};
use crate::types::{GenericCallType, Type};

impl<'a> Checker<'a> {
  pub fn check_stdlib_type(&mut self, generic_call: &GenericCallType) -> CheckResult<Option<Type>> {
    if generic_call.name.as_str() == "option" {
      let option_type = self.check_option_stdlib_type(&generic_call)?;
      return Ok(option_type);
    }

    if generic_call.name.as_str() == "union" {
      let union_type = self.check_union_stdlib_type(&generic_call)?;
      return Ok(union_type);
    }

    Ok(None)
  }
  pub fn check_option_stdlib_type(&mut self, call: &GenericCallType) -> CheckResult<Option<Type>> {
    if call.types.len() != 1 {
      let range = call.range.clone();
      let found = call.types.len();
      let diagnostic = TypeError::OptionCallArityMismatch(found, Some(range));
      return Err(self.create_diagnostic(diagnostic));
    }
    let infer_type = self.check_type(call.types.first().unwrap().clone())?;
    let option_type = Type::new_option(infer_type);
    Ok(Some(option_type))
  }

  pub fn check_union_stdlib_type(&mut self, call: &GenericCallType) -> CheckResult<Option<Type>> {
    let types = call.types.iter().map(|ty| self.check_type(ty.clone())).collect::<Result<Vec<Type>, Diagnostic>>()?;
    let union_type = Type::new_union(types);
    Ok(Some(union_type))
  }
}
