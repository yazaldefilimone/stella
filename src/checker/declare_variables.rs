use crate::{
  ast::tokens::Token,
  diagnostics::{Diagnostic, TypeWarning},
  types::Type,
  utils::range::Range,
};

use super::Checker;

type NameType<'a> = &'a Vec<(Token, Option<Type>)>;
type ValueType<'a> = &'a (Token, Option<Type>);

impl<'a> Checker<'a> {
  pub fn declare_variables(&mut self, names: NameType, ty: Type, local: bool, range: Range) -> Result<(), Diagnostic> {
    match ty {
      Type::Grup(group) => {
        for (index, token) in names.iter().enumerate() {
          let assigned_type = group.types.get(index).cloned().unwrap_or(Type::Nil);
          // Check for shadowing and emit a warning if necessary
          self.check_shadowing(token.0.lexeme(), local, &token.0.range)?;
          self.declare_variable(token, assigned_type, local)?;
        }
      }
      _ => {
        for (index, token) in names.iter().enumerate() {
          let assigned_type = if index == 0 { self.check_type(ty.clone())? } else { Type::Nil };
          // Check for shadowing and emit a warning if necessary
          self.check_shadowing(token.0.lexeme(), local, &token.0.range)?;
          self.declare_variable(token, assigned_type, local)?;
        }
      }
    }
    Ok(())
  }

  pub fn declare_variable(&mut self, value: ValueType, mut assigned_type: Type, local: bool) -> Result<(), Diagnostic> {
    let lexeme = value.0.lexeme();
    let value_range = value.0.range.clone();

    // Check if the variable already exists in the current context
    let existing_type = self.ctx.get_variable(lexeme, None);
    if let Some(existing_type) = existing_type {
      // If the variable exists, check if the new type is compatible
      if !existing_type.check_match(&assigned_type) {
        return Err(self.create_type_mismatch(existing_type.to_owned(), assigned_type, value_range.clone()));
      }

      // If the existing type can replace the new type, use the existing type
      if existing_type.can_replace(&assigned_type) {
        assigned_type = existing_type.clone();
      }

      // Update the variable type in the context
      self.ctx.redeclare_variable(lexeme, assigned_type, None);
      return Ok(());
    }

    // If there is a type specified in the declaration, check if it's redundant
    //
    if let Some(value_type) = &value.1 {
      let value_type = self.check_type(value_type.to_owned())?;
      // todo: previne to get variable twice....
      if let Some(existing_type) = self.ctx.get_variable(lexeme, None) {
        if !existing_type.check_match(&value_type) {
          return Err(self.create_type_mismatch(existing_type.to_owned(), value_type.to_owned(), value_range));
        }

        let diagnostic = TypeWarning::RedundantType(lexeme.to_string(), existing_type.to_string(), Some(value_range));
        self.diagnostics.add(diagnostic.into());
      }

      assigned_type = self.infer_type(value_type, assigned_type, &value.0.range)?;
    }

    // If the variable doesn't exist, declare it
    self.ctx.declare_variable(lexeme, assigned_type, None);

    if local {
      self.ctx.set_local_declaration(lexeme);
    }
    self.ctx.declare_variable_range(lexeme, value.0.range.clone());
    Ok(())
  }

  pub fn infer_type(&mut self, expected: Type, found: Type, range: &Range) -> Result<Type, Diagnostic> {
    // If the types don't match, return a type mismatch error
    if !expected.check_match(&found) {
      return Err(self.create_type_mismatch(expected, found, range.clone()));
    }
    // Check if the expected type can replace the found type...
    return if expected.can_replace(&found) { Ok(found) } else { Ok(expected) };
  }
}
