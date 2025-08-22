use super::dataset::Dataset;

pub struct Pool {
  pub x: Vec<Vec<f32>>,
  pub y: Vec<f32>,
}

impl Pool {
  pub fn from_xy(x: Vec<Vec<f32>>, y: Vec<f32>) -> Self {
    Pool { x, y }
  }
  pub fn from_dataset(_dataset: &Dataset, _cat_features: Option<&[usize]>) -> Self {
    Pool {
      x: vec![],
      y: vec![],
    }
  }
  pub fn num_rows(&self) -> usize {
    self.x.len()
  }
  pub fn num_cols(&self) -> usize {
    self.x.get(0).map_or(0, |row| row.len())
  }
}
