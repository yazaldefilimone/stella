#![allow(dead_code)]

pub struct Emitter {
  indentation: usize,
  output: String,
}

impl Emitter {
  pub fn new() -> Self {
    Emitter { indentation: 0, output: String::new() }
  }

  pub fn indent(&mut self) {
    self.indentation += 1;
  }

  pub fn dedent(&mut self) {
    if self.indentation > 0 {
      self.indentation -= 1;
    }
  }

  pub fn emit_line(&mut self, line: &str) {
    self.output.push_str(&"  ".repeat(self.indentation));
    self.output.push_str(line);
    self.output.push('\n');
  }

  pub fn emit_raw(&mut self, text: &str) {
    self.output.push_str(text);
  }

  pub fn result(self) -> String {
    self.output
  }
}
