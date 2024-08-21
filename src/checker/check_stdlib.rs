use super::type_utils::CheckResult;
use super::Checker;
use crate::diagnostics::TypeError;
use crate::types::{GenericCallType, Type};

impl<'a> Checker<'a> {
  pub fn check_stdlib_type<'t>(&mut self, generic_call: &'t GenericCallType) -> CheckResult<Option<Type>> {
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
  pub fn check_option_stdlib_type<'t>(&mut self, call: &'t GenericCallType) -> CheckResult<Option<Type>> {
    if call.types.len() != 1 {
      let range = call.range.clone();
      let found = call.types.len();
      let diagnostic = TypeError::OptionCallArityMismatch(found, Some(range));
      return Err(self.create_diagnostic(diagnostic));
    }
    let infer_type = self.check_type(call.types.first().unwrap())?;

    let option_type = Type::new_option(infer_type.clone());

    Ok(Some(option_type))
  }

  pub fn check_union_stdlib_type<'t>(&mut self, call: &'t GenericCallType) -> CheckResult<Option<Type>> {
    let types = call.types.iter().map(|ty| Ok(self.check_type(ty)?.to_owned())).collect::<CheckResult<Vec<Type>>>()?;

    let union_type = Type::new_union(types);

    Ok(Some(union_type))
  }
}
