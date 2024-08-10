use super::Checker;
use crate::ast::tokens::Token;
use crate::diagnostics::{Diagnostic, TypeWarning};
use crate::types::{FunctionType, Type};
use crate::utils::range::Range;

type NameType<'a> = &'a Vec<(Token, Option<Type>)>;

impl<'a> Checker<'a> {
  pub fn declare_variables(&mut self, names: NameType, ty: Type, local: bool, range: Range) -> Result<(), Diagnostic> {
    match ty {
      Type::Grup(group) => {
        for (index, token) in names.iter().enumerate() {
          let assigned_type = group.types.get(index).cloned().unwrap_or(Type::Nil);
          self.declare_variable(token, assigned_type, local, &range)?;
        }
      }
      _ => {
        for (index, token) in names.iter().enumerate() {
          let assigned_type = if index == 0 { self.check_type(ty.clone())? } else { Type::Nil };
          self.declare_variable(token, assigned_type, local, &range)?;
        }
      }
    }
    Ok(())
  }

  pub fn declare_variable(
    &mut self,
    value: &(Token, Option<Type>),
    ty: Type,
    local: bool,
    range: &Range,
  ) -> Result<(), Diagnostic> {
    let lexeme = value.0.lexeme();
    let mut checked_type = self.check_type(ty)?;
    if let Some(value_type) = &value.1 {
      let checked_value_type = self.check_type(value_type.to_owned())?;
      if !checked_value_type.check_match(&checked_type) {
        return Err(self.create_type_mismatch(checked_value_type, checked_type, value.0.range.clone()));
      }
      // if ty is a function type, infer parameters and return type if not provided
      match (&checked_type, &checked_value_type) {
        (Type::Function(expected), Type::Function(found)) => {
          self.bind_function(expected, found, range)?;
          checked_type = Type::Function(found.clone());
        }
        _ => {}
      }
    }

    self.emmit_shadowing_and_redeclaration(lexeme, local, &value.0.range)?;

    self.ctx.declare_variable(lexeme, checked_type);
    if local {
      self.ctx.set_local_variable(lexeme);
    } else {
    }
    self.ctx.set_variable_range(lexeme, value.0.range.clone());
    Ok(())
  }

  pub fn emmit_shadowing_and_redeclaration(
    &mut self,
    lexeme: &str,
    local: bool,
    range: &Range,
  ) -> Result<(), Diagnostic> {
    if local {
      if self.ctx.defined_in_current_scope(lexeme) {
        if self.ctx.lookup_local_variable(lexeme) {
          return Err(self.create_redeclaration(lexeme, range.clone()));
        } else {
          let diagnostic = TypeWarning::ShadowedVariable(lexeme.to_string(), Some(range.clone()));
          self.diagnostics.add(diagnostic.into());
        }
      }
    } else if let Some(previous_type) = self.ctx.get_variable_in_global_scope(lexeme) {
      let current_type = self.ctx.get_variable(lexeme).unwrap_or_else(|| &Type::Unknown);
      if !previous_type.check_match(&current_type) {
        return Err(self.create_type_mismatch(previous_type.to_owned(), current_type.to_owned(), range.clone()));
      }
    }
    Ok(())
  }

  pub fn bind_function(
    &mut self,
    expected: &FunctionType,
    found: &FunctionType,
    range: &Range,
  ) -> Result<(), Diagnostic> {
    if expected.params.len() != found.params.len() {
      return Err(self.create_function_arity_mismatch(expected.params.len(), found.params.len(), range.clone()));
    }

    for (expected_param, found_param) in expected.params.iter().zip(found.params.iter()) {
      if expected_param.can_replace(found_param) {
        continue;
      }

      if !expected_param.check_match(found_param) {
        return Err(self.create_type_mismatch(expected_param.to_owned(), found_param.to_owned(), range.clone()));
      }
    }

    let expected_return_type = *expected.return_type.clone();
    let found_return_type = *found.return_type.clone();

    if expected_return_type.can_replace(&found_return_type) {
      return Ok(());
    }

    if !expected_return_type.check_match(&found_return_type) {
      return Err(self.create_type_mismatch(expected_return_type, found_return_type, range.clone()));
    }
    return Ok(());
  }
}
