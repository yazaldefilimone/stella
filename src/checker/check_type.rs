use std::collections::HashMap;

use super::Checker;
use crate::{
  diagnostics::{Diagnostic, TypeError},
  types::{self, Type},
};

impl<'a> Checker<'a> {
  pub fn check_optional_type(&mut self, ty: &Option<Type>, assume_nil: bool) -> Result<Type, Diagnostic> {
    match ty {
      Some(Type::Identifier(identifier)) => self.check_type_identifier(identifier),
      Some(Type::Generic(generic)) => self.check_generic_type(generic),
      Some(t) => Ok(t.clone()),
      None => Ok(if assume_nil { Type::Nil } else { Type::Unknown }),
    }
  }

  pub fn check_type(&mut self, ty: Type) -> Result<Type, Diagnostic> {
    match ty {
      Type::Identifier(identifier) => self.check_type_identifier(&identifier),
      Type::Generic(generic) => self.check_generic_type(&generic),
      Type::GenericCall(generic_call) => self.check_generic_call(&generic_call),
      _ => Ok(ty),
    }
  }

  pub fn check_generic_call(&mut self, generic_call: &types::GenericCallType) -> Result<Type, Diagnostic> {
    let ty = self.ctx.get_type(generic_call.name.as_str()).ok_or_else(|| {
      self.create_diagnostic(TypeError::UndeclaredType(generic_call.name.to_string(), Some(generic_call.range.clone())))
    })?;

    if let Type::Generic(generic) = ty {
      let binds = self.create_generic_table(&generic_call.types, &generic.variables);
      self.apply_generic_binds(&generic.value, &binds)
    } else {
      Err(
        self.create_diagnostic(TypeError::UndeclaredType(
          generic_call.name.to_string(),
          Some(generic_call.range.clone()),
        )),
      )
    }
  }

  pub fn create_generic_table(&self, types: &[Type], variables: &[String]) -> HashMap<String, Type> {
    variables.iter().cloned().zip(types.iter().cloned()).collect()
  }

  pub fn apply_generic_binds(&self, generic_value: &Type, binds: &HashMap<String, Type>) -> Result<Type, Diagnostic> {
    match generic_value {
      Type::Identifier(identifier) => {
        if let Some(bound_type) = binds.get(&identifier.name) {
          Ok(bound_type.clone())
        } else {
          Ok(Type::Identifier(identifier.clone()))
        }
      }
      Type::Function(function) => {
        let params =
          function.params.iter().map(|param| self.apply_generic_binds(param, binds)).collect::<Result<Vec<_>, _>>()?;
        let return_type = self.apply_generic_binds(&function.return_type, binds)?;
        Ok(Type::Function(types::FunctionType { params, return_type: Box::new(return_type) }))
      }
      Type::Table(table) => {
        let key_type = self.apply_generic_binds(&table.key_type, binds)?;
        let value_type = self.apply_generic_binds(&table.value_type, binds)?;
        Ok(Type::Table(types::TableType { key_type: Box::new(key_type), value_type: Box::new(value_type) }))
      }
      Type::Union(union) => {
        let types = union.types.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<Vec<_>, _>>()?;
        Ok(Type::Union(types::UnionType { types }))
      }
      Type::Optional(optional) => {
        let inner_type = self.apply_generic_binds(&optional.inner_type, binds)?;
        Ok(Type::Optional(types::OptionalType { inner_type: Box::new(inner_type) }))
      }
      Type::Grup(group) => {
        let types = group.types.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<Vec<_>, _>>()?;
        Ok(Type::Grup(types::GrupType { types }))
      }
      Type::GenericCall(generic_call) => {
        let types =
          generic_call.types.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<Vec<_>, _>>()?;

        Ok(Type::GenericCall(types::GenericCallType {
          name: generic_call.name.clone(),
          types,
          range: generic_call.range.clone(),
        }))
      }
      _ => Ok(generic_value.clone()),
    }
  }

  pub fn check_generic_type(&mut self, generic: &types::GenericType) -> Result<Type, Diagnostic> {
    let ty = self.ctx.get_type(generic.name.as_str()).ok_or_else(|| {
      self.create_diagnostic(TypeError::UndeclaredType(generic.name.to_string(), Some(generic.range.clone())))
    })?;

    let binds = self.create_generic_table(&[generic.value.as_ref().clone()], &generic.variables);
    self.apply_generic_binds(&ty, &binds)
  }

  pub fn check_type_identifier(&mut self, ident: &types::IdentifierType) -> Result<types::Type, Diagnostic> {
    self.ctx.get_type(ident.name.as_str()).cloned().ok_or_else(|| {
      self.create_diagnostic(TypeError::UndeclaredVariable(ident.name.to_string(), Some(ident.range.clone())))
    })
  }

  pub fn create_generic_table_type(&self, generics: &[String], inferred: &[Type]) -> HashMap<String, Type> {
    generics.iter().cloned().zip(inferred.iter().cloned()).collect()
  }
}
