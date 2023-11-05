use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::order::{Mechanic, NewOrder};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::mechanic::TodoQueryParams;

#[async_trait]
pub trait CoreService: Sync + Send {
    async fn register_order(&self, order: NewOrder) -> Result<(), CommonError>;
    async fn new_mechanic(&self, mechanic: Mechanic) -> Result<(), CommonError>;
}

