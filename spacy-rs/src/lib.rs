pub struct Language {/* fields omitted */}
pub struct Pipeline {/* fields omitted */}
pub struct Document {/* fields omitted */}
pub struct Span {/* fields omitted */}
pub struct Token {/* fields omitted */}
pub struct Entity {/* fields omitted */}
pub struct Tag {/* fields omitted */}
pub struct Dependency {/* fields omitted */}

impl Language {
  pub fn new(model: &str) -> Self {
    unimplemented!()
  }
  pub fn pipeline(&self) -> Pipeline {
    unimplemented!()
  }
  pub fn process(&self, text: &str) -> Document {
    unimplemented!()
  }
  pub fn save(&self, path: &str) {
    unimplemented!()
  }
  pub fn load(path: &str) -> Self {
    unimplemented!()
  }
}

impl Pipeline {
  pub fn add_component(&mut self, name: &str, component: PipelineComponent) {
    unimplemented!()
  }
  pub fn remove_component(&mut self, name: &str) {
    unimplemented!()
  }
  pub fn list_components(&self) -> Vec<String> {
    unimplemented!()
  }
}

pub enum PipelineComponent {
  Tokenizer,
  Tagger,
  Parser,
  NER,
  Lemmatizer,
  // Add more as needed
}

impl Document {
  pub fn text(&self) -> &str {
    unimplemented!()
  }
  pub fn tokens(&self) -> &[Token] {
    unimplemented!()
  }
  pub fn sents(&self) -> Vec<Span> {
    unimplemented!()
  }
  pub fn ents(&self) -> &[Entity] {
    unimplemented!()
  }
  pub fn noun_chunks(&self) -> Vec<Span> {
    unimplemented!()
  }
  pub fn to_json(&self) -> String {
    unimplemented!()
  }
  pub fn from_json(json: &str) -> Self {
    unimplemented!()
  }
}

impl Span {
  pub fn text(&self) -> &str {
    unimplemented!()
  }
  pub fn start(&self) -> usize {
    unimplemented!()
  }
  pub fn end(&self) -> usize {
    unimplemented!()
  }
  pub fn label(&self) -> Option<EntityLabel> {
    unimplemented!()
  }
}

impl Token {
  pub fn text(&self) -> &str {
    unimplemented!()
  }
  pub fn lemma(&self) -> &str {
    unimplemented!()
  }
  pub fn pos(&self) -> POS {
    unimplemented!()
  }
  pub fn tag(&self) -> &str {
    unimplemented!()
  }
  pub fn dep(&self) -> DependencyLabel {
    unimplemented!()
  }
  pub fn head(&self) -> usize {
    unimplemented!()
  }
  pub fn is_alpha(&self) -> bool {
    unimplemented!()
  }
  pub fn is_stop(&self) -> bool {
    unimplemented!()
  }
}

impl Entity {
  pub fn text(&self) -> &str {
    unimplemented!()
  }
  pub fn label(&self) -> EntityLabel {
    unimplemented!()
  }
  pub fn start(&self) -> usize {
    unimplemented!()
  }
  pub fn end(&self) -> usize {
    unimplemented!()
  }
}

pub enum POS {
  ADJ,
  ADP,
  ADV,
  AUX,
  CONJ,
  CCONJ,
  DET,
  INTJ,
  NOUN,
  NUM,
  PART,
  PRON,
  PROPN,
  PUNCT,
  SCONJ,
  SYM,
  VERB,
  X,
  SPACE,
}

pub enum EntityLabel {
  PERSON,
  NORP,
  FAC,
  ORG,
  GPE,
  LOC,
  PRODUCT,
  EVENT,
  WORK_OF_ART,
  LAW,
  LANGUAGE,
  DATE,
  TIME,
  PERCENT,
  MONEY,
  QUANTITY,
  ORDINAL,
  CARDINAL,
}

pub enum DependencyLabel {
  ROOT,
  ACL,
  ADVCL,
  ADVMOD,
  AMOD,
  APPOS,
  AUX,
  CASE,
  CC,
  CCOMP,
  CLF,
  COMPOUND,
  CONJ,
  COP,
  CSUBJ,
  DEP,
  DET,
  DISCOURSE,
  DISLOCATED,
  DOBJ,
  EXPL,
  FIXED,
  FLAT,
  GOESWITH,
  IOBJ,
  LIST,
  MARK,
  NMOD,
  NSUBJ,
  NUMMOD,
  OBJ,
  OBL,
  ORPHAN,
  PARATAXIS,
  PUNCT,
  REPARANDUM,
  VOCATIVE,
  XCOMP,
}
