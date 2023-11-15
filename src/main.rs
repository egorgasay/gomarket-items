use gomarket_items::create_app::create_app;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use log::{Level, log};
use gomarket_items::api::controllers::items_handler::get_items;
use gomarket_items::container::Container;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    log!(Level::Info, "started on 8080");


    let server = HttpServer::new(move || {
        let container = Container::new();
        let core_service = container.core_service.clone();
        // let service_context_service = container.service_context_service.clone();

        env_logger::init();
        log!(Level::Warn, "started on 8000");

        App::new()
            .app_data(web::Data::from(core_service.clone()))
            // .app_data(web::Data::from(service_context_service.clone()))
            .wrap(Logger::default())
            //.wrap(ServiceContextMaintenanceCheck)
            .service(web::scope("").route("/v1/items", web::get().to(get_items)))
    }).bind(("0.0.0.0", 8000))?;
    server.run().await
}
