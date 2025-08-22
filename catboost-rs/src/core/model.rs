use crate::data::{Dataset, Pool};
use crate::metrics::{FeatureImportance, Metric};
use crate::params::Params;

pub struct CatBoost {
  split_feature: Option<usize>,
  split_value: f32,
  left_mean: f32,
  right_mean: f32,
  params: crate::params::Params,
}

impl CatBoost {
  pub fn new(params: &Params) -> Self {
    CatBoost {
      split_feature: None,
      split_value: 0.0,
      left_mean: 0.0,
      right_mean: 0.0,
      params: params.clone(),
    }
  }

  pub fn fit(&mut self, x: &Vec<Vec<f32>>, y: &Vec<f32>) {
    self.fit_with_params(x, y);
  }

  pub fn fit_pool(&mut self, pool: &Pool) {
    self.fit_with_params(&pool.x, &pool.y);
  }

  fn fit_with_params(&mut self, x: &Vec<Vec<f32>>, y: &Vec<f32>) {
    let reg_type = self.params.get("regressor_type").unwrap_or("stump");
    match reg_type {
      "mean" => self.fit_mean(y),
      _ => self.fit_stump(x, y),
    }
  }

  fn fit_mean(&mut self, y: &Vec<f32>) {
    let mean = if y.is_empty() {
      0.0
    } else {
      y.iter().sum::<f32>() / y.len() as f32
    };
    self.split_feature = None;
    self.split_value = 0.0;
    self.left_mean = mean;
    self.right_mean = mean;
  }

  fn fit_stump(&mut self, x: &Vec<Vec<f32>>, y: &Vec<f32>) {
    // Decision stump: find best split on any feature
    let n_samples = x.len();
    let n_features = if n_samples > 0 { x[0].len() } else { 0 };
    let mut best_feature = None;
    let mut best_split = 0.0;
    let mut best_loss = f32::INFINITY;
    let mut best_left_mean = 0.0;
    let mut best_right_mean = 0.0;

    for feature in 0..n_features {
      // Collect all values for this feature
      let mut values: Vec<f32> = x.iter().map(|row| row[feature]).collect();
      values.sort_by(|a, b| a.partial_cmp(b).unwrap());
      // Try all possible splits (midpoints between unique values)
      for i in 1..n_samples {
        if (values[i] - values[i - 1]).abs() < 1e-6 {
          continue;
        }
        let split = (values[i] + values[i - 1]) / 2.0;
        let mut left = vec![];
        let mut right = vec![];
        for (row, &target) in x.iter().zip(y.iter()) {
          if row[feature] <= split {
            left.push(target);
          } else {
            right.push(target);
          }
        }
        if left.is_empty() || right.is_empty() {
          continue;
        }
        let left_mean = left.iter().sum::<f32>() / left.len() as f32;
        let right_mean = right.iter().sum::<f32>() / right.len() as f32;
        let loss = left.iter().map(|v| (v - left_mean).powi(2)).sum::<f32>()
          + right.iter().map(|v| (v - right_mean).powi(2)).sum::<f32>();
        if loss < best_loss {
          best_loss = loss;
          best_feature = Some(feature);
          best_split = split;
          best_left_mean = left_mean;
          best_right_mean = right_mean;
        }
      }
    }
    // If no split found, fallback to mean regressor
    if let Some(feature) = best_feature {
      self.split_feature = Some(feature);
      self.split_value = best_split;
      self.left_mean = best_left_mean;
      self.right_mean = best_right_mean;
    } else {
      let mean = if y.is_empty() {
        0.0
      } else {
        y.iter().sum::<f32>() / y.len() as f32
      };
      self.split_feature = None;
      self.split_value = 0.0;
      self.left_mean = mean;
      self.right_mean = mean;
    }
  }

  pub fn predict(&self, x: &Vec<Vec<f32>>) -> Vec<f32> {
    self.predict_stump(x)
  }

  pub fn predict_pool(&self, pool: &Pool) -> Vec<f32> {
    self.predict_stump(&pool.x)
  }

  fn predict_stump(&self, x: &Vec<Vec<f32>>) -> Vec<f32> {
    if let Some(feature) = self.split_feature {
      x.iter()
        .map(|row| {
          if row[feature] <= self.split_value {
            self.left_mean
          } else {
            self.right_mean
          }
        })
        .collect()
    } else {
      x.iter().map(|_| self.left_mean).collect()
    }
  }
  pub fn save(&self, _path: &str) {}
  pub fn load(_path: &str) -> Self {
    CatBoost {
      split_feature: None,
      split_value: 0.0,
      left_mean: 0.0,
      right_mean: 0.0,
      params: crate::params::Params::new(),
    }
  }
  pub fn get_feature_importance(&self, _pool: Option<&Pool>) -> FeatureImportance {
    unimplemented!()
  }
  pub fn eval_metrics(
    &self,
    _pool: &Pool,
    _metrics: &[Metric],
    _ntree_end: Option<usize>,
  ) -> Vec<f32> {
    unimplemented!()
  }
  pub fn best_iteration(&self) -> Option<usize> {
    None
  }
  pub fn best_score(&self) -> Option<f32> {
    None
  }
}
