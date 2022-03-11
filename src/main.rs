use actix_web::{error, web, App, HttpRequest, HttpResponse, Error, HttpServer};
use futures_util::StreamExt as _;
use my_obj::MyObj;

const MAX_SIZE: usize = 262_144; // max payload size is 256k
mod read_request_body;
mod my_obj;

/// This handler uses json extractor
/// This handler manually load request payload and parse json object
pub async fn index_raw(mut payload: web::Payload, _req: HttpRequest) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    log::info!("{:?}", &body);
    Ok(HttpResponse::Ok().body(body)) // <- send response
}

/// This handler uses json extractor
/// This handler manually load request payload and parse json object
pub async fn index(item: web::Json<MyObj>, _req: HttpRequest) -> Result<HttpResponse, Error> {
   log::info!("{:?}", &item.0);
    Ok(HttpResponse::Ok().json(item.0)) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(read_request_body::Logging)
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
