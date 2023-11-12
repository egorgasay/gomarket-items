use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::item::{GetItemsQuery, Item};
use crate::domain::repositories::mechanic::{Repository, TodoQueryParams};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::services::order::CoreService;
use crate::infrastructure::models::items::{ItemDiesel, ItemsSizesDiesel, SizeDiesel};

#[derive(Clone)]
pub struct CoreServiceImpl {
    pub repository: Arc<dyn Repository>,
}

impl CoreServiceImpl {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        CoreServiceImpl { repository }
    }
}

#[async_trait]
impl CoreService for CoreServiceImpl {
    async fn get_items(
        &self,
        query: Option<GetItemsQuery>,
        sort_by: Option<String>,
        offset: i64,
        limit: i64,
    ) -> Result<ResultPaging<Item>, CommonError> {
        let items: Vec<Item> = self
            .repository
            .get_items(query, sort_by, offset, limit)
            .await?
            .into_iter()
            .map(
                |item: (ItemDiesel, Vec<SizeDiesel>, Vec<ItemsSizesDiesel>)| -> Item {
                    item.into()
                },
            )
            .collect();

        Ok(ResultPaging {
            offset,
            total: items.len() as i64,
            items,
        })
    }
}
