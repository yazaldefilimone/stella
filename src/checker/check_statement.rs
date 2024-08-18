use super::type_utils::CheckResult;
use super::Checker;
use crate::ast::ast;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_statement(&mut self, statement: &ast::Statement) -> CheckResult<Option<Type>> {
    match statement {
      // ast::Statement::VariableDeclaration(vaiable) => self.check_variable_declaration(vaiable),
      ast::Statement::Empty(empty) => self.check_empty_statement(empty),
      ast::Statement::Block(block) => self.check_block_statement(block),
      ast::Statement::Function(function) => self.check_function_statement(function),
      ast::Statement::Return(return_) => self.check_return_statement(return_),
      ast::Statement::If(if_) => self.check_if_statement(if_),
      ast::Statement::While(while_) => self.check_while_statement(while_),
      ast::Statement::Repeat(repeat) => self.check_repeat_statement(repeat),
      ast::Statement::For(for_) => self.check_for_statement(for_),
      ast::Statement::Expression(expression) => self.check_expression(&expression),
      ast::Statement::TypeDeclaration(declaration) => self.check_type_declaration(declaration),
      ast::Statement::Local(local) => self.check_local_statement(local),
      _ => todo!("Implement more statement checks: {:#?}", statement),
    }
  }
}
