use crate::diagnostics::TypeWarning;

use super::Checker;

impl<'a> Checker<'a> {
  pub fn check_unused_variables(&mut self) {
    let used_variables = self.ctx.check_unused_variables();
    // println!("{:#?}", used_variables);
    for used_variable in used_variables {
      let used_variable_range = self.ctx.get_variable_range(&used_variable);
      if used_variable_range.is_none() {
        continue;
      }
      let report = TypeWarning::UnusedVariable(used_variable.clone(), used_variable_range);
      self.diagnostics.add(report.into());
    }
  }
}
