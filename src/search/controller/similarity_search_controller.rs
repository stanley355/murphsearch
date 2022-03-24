use crate::search::model::similarity_search::{SimilarityResult, SimilaritySearch};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimilaritySearchController;

impl SimilaritySearchController {
  pub fn check_by_substring(payload: SimilaritySearch) -> Box<Vec<SimilarityResult>> {
    let mut new_word_array = Box::new(vec![]);
    let target_word = &payload.word;

    for word in payload.word_array {
      if word.contains(target_word) {
        let result = SimilarityResult {
          word: word,
          similarity: 100.0
        };
        new_word_array.push(result);
      }
    }

    return new_word_array;
  }

  pub fn check_by_letter(payload: SimilaritySearch) -> Box<Vec<SimilarityResult>> {
    let mut new_word_array = Box::new(vec![]);
    let target_word = &payload.word;

    for word in payload.word_array {
      let mut similar_count = 0;
      let mut while_count = 0;

      while while_count < word.len() {
        let letter = word.chars().nth(while_count).unwrap();
        if target_word.contains(letter) {
          similar_count += 1;
        }

        while_count += 1;
      }

      let percentage = similar_count as f64 / word.len() as f64 * 100.0;
      let result = SimilarityResult {
        word: word,
        similarity: percentage,
      };
      new_word_array.push(result);
    }

    return new_word_array;
  }
}
