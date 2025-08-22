pub struct Booster {/* fields omitted */}
pub struct Dataset {/* fields omitted */}
pub struct Params {/* fields omitted */}
pub struct FeatureImportance {/* fields omitted */}

impl Booster {
  pub fn new(params: &Params) -> Self {
    unimplemented!()
  }
  pub fn train(
    &mut self,
    train_data: &Dataset,
    valid_data: Option<&Dataset>,
    early_stopping_rounds: Option<usize>,
    verbose: bool,
  ) {
    unimplemented!()
  }
  pub fn predict(&self, data: &Dataset, pred_type: PredictType) -> Vec<f32> {
    unimplemented!()
  }
  pub fn save(&self, path: &str) {
    unimplemented!()
  }
  pub fn load(path: &str) -> Self {
    unimplemented!()
  }
  pub fn get_feature_importance(&self, importance_type: ImportanceType) -> FeatureImportance {
    unimplemented!()
  }
  pub fn eval_metrics(&self, data: &Dataset, metrics: &[Metric]) -> Vec<f32> {
    unimplemented!()
  }
  pub fn best_iteration(&self) -> Option<usize> {
    unimplemented!()
  }
  pub fn best_score(&self) -> Option<f32> {
    unimplemented!()
  }
}

impl Dataset {
  pub fn from_csv(path: &str, target_column: &str, categorical_features: Option<&[usize]>) -> Self {
    unimplemented!()
  }
  pub fn num_rows(&self) -> usize {
    unimplemented!()
  }
  pub fn num_cols(&self) -> usize {
    unimplemented!()
  }
}

impl Params {
  pub fn new() -> Self {
    unimplemented!()
  }
  pub fn set(&mut self, key: &str, value: &str) {
    unimplemented!()
  }
  pub fn get(&self, key: &str) -> Option<&str> {
    unimplemented!()
  }
}

pub enum PredictType {
  Normal,
  RawScore,
  LeafIndex,
  Contribution,
}

pub enum ImportanceType {
  Split,
  Gain,
}

pub enum Metric {
  L1,
  L2,
  RMSE,
  MAE,
  Logloss,
  BinaryLogloss,
  CrossEntropy,
  AUC,
  MAP,
  NDCG,
  Poisson,
  Gamma,
  Tweedie,
  Fair,
  Huber,
  MAPE,
  KullbackLeibler,
}
