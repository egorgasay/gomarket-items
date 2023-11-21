use async_trait::async_trait;
#[allow(unused_imports)]
use mockall::automock;

use crate::domain::error::CommonError;
use crate::domain::models::items::{GetItemsQuery, GetItemsSortBy, Item};
use crate::domain::repositories::repository::ResultPaging;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait CoreService: Sync + Send {
    async fn get_items(
        &self,
        query: Option<GetItemsQuery>,
        sort_by: Option<GetItemsSortBy>,
        offset: i64,
        limit: i64,
    ) -> Result<ResultPaging<Item>, CommonError>;

    async fn create_item(&self, item: Item) -> Result<i64, CommonError>;
}
