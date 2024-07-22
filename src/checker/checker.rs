#![allow(dead_code, unused_variables)]

use crate::ast::ast::{self, Type};
use crate::context::context::Context;
use crate::diagnostics::{Diagnostic, DiagnosticManager, TypeError, TypeWarning};

pub struct Checker {
  ctx: Context,
  pub diagnostics: DiagnosticManager,
}

impl Checker {
  pub fn new() -> Checker {
    Checker { ctx: Context::new(), diagnostics: DiagnosticManager::new() }
  }

  pub fn check(&mut self, program: &ast::Program) -> Result<Type, Diagnostic> {
    let mut last_t = Type::Void;
    for statement in &program.statements {
      match self.check_statement(statement) {
        Ok(ty) => last_t = ty,
        Err(diag) => self.diagnostics.add(diag),
      }
    }

    for unused_variable in self.ctx.check_unused_variables() {
      let unused_variable_location = self.ctx.get_variable_location(&unused_variable);
      if unused_variable_location.is_none() {
        continue;
      }
      let report = TypeWarning::UnusedVariable(unused_variable.clone(), unused_variable_location);
      self.diagnostics.add(report.into());
    }

    return Ok(last_t);
  }

  fn check_statement(&mut self, statement: &ast::Statement) -> Result<Type, Diagnostic> {
    match statement {
      ast::Statement::LocalStatement(local) => self.check_local_statement(local),
      ast::Statement::EmptyStatement(empty) => {
        self.check_empty_statement(empty);
        Ok(Type::Void)
      }
      ast::Statement::BlockStatement(block) => self.check_block_statement(block),
      ast::Statement::AssignStatement(assign) => self.check_assign_statement(assign),
      ast::Statement::FunctionStatement(function) => {
        self.check_function_statement(function);
        Ok(Type::Void)
      }
      ast::Statement::ReturnStatement(return_) => {
        self.check_return_statement(return_);
        Ok(Type::Void)
      }
      ast::Statement::IfStatement(if_) => {
        self.check_if_statement(if_);
        Ok(Type::Void)
      }
      ast::Statement::WhileStatement(while_) => {
        self.check_while_statement(while_);
        Ok(Type::Void)
      }
      ast::Statement::RepeatStatement(repeat) => {
        self.check_repeat_statement(repeat);
        Ok(Type::Void)
      }
      _ => todo!("Implement more statement checks"),
    }
  }

  fn check_t(&mut self, t: &Option<Type>) -> Type {
    match t {
      Some(t) => t.clone(),
      None => Type::Any,
    }
  }

  fn check_local_statement(&mut self, local: &ast::LocalStatement) -> Result<Type, Diagnostic> {
    let text_name = local.name.lexeme();
    let right_t = self.check_t(&local.type_);

    let left_t = if let Some(init) = &local.init {
      self.check_expression_statement(init).unwrap()
    } else {
      Type::Any
    };

    let location = local.location.clone();
    if !self.check_match_type(&left_t, &right_t) {
      let diagnostic = TypeError::MismatchedTypes(right_t.to_string(), left_t.to_string(), Some(location));
      return Err(self.create_diagnostic(diagnostic));
    }

    self.ctx.set_variable_location(text_name.as_str(), location);

    self.ctx.declare_variable(text_name.as_str(), right_t.clone());
    Ok(right_t)
  }

  fn check_empty_statement(&mut self, _empty: &ast::EmptyStatement) {
    // Empty statements don't change the type context
  }

  fn check_block_statement(&mut self, block: &ast::BlockStatement) -> Result<Type, Diagnostic> {
    let mut last_t = Type::Void;
    for statement in &block.body {
      match self.check_statement(statement) {
        Ok(ty) => last_t = ty,
        Err(diag) => self.diagnostics.add(diag),
      }
    }
    Ok(last_t)
  }

  fn check_expression_statement(&mut self, expression: &ast::ExpressionStatement) -> Result<Type, Diagnostic> {
    match expression {
      ast::ExpressionStatement::LiteralExpression(literal) => self.check_literal_expression(literal),
      ast::ExpressionStatement::IdentifierExpression(identifier) => self.check_identifier_expression(identifier),
      _ => todo!("Implement more expression checks"),
    }
  }

  fn check_identifier_expression(&mut self, identifier: &ast::IdentifierExpression) -> Result<Type, Diagnostic> {
    let text_name = identifier.name.clone();
    if !self.ctx.is_variable_defined(text_name.as_str()) {
      return Err(self.create_diagnostic(TypeError::UndeclaredVariable(
        text_name.to_string(),
        // TODO: hei :), use name location or a value location?
        Some(identifier.location.clone()),
      )));
    }
    self.ctx.use_variable(text_name.as_str());
    let type_ = self.ctx.get_variable(text_name.as_str()).unwrap().clone();
    Ok(type_)
  }

  fn check_literal_expression(&mut self, literal: &ast::LiteralExpression) -> Result<Type, Diagnostic> {
    match literal {
      ast::LiteralExpression::NumberLiteral(_) => Ok(Type::Number),
      ast::LiteralExpression::StringLiteral(_) => Ok(Type::String),
      ast::LiteralExpression::BoolLiteral(_) => Ok(Type::Boolean),
    }
  }

  fn check_assign_statement(&mut self, assign: &ast::AssignStatement) -> Result<Type, Diagnostic> {
    let right_t = self.check_expression_statement(&assign.value)?;
    let lexema = assign.name.lexeme();
    if !self.ctx.is_variable_defined(lexema.as_str()) {
      return Err(self.create_diagnostic(TypeError::UndeclaredVariable(
        lexema.to_string(),
        // TODO: hei :), use name location or a value location?
        Some(assign.name.location.clone()),
      )));
    }
    let left_t = self.ctx.get_variable(lexema.as_str()).unwrap().clone();
    if !self.check_match_type(&left_t, &right_t) {
      let location = Some(assign.location.clone());
      let diagnostic = TypeError::TypeMismatchAssignment(left_t.to_string(), right_t.to_string(), location);
      return Err(self.create_diagnostic(diagnostic));
    }
    if self.check_can_replace_type(&left_t, &right_t) {
      self.ctx.declare_variable(lexema.as_str(), right_t.clone());
      return Ok(right_t);
    }
    Ok(left_t)
  }

  fn check_function_statement(&mut self, function: &ast::FunctionStatement) {
    // Implement function statement checks
  }

  fn check_return_statement(&mut self, return_: &ast::ReturnStatement) {
    // Implement return statement checks
  }

  fn check_if_statement(&mut self, if_: &ast::IfStatement) {
    // Implement if statement checks
  }

  fn check_while_statement(&mut self, while_: &ast::WhileStatement) {
    // Implement while statement checks
  }

  fn check_repeat_statement(&mut self, repeat: &ast::RepeatStatement) {
    // Implement repeat statement checks
  }

  fn check_match_type(&self, left: &Type, right: &Type) -> bool {
    match (left, right) {
      (Type::Number, Type::Number) => true,
      (Type::String, Type::String) => true,
      (Type::Boolean, Type::Boolean) => true,
      (Type::Identifier(left), Type::Identifier(right)) => left == right,
      (Type::Any, _) => true,
      (_, Type::Any) => true,
      _ => false,
    }
  }

  fn check_can_replace_type(&self, left: &Type, right: &Type) -> bool {
    // if left is any, then it can replace any type with right
    match (left, right) {
      (Type::Any, Type::Any) => false,
      (Type::Any, _) => true,
      _ => false,
    }
  }

  fn create_diagnostic(&self, error: TypeError) -> Diagnostic {
    error.into()
  }
}
