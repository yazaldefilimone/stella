use crate::ast::tokens::TokenKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Precedence {
  Assignment,
  Or,
  And,
  Equality,
  Comparison,
  Term,
  Factor,
  Unary,
  Primary,
  Concat,
}

impl Precedence {
  pub fn operators(self) -> &'static [TokenKind] {
    match self {
      Precedence::Or => &[TokenKind::Or],
      Precedence::And => &[TokenKind::And],
      Precedence::Equality => &[TokenKind::Equal, TokenKind::NotEqual],
      Precedence::Comparison => &[TokenKind::Less, TokenKind::LessEqual, TokenKind::Greater, TokenKind::GreaterEqual],
      Precedence::Term => &[TokenKind::Plus, TokenKind::Minus],
      Precedence::Concat => &[TokenKind::DoubleDot],
      Precedence::Factor => &[TokenKind::Star, TokenKind::Slash, TokenKind::Percent],
      _ => &[], // no binary operators for unary and primary precedence :(
    }
  }

  pub fn next(self) -> Precedence {
    match self {
      Precedence::Assignment => Precedence::Or,
      Precedence::Or => Precedence::And,
      Precedence::And => Precedence::Equality,
      Precedence::Equality => Precedence::Comparison,
      Precedence::Comparison => Precedence::Concat,
      Precedence::Concat => Precedence::Term,
      Precedence::Term => Precedence::Factor,
      Precedence::Factor => Precedence::Unary,
      Precedence::Unary => Precedence::Primary,
      Precedence::Primary => Precedence::Primary,
    }
  }
}
