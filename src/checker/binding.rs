use crate::{types::FunctionType, utils::range::Range};

use super::{type_utils::CheckResult, Checker};

type FnType = FunctionType;

impl<'a> Checker<'a> {
  pub fn bind_function(&mut self, expected: &FnType, found: &FnType, rg: &Range) -> CheckResult<()> {
    self.validate_function_arity(expected, found, rg)?;
    self.validate_function_parameters(expected, found, rg)?;
    self.validate_function_return_type(expected, found, rg)
  }

  fn validate_function_arity(&mut self, expected: &FnType, found: &FnType, rg: &Range) -> CheckResult<()> {
    if expected.params.len() != found.params.len() {
      return Err(self.create_function_arity_mismatch(expected.params.len(), found.params.len(), rg.clone()));
    }
    Ok(())
  }

  fn validate_function_parameters(&mut self, expected: &FnType, found: &FnType, rg: &Range) -> CheckResult<()> {
    for (expected_param, found_param) in expected.params.iter().zip(found.params.iter()) {
      if !expected_param.check_match(found_param) {
        return Err(self.create_type_mismatch(expected_param.to_owned(), found_param.to_owned(), rg.clone()));
      }
    }
    Ok(())
  }

  fn validate_function_return_type(&mut self, expected: &FnType, found: &FnType, rg: &Range) -> CheckResult<()> {
    let expected_return_type = *expected.return_type.clone();
    let found_return_type = *found.return_type.clone();

    if !expected_return_type.check_match(&found_return_type) {
      return Err(self.create_type_mismatch(expected_return_type, found_return_type, rg.clone()));
    }
    Ok(())
  }
}
