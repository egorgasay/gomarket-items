use std::sync::Arc;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use gomarket_items::api::controllers::items_handler::{create_item, get_items};
use gomarket_items::container::Container;
use log::info;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    info!("Starting server on 0.0.0.0:8000");

    let container = Container::new();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(Arc::clone(&container.core_service)))
            .wrap(Logger::default())
            .service(web::scope("").
                route("/v1/items", web::get().to(get_items)).
                route("/v1/items", web::post().to(create_item))
            )
    })
    .bind(("0.0.0.0", 8000))?;
    server.run().await
}
