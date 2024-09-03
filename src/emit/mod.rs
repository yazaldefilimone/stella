use crate::ast::ast::*;
pub mod emitter;

impl Program {
  pub fn emit(&self) -> String {
    self.statements.iter().map(|stmt| stmt.emit()).collect::<Vec<String>>().join("\n")
  }
}

impl Statement {
  pub fn emit(&self) -> String {
    match self {
      Statement::Function(function) => function.emit(),
      Statement::Return(return_) => return_.emit(),
      Statement::If(if_) => if_.emit(),
      Statement::While(while_) => while_.emit(),
      Statement::Repeat(repeat) => repeat.emit(),
      Statement::For(for_) => for_.emit(),
      Statement::Break(break_) => break_.emit(),
      Statement::Goto(goto) => goto.emit(),
      Statement::Block(block) => block.emit(),
      Statement::Empty(empty) => empty.emit(),
      Statement::TypeDeclaration(declaration) => declaration.emit(),
      Statement::Continue(continue_) => continue_.emit(),
      Statement::Local(local) => local.emit(),
      Statement::Expression(expression) => expression.emit(),
    }
  }
}

impl LocalStatement {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("local ");
    for (index, variable) in self.variables.iter().enumerate() {
      if index > 0 {
        raw.push_str(", ");
      }
      raw.push_str(&variable.emit());
    }
    raw.push_str(" = ");
    for (index, initializer) in self.initializer.iter().enumerate() {
      if index > 0 {
        raw.push_str(", ");
      }
      raw.push_str(&initializer.emit());
    }
    raw.push_str("\n");
    return raw;
  }
}

impl FunctionStatement {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("function ");
    raw.push_str(&self.name.lexeme());
    raw.push_str("(");
    for (index, argument) in self.arguments.iter().enumerate() {
      if index > 0 {
        raw.push_str(", ");
      }
      raw.push_str(&argument.emit());
    }
    raw.push_str(")");
    raw.push_str("\n");
    raw.push_str(&self.body.emit());
    raw.push_str("\nend\n");
    return raw;
  }
}

impl ReturnStatement {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("return ");
    for (index, value) in self.values.iter().enumerate() {
      if index > 0 {
        raw.push_str(", ");
      }
      raw.push_str(&value.emit());
    }
    raw.push_str("\n");
    return raw;
  }
}

impl ContinueStatement {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("continue\n");
    return raw;
  }
}

impl IfStatement {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("if ");
    raw.push_str(&self.condition.emit());
    raw.push_str(" then\n");
    raw.push_str(&self.then_body.emit());
    for else_if_branch in &self.else_if_branches {
      raw.push_str(&else_if_branch.emit());
    }
    if let Some(else_body) = &self.else_body {
      raw.push_str(&else_body.emit());
    }
    raw.push_str("\nend\n");
    return raw;
  }
}

impl ElseIfStatement {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("elseif ");
    raw.push_str(&self.condition.emit());
    raw.push_str(" then\n");
    raw.push_str(&self.then_branch.emit());
    raw.push_str("\n");
    return raw;
  }
}

impl WhileStatement {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("while ");
    raw.push_str(&self.condition.emit());
    raw.push_str(" do\n");
    raw.push_str(&self.body.emit());
    raw.push_str("\nend\n");
    return raw;
  }
}

impl RepeatStatement {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("repeat\n");
    raw.push_str(&self.body.emit());
    raw.push_str("until ");
    raw.push_str(&self.condition.emit());
    raw.push_str("\n");
    return raw;
  }
}

impl ForStatement {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("for ");
    raw.push_str(&self.init.emit());
    raw.push_str(", ");
    raw.push_str(&self.limit.emit());
    if let Some(step) = &self.step {
      raw.push_str(", ");
      raw.push_str(&step.emit());
    }
    raw.push_str(" do\n");
    raw.push_str(&self.body.emit());
    raw.push_str("\nend\n");
    return raw;
  }
}
impl BreakStatement {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("break\n");
    return raw;
  }
}
impl GotoStatement {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("goto ");
    if let Some(label) = &self.label {
      raw.push_str(&format!("{}\n", label));
    }
    raw.push_str("\n");
    return raw;
  }
}

impl BlockStatement {
  fn emit(&self) -> String {
    let mut raw = String::new();
    for statement in &self.statements {
      raw.push_str(&statement.emit());
    }
    return raw;
  }
}

impl EmptyStatement {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("\n");
    return raw;
  }
}

impl TypeDeclaration {
  fn emit(&self) -> String {
    let raw = String::new();
    // don't emit type's
    return raw;
  }
}

impl Variable {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str(&self.name.lexeme());
    // don't emit type's
    return raw;
  }
}

impl LiteralExpression {
  fn emit(&self) -> String {
    match self {
      LiteralExpression::Number(number) => number.emit(),
      LiteralExpression::String(string) => string.emit(),
      LiteralExpression::Boolean(boolean) => boolean.emit(),
      LiteralExpression::Nil(nil) => nil.emit(),
    }
  }
}

