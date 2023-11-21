use crate::api::dto::item::{CreateItemResponseDTO, GetItemsRequestDTO, ItemDTO};
use crate::domain::error::ApiError;
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

pub async fn create_item(
    core_service: web::Data<dyn CoreService>,
    data: web::Json<ItemDTO>,
) -> Result<web::Json<CreateItemResponseDTO>, ApiError> {
    let data = data.into_inner();

    let id = core_service.create_item(data.into()).await?;

    Ok(web::Json(CreateItemResponseDTO { id }))
}

#[cfg(test)]
mod tests {
    use crate::api::controllers::items_handler::{create_item, get_items};
    use crate::api::dto::item::{CreateItemResponseDTO, GetItemsRequestDTO, ItemDTO};
    use crate::domain::models::items::Item;
    use crate::domain::repositories::repository::ResultPaging;
    use crate::domain::services::order::{CoreService, MockCoreService};
    use actix_web;
    use actix_web::middleware::Logger;
    use actix_web::web;
    use std::sync::Arc;
    use crate::domain::error::{ApiError, CommonError, CommonErrorKind};

    #[actix_web::test]
    async fn test_should_get_items() {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();

        let mut mock_core = MockCoreService::new();

        mock_core.expect_get_items().returning(|q, s, o, l| -> _ {
            assert_eq!(q, None);
            assert_eq!(s, None);
            assert_eq!(l, 10);
            assert_eq!(o, 0);

            Box::pin(async move {
                Ok(ResultPaging {
                    items: vec![Item {
                        id: 0,
                        name: String::from("test"),
                        price: 0.0,
                        description: String::from("test"),
                        sizes: vec![],
                    }],
                    total: 1,
                    offset: 0,
                })
            })
        });

        let core_service: Arc<dyn CoreService> = Arc::new(mock_core);

        let app = actix_web::test::init_service(
            actix_web::App::new()
                .app_data(web::Data::from(core_service))
                .wrap(Logger::default())
                .service(web::scope("").route("/items", web::get().to(get_items))),
        )
        .await;

        let req_model = GetItemsRequestDTO {
            query: None,
            sort_by: None,
            offset: 0,
            limit: 10,
        };
        let want_resp = ResultPaging {
            items: vec![ItemDTO {
                id: Some(0),
                name: String::from("test"),
                price: 0.0,
                description: String::from("test"),
                sizes: vec![],
            }],
            total: 1,
            offset: 0,
        };

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

    #[actix_web::test]
    async fn test_should_not_get_items() {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();

        let mut mock_core = MockCoreService::new();

        mock_core.expect_get_items().returning(|q, s, o, l| -> _ {
            assert_eq!(q, None);
            assert_eq!(s, None);
            assert_eq!(l, 10);
            assert_eq!(o, 0);

            Box::pin(async move {
                Ok(ResultPaging {
                    items: vec![],
                    total: 0,
                    offset: 0,
                })
            })
        });

        let core_service: Arc<dyn CoreService> = Arc::new(mock_core);

        let app = actix_web::test::init_service(
            actix_web::App::new()
                .app_data(web::Data::from(core_service))
                .wrap(Logger::default())
                .service(web::scope("").route("/items", web::get().to(get_items))),
        )
        .await;

        let req_model = GetItemsRequestDTO {
            query: None,
            sort_by: None,
            offset: 0,
            limit: 10,
        };
        let want_resp = ResultPaging {
            items: vec![],
            total: 0,
            offset: 0,
        };

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

    #[actix_web::test]
    async fn test_err_unknown_get_items() {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();

        let mut mock_core = MockCoreService::new();

        mock_core.expect_get_items().returning(|q, s, o, l| -> _ {
            assert_eq!(q, None);
            assert_eq!(s, None);
            assert_eq!(l, 10);
            assert_eq!(o, 0);

            Box::pin(async move {
                Err(CommonError{code: CommonErrorKind::Unknown, message: "x".to_string()})
            })
        });

        let core_service: Arc<dyn CoreService> = Arc::new(mock_core);

        let app = actix_web::test::init_service(
            actix_web::App::new()
                .app_data(web::Data::from(core_service))
                .wrap(Logger::default())
                .service(web::scope("").route("/items", web::get().to(get_items))),
        )
            .await;

        let req_model = GetItemsRequestDTO {
            query: None,
            sort_by: None,
            offset: 0,
            limit: 10,
        };
        let want_resp = ApiError::from(CommonError{code: CommonErrorKind::Unknown, message: "x".to_string()});

        let req = actix_web::test::TestRequest::get()
            .uri("/items")
            .set_json(&req_model)
            .to_request();

        let resp = actix_web::test::call_service(&app, req).await;

        // let body = actix_web::test::read_body(resp).await;
        // println!("Response body: {:?}", body);

        assert!(resp.status().is_server_error());

        let resp_model: ApiError = actix_web::test::read_body_json(resp).await;

        assert_eq!(resp_model, want_resp);
    }

    #[actix_web::test]
    async fn test_should_create_item() {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();

        let mut mock_core = MockCoreService::new();

        let input = ItemDTO {
            id: Some(0),
            name: String::from("test"),
            price: 0.0,
            description: String::from("test"),
            sizes: vec![],
        };
        let input_clone: Item = input.clone().into();


        mock_core.expect_create_item().returning(move |i| -> _ {
            assert_eq!(i, input_clone);

            Box::pin(async move {
                Ok(input_clone.id)
            })
        });

        let core_service: Arc<dyn CoreService> = Arc::new(mock_core);

        let app = actix_web::test::init_service(
            actix_web::App::new()
                .app_data(web::Data::from(core_service))
                .wrap(Logger::default())
                .service(web::scope("").route("/items", web::post().to(create_item))),
        )
            .await;


        let want_resp = CreateItemResponseDTO{id: input.id.unwrap()};

        let req = actix_web::test::TestRequest::post()
            .uri("/items")
            .set_json(&input)
            .to_request();

        let resp = actix_web::test::call_service(&app, req).await;

        // let body = actix_web::test::read_body(resp).await;
        // println!("Response body: {:?}", body);

        assert!(resp.status().is_success());

        let resp_model: CreateItemResponseDTO = actix_web::test::read_body_json(resp).await;

        assert_eq!(resp_model, want_resp);
    }

    #[actix_web::test]
    async fn test_should_not_create_item() {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::init();

        let mut mock_core = MockCoreService::new();

        let input = ItemDTO {
            id: Some(0),
            name: String::from("test"),
            price: 0.0,
            description: String::from("test"),
            sizes: vec![],
        };
        let input_clone: Item = input.clone().into();


        mock_core.expect_create_item().returning(move |i| -> _ {
            assert_eq!(i, input_clone);

            Box::pin(async move {
                Err(CommonError{message: "x".to_string(), code: CommonErrorKind::Unknown})
            })
        });

        let core_service: Arc<dyn CoreService> = Arc::new(mock_core);

        let app = actix_web::test::init_service(
            actix_web::App::new()
                .app_data(web::Data::from(core_service))
                .wrap(Logger::default())
                .service(web::scope("").route("/items", web::post().to(create_item))),
        )
            .await;


        let want_resp = ApiError::from(CommonError{message: "x".to_string(), code: CommonErrorKind::Unknown});

        let req = actix_web::test::TestRequest::post()
            .uri("/items")
            .set_json(&input)
            .to_request();

        let resp = actix_web::test::call_service(&app, req).await;

        // let body = actix_web::test::read_body(resp).await;
        // println!("Response body: {:?}", body);

        assert!(resp.status().is_server_error());

        let resp_model: ApiError = actix_web::test::read_body_json(resp).await;

        assert_eq!(resp_model, want_resp);
    }
}
