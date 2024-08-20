use super::{type_utils::CheckResult, Checker};
use crate::{ast::ast, diagnostics::TypeError, types::Type};

impl<'a> Checker<'a> {
  // expression = expression
  //
  pub fn assign_variables(&mut self, left: &ast::Expression, right: Option<&ast::Expression>) -> CheckResult<()> {
    match left {
      ast::Expression::Identifier(identifier) => self.assign_identifier(identifier, right),
      ast::Expression::Variable(variable) => self.assign_variable(variable, right),
      ast::Expression::Index(index) => self.assign_index(index, right),
      ast::Expression::Member(member) => self.assign_member(member, right),
      // todo: implement more assign expressions
      // left[expression] = expression
      // left.expression = expression
      // ...
      _ => todo!("Implement more assign expressions"),
    }
  }

  // age = 20
  //
  pub fn assign_identifier(&mut self, left: &ast::Identifier, right: Option<&ast::Expression>) -> CheckResult<()> {
    let lexeme = left.name.as_str();

    let right_type = match right {
      Some(right) => self.check_expression(right)?.unwrap_or(Type::Nil),
      None => Type::Nil,
    };

    let range = left.range.clone();
    let left_hand_side = &(lexeme, None);
    self.declare_global_variable(&left_hand_side, right_type, range)?;
    Ok(())
  }

  // age:number = 20
  //
  pub fn assign_variable(&mut self, left: &ast::Variable, right: Option<&ast::Expression>) -> CheckResult<()> {
    let lexeme = left.name.lexeme();

    let right_type = match right {
      Some(right) => self.check_expression(right)?.unwrap_or(Type::Nil),
      None => Type::Nil,
    };

    let range = left.get_range();
    let left_hand_side = &(lexeme, left.ty.clone());
    self.declare_global_variable(&left_hand_side, right_type, range)?;
    Ok(())
  }

  // a[1] = b[2]
  pub fn assign_index(&mut self, index: &ast::IndexExpression, right: Option<&ast::Expression>) -> CheckResult<()> {
    let index_type = self.check_index_expression(index)?.unwrap_or(Type::Nil);
    let right_type = match right {
      Some(right) => self.check_expression(right)?.unwrap_or(Type::Nil),
      None => Type::Nil,
    };
    let range = index.get_range();

    if !index_type.check_match(&right_type) {
      let diagnostic = TypeError::TypeMismatchAssignment(index_type.to_string(), right_type.to_string(), Some(range));
      return Err(self.create_diagnostic(diagnostic));
    }
    Ok(())
  }

  // a.b = b.c
  pub fn assign_member(&mut self, member: &ast::MemberExpression, right: Option<&ast::Expression>) -> CheckResult<()> {
    let member_type = self.check_member_expression(member)?.unwrap_or(Type::Nil);

    let right_type = match right {
      Some(right) => self.check_expression(right)?.unwrap_or(Type::Nil),
      None => Type::Nil,
    };
    let range = member.get_range();

    if !member_type.check_match(&right_type) {
      let diagnostic = TypeError::TypeMismatchAssignment(member_type.to_string(), right_type.to_string(), Some(range));
      return Err(self.create_diagnostic(diagnostic));
    }
    Ok(())
  }
}
