#![allow(dead_code)]
use crate::{ast::ast::Type, utils::location::Location};
use std::collections::{HashMap, HashSet};

pub struct Context {
  pub scope_pointer: usize,
  pub scopes: Vec<Scope>,
}

impl Context {
  pub fn new() -> Context {
    let global_scope = Scope::new();
    Context { scopes: vec![global_scope], scope_pointer: 0 }
  }

  pub fn declare_variable(&mut self, name: &str, type_: Type) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
      scope.variables.insert(name.to_owned(), type_);
      self.set_unused_variable(name);
    }
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

  pub fn add_scope(&mut self) {
    self.scope_pointer += 1;
    let scope = Scope::new();
    self.scopes.push(scope);
  }

  pub fn remove_scope(&mut self) {
    if self.scope_pointer > 0 {
      self.scopes.pop();
      self.scope_pointer -= 1;
    }
  }

  pub fn get_variable(&self, name: &str) -> Option<&Type> {
    for i in (0..=self.scope_pointer).rev() {
      if let Some(scope) = self.scopes.get(i) {
        if let Some(ty) = scope.variables.get(name) {
          return Some(ty);
        }
      }
    }
    None
  }

  pub fn is_variable_defined(&self, name: &str) -> bool {
    if let Some(scope) = self.scopes.get(self.scope_pointer) {
      return scope.variables.contains_key(name);
    }
    false
  }

  pub fn is_variable_defined_in_any_scope(&self, name: &str) -> bool {
    for i in (0..=self.scope_pointer).rev() {
      if let Some(scope) = self.scopes.get(i) {
        if scope.variables.contains_key(name) {
          return true;
        }
      }
    }
    false
  }

  pub fn check_unused_variables(&self) -> Vec<String> {
    let mut unused = Vec::new();
    for scope in &self.scopes {
      for var in &scope.unused_variables {
        unused.push(var.clone());
      }
    }
    unused
  }

  pub fn use_variable(&mut self, name: &str) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
      scope.unused_variables.remove(name);
    }
  }

  pub fn set_unused_variable(&mut self, name: &str) {
    if let Some(scope) = self.scopes.get_mut(self.scope_pointer) {
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
}

impl Scope {
  pub fn new() -> Scope {
    Scope { variables: HashMap::new(), unused_variables: HashSet::new(), variables_location: HashMap::new() }
  }
}
