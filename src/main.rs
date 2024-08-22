mod ast;
mod checker;
mod cli;
mod context;
mod diagnostics;
mod emit;
mod formatting;
mod lexer;
mod modules;
mod parser;
mod stdlib;
mod types;
mod utils;

use std::{
  fs::{self, File},
  io::Write,
  path::Path,
};

use checker::Checker;
use parser::parser::Parser;
use rlua::Lua;
use stella_checker::utils::highlight_text_with_red;

const OUTPUT_DIRECTORY: &str = "build";

fn main() {
  let matches = cli::command_line();

  match matches.subcommand() {
    Some(("check", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      run_check(path_name);
    }

    Some(("compile", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      let result = run_compile(path_name);
      if result.is_err() {
        std::process::exit(1);
      }
    }
    Some(("run", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      run(path_name);
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
  println!("Result Type: {}", type_);
}

fn run_compile(path_name: &str) -> Result<(), std::io::Error> {
  let raw = std::fs::read_to_string(path_name).unwrap();
  let mut parser = Parser::new(&raw, path_name);
  let program = parser.parse_program();
  let mut checker = Checker::new(path_name, &raw);
  let type_result = checker.check(&program);

  if type_result.is_err() || checker.diagnostics.error_count > 0 {
    checker.diagnostics.emit_all(&raw, path_name);
    std::process::exit(1);
  }

  let output = create_output_directory(path_name);
  let raw = emit::emit(&program, &path_name);
  let path = Path::new(&output);
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?;
  }
  let mut file = File::create(path)?;
  file.write_all(raw.as_bytes())?;
  Ok(())
}

fn run(path_name: &str) {
  let raw = std::fs::read_to_string(path_name).unwrap();
  let mut parser = Parser::new(&raw, path_name);
  let program = parser.parse_program();
  let mut checker = Checker::new(path_name, &raw);
  let type_result = checker.check(&program);

  if type_result.is_err() || checker.diagnostics.error_count > 0 {
    checker.diagnostics.emit_all(&raw, path_name);
    return;
  }
  let raw = emit::emit(&program, &path_name);
  let lua = Lua::new();
  let result = lua.load(raw).exec();

  match result {
    Ok(_) => {}
    Err(err) => {
      println!("{}", highlight_text_with_red(remove_location(err.to_string().as_str()).as_str()));
    }
  }
}

fn create_output_directory(path_name: &str) -> String {
  let output_path = std::path::Path::new(OUTPUT_DIRECTORY);
  if !output_path.exists() {
    std::fs::create_dir(output_path).expect("failed to create output directory");
  }
  let out_path = format!("{}/{}", OUTPUT_DIRECTORY, path_name.replace("./", ""));
  return out_path;
}

fn remove_location(error_message: &str) -> String {
  if let Some(start) = error_message.find("[string \"") {
    if let Some(end) = error_message[start..].find(": ") {
      let clean_message = format!("{}{}", &error_message[..start], &error_message[start + end + 1..].trim());
      return clean_message.trim().to_string();
    }
  }
  error_message.to_string()
}
