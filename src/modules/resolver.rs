use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};

pub struct Resolver {
  pub cache: HashMap<String, PathBuf>,
  pub paths: Vec<PathBuf>,
}

impl Resolver {
  pub fn new() -> Self {
    Resolver { cache: HashMap::new(), paths: vec![] }
  }

  pub fn add_search_path(&mut self, input_file: &str) {
    let path = Path::new(input_file);
    let mut path = PathBuf::from(path);
    path.pop(); // remove file name
    self.paths.push(path);
  }
  pub fn resolve(&mut self, module_name: &str) -> Result<PathBuf, String> {
    if self.cache.contains_key(module_name) {
      return Ok(self.cache.get(module_name).unwrap().clone());
    }
    for search_path in &self.paths {
      let candidate = search_path.join(format!("{}.lua", module_name));
      if !candidate.exists() {
        continue;
      }
      self.cache.insert(module_name.to_owned(), candidate.clone());
      return Ok(candidate);
    }
    Err(format!("Module '{}' not found", module_name))
  }
}
