pub struct Dictionary {/* fields omitted */}
pub struct Corpus {/* fields omitted */}
pub struct LdaModel {/* fields omitted */}
pub struct LsiModel {/* fields omitted */}
pub struct Word2Vec {/* fields omitted */}

impl Dictionary {
  pub fn new() -> Self {
    unimplemented!()
  }
  pub fn add_documents(&mut self, docs: &[Vec<String>]) {
    unimplemented!()
  }
  pub fn doc2bow(&self, doc: &[String]) -> Vec<(usize, usize)> {
    unimplemented!()
  }
  pub fn save(&self, path: &str) {
    unimplemented!()
  }
  pub fn load(path: &str) -> Self {
    unimplemented!()
  }
}

impl Corpus {
  pub fn from_documents(docs: &[Vec<String>], dictionary: &Dictionary) -> Self {
    unimplemented!()
  }
  pub fn iter(&self) -> CorpusIter {
    unimplemented!()
  }
}

pub struct CorpusIter {/* fields omitted */}

impl Iterator for CorpusIter {
  type Item = Vec<(usize, usize)>;
  fn next(&mut self) -> Option<Self::Item> {
    unimplemented!()
  }
}

impl LdaModel {
  pub fn new(corpus: &Corpus, dictionary: &Dictionary, num_topics: usize) -> Self {
    unimplemented!()
  }
  pub fn get_document_topics(&self, doc: &[String]) -> Vec<(usize, f32)> {
    unimplemented!()
  }
  pub fn get_topic_terms(&self, topic_id: usize, topn: usize) -> Vec<(String, f32)> {
    unimplemented!()
  }
  pub fn save(&self, path: &str) {
    unimplemented!()
  }
  pub fn load(path: &str) -> Self {
    unimplemented!()
  }
}

impl LsiModel {
  pub fn new(corpus: &Corpus, dictionary: &Dictionary, num_topics: usize) -> Self {
    unimplemented!()
  }
  pub fn get_document_topics(&self, doc: &[String]) -> Vec<(usize, f32)> {
    unimplemented!()
  }
  pub fn get_topic_terms(&self, topic_id: usize, topn: usize) -> Vec<(String, f32)> {
    unimplemented!()
  }
  pub fn save(&self, path: &str) {
    unimplemented!()
  }
  pub fn load(path: &str) -> Self {
    unimplemented!()
  }
}

impl Word2Vec {
  pub fn new(docs: &[Vec<String>], vector_size: usize, window: usize, min_count: usize) -> Self {
    unimplemented!()
  }
  pub fn most_similar(&self, word: &str, topn: usize) -> Vec<(String, f32)> {
    unimplemented!()
  }
  pub fn similarity(&self, word1: &str, word2: &str) -> f32 {
    unimplemented!()
  }
  pub fn save(&self, path: &str) {
    unimplemented!()
  }
  pub fn load(path: &str) -> Self {
    unimplemented!()
  }
}
