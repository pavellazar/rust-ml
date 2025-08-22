use super::pool::Pool;

pub struct Dataset {/* fields omitted */}

impl Dataset {
  pub fn from_csv(path: &str, target_column: &str, cat_features: Option<&[usize]>) -> Self {
    unimplemented!()
  }
  pub fn to_pool(&self, cat_features: Option<&[usize]>) -> Pool {
    unimplemented!()
  }
}
