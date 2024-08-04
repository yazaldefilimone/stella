mod ast;
mod checker;
mod cli;
mod context;
mod diagnostics;
mod lexer;
mod modules;
mod parser;
mod stdlib;
mod types;
mod utils;

use checker::Checker;
use parser::parser::Parser;

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
  let mut parser = Parser::new(&raw, path_name);
  let program = parser.parse_program();
  let mut checker = Checker::new(path_name, &raw);
  let type_result = checker.check(&program);
  if type_result.is_err() || checker.diagnostics.error_count > 0 {
    checker.diagnostics.emit_all(&raw, path_name);
    return;
  }
  let type_ = type_result.unwrap();
  println!("Result: {}", type_.to_string());
}
fn run_compile(path_name: &str) {
  let raw = std::fs::read_to_string(path_name).unwrap();
  let mut parser = Parser::new(&raw, path_name);
  let program = parser.parse_program();
  println!("{:#?}", program);
}
