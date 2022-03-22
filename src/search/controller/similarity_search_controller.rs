use crate::search::model::similarity_search::SimilaritySearch;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimilaritySearchController;

impl SimilaritySearchController {
  pub fn check_by_substring(payload: SimilaritySearch) -> Box<Vec<String>> {
    let mut new_word_array = Box::new(vec![]);

    for word in payload.word_array {
      if word.contains(&payload.word) {
        new_word_array.push(word);
      }
    }

    return new_word_array;
  }
}
