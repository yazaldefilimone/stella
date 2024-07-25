use super::Checker;
use crate::ast::ast;
use crate::diagnostics::Diagnostic;
use crate::types::Type;

impl Checker {
  pub fn check_statement(&mut self, statement: &ast::Statement) -> Result<Type, Diagnostic> {
    match statement {
      ast::Statement::VariableDeclaration(vaiable) => self.check_variable_declaration(vaiable),
      ast::Statement::EmptyStatement(empty) => {
        self.check_empty_statement(empty);
        Ok(Type::Nil)
      }
      ast::Statement::BlockStatement(block) => self.check_block_statement(block),
      ast::Statement::AssignStatement(assign) => self.check_assign_statement(assign),
      ast::Statement::FunctionStatement(function) => self.check_function_statement(function),
      ast::Statement::ReturnStatement(return_) => self.check_return_statement(return_),
      ast::Statement::CallStatement(call) => self.check_call_statement(call),
      ast::Statement::IfStatement(if_) => {
        self.check_if_statement(if_);
        Ok(Type::Nil)
      }
      ast::Statement::WhileStatement(while_) => {
        self.check_while_statement(while_);
        Ok(Type::Nil)
      }
      ast::Statement::RepeatStatement(repeat) => {
        self.check_repeat_statement(repeat);
        Ok(Type::Nil)
      }
      _ => todo!("Implement more statement checks"),
    }
  }

  pub fn check_call_statement(&mut self, call: &ast::CallStatement) -> Result<Type, Diagnostic> {
    return self.check_expression(&call.expression);
  }
}
