use actix_web::{web, HttpRequest, HttpResponse, Result, Error};

use crate::search::model::similarity_search::SimilaritySearch;

pub async fn find_similar_words(req: web::Json<SimilaritySearch>) -> Result<HttpResponse, Error> {
  let a = SimilaritySearch {
    word: req.word.clone(),
    word_array: req.word_array.clone()
  };

  return Ok(HttpResponse::Ok().json(a));
}
