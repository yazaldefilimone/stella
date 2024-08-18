use std::fmt::{self};

use crate::{
  ast::{
    ast::{BinaryOperator, UnaryOperator},
    tokens::TokenKind,
  },
  types::{
    FunctionType, GenericCallType, GenericType, GroupType, IdentifierType, OptionType, TableType, Type, UnionType,
    VariadicType,
  },
};

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Type::Number => write!(f, "number"),
      Type::String => write!(f, "string"),
      Type::Boolean => write!(f, "boolean"),
      Type::Nil => write!(f, "nil"),
      Type::Unknown => write!(f, "unknown"),
      Type::Table(table) => write!(f, "{}", table),
      Type::Function(function) => write!(f, "{}", function),
      Type::Generic(generic) => write!(f, "{}", generic),
      Type::Union(union) => write!(f, "{}", union),
      Type::Option(option) => write!(f, "{}", option),
      Type::Identifier(identifier) => write!(f, "{}", identifier),
      Type::Group(group) => write!(f, "{}", group),
      Type::GenericCall(generic_call) => write!(f, "{}", generic_call),
      Type::Variadic(variadic) => write!(f, "{}", variadic),
    }
  }
}

impl fmt::Display for IdentifierType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl fmt::Display for GenericCallType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let types_str = self.types.iter().map(Type::to_string).collect::<Vec<_>>().join(", ");
    write!(f, "{}<{}>", self.name, types_str)
  }
}

impl fmt::Display for GroupType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.types.len() == 1 {
      write!(f, "{}", self.types[0])
    } else {
      let types_str = self.types.iter().map(Type::to_string).collect::<Vec<_>>().join(", ");
      write!(f, "({})", types_str)
    }
  }
}

impl fmt::Display for VariadicType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "...{}", self.inner_type)
  }
}

impl fmt::Display for UnionType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let types_str = self.types.iter().map(Type::to_string).collect::<Vec<_>>().join(", ");
    write!(f, "union<{}>", types_str)
  }
}

impl fmt::Display for OptionType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "option<{}>", self.inner_type)
  }
}

impl fmt::Display for TableType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let array_str = self
      .array
      .as_ref()
      .map(|array| format!("<{}>", array.iter().map(Type::to_string).collect::<Vec<_>>().join(", ")))
      .unwrap_or_else(String::new);

    let map_str = self
      .map
      .as_ref()
      .map(|map| format!("<{}>", map.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<_>>().join(", ")))
      .unwrap_or_else(String::new);

    write!(f, "table{}{}", array_str, map_str)
  }
}

impl fmt::Display for FunctionType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let params_str = self.params.iter().map(Type::to_string).collect::<Vec<_>>().join(", ");
    write!(f, "function({}): {}", params_str, self.return_type)
  }
}

impl fmt::Display for GenericType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let types_str = self.variables.join(", ");
    if types_str.is_empty() {
      write!(f, "{}", self.name)
    } else {
      write!(f, "{}<{}>", self.name, types_str)
    }
  }
}

impl fmt::Display for BinaryOperator {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      BinaryOperator::Add => write!(f, "+"),
      BinaryOperator::Subtract => write!(f, "-"),
      BinaryOperator::Multiply => write!(f, "*"),
      BinaryOperator::Divide => write!(f, "/"),
      BinaryOperator::Modulus => write!(f, "%"),
      BinaryOperator::And => write!(f, "and"),
      BinaryOperator::Or => write!(f, "or"),
      BinaryOperator::Equal => write!(f, "=="),
      BinaryOperator::NotEqual => write!(f, "~="),
      BinaryOperator::LessThan => write!(f, "<"),
      BinaryOperator::GreaterThan => write!(f, ">"),
      BinaryOperator::LessThanOrEqual => write!(f, "<="),
      BinaryOperator::GreaterThanOrEqual => write!(f, ">="),
      BinaryOperator::DoubleDot => write!(f, ".."),
      BinaryOperator::DoubleSlash => write!(f, "//"),
    }
  }
}

