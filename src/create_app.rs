use std::sync::Arc;
use crate::api::controllers::items_handler::get_items;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::middleware::Logger;
use actix_web::{web, App};
use log::{info, Level, log};
//use crate::api::middleware::{ServiceContextMaintenanceCheck};
use crate::container::Container;


pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
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
}
