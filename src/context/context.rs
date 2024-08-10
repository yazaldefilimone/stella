#![allow(dead_code)]

use crate::{stdlib::create_stdlib, types::Type, utils::range::Range};
use std::collections::{BTreeMap, BTreeSet};

pub struct Context {
  pub scope_pointer: usize,
  pub scopes: Vec<Scope>,
  pub return_decl_name: String,
}

impl Context {
  pub fn new() -> Self {
    Context { scopes: vec![Self::create_global_scope()], scope_pointer: 0, return_decl_name: "return".to_string() }
  }

  fn create_global_scope() -> Scope {
    let mut global_scope = Scope::new();
    let stdlib = create_stdlib();
    global_scope.variables.extend(stdlib);
    global_scope
  }

  fn extend_global_scope(&mut self, table: BTreeMap<String, Type>, table_type: BTreeMap<String, Type>) {
    self.scopes.get_mut(0).unwrap().variables.extend(table);
    self.scopes.get_mut(0).unwrap().types.extend(table_type);
  }

  fn current_scope(&self) -> Option<&Scope> {
    self.scopes.get(self.scope_pointer)
  }

  fn current_scope_mut(&mut self) -> Option<&mut Scope> {
    self.scopes.get_mut(self.scope_pointer)
  }

  fn global_scope_mut(&mut self) -> Option<&mut Scope> {
    self.scopes.get_mut(0)
  }

  fn get_scope(&self, scope_pointer: usize) -> Option<&Scope> {
    self.scopes.get(scope_pointer)
  }

  fn get_scope_mut(&mut self, scope_pointer: usize) -> Option<&mut Scope> {
    self.scopes.get_mut(scope_pointer)
  }

  // Scope Management
  pub fn enter_scope(&mut self) {
    self.scopes.push(Scope::new());
    self.scope_pointer = self.scopes.len() - 1;
  }

  pub fn leave_scope(&mut self) {
    if !self.scopes.is_empty() {
      self.scopes.pop();
    }
    self.scope_pointer = self.scopes.len().saturating_sub(1);
  }

  // Variable Management
  pub fn declare_variable(&mut self, name: &str, tyy: Type, scope_pointer: Option<usize>) -> usize {
    let (scope, scope_pointer) = match scope_pointer {
      Some(pointer) => (self.get_scope_mut(pointer), pointer as i32),
      None => (self.current_scope_mut(), -1),
    };
    if let Some(scope) = scope {
      scope.variables.insert(name.to_owned(), tyy);
      scope.unused_variables.insert(name.to_owned());
    };
    return if scope_pointer < 0 { self.scope_pointer } else { scope_pointer as usize };
  }

  pub fn redeclare_variable(&mut self, name: &str, tyy: Type, scope_pointer: Option<usize>) -> bool {
    if let Some(pointer) = scope_pointer {
      if let Some(scope) = self.get_scope_mut(pointer) {
        if scope.variables.contains_key(name) {
          scope.variables.insert(name.to_owned(), tyy);
          return true;
        }
      }
      return false;
    }
    for scope_pointer in (0..=self.scope_pointer).rev() {
      if let Some(scope) = self.get_scope_mut(scope_pointer) {
        if scope.variables.contains_key(name) {
          scope.variables.insert(name.to_owned(), tyy);
          return true;
        }
      }
    }
    return false;
  }

  pub fn get_variable(&self, name: &str, scope_pointer: Option<usize>) -> Option<&Type> {
    if let Some(pointer) = scope_pointer {
      if let Some(scope) = self.get_scope(pointer) {
        if let Some(ty) = scope.variables.get(name) {
          return Some(ty);
        }
      }
      return None;
    }
    for scope in self.scopes.iter().rev() {
      if let Some(ty) = scope.variables.get(name) {
        return Some(ty);
      }
    }
    None
  }
  pub fn defined_in_current_scope(&self, name: &str) -> bool {
    self.current_scope().map_or(false, |scope| scope.variables.contains_key(name))
  }

