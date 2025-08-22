pub struct Tensor {/* fields omitted */}
pub struct Model {/* fields omitted */}
pub struct Dataset {/* fields omitted */}

impl Model {
  pub fn new() -> Self {
    unimplemented!()
  }
  pub fn add_layer(&mut self, layer: Layer) {
    unimplemented!()
  }
  pub fn compile(&mut self, optimizer: Optimizer, loss: Loss) {
    unimplemented!()
  }
  pub fn fit(&mut self, x: &Tensor, y: &Tensor, epochs: usize) {
    unimplemented!()
  }
  pub fn fit_dataset(
    &mut self,
    dataset: &Dataset,
    epochs: usize,
    callbacks: Option<Vec<Box<dyn Callback>>>,
  ) {
    unimplemented!()
  }
  pub fn evaluate(&self, x: &Tensor, y: &Tensor, metrics: &[Metric]) -> Vec<f32> {
    unimplemented!()
  }
  pub fn predict(&self, x: &Tensor) -> Tensor {
    unimplemented!()
  }
  pub fn save(&self, path: &str) {
    unimplemented!()
  }
  pub fn load(path: &str) -> Self {
    unimplemented!()
  }
}

pub enum Layer {
  Dense {
    units: usize,
    activation: Activation,
  },
  Dropout {
    rate: f32,
  },
  Conv2D {
    filters: usize,
    kernel_size: (usize, usize),
    activation: Activation,
  },
  MaxPooling2D {
    pool_size: (usize, usize),
  },
  Flatten,
  BatchNormalization,
  LSTM {
    units: usize,
    activation: Activation,
  },
  GRU {
    units: usize,
    activation: Activation,
  },
  Embedding {
    input_dim: usize,
    output_dim: usize,
  },
  GlobalAveragePooling1D,
  GlobalMaxPooling1D,
  Activation(Activation),
}

pub enum Optimizer {
  SGD {
    lr: f32,
    momentum: Option<f32>,
    nesterov: bool,
  },
  Adam {
    lr: f32,
    beta_1: f32,
    beta_2: f32,
    epsilon: f32,
  },
  RMSProp {
    lr: f32,
    rho: f32,
    momentum: Option<f32>,
  },
  Adagrad {
    lr: f32,
  },
  Adadelta {
    lr: f32,
    rho: f32,
  },
  Adamax {
    lr: f32,
    beta_1: f32,
    beta_2: f32,
  },
  Nadam {
    lr: f32,
    beta_1: f32,
    beta_2: f32,
  },
  Ftrl {
    lr: f32,
    l1: f32,
    l2: f32,
  },
}

pub enum Loss {
  MSE,
  MAE,
  MAPE,
  MSLE,
  CategoricalCrossentropy,
  BinaryCrossentropy,
  SparseCategoricalCrossentropy,
  KLDivergence,
  Hinge,
  SquaredHinge,
  Poisson,
  CosineProximity,
  LogCosh,
  Huber,
}

pub enum Activation {
  Relu,
  Softmax,
  Sigmoid,
  Tanh,
  Linear,
  Softplus,
  Softsign,
  HardSigmoid,
  Exponential,
  ELU,
  SELU,
  LeakyRelu,
  Swish,
}

pub enum Metric {
  Accuracy,
  CategoricalAccuracy,
  BinaryAccuracy,
  Precision,
  Recall,
  F1,
  AUC,
  TopKCategoricalAccuracy { k: usize },
  MeanAbsoluteError,
  MeanSquaredError,
  MeanAbsolutePercentageError,
  MeanSquaredLogarithmicError,
}

pub trait Callback {
  fn on_epoch_end(&mut self, epoch: usize, logs: &[(String, f32)]);
  // ...
}

impl Dataset {
  pub fn from_tensors(x: &Tensor, y: &Tensor) -> Self {
    unimplemented!()
  }
  pub fn batch(&self, size: usize) -> Vec<Dataset> {
    unimplemented!()
  }
  pub fn shuffle(&self) -> Self {
    unimplemented!()
  }
}
