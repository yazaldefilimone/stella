use super::type_utils::CheckResult;
use super::Checker;
use crate::diagnostics::{Diagnostic, TypeError};
use crate::types::{FunctionType, GenericCallType, GenericType, GroupType, OptionType, TableType, Type, UnionType};
use std::collections::{BTreeMap, HashMap, HashSet};

type GenericBinds = HashMap<String, Type>;

impl<'a> Checker<'a> {
  pub fn check_generic_type<'t>(&mut self, generic: &'t GenericType) -> CheckResult<Type> {
    let ty = self.ctx.get_type(generic.name.as_str()).ok_or_else(|| {
      self.create_diagnostic(TypeError::UndeclaredType(generic.name.to_string(), Some(generic.range.clone())))
    })?;

    let binds = self.create_generic_table(&[generic.value.as_ref().clone()], &generic.variables);
    self.apply_generic_binds(ty, &binds)
  }

  pub fn check_generic_call<'t>(&mut self, generic_call: &'t GenericCallType) -> CheckResult<Type> {
    if let Some(stdlib_type) = self.check_stdlib_type(generic_call)? {
      return Ok(stdlib_type);
    }

    let ty = self.ctx.get_type(generic_call.name.as_str()).ok_or_else(|| {
      self.create_diagnostic(TypeError::UndeclaredType(generic_call.name.to_string(), Some(generic_call.range.clone())))
    })?;

    if let Type::Generic(generic) = ty {
      let binds = self.create_generic_table(&generic_call.types, &generic.variables);
      self.apply_generic_binds(&generic.value, &binds)
    } else {
      let diagnostic = TypeError::UndeclaredType(generic_call.name.to_string(), Some(generic_call.range.clone()));
      Err(self.create_diagnostic(diagnostic))
    }
  }

  // pub fn apply_generic_binds(&self, generic_value: &Type, binds: &GenericBinds) -> CheckResult<Type> {
  //   let bind = match generic_value {
  //     Type::Identifier(identifier) => {
  //       if let Some(bound_type) = binds.get(&identifier.name) {
  //         Ok(bound_type.clone())
  //       } else {
  //         Ok(Type::Identifier(identifier.clone()))
  //       }
  //     }
  //     Type::Function(function) => self.apply_generic_bind_function(function, binds),
  //     Type::Table(table) => self.apply_generic_bind_table(table, binds),
  //     Type::Union(union) => self.apply_generic_bind_union(union, binds),
  //     Type::Option(option) => self.apply_generic_bind_option(option, binds),
  //     Type::Group(group) => self.apply_generic_bind_group(group, binds),
  //     Type::GenericCall(generic_call) => self.apply_generic_bind_call(generic_call, binds),
  //     Type::Generic(generic) => self.apply_generic_bind_generic(generic, binds),
  //     _ => Ok(generic_value.clone()),
  //   };
  //   return bind;
  // }

  // pub fn apply_generic_bind_function(&self, function: &FunctionType, binds: &GenericBinds) -> CheckResult<Type> {
  //   let params =
  //     function.params.iter().map(|param| self.apply_generic_binds(param, binds)).collect::<Result<Vec<_>, _>>()?;
  //   let return_type = self.apply_generic_binds(&function.return_type, binds)?;
  //   Ok(Type::new_function(params, return_type))
  // }

  // pub fn apply_generic_bind_table(&self, table: &TableType, binds: &GenericBinds) -> CheckResult<Type> {
  //   let array = table
  //     .array
  //     .as_ref()
  //     .map(|array| array.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<HashSet<_>, _>>());

  //   let map = table.map.as_ref().map(|map| {
  //     map
  //       .iter()
  //       .map(|(key, ty)| Ok((key.clone(), self.apply_generic_binds(ty, binds)?)))
  //       .collect::<Result<BTreeMap<_, _>, Diagnostic>>()
  //   });

  //   Ok(Type::new_table(array.transpose()?, map.transpose()?))
  // }

  // pub fn apply_generic_bind_union(&self, union: &UnionType, binds: &GenericBinds) -> CheckResult<Type> {
  //   let types = union.types.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<Vec<_>, _>>()?;
  //   Ok(Type::new_union(types))
  // }

  // pub fn apply_generic_bind_option(&self, option: &OptionType, binds: &GenericBinds) -> CheckResult<Type> {
  //   let inner_type = self.apply_generic_binds(&option.inner_type, binds)?;
  //   Ok(Type::new_option(inner_type))
  // }

  // pub fn apply_generic_bind_group(&self, group: &GroupType, binds: &GenericBinds) -> CheckResult<Type> {
  //   let types = group.types.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<Vec<_>, _>>()?;
  //   Ok(Type::new_group(types))
  // }

  // pub fn apply_generic_bind_call(&self, call: &GenericCallType, binds: &GenericBinds) -> CheckResult<Type> {
  //   let types = call.types.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<Vec<_>, _>>()?;
  //   let applyed = self.ctx.get_type(&call.name);
  // }

  // pub fn apply_generic_bind_generic(&self, generic: &GenericType, binds: &GenericBinds) -> CheckResult<Type> {
  //   let table_bind = HashMap::new();
  //   for value in &generic.variables {}
  //   let types = self.apply_generic_binds(&generic.value, binds)?;
  //   let value = self.apply_generic_binds(&generic.value, binds)?;
  // }

  // ----
  pub fn apply_generic_binds(&self, generic_value: &Type, binds: &GenericBinds) -> CheckResult<Type> {
    match generic_value {
      Type::Alias(identifier) => {
        if let Some(bound_type) = binds.get(&identifier.name) {
          Ok(bound_type.clone())
        } else {
          return Ok(Type::Alias(identifier.clone()));
        }
      }
      Type::Function(function) => self.apply_generic_bind_function(function, binds),
      Type::Table(table) => self.apply_generic_bind_table(table, binds),
      Type::Union(union) => self.apply_generic_bind_union(union, binds),
      Type::Option(option) => self.apply_generic_bind_option(option, binds),
      Type::Group(group) => self.apply_generic_bind_group(group, binds),
      Type::GenericCall(generic_call) => self.apply_generic_bind_call(generic_call, binds),
      Type::Generic(generic) => self.apply_generic_bind_generic(generic, binds),
      _ => Ok(generic_value.clone()),
    }
  }

  pub fn apply_generic_bind_function(&self, function: &FunctionType, binds: &GenericBinds) -> CheckResult<Type> {
    let params =
      function.params.iter().map(|param| self.apply_generic_binds(param, binds)).collect::<Result<Vec<_>, _>>()?;
    let return_type = self.apply_generic_binds(&function.return_type, binds)?;
    Ok(Type::new_function(params, return_type))
  }

  pub fn apply_generic_bind_table(&self, table: &TableType, binds: &GenericBinds) -> CheckResult<Type> {
    let array = if let Some(array) = &table.array {
      let array_bind = array.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<HashSet<_>, _>>()?;
      Some(array_bind)
    } else {
      None
    };

    let map = if let Some(map) = &table.map {
      let map_bind = map
        .iter()
        .map(|(key, ty)| Ok((key.clone(), self.apply_generic_binds(ty, binds)?)))
        .collect::<Result<BTreeMap<_, _>, Diagnostic>>()?;

      Some(map_bind)
    } else {
      None
    };

    Ok(Type::new_table(array, map))
  }

  pub fn apply_generic_bind_union(&self, union: &UnionType, binds: &GenericBinds) -> CheckResult<Type> {
    let types = union.types.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<Vec<_>, _>>()?;
    Ok(Type::new_union(types))
  }

  pub fn apply_generic_bind_option(&self, option: &OptionType, binds: &GenericBinds) -> CheckResult<Type> {
    let inner_type = self.apply_generic_binds(&option.inner_type, binds)?;
    Ok(Type::new_option(inner_type))
  }

  pub fn apply_generic_bind_group(&self, group: &GroupType, binds: &GenericBinds) -> CheckResult<Type> {
    let types = group.types.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<Vec<_>, _>>()?;
    Ok(Type::new_group(types))
  }

  pub fn apply_generic_bind_call(&self, call: &GenericCallType, binds: &GenericBinds) -> CheckResult<Type> {
    let types = call.types.iter().map(|ty| self.apply_generic_binds(ty, binds)).collect::<Result<Vec<_>, _>>()?;

    let base_type = self
      .ctx
      .get_type(&call.name)
      .ok_or_else(|| self.create_diagnostic(TypeError::UndeclaredType(call.name.clone(), Some(call.range.clone()))))?;

    match base_type {
      Type::Generic(generic) => {
        let binds = self.create_generic_table(&types, &generic.variables);
        self.apply_generic_binds(&generic.value, &binds)
      }
      _ => self.apply_generic_binds(&base_type, &binds),
    }
    // let binds = self.create_generic_table(&types, &generic.variables);

    // self.apply_generic_binds(&base_type, binds)
  }

  pub fn apply_generic_bind_generic(&self, generic: &GenericType, binds: &GenericBinds) -> CheckResult<Type> {
    let value = self.apply_generic_binds(&generic.value, binds)?;
    Ok(Type::Generic(GenericType {
      name: generic.name.clone(),
      variables: generic.variables.clone(),
      value: Box::new(value),
      range: generic.range.clone(),
    }))
  }
}
