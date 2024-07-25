#![allow(dead_code)]
use crate::types::Type;
use crate::utils::location::Location;
use std::collections::{HashMap, HashSet};

pub struct Context {
  pub scope_pointer: usize,
  pub scopes: Vec<Scope>,
  pub return_decl_name: String,
}

impl Context {
  pub fn new() -> Context {
    let global_scope = Scope::new();
    Context { scopes: vec![global_scope], scope_pointer: 0, return_decl_name: "return".to_string() }
  }

  pub fn declare_variable(&mut self, name: &str, type_: Type) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
      scope.variables.insert(name.to_owned(), type_);
      self.set_unused_variable(name);
    }
  }

  pub fn declare_return_variable_type(&mut self, type_: Type) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
      scope.variables.insert("return".to_owned(), type_);
    }
  }

  pub fn get_return_variable_type(&self) -> Option<&Type> {
    if let Some(scope) = self.scopes.get(self.scope_pointer) {
      return scope.variables.get("return");
    }
    None
  }

  pub fn set_remove_return_variable_type(&mut self) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
      scope.variables.remove("return");
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
      scope.function_types.insert(
        name.to_owned(),
        FunctionType { params, return_type: Box::new(return_type) },
      );
    }
  }

  pub fn get_function(&self, name: &str) -> Option<&FunctionType> {
    for i in (0..=self.scope_pointer).rev() {
      if let Some(scope) = self.scopes.get(i) {
        if let Some(function_type) = scope.function_types.get(name) {
          return Some(function_type);
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
  fn contains(&self, name: &str, scope: &Option<&Scope>, check_variable: bool) -> bool {
    if scope.is_none() {
      return false;
    }
    if check_variable {
      scope.unwrap().variables.contains_key(name) || scope.unwrap().function_types.contains_key(name)
    } else {
      scope.unwrap().function_types.contains_key(name) || scope.unwrap().variables.contains_key(name)
    }
  }

  pub fn defined_in_current_scope(&self, name: &str, variable_first: bool) -> bool {
    self.contains(name, &self.scopes.get(self.scope_pointer), variable_first)
  }

  pub fn defined_in_any_scope(&self, name: &str, variable_first: bool) -> (bool, usize) {
    for (scope_pointer, scope) in self.scopes.iter().enumerate().rev() {
      if self.contains(name, &Some(scope), variable_first) {
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
  pub variables: HashMap<String, Type>,
  pub unused_variables: HashSet<String>,
  pub variables_location: HashMap<String, Location>,
  pub function_types: HashMap<String, FunctionType>,
}

impl Scope {
  pub fn new() -> Scope {
    Scope {
      variables: HashMap::new(),
      unused_variables: HashSet::new(),
      variables_location: HashMap::new(),
      function_types: HashMap::new(),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
  pub params: Vec<Type>,
  pub return_type: Box<Type>,
}
