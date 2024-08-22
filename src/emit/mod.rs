use crate::ast::ast;

pub fn emit(progrma: &ast::Program, _path_name: &str) -> String {
  println!("emitting lua code...");
  let raw = progrma.emit(&mut String::new());
  return raw;
}
