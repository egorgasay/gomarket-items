use std::sync::Arc;

use async_trait::async_trait;
use crate::api::dto::order::NewOrderDTO;

use crate::domain::error::CommonError;
use crate::domain::models::order::{Mechanic, NewOrder};
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
    async fn register_order(&self, order: NewOrder) -> Result<(), CommonError> {
        Ok(())
    }

    async fn new_mechanic(&self, mechanic: Mechanic) -> Result<(), CommonError> {
        self.repository
            .new_mechanic(mechanic.into())
            .await
            .map_err(|e| -> CommonError { e.into() })
    }
}
