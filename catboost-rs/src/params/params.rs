use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Params {
  inner: HashMap<String, String>,
}

impl Params {
  pub fn new() -> Self {
    Params {
      inner: HashMap::new(),
    }
  }
  pub fn set(&mut self, key: &str, value: &str) {
    self.inner.insert(key.to_string(), value.to_string());
  }
  pub fn get(&self, key: &str) -> Option<&str> {
    self.inner.get(key).map(|s| s.as_str())
  }
}

impl Clone for Params {
  fn clone(&self) -> Self {
    Params {
      inner: self.inner.clone(),
    }
  }
}
