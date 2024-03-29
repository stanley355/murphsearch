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
                    similarity: 50.0,
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
            let mut count = 0;

            while count < word.len() {
                let letter = word.chars().nth(count).unwrap();
                if target_word.contains(letter) {
                    similar_count += 1;
                }

                count += 1;
            }

            let percentage = similar_count as f64 / word.len() as f64 * 100.0;
            let result = SimilarityResult {
                word: word,
                similarity: percentage,
            };

            if result.similarity > 49.0 {
                new_word_array.push(result);
            }
        }

        return new_word_array;
    }

    pub fn sort_similarity_result(
        result_list: Box<Vec<SimilarityResult>>,
    ) -> Box<Vec<SimilarityResult>> {
        let mut list = result_list;

        if !list.is_empty() {
            let mut count = 0;

            while count < list.len() - 1 {
                let prev_index = list[count].clone();
                let next_index = list[count + 1].clone();

                if prev_index.similarity < next_index.similarity {
                    list[count] = next_index;
                    list[count + 1] = prev_index;
                }

                count += 1;
            }
        }

        return list;
    }
}
