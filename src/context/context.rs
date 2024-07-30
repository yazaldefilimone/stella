#![allow(dead_code)]
use crate::types::{FunctionType, Type};
use crate::utils::location::Location;
use std::collections::{BTreeMap, BTreeSet};

pub struct Context {
  pub scope_pointer: usize,
  pub scopes: Vec<Scope>,
  pub exports: BTreeMap<String, Type>,
  pub modules: BTreeMap<String, Type>,
  pub return_decl_name: String,
}

fn create_global_scope() -> Scope {
  let mut global_scope = Scope::new();
  global_scope.variables.insert("nil".to_string(), Type::Nil);
  // stdlib (print, type, etc)
  global_scope.variables.insert("print".to_string(), Type::new_function(vec![Type::String], Type::Nil));
  global_scope.variables.insert(
    "type".to_string(),
    Type::new_function(vec![Type::Unknown], Type::String),
  );
  global_scope.variables.insert(
    "tostring".to_string(),
    Type::new_function(vec![Type::Unknown], Type::String),
  );
  global_scope.variables.insert(
    "tonumber".to_string(),
    Type::new_function(vec![Type::String], Type::Number),
  );
  global_scope
}

impl Context {
  pub fn new() -> Context {
    let scopes = vec![create_global_scope()];
    let exports = BTreeMap::new();
    let return_decl_name = "return".to_string();
    let modules = BTreeMap::new();
    Context { scopes, scope_pointer: 0, exports, return_decl_name, modules }
  }

