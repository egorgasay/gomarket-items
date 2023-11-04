use std::sync::Arc;

use async_trait::async_trait;
use crate::api::dto::order::NewOrderDTO;

use crate::domain::error::CommonError;
use crate::domain::models::order::{NewOrder};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::todo::{TodoQueryParams, Repository};
use crate::domain::services::order::CoreService;

#[derive(Clone)]
pub struct CoreServiceImpl {
    pub repository: Arc<dyn Repository>,
}

impl CoreServiceImpl {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        CoreServiceImpl {
            repository,
        }
    }
}

#[async_trait]
impl CoreService for CoreServiceImpl {
    async fn register_order(&self, todo: NewOrder) -> Result<(), CommonError> {
        let mut cloned = todo.clone();
        self.repository
            .register_order(&mut cloned)
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
