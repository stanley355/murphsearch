use actix_web::{web, Error, HttpResponse, Result};

use crate::search::controller::similarity_search_controller::SimilaritySearchController;
use crate::search::model::similarity_search::SimilaritySearch;

pub async fn find_similar_words(req: web::Json<SimilaritySearch>) -> Result<HttpResponse, Error> {
  let search_payload = SimilaritySearch {
    word: req.word.clone(),
    word_array: req.word_array.clone(),
  };

  let result = Box::new(SimilaritySearchController::check_by_substring(
    search_payload,
  ));

  return Ok(HttpResponse::Ok().json(result));
}