  pub fn declare_variable(&mut self, name: &str, type_: Type) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
      scope.variables.insert(name.to_owned(), type_);
      self.set_unused_variable(name);
    }
  }

  pub fn declare_return_param_type(&mut self, type_: Type) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
      scope.variables.insert("return_param".to_owned(), type_);
    }
  }
  pub fn get_last_return(&self) -> Option<&Type> {
    if let Some(scope) = self.scopes.get(self.scope_pointer) {
      return scope.variables.get("return");
    }
    None
  }

  pub fn set_last_return(&mut self, type_: Type) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
      scope.variables.insert("return".to_owned(), type_);
    }
  }

  pub fn get_return_param_type(&self) -> Option<&Type> {
    if let Some(scope) = self.scopes.get(self.scope_pointer) {
      return scope.variables.get("return_param");
    }
    None
  }

  pub fn set_remove_return_param_type(&mut self) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
      scope.variables.remove("return_param");
    }
  }

  pub fn declare_variable_in_scope(&mut self, name: &str, type_: Type, scope_pointer: usize) {
    if let Some(scope) = self.scopes.get_mut(scope_pointer) {
      scope.variables.insert(name.to_owned(), type_);
      self.set_unused_variable_in_scope(name, scope_pointer);
    }
  }

  pub fn redeclare_variable(&mut self, name: &str, type_: Type) -> bool {
    for scope_pointer in (0..=self.scope_pointer).rev() {
      if let Some(scope) = self.scopes.get_mut(scope_pointer) {
        if let Some(ty) = scope.variables.get_mut(name) {
          *ty = type_;
          return true;
        }
      }
    }
    return false;
  }

  pub fn redeclare_in_scope(&mut self, name: &str, type_: Type, scope_pointer: usize) -> bool {
    if let Some(scope) = self.scopes.get_mut(scope_pointer) {
      if let Some(ty) = scope.variables.get_mut(name) {
        *ty = type_;
        return true;
      }
    }
    return false;
  }

  pub fn declare_function(&mut self, name: &str, params: Vec<Type>, return_type: Type) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
      scope.variables.insert(name.to_owned(), Type::new_function(params, return_type));
    }
  }

  pub fn get_function(&self, name: &str) -> Option<&FunctionType> {
    for scope in self.scopes.iter().rev() {
      if let Some(function_type) = scope.variables.get(name) {
        match function_type {
          Type::Function(function_type) => return Some(function_type),
          _ => return None,
        }
      }
    }
    None
  }

  pub fn declare_global_variable(&mut self, name: &str, type_: Type) {
    if let Some(scope) = self.scopes.get_mut(0) {
      scope.variables.insert(name.to_owned(), type_);
      self.set_unused_variable(name);
    }
  }

  pub fn remove_global_variable(&mut self, name: &str) {
    if let Some(scope) = self.scopes.get_mut(0) {
      scope.variables.remove(name);
    }
  }

  pub fn enter_scope(&mut self) {
    let scope = Scope::new();
    self.scopes.push(scope);
    self.scope_pointer = self.scopes.len() - 1;
  }

  pub fn leave_scope(&mut self) {
    self.scopes.pop();
    self.scope_pointer = self.scopes.len() - 1;
  }

  pub fn get_variable(&self, name: &str) -> Option<&Type> {
    for scope in self.scopes.iter().rev() {
      if let Some(ty) = scope.variables.get(name) {
        return Some(ty);
      }
    }
    return None;
  }

  pub fn get_variable_in_scope(&self, name: &str, scope_pointer: usize) -> Option<&Type> {
    if let Some(scope) = self.scopes.get(scope_pointer) {
      return scope.variables.get(name);
    }
    None
  }
  fn contains(&self, name: &str, scope: &Option<&Scope>) -> bool {
    if scope.is_none() {
      return false;
    }
    scope.unwrap().variables.contains_key(name)
  }

  pub fn defined_in_current_scope(&self, name: &str) -> bool {
    self.contains(name, &self.scopes.get(self.scope_pointer))
  }

  pub fn defined_in_any_scope(&self, name: &str) -> (bool, usize) {
    for (scope_pointer, scope) in self.scopes.iter().enumerate().rev() {
      if self.contains(name, &Some(scope)) {
        return (true, scope_pointer);
      }
    }
    (false, 0)
  }

  pub fn check_unused_variables(&self) -> Vec<String> {
    let mut unused = Vec::new();
    for scope in &self.scopes {
      for var in &scope.unused_variables {
        if var != &self.return_decl_name {
          unused.push(var.clone());
        }
      }
    }
    unused
  }

  pub fn use_variable(&mut self, name: &str) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
      scope.unused_variables.remove(name);
    }
  }

  pub fn use_variable_in_scope(&mut self, name: &str, scope_pointer: usize) {
    if let Some(scope) = self.scopes.get_mut(scope_pointer) {
      scope.unused_variables.remove(name);
    }
  }
  pub fn set_unused_variable(&mut self, name: &str) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
      scope.unused_variables.insert(name.to_owned());
    }
  }
  pub fn set_unused_variable_in_scope(&mut self, name: &str, scope_pointer: usize) {
    if let Some(scope) = self.scopes.get_mut(scope_pointer) {
      scope.unused_variables.insert(name.to_owned());
    }
  }

  pub fn get_variable_location(&self, name: &str) -> Option<Location> {
    if let Some(scope) = self.scopes.get(self.scope_pointer) {
      if let Some(location) = scope.variables_location.get(name) {
        return Some(location.clone());
      }
    }
    None
  }

  pub fn set_variable_location(&mut self, name: &str, location: Location) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
      scope.variables_location.insert(name.to_owned(), location);
    }
  }

  pub fn is_global_scope(&self) -> bool {
    if self.scope_pointer == 0 && self.scopes.len() == 1 {
      return true;
    }
    return false;
  }

  pub fn get_module(&self, name: &str) -> Option<&Type> {
    if self.modules.contains_key(name) {
      return Some(self.modules.get(name).unwrap());
    }
    return None;
  }
  pub fn set_module(&mut self, name: &str, module: Type) {
    self.modules.insert(name.to_owned(), module);
  }

  pub fn get_exports(&self) -> Vec<&Type> {
    let values = self.exports.values().collect::<Vec<&Type>>();
    return values.to_vec();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
  pub variables: BTreeMap<String, Type>,
  pub unused_variables: BTreeSet<String>,
  pub variables_location: BTreeMap<String, Location>,
}

impl Scope {
  pub fn new() -> Scope {
    Scope { variables: BTreeMap::new(), unused_variables: BTreeSet::new(), variables_location: BTreeMap::new() }
  }
}
