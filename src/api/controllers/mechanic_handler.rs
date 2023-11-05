use actix_web::{web, Result, HttpResponse};
use crate::api::dto::order::{MechanicDTO, NewOrderDTO};
use crate::domain::error::{ApiError, CommonError};
use crate::domain::models::order::NewOrder;
use crate::domain::services::order::CoreService;

pub async fn new_mechanic(
    core_service: web::Data<dyn CoreService>, post_data: web::Json<MechanicDTO>,
) -> Result<HttpResponse, ApiError> {
    core_service
        .new_mechanic(post_data.into_inner().into())
        .await?;

    Ok(HttpResponse::Ok().finish())
}

// pub async fn list_todos_handler(
//     core_service: web::Data<dyn CoreService>, params: web::Query<TodoQueryParams>,
// ) -> Result<web::Json<ResultPaging<TodoDTO>>, ApiError> {
//     let selection = core_service.list(params.into_inner()).await?;
//     Ok(web::Json(selection.into()))
// }
//
// pub async fn get_todo_handler(
//     core_service: web::Data<dyn CoreService>, params: web::Path<i32>,
// ) -> Result<web::Json<TodoDTO>, ApiError> {
//     let todo = core_service.get(params.into_inner()).await?;
//     Ok(web::Json(todo.into()))
// }
//
// pub async fn delete_todo_handler(
//     core_service: web::Data<dyn CoreService>, params: web::Path<i32>,
// ) -> Result<HttpResponse, ApiError> {
//     core_service.delete(params.into_inner()).await?;
//     Ok(HttpResponse::NoContent().finish())
// }
