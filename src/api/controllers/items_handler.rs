use crate::api::dto::item::{GetItemsRequestDTO, ItemDTO};
use crate::domain::error::{ApiError};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::order::CoreService;
use actix_web::{web, Result};

pub async fn get_items(
    core_service: web::Data<dyn CoreService>,
    data: web::Json<GetItemsRequestDTO>,
) -> Result<web::Json<ResultPaging<ItemDTO>>, ApiError> {

    let data = data.into_inner();
    let query = data.query.clone().map(|q| q.into());
    let sort_by = data.sort_by.clone().map(|q| q.into());

    let selection = core_service
        .get_items(query, sort_by, data.offset, data.limit)
        .await?;

    Ok(web::Json(selection.into()))
}
