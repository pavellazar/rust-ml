#[test]
fn test_catboost_mean_regressor() {
  let mut params = catboost_rs::Params::new();
  params.set("regressor_type", "mean");
  let mut model = catboost_rs::CatBoost::new(&params);
  let x = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
  let y = vec![1.0, 3.0];
  let pool = catboost_rs::Pool::from_xy(x.clone(), y.clone());
  model.fit_pool(&pool);
  let pred = model.predict_pool(&pool);
  let mean = (1.0 + 3.0) / 2.0;
  assert_eq!(pred, vec![mean, mean]);
}

#[test]
fn test_catboost_decision_stump() {
  let params = catboost_rs::Params::new(); // default: stump
  let mut model = catboost_rs::CatBoost::new(&params);
  let x = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
  let y = vec![1.0, 3.0];
  let pool = catboost_rs::Pool::from_xy(x.clone(), y.clone());
  model.fit_pool(&pool);
  let pred = model.predict_pool(&pool);
  assert_eq!(pred, vec![1.0, 3.0]);
}
