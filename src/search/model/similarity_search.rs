use crate::search::controller::similarity_search_controller::SimilaritySearchController as Controller;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimilaritySearch {
  pub word: String,
  pub word_array: Vec<String>,
}

impl SimilaritySearch {
  pub fn run_similarity_search(self) -> Box<Vec<SimilarityResult>> {
    let mut result = Controller::check_by_substring(self.clone());

    if result.is_empty() {
      result = Controller::check_by_letter(self)
    }

    let sorted_result = Controller::sort_similarity_result(result);

    return sorted_result;
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimilarityResult {
  pub word: String,
  pub similarity: f64,
}
