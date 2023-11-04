use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::domain::repositories::repository::{QueryParams, ResultPaging, RepositoryResult, DEFAULT_LIMIT, DEFAULT_OFFSET};
use crate::domain::models::order::{NewOrder};

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
pub trait Repository: Send + Sync {
    async fn register_order(&self, new_order: &NewOrder) -> RepositoryResult<()>;
}
