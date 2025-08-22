use super::dataset::Dataset;

pub struct Pool {/* fields omitted */}

impl Pool {
  pub fn from_dataset(dataset: &Dataset, cat_features: Option<&[usize]>) -> Self { unimplemented!() }
  pub fn num_rows(&self) -> usize { unimplemented!() }
  pub fn num_cols(&self) -> usize { unimplemented!() }
}
