use std::collections::HashMap;

use super::Checker;
use crate::{
  diagnostics::{Diagnostic, TypeError},
  types::{
    FunctionType, GenericCallType, GenericType, GrupType, IdentifierType, OptionalType, TableType, Type, UnionType,
  },
};

impl<'a> Checker<'a> {
  pub fn check_optional_type(&mut self, ty: &Option<Type>) -> Result<Type, Diagnostic> {
    match ty {
      Some(Type::Identifier(identifier)) => self.check_type_identifier(identifier),
      Some(Type::Generic(generic)) => self.check_generic_type(generic),
      Some(t) => Ok(t.clone()),
      None => Ok(Type::Unknown),
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

  pub fn check_generic_call(&mut self, generic_call: &GenericCallType) -> Result<Type, Diagnostic> {
    let ty = self.ctx.get_type(generic_call.name.as_str()).ok_or_else(|| {
      self.create_diagnostic(TypeError::UndeclaredType(
        generic_call.name.to_string(),
        Some(generic_call.location.clone()),
      ))
    })?;

    if let Type::Generic(generic) = ty {
      let binds = self.create_generic_table(&generic_call.types, &generic.variables);
      self.apply_generic_binds(&generic.value, &binds)
    } else {
      Err(self.create_diagnostic(TypeError::UndeclaredType(
        generic_call.name.to_string(),
        Some(generic_call.location.clone()),
      )))
    }
  }

  pub fn bind_generic(&self, table: &HashMap<String, Type>, ty: Type) -> Type {
    match ty {
      Type::Identifier(ident) => table.get(&ident.name).cloned().unwrap_or(Type::Unknown),
      Type::Function(mut f) => {
        f.params = f.params.into_iter().map(|p| self.bind_generic(table, p)).collect();
        f.return_type = Box::new(self.bind_generic(table, *f.return_type));
        Type::Function(f)
      }
      Type::Table(mut t) => {
        t.key_type = Box::new(self.bind_generic(table, *t.key_type));
        t.value_type = Box::new(self.bind_generic(table, *t.value_type));
        Type::Table(t)
      }
      Type::Generic(mut g) => {
        g.value = Box::new(self.bind_generic(table, *g.value));
        Type::Generic(g)
      }
      Type::Optional(mut o) => {
        o.inner_type = Box::new(self.bind_generic(table, *o.inner_type));
        Type::Optional(o)
      }
      Type::Union(mut u) => {
        u.types = u.types.into_iter().map(|t| self.bind_generic(table, t)).collect();
        Type::Union(u)
      }
      Type::Grup(mut g) => {
        g.types = g.types.into_iter().map(|t| self.bind_generic(table, t)).collect();
        Type::Grup(g)
      }
      _ => ty,
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
        Ok(Type::Function(FunctionType { params, return_type: Box::new(return_type) }))
      }
      Type::Table(table) => {
        let key_type = self.apply_generic_binds(&table.key_type, binds)?;
        let value_type = self.apply_generic_binds(&table.value_type, binds)?;
        Ok(Type::Table(TableType { key_type: Box::new(key_type), value_type: Box::new(value_type) }))
      }
      Type::Union(union) => {
        let types = union.types.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<Vec<_>, _>>()?;
        Ok(Type::Union(UnionType { types }))
      }
      Type::Optional(optional) => {
        let inner_type = self.apply_generic_binds(&optional.inner_type, binds)?;
        Ok(Type::Optional(OptionalType { inner_type: Box::new(inner_type) }))
      }
      Type::Grup(group) => {
        let types = group.types.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<Vec<_>, _>>()?;
        Ok(Type::Grup(GrupType { types }))
      }
      Type::GenericCall(generic_call) => {
        let types =
          generic_call.types.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<Vec<_>, _>>()?;

        Ok(Type::GenericCall(GenericCallType {
          name: generic_call.name.clone(),
          types,
          location: generic_call.location.clone(),
        }))
      }
      _ => Ok(generic_value.clone()),
    }
  }

  pub fn check_generic_type(&mut self, generic: &GenericType) -> Result<Type, Diagnostic> {
    let ty = self.ctx.get_type(generic.name.as_str()).ok_or_else(|| {
      self.create_diagnostic(TypeError::UndeclaredType(generic.name.to_string(), Some(generic.location.clone())))
    })?;

    let binds = self.create_generic_table(&[generic.value.as_ref().clone()], &generic.variables);
    self.apply_generic_binds(&ty, &binds)
  }

  pub fn check_type_identifier(&mut self, ident: &IdentifierType) -> Result<Type, Diagnostic> {
    self.ctx.get_type(ident.name.as_str()).cloned().ok_or_else(|| {
      self.create_diagnostic(TypeError::UndeclaredVariable(ident.name.to_string(), Some(ident.location.clone())))
    })
  }

  pub fn create_generic_table_type(&self, generics: &[String], inferred: &[Type]) -> HashMap<String, Type> {
    generics.iter().cloned().zip(inferred.iter().cloned()).collect()
  }
}
