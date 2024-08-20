use super::{type_utils::CheckResult, Checker};
use crate::{ast::tokens::Token, diagnostics::TypeWarning, types::Type, utils::range::Range};

type NameType<'a> = &'a Vec<(Token, Option<Type>)>;

type LeftHandSide<'a> = &'a (&'a str, Option<Type>);

impl<'a> Checker<'a> {
  pub fn declare_variables(&mut self, names: NameType, ty: Type, local: bool, range: Range) -> CheckResult<()> {
    match ty {
      Type::Group(group) => {
        // Handle group type where multiple values are assigned at once
        for (index, token) in names.iter().enumerate() {
          let left_hand_side: LeftHandSide = &(token.0.lexeme(), token.1.clone());
          let range = token.0.range.clone();
          let assigned_type = group.types.get(index).cloned().unwrap_or(Type::Nil);
          if local {
            self.declare_local_variable(&left_hand_side, assigned_type, range)?;
          } else {
            self.declare_global_variable(left_hand_side, assigned_type, range)?;
          }
        }
      }
      _ => {
        // Handle single type where one type is assigned to multiple variables
        for (index, token) in names.iter().enumerate() {
          let assigned_type = if index == 0 { self.check_type(ty.clone())? } else { Type::Nil };
          let range = token.0.range.clone();
          let left_hand_side: LeftHandSide = &(token.0.lexeme(), token.1.clone());
          if local {
            self.declare_local_variable(&left_hand_side, assigned_type, range)?;
          } else {
            self.declare_global_variable(left_hand_side, assigned_type, range)?;
          }
        }
      }
    }
    Ok(())
  }

  // declares a global variable with a given type
  pub fn declare_global_variable(&mut self, left: LeftHandSide, assign_ty: Type, range: Range) -> CheckResult<()> {
    let (name, current_ty) = left;
    // check shadowing
    self.check_shadowing(name, false, &range)?;

    // Global variables are usually not redeclared, but we should check if they exist
    if let Some(existing_type) = self.ctx.get_global_variable(name) {
      if !existing_type.check_match(&assign_ty) {
        return Err(self.create_type_mismatch(existing_type.to_owned(), assign_ty, range));
      }
    }

    // If a type is specified in the variable declaration, check if it's redundant
    if let Some(declared_type) = &current_ty {
      let declared_type = self.check_type(declared_type.to_owned())?;
      if !assign_ty.check_match(&declared_type) {
        return Err(self.create_type_mismatch(assign_ty, declared_type, range));
      }

      // check redundant type
      if let Some(existing_type) = self.ctx.get_variable(name, None) {
        if !existing_type.check_match(&assign_ty) {
          return Err(self.create_type_mismatch(existing_type.to_owned(), assign_ty, range));
        }

        let diagnostic = TypeWarning::RedundantType(name.to_string(), existing_type.to_string(), Some(range.clone()));

        self.diagnostics.add(diagnostic.into());
      }
    }

    // declare the variable as global
    self.ctx.declare_global_variable(name, assign_ty);
    // declare the variable range in global scope
    self.ctx.declare_variable_range(name, range, Some(0));
    Ok(())
  }

  // declares a local variable with a given type
  //   pub fn declare_global_variable(&mut self, left: &LeftHandSide, assign_ty: Type, range: Range) -> CheckResult<()> {

  pub fn declare_local_variable(&mut self, left: LeftHandSide, assign_ty: Type, range: Range) -> CheckResult<()> {
    let (name, current_ty) = left;
    // check shadowing
    self.check_shadowing(name, true, &range)?;
    // Local variables can shadow globals or other locals
    if let Some(existing_type) = self.ctx.get_variable(name, Some(self.ctx.scope_pointer)) {
      if self.ctx.is_local_declaration(name) {
        return Err(self.create_redeclaration(name, range));
      }

      if !existing_type.check_match(&assign_ty) {
        return Err(self.create_type_mismatch(existing_type.to_owned(), assign_ty, range));
      }
    }

    // If a type is specified in the variable declaration, check if it's redundant
    if let Some(declared_type) = &current_ty {
      let declared_type = self.check_type(declared_type.to_owned())?;
      if !assign_ty.check_match(&declared_type) {
        return Err(self.create_type_mismatch(assign_ty, declared_type, range));
      }

      // check redundant type
      if let Some(existing_type) = self.ctx.get_variable(name, None) {
        if !existing_type.check_match(&assign_ty) {
          return Err(self.create_type_mismatch(existing_type.to_owned(), assign_ty, range));
        }
        let diagnostic = TypeWarning::RedundantType(name.to_string(), existing_type.to_string(), Some(range.clone()));
        self.diagnostics.add(diagnostic.into());
      }
    }

    // declare the variable as local
    self.ctx.declare_variable(name, assign_ty, None);
    self.ctx.set_local_declaration(name);
    // declare the variable range in current scope
    self.ctx.declare_variable_range(name, range, None);
    Ok(())
  }

  // Infers the type of a variable by checking if the types match
  pub fn infer_type(&mut self, expected: Type, found: Type, range: &Range) -> CheckResult<Type> {
    if !expected.check_match(&found) {
      return Err(self.create_type_mismatch(expected, found, range.clone()));
    }
    if expected.can_replace(&found) {
      Ok(found)
    } else {
      Ok(expected)
    }
  }
}