impl fmt::Display for UnaryOperator {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      UnaryOperator::Negate => write!(f, "-"),
      UnaryOperator::Not => write!(f, "not"),
      UnaryOperator::Hash => write!(f, "#"),
    }
  }
}
/*


*/
impl fmt::Display for TokenKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TokenKind::Identifier(name) => write!(f, "{}", name),
      TokenKind::Number(number) => write!(f, "{}", number.as_str()),
      TokenKind::String(string) => write!(f, "{}", string.as_str()),
      TokenKind::EOF => write!(f, "EOF"),
      TokenKind::Function => write!(f, "function"),
      TokenKind::Local => write!(f, "local"),
      TokenKind::If => write!(f, "if"),
      TokenKind::Then => write!(f, "then"),
      TokenKind::Else => write!(f, "else"),
      TokenKind::ElseIf => write!(f, "elseif"),
      TokenKind::End => write!(f, "end"),
      TokenKind::While => write!(f, "while"),
      TokenKind::Do => write!(f, "do"),
      TokenKind::For => write!(f, "for"),
      TokenKind::In => write!(f, "in"),
      TokenKind::Repeat => write!(f, "repeat"),
      TokenKind::Until => write!(f, "until"),
      TokenKind::Return => write!(f, "return"),
      TokenKind::Break => write!(f, "break"),
      TokenKind::True => write!(f, "true"),
      TokenKind::False => write!(f, "false"),
      TokenKind::Nil => write!(f, "nil"),
      TokenKind::Type => write!(f, "type"),
      TokenKind::Enum => write!(f, "enum"),
      TokenKind::Continue => write!(f, "continue"),
      TokenKind::Assign => write!(f, "="),
      TokenKind::PlusAssign => write!(f, "+="),
      TokenKind::MinusAssign => write!(f, "-="),
      TokenKind::StarAssign => write!(f, "*="),
      TokenKind::SlashAssign => write!(f, "/="),
      TokenKind::NotEqual => write!(f, "~="),
      TokenKind::LessEqual => write!(f, "<="),
      TokenKind::GreaterEqual => write!(f, ">="),
      TokenKind::DoubleDot => write!(f, ".."),
      TokenKind::TripleDot => write!(f, "..."),
      TokenKind::LeftParen => write!(f, "("),
      TokenKind::RightParen => write!(f, ")"),
      TokenKind::LeftBrace => write!(f, "{}", "{"),
      TokenKind::RightBrace => write!(f, "{}", "}"),
      TokenKind::LeftBracket => write!(f, "["),
      TokenKind::RightBracket => write!(f, "]"),
      TokenKind::Comma => write!(f, ","),
      TokenKind::Semicolon => write!(f, ";"),
      TokenKind::Colon => write!(f, ":"),
      TokenKind::DoubleColon => write!(f, "::"),
      TokenKind::Dot => write!(f, "."),
      TokenKind::Tilde => write!(f, "~"),
      TokenKind::Hash => write!(f, "#"),
      TokenKind::Plus => write!(f, "+"),
      TokenKind::Minus => write!(f, "-"),
      TokenKind::Star => write!(f, "*"),
      TokenKind::Slash => write!(f, "/"),
      TokenKind::DoubleSlash => write!(f, "//"),
      TokenKind::Not => write!(f, "not"),
      TokenKind::Percent => write!(f, "%"),
      TokenKind::Equal => write!(f, "=="),
      TokenKind::Less => write!(f, "<"),
      TokenKind::Greater => write!(f, ">"),
      TokenKind::Or => write!(f, "or"),
      TokenKind::And => write!(f, "and"),
      TokenKind::Comment(_) => write!(f, "comment"),
      TokenKind::BlockComment(_) => write!(f, "block comment"),
      TokenKind::Require => write!(f, "require"),
    }
  }
}
