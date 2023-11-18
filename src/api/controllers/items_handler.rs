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

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::sync::Arc;
    use actix_web;
    use actix_web::web;
    use actix_web::middleware::Logger;
    use crate::api::controllers::items_handler::get_items;
    use crate::api::dto::item::{GetItemsRequestDTO, ItemDTO};
    use crate::domain::error::CommonError;
    use crate::domain::models::items::Item;
    use crate::domain::repositories::repository::ResultPaging;
    use crate::domain::services::order::{CoreService, MockCoreService};
    use crate::infrastructure::schema::items::dsl::items;

    #[actix_web::test]
    async fn test_should_get_items() {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();

        let mut mock_core = MockCoreService::new();

        let req = actix_web::test::TestRequest::get();

        mock_core
            .expect_get_items()
            .returning(|q, s, l, o| -> _ {
                Box::pin(async move { Ok(ResultPaging{items: vec![
                    Item{ id: 0, name: String::from("test"), price: 0.0, description: String::from("test"), sizes: vec![] },
                ], total: 0, offset: 0}) })
            });

        let core_service: Arc<dyn CoreService> = Arc::new(mock_core);

        let app = actix_web::test::init_service(
            actix_web::App::new()
                .app_data(web::Data::from(core_service))
                .wrap(Logger::default())
                .service(web::scope("").route("/items", web::get().to(get_items)))
        )
            .await;

        let req_model = GetItemsRequestDTO{ query: None, sort_by: None, offset: 0, limit: 10 };
        let want_resp = ResultPaging{items: vec![ItemDTO{ id: 0, name: String::from("test"), price: 0.0, description: String::from("test"), sizes: vec![]}], total: 0, offset: 0};

        let req = actix_web::test::TestRequest::get()
            .uri("/items")
            .set_json(&req_model)
            .to_request();

        let resp = actix_web::test::call_service(&app, req).await;

        // let body = actix_web::test::read_body(resp).await;
        // println!("Response body: {:?}", body);

        assert!(resp.status().is_success());

        let resp_model: ResultPaging<ItemDTO> = actix_web::test::read_body_json(resp).await;

        assert_eq!(resp_model, want_resp);

    }
}