impl NumberLiteral {
  fn emit(&self) -> String {
    self.value.to_string()
  }
}

impl StringLiteral {
  fn emit(&self) -> String {
    format!("\"{}\"", self.value)
  }
}

impl BooleanLiteral {
  fn emit(&self) -> String {
    if self.value {
      "true".to_string()
    } else {
      "false".to_string()
    }
  }
}

impl NilLiteral {
  fn emit(&self) -> String {
    "nil".to_string()
  }
}

impl UnaryOperator {
  fn emit(&self) -> String {
    return self.to_str().to_string();
  }
}

impl BinaryOperator {
  fn emit(&self) -> String {
    return self.to_string();
  }
}

impl Expression {
  fn emit(&self) -> String {
    match self {
      Expression::Literal(literal) => literal.emit(),
      Expression::Identifier(identifier) => identifier.emit(),
      Expression::Call(call) => call.emit(),
      Expression::Unary(unary) => unary.emit(),
      Expression::Grouped(grouped) => grouped.emit(),
      Expression::Binary(binary) => binary.emit(),
      Expression::Require(require) => require.emit(),
      Expression::Function(function) => function.emit(),
      Expression::Table(table) => table.emit(),
      Expression::Member(member) => member.emit(),
      Expression::Index(index) => index.emit(),
      Expression::Assign(assign) => assign.emit(),
      Expression::Variable(variable) => variable.emit(),
    }
  }
}

impl AssignExpression {
  fn emit(&self) -> String {
    let mut raw = String::new();
    for (index, variable) in self.left.iter().enumerate() {
      if index > 0 {
        raw.push_str(", ");
      }
      raw.push_str(&variable.emit());
    }
    raw.push_str(" = ");
    for (index, initializer) in self.right.iter().enumerate() {
      if index > 0 {
        raw.push_str(", ");
      }
      raw.push_str(&initializer.emit());
    }
    return raw;
  }
}
impl RequireExpression {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("require ");
    raw.push_str(&self.module_name.lexeme());
    raw.push_str("\n");
    return raw;
  }
}

impl GroupedExpression {
  fn emit(&self) -> String {
    let mut raw = String::new();
    for (index, expression) in self.expressions.iter().enumerate() {
      if index > 0 {
        raw.push_str(", ");
      }
      raw.push_str(&expression.emit());
    }
    return raw;
  }
}

impl Identifier {
  fn emit(&self) -> String {
    return self.name.clone();
  }
}

impl CallExpression {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str(&self.left.emit());
    raw.push_str("(");
    raw.push_str(&self.args.emit());
    raw.push_str(")");
    return raw;
  }
}

impl UnaryExpression {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str(&self.operator.emit());
    raw.push_str(&self.operand.emit());
    return raw;
  }
}

impl MemberExpression {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str(&self.base.emit());
    raw.push_str(".");
    raw.push_str(&self.identifier.emit());
    return raw;
  }
}

impl IndexExpression {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str(&self.base.emit());
    raw.push_str("[");
    raw.push_str(&self.index.emit());
    raw.push_str("]");
    return raw;
  }
}
impl TableExpression {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("{");
    for (index, value) in self.values.iter().enumerate() {
      if index > 0 {
        raw.push_str(", ");
      }
      raw.push_str(&value.0.emit());
      if let Some(value) = &value.1 {
        raw.push_str(" = ");
        raw.push_str(&value.emit());
      }
    }
    raw.push_str("}");
    return raw;
  }
}

impl FunctionExpression {
  fn emit(&self) -> String {
    let mut raw = String::new();
    raw.push_str("function ");
    raw.push_str("(");
    for (index, argument) in self.arguments.iter().enumerate() {
      if index > 0 {
        raw.push_str(", ");
      }
      raw.push_str(&argument.emit());
    }
    raw.push_str(")");
    raw.push_str("\n");
    raw.push_str(&self.body.emit());
    raw.push_str("\nend\n");
    return raw;
  }
}

impl BinaryExpression {
  fn emit(&self) -> String {
    let mut raw = String::new();
    // todo: fix this...
    raw.push_str("(");
    raw.push_str(&self.left.emit());
    raw.push_str(&self.operator.emit());
    raw.push_str(&self.right.emit());
    raw.push_str(")");
    return raw;
  }
}

impl AssignExpresion {
  fn emit(&self) -> String {
    let mut raw = String::new();
    for (index, variable) in self.variables.iter().enumerate() {
      if index > 0 {
        raw.push_str(", ");
      }
      raw.push_str(&variable.emit());
    }
    raw.push_str(" = ");
    for (index, initializer) in self.initializer.iter().enumerate() {
      if index > 0 {
        raw.push_str(", ");
      }
      raw.push_str(&initializer.emit());
    }
    return raw;
  }
}
