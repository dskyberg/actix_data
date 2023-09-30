use actix_web::{web, App, HttpServer};

mod index;
mod my_obj;
mod payload_accessor_middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(payload_accessor_middleware::PayloadAccessor)
            .service(web::resource("/").route(web::get().to(index::get)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
