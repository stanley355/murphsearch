use actix_web::{web, Error, HttpResponse, Result};

use crate::search::model::similarity_search::SimilaritySearch;

pub async fn find_similar_words(req: web::Json<SimilaritySearch>) -> Result<HttpResponse, Error> {
  let search_payload = SimilaritySearch {
    word: req.word.clone(),
    word_array: req.word_array.clone(),
  };

  let result = Box::new(search_payload.run_similarity_search());

  return Ok(HttpResponse::Ok().json(result));
}
