// Lua module loader

use std::{collections::HashMap, path::PathBuf};

pub struct Loader {
  pub cache: HashMap<String, String>,
}

impl Loader {
  pub fn new() -> Self {
    Loader { cache: HashMap::new() }
  }
  pub fn load_module_from_path(&mut self, path: &PathBuf) -> Result<String, String> {
    if self.cache.contains_key(path.to_str().unwrap()) {
      let content = self.cache.get(path.to_str().unwrap()).unwrap();
      return Ok(content.to_string());
    }

    let content = std::fs::read_to_string(path).expect("Failed to read file");

    self.cache.insert(path.to_str().unwrap().to_string(), content.clone());

    return Ok(content);
  }
}
