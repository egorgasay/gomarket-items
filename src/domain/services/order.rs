use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::item::{GetItemsQuery, Item};
use crate::domain::repositories::repository::ResultPaging;

#[async_trait]
pub trait CoreService: Sync + Send {
    async fn get_items(
        &self,
        query: GetItemsQuery,
        offset: i64,
        limit: i64,
        sort_by: &str,
    ) -> Result<ResultPaging<Item>, CommonError>;
}
