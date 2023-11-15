use crate::domain::models::item::GetItemsQuery;
use crate::domain::repositories::repository::{
    QueryParams, RepositoryResult, DEFAULT_LIMIT, DEFAULT_OFFSET,
};
use crate::infrastructure::models::items::{ItemDiesel, ItemsSizesDiesel, SizeDiesel};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg(test)]
use mockall::{predicate::*, *};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for TodoQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait Repository: Send + Sync {
    async fn get_items(
        &self,
        query: Option<GetItemsQuery>,
        sort_by: Option<String>,
        offset: i64,
        limit: i64,
    ) -> RepositoryResult<Vec<(ItemDiesel, Vec<SizeDiesel>, Vec<ItemsSizesDiesel>)>>;
}
