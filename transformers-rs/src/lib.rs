pub struct Pipeline {/* fields omitted */}
pub struct Model {/* fields omitted */}
pub struct Tokenizer {/* fields omitted */}
pub struct Config {/* fields omitted */}
pub struct TokenizedInput {/* fields omitted */}
pub struct Output {/* fields omitted */}

impl Pipeline {
  pub fn new(task: Task, model: &str, tokenizer: &str, config: Option<&Config>) -> Self {
    unimplemented!()
  }
  pub fn run(&self, input: &str) -> Output {
    unimplemented!()
  }
  pub fn batch_run(&self, inputs: &[String]) -> Vec<Output> {
    unimplemented!()
  }
  pub fn set_device(&mut self, device: Device) {
    unimplemented!()
  }
}

impl Model {
  pub fn from_pretrained(name: &str, config: Option<&Config>) -> Self {
    unimplemented!()
  }
  pub fn save(&self, path: &str) {
    unimplemented!()
  }
  pub fn load(path: &str) -> Self {
    unimplemented!()
  }
}

impl Tokenizer {
  pub fn from_pretrained(name: &str) -> Self {
    unimplemented!()
  }
  pub fn encode(&self, text: &str) -> TokenizedInput {
    unimplemented!()
  }
  pub fn decode(&self, tokens: &[i64]) -> String {
    unimplemented!()
  }
}

impl Config {
  pub fn from_json(json: &str) -> Self {
    unimplemented!()
  }
  pub fn to_json(&self) -> String {
    unimplemented!()
  }
}

pub enum Task {
  TextClassification,
  TokenClassification,
  QuestionAnswering,
  TextGeneration,
  Translation,
  Summarization,
  FillMask,
  ZeroShotClassification,
  Conversational,
  FeatureExtraction,
}

pub enum ModelType {
  Bert,
  Roberta,
  GPT2,
  GPTNeo,
  GPTJ,
  T5,
  Bart,
  DistilBert,
  Albert,
  Electra,
  XLNet,
  XLMRoberta,
  Marian,
  Deberta,
  Bloom,
  Llama,
  Mistral,
}

pub enum Device {
  CPU,
  CUDA(usize),
}