  pub fn defined_in_any_scope(&self, name: &str) -> (bool, usize) {
    for (scope_pointer, scope) in self.scopes.iter().enumerate().rev() {
      if scope.variables.contains_key(name) {
        return (true, scope_pointer);
      }
    }
    (false, 0)
  }

  pub fn check_unused_variables(&self) -> Vec<String> {
    self
      .scopes
      .iter()
      .flat_map(|scope| scope.unused_variables.iter().filter(|var| *var != &self.return_decl_name).cloned())
      .collect()
  }

  pub fn use_variable(&mut self, name: &str, scope_pointer: Option<usize>) {
    let scope = match scope_pointer {
      Some(pointer) => self.get_scope_mut(pointer),
      None => self.current_scope_mut(),
    };

    if let Some(scope) = scope {
      scope.unused_variables.remove(name);
    }
  }

  // local declarations
  pub fn set_local_declaration(&mut self, name: &str) {
    if let Some(scope) = self.current_scope_mut() {
      scope.local_declarations.insert(name.to_owned(), true);
    }
  }

  pub fn is_local_declaration(&self, name: &str) -> bool {
    self.current_scope().and_then(|scope| scope.local_declarations.get(name)).cloned().unwrap_or(false)
  }

  pub fn create_anonymous_function(&self) -> Type {
    Type::new_function(vec![], Type::Unknown)
  }

  // Module Management
  // pub fn get_module(&self, name: &str) -> Option<&Type> {
  //   self.modules.get(name)
  // }

  // pub fn set_module(&mut self, name: &str, module: Type) {
  //   self.modules.insert(name.to_owned(), module);
  // }

  // pub fn get_exports(&self) -> Vec<&Type> {
  //   self.exports.values().collect()
  // }

  // Return Parameter Management
  pub fn declare_return_param_type(&mut self, type_: Type) {
    if let Some(scope) = self.current_scope_mut() {
      scope.variables.insert("return_param".to_owned(), type_);
    }
  }

  pub fn get_return_param_type(&self) -> Option<&Type> {
    self.current_scope().and_then(|scope| scope.variables.get("return_param"))
  }

  pub fn set_remove_return_param_type(&mut self) {
    if let Some(scope) = self.current_scope_mut() {
      scope.variables.remove("return_param");
    }
  }

  pub fn get_last_return(&self) -> Option<&Type> {
    self.current_scope().and_then(|scope| scope.variables.get("return"))
  }

  pub fn set_last_return(&mut self, type_: Type) {
    if let Some(scope) = self.current_scope_mut() {
      scope.variables.insert("return".to_owned(), type_);
    }
  }

  // Type Management
  pub fn declare_type(&mut self, name: &str, type_: Type) {
    if let Some(scope) = self.current_scope_mut() {
      scope.types.insert(name.to_owned(), type_);
    }
  }
  pub fn get_type(&self, name: &str) -> Option<&Type> {
    self.current_scope().and_then(|scope| scope.types.get(name))
  }

  // Range Management
  pub fn declare_variable_range(&mut self, name: &str, range: Range) {
    if let Some(scope) = self.current_scope_mut() {
      scope.ranges.insert(name.to_owned(), range);
    }
  }
  pub fn get_variable_range(&self, name: &str) -> Option<Range> {
    self.current_scope().and_then(|scope| scope.ranges.get(name).cloned())
  }

  pub fn is_global_scope(&self) -> bool {
    if self.scope_pointer == 0 && self.scopes.len() == 1 {
      return true;
    }
    return false;
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
  pub variables: BTreeMap<String, Type>,
  pub types: BTreeMap<String, Type>,
  pub unused_variables: BTreeSet<String>,
  pub ranges: BTreeMap<String, Range>,
  pub local_declarations: BTreeMap<String, bool>,
}

impl Scope {
  pub fn new() -> Self {
    Scope {
      local_declarations: BTreeMap::new(),
      variables: BTreeMap::new(),
      types: BTreeMap::new(),
      unused_variables: BTreeSet::new(),
      ranges: BTreeMap::new(),
    }
  }
}
