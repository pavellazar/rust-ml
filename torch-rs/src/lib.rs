pub struct Tensor {/* fields omitted */}
pub struct Module {/* fields omitted */}
pub struct DataLoader {/* fields omitted */}
pub struct Device {/* fields omitted */}

impl Module {
  pub fn new() -> Self {
    unimplemented!()
  }
  pub fn add(&mut self, layer: Layer) {
    unimplemented!()
  }
  pub fn forward(&self, x: &Tensor) -> Tensor {
    unimplemented!()
  }
  pub fn train(
    &mut self,
    x: &Tensor,
    y: &Tensor,
    epochs: usize,
    optimizer: Optimizer,
    loss: LossFn,
    callbacks: Option<Vec<Box<dyn Callback>>>,
  ) {
    unimplemented!()
  }
  pub fn eval(&self, x: &Tensor, y: &Tensor, metrics: &[Metric]) -> Vec<f32> {
    unimplemented!()
  }
  pub fn save(&self, path: &str) {
    unimplemented!()
  }
  pub fn load(path: &str) -> Self {
    unimplemented!()
  }
  pub fn to_device(&mut self, device: Device) {
    unimplemented!()
  }
}

pub enum Layer {
  Linear {
    in_features: usize,
    out_features: usize,
    activation: Option<Activation>,
  },
  Conv2d {
    in_channels: usize,
    out_channels: usize,
    kernel_size: (usize, usize),
    activation: Option<Activation>,
  },
  Dropout {
    rate: f32,
  },
  BatchNorm2d {
    num_features: usize,
  },
  Flatten,
  MaxPool2d {
    kernel_size: (usize, usize),
  },
  AvgPool2d {
    kernel_size: (usize, usize),
  },
  ConvTranspose2d {
    in_channels: usize,
    out_channels: usize,
    kernel_size: (usize, usize),
    activation: Option<Activation>,
  },
  LSTM {
    input_size: usize,
    hidden_size: usize,
  },
  GRU {
    input_size: usize,
    hidden_size: usize,
  },
  Embedding {
    num_embeddings: usize,
    embedding_dim: usize,
  },
  Sequential(Vec<Layer>),
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

pub enum LossFn {
  MSE,
  MAE,
  CrossEntropy,
  NLLLoss,
  BCELoss,
  BCEWithLogitsLoss,
  SmoothL1Loss,
  HingeEmbeddingLoss,
  KLDivLoss,
  PoissonNLLLoss,
  CosineEmbeddingLoss,
  MarginRankingLoss,
  MultiLabelMarginLoss,
  MultiMarginLoss,
  TripletMarginLoss,
  CTCLoss,
}

pub enum Activation {
  Relu,
  Tanh,
  Sigmoid,
  Softmax,
  LeakyRelu,
  ELU,
  SELU,
  Hardshrink,
  Hardtanh,
  LogSigmoid,
  PReLU,
  ReLU6,
  RReLU,
  Softplus,
  Softshrink,
  Softsign,
  Tanhshrink,
  Threshold,
}

pub enum Metric {
  Accuracy,
  Precision,
  Recall,
  F1,
  AUC,
  TopKAccuracy { k: usize },
  MeanAbsoluteError,
  MeanSquaredError,
  MeanAbsolutePercentageError,
  MeanSquaredLogarithmicError,
}

pub trait Callback {
  fn on_epoch_end(&mut self, epoch: usize, logs: &[(String, f32)]);
  // ...
}

impl DataLoader {
  pub fn from_tensors(x: &Tensor, y: &Tensor, batch_size: usize, shuffle: bool) -> Self {
    unimplemented!()
  }
  pub fn next_batch(&mut self) -> Option<(Tensor, Tensor)> {
    unimplemented!()
  }
}

impl Device {
  pub fn cpu() -> Self {
    unimplemented!()
  }
  pub fn cuda(index: usize) -> Self {
    unimplemented!()
  }
}
