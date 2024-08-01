#![allow(dead_code, unused_variables)]

pub mod check_assign_statement;
pub mod check_binary_expression;
pub mod check_block_statement;
pub mod check_call_expression;
pub mod check_empty_statement;
pub mod check_expression;
pub mod check_for_statement;
pub mod check_function_statement;
pub mod check_grouped_expression;
pub mod check_identifier;
pub mod check_if_statement;
pub mod check_literal_expression;
pub mod check_repeat_statement;
pub mod check_require_expression;
pub mod check_return_statement;
pub mod check_statement;
pub mod check_type;
pub mod check_unary_expression;
pub mod check_variable_declaration;
pub mod check_while_statement;

use crate::ast::ast;
use crate::ast::tokens::Token;
use crate::context::context::Context;
use crate::diagnostics::{Diagnostic, DiagnosticManager, TypeError, TypeWarning};
use crate::modules::loader::Loader;
use crate::modules::resolver::Resolver;
use crate::types::Type;
use crate::utils::location::Location;

pub struct Checker<'a> {
  pub ctx: Context,
  pub file_name: String,
  pub diagnostics: DiagnosticManager,
  pub loader: Loader,
  pub resolver: Resolver,
  pub raw: &'a str,
}

impl<'a> Checker<'a> {
  pub fn new(file_name: &str, raw: &'a str) -> Checker<'a> {
    let ctx = Context::new();
    let loader = Loader::new();
    let mut resolver = Resolver::new();
    resolver.add_search_path(file_name);
    let diagnostics = DiagnosticManager::new();
    Checker { ctx, file_name: file_name.to_string(), diagnostics, loader, resolver, raw }
  }
  pub fn check(&mut self, program: &ast::Program) -> Result<Type, Diagnostic> {
    let mut last_t = Type::Nil;
    for statement in &program.statements {
      match self.check_statement(statement) {
        Ok(ty) => last_t = ty,
        Err(diag) => self.diagnostics.add(diag),
      }
    }

    self.show_diagnostics();
    return Ok(last_t);
  }

  pub fn check_break_statement(&mut self, break_: &ast::BreakStatement) {
    // Empty statements don't change the type context
  }

  pub fn create_diagnostic(&self, error: TypeError) -> Diagnostic {
    error.into()
  }

  pub fn check_used_variable_in_current_scope(&mut self) {
    let used_variables = self.ctx.check_unused_variables();
    for used_variable in used_variables {
      let used_variable_location = self.ctx.get_variable_location(&used_variable);
      if used_variable_location.is_none() {
        continue;
      }
      let report = TypeWarning::UnusedVariable(used_variable.clone(), used_variable_location);
      self.diagnostics.add(report.into());
    }
  }

  pub fn show_diagnostics(&mut self) {
    self.check_used_variable_in_current_scope();
    self.diagnostics.emit_all(self.raw, &self.file_name);
  }

  pub fn _declaration(
    &mut self,
    names: &Vec<(Token, Option<Type>)>,
    ty: Type,
    local: bool,
    loc: Location,
  ) -> Result<(), Diagnostic> {
    if let Type::Grup(group) = ty {
      for (index, token) in names.iter().enumerate() {
        let tt = if index >= group.types.len() { Type::Nil } else { group.types[index].clone() };
        self._declare_variable(token, tt, local)?;
      }
      return Ok(());
    }
    for (index, token) in names.iter().enumerate() {
      if index == 0 {
        self._declare_variable(token, ty.clone(), local)?;
        continue;
      }

      self._declare_variable(token, Type::Nil, local)?;
    }

    Ok(())
  }

  pub fn _declare_variable(&mut self, value: &(Token, Option<Type>), ty: Type, local: bool) -> Result<(), Diagnostic> {
    let lexeme = value.0.lexeme();

    if let Some(value_type) = &value.1 {
      if !value_type.check_match(&ty) {
        let location = value.0.location.clone();
        let diagnostic = TypeError::TypeMismatchAssignment(value_type.to_string(), ty.to_string(), Some(location));
        return Err(self.create_diagnostic(diagnostic));
      }
    }

    if local && self.ctx.defined_in_current_scope(lexeme) {
      let location = value.0.location.clone();
      if self.ctx.lookup_local_variable(lexeme) {
        let diagnostic = TypeError::RedeclaredInSameScope(lexeme.to_string(), Some(location));
        return Err(self.create_diagnostic(diagnostic));
      }

      let diagnostic = TypeWarning::ShadowedVariable(lexeme.to_string(), Some(location));
      self.diagnostics.add(diagnostic.into());
    }

    if let Some(previous_type) = self.ctx.get_variable_in_global_scope(lexeme) {
      let loc = value.0.location.clone();
      if !ty.check_match(&previous_type) {
        let diagnostic = TypeError::TypeMismatchAssignment(previous_type.to_string(), ty.to_string(), Some(loc));
        return Err(self.create_diagnostic(diagnostic));
      }
      self.ctx.set_variable_location(lexeme, loc);
    }

    // if it's a local variable, then set it in the current scope
    if local {
      self.ctx.set_local_variable(lexeme);
    }

    self.ctx.declare_variable(lexeme, ty);
    return Ok(());
  }
}
