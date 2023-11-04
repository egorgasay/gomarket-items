use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::order::{NewOrder};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::todo::TodoQueryParams;

#[async_trait]
pub trait CoreService: Sync + Send {
    async fn register_order(&self, todo: NewOrder) -> Result<(), CommonError>;
}

