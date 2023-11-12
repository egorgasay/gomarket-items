use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::item::{GetItemsQuery, Item};
use crate::domain::repositories::repository::ResultPaging;

#[async_trait]
pub trait CoreService: Sync + Send {
    async fn get_items(
        &self,
        query: Option<GetItemsQuery>,
        sort_by: Option<String>,
        offset: i64,
        limit: i64,
    ) -> Result<ResultPaging<Item>, CommonError>;
}
