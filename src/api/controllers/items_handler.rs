use crate::api::dto::item::{GetItemsRequestDTO, ItemDTO};
use crate::domain::error::{ApiError, CommonError};
use crate::domain::models::item::GetItemsQuery;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::order::CoreService;
use actix_web::{web, HttpResponse, Result};

pub async fn get_items(
    core_service: web::Data<dyn CoreService>,
    data: web::Json<GetItemsRequestDTO>,
) -> Result<web::Json<ResultPaging<ItemDTO>>, ApiError> {
    let data = data.into_inner();
    let mut query: Option<GetItemsQuery> = None;
    if data.query.is_some() {
        query = Some(data.query.clone().unwrap_or(Default::default()).into())
    }

    let selection = core_service
        .get_items(query, data.offset, data.limit, "")
        .await?;

    Ok(web::Json(selection.into()))
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
