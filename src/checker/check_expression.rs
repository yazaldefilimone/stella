use super::type_utils::CheckResult;
use super::Checker;
use crate::ast::ast;
use crate::types::Type;

impl<'a> Checker<'a> {
  pub fn check_expression(&mut self, expression: &ast::Expression) -> CheckResult<Option<Type>> {
    match expression {
      ast::Expression::Literal(literal) => self.check_literal_expression(literal),
      ast::Expression::Identifier(ident) => self.check_identifier(ident),
      ast::Expression::Call(call) => self.check_call_expression(call),
      ast::Expression::Binary(binary_expr) => self.check_binary_expression(binary_expr),
      ast::Expression::Require(require) => self.check_require_expression(require),
      ast::Expression::Unary(unary_expr) => self.check_unary_expression(unary_expr),
      ast::Expression::Grouped(grup_expr) => self.check_grouped_expression(grup_expr),
      ast::Expression::Function(function) => self.check_function_expression(function),
      ast::Expression::Table(table) => self.check_table_expression(table),
      ast::Expression::Member(member) => self.check_member_expression(member),
      ast::Expression::Index(index) => self.check_index_expression(index),
      ast::Expression::Assign(assign) => self.check_assign_expression(assign),
      _ => unreachable!(),
    }
  }
}
