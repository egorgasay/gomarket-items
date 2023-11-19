use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use log::{warn};
use gomarket_items::api::controllers::items_handler::get_items;
use gomarket_items::container::Container;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    warn!("Starting server on 0.0.0.0:8000");

    let server = HttpServer::new(move || {
        let container = Container::new();
        let core_service = container.core_service.clone();

        App::new()
            .app_data(web::Data::from(core_service.clone()))
            .wrap(Logger::default())
            .service(web::scope("").route("/v1/items", web::get().to(get_items)))
    }).bind(("0.0.0.0", 8000))?;
    server.run().await
}
