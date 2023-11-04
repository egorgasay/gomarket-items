use actix_web::{App, web};
use actix_web::{Error};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use crate::api::controllers::order_handler::{register_order};
use crate::api::middleware::{ServiceContextMaintenanceCheck};
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
    let todo_service = container.core_service.clone();
    // let service_context_service = container.service_context_service.clone();

    App::new()
        .app_data(web::Data::from(todo_service.clone()))
        // .app_data(web::Data::from(service_context_service.clone()))
        .wrap(Logger::default())
        .wrap(ServiceContextMaintenanceCheck)
        .service(
            web::scope("/")
                .route("v1/orders", web::post().to(register_order))
        )
}