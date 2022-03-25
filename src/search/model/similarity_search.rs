use crate::search::controller::similarity_search_controller;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimilaritySearch {
  pub word: String,
  pub word_array: Vec<String>,
}

impl SimilaritySearch {
  pub fn run_similarity_search(self) {
    
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimilarityResult {
  pub word: String,
  pub similarity: f64,
}
