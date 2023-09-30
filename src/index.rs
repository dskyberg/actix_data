use actix_web::{web, Error, HttpResponse};

use crate::my_obj::MyObj;

// This handler uses json extractor
/// This handler manually load request payload and parse json object
pub async fn get(item: web::Json<MyObj>) -> Result<HttpResponse, Error> {
    log::info!("{:?}", &item.0);
    Ok(HttpResponse::Ok().json(item.0)) // <- send response
}
