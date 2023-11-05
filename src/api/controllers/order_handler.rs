use actix_web::{web, Result, HttpResponse};
use crate::api::dto::order::{NewOrderDTO};
use crate::domain::error::{ApiError, CommonError};
use crate::domain::models::order::NewOrder;
use crate::domain::services::order::CoreService;

pub async fn register_order(
    core_service: web::Data<dyn CoreService>, post_data: web::Json<NewOrderDTO>,
) -> Result<HttpResponse, ApiError> {
    let y = core_service
        .register_order(post_data.into_inner().into())
        .await;

    match y {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(ApiError::from(e)),
    }?;

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
