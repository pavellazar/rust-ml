use crate::data::{Dataset, Pool};
use crate::params::Params;
use crate::metrics::{FeatureImportance, Metric};

pub struct CatBoost {/* fields omitted */}

impl CatBoost {
  pub fn new(params: &Params) -> Self { unimplemented!() }
  pub fn train(&mut self, pool: &Pool, eval_set: Option<&Pool>, early_stopping_rounds: Option<usize>, verbose: bool) { unimplemented!() }
  pub fn predict(&self, pool: &Pool) -> Vec<f32> { unimplemented!() }
  pub fn save(&self, path: &str) { unimplemented!() }
  pub fn load(path: &str) -> Self { unimplemented!() }
  pub fn get_feature_importance(&self, pool: Option<&Pool>) -> FeatureImportance { unimplemented!() }
  pub fn eval_metrics(&self, pool: &Pool, metrics: &[Metric], ntree_end: Option<usize>) -> Vec<f32> { unimplemented!() }
  pub fn best_iteration(&self) -> Option<usize> { unimplemented!() }
  pub fn best_score(&self) -> Option<f32> { unimplemented!() }
}
