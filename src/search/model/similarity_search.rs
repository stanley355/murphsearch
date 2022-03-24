use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimilaritySearch {
  pub word: String,
  pub word_array: Vec<String>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimilarityResult {
  pub word: String,
  pub similarity: f64
}
