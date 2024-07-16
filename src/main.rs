mod ast;
mod checker;
mod cli;
mod context;
mod diagnostics;
mod lexer;
mod parser;
mod utils;

use ast::tokens::TokenKind;
use lexer::Lexer;

fn main() {
  let matches = cli::command_line();

  match matches.subcommand() {
    Some(("check", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      run_check(path_name);
    }
    Some(("compile", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      run_compile(path_name);
    }
    _ => panic!("No subcommand provided."),
  }
}

fn run_check(path_name: &str) {
  let raw = std::fs::read_to_string(path_name).unwrap();
  let mut lexer = Lexer::new(raw);
  loop {
    let token = lexer.next_token();
    // println!("{:?}", token);
    if token.kind == TokenKind::EOF {
      break;
    }
  }
}
fn run_compile(path_name: &str) {
  println!("compile {:?}", path_name);
  println!("Oops, compile don't work yet. Coming soon. Stay tuned.");
}
