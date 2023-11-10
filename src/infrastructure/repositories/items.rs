use crate::domain::models::item::{GetItemsQuery, Item};
use actix_web::web::block;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::domain::repositories::mechanic::Repository;
use crate::domain::repositories::repository::RepositoryResult;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::items::{ItemDiesel, ItemsSizesDiesel, SizeDiesel};
use crate::infrastructure::schema::items::dsl::items;

pub struct DieselRepository {
    pub pool: Arc<DBConn>,
}

impl DieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        DieselRepository { pool: db }
    }
}

#[async_trait]
impl Repository for DieselRepository {
    async fn get_items(
        &self,
        query: GetItemsQuery,
        offset: i64,
        limit: i64,
    ) -> RepositoryResult<Vec<(ItemDiesel, Vec<SizeDiesel>, Vec<ItemsSizesDiesel>)>> {
        let pool = self.pool.clone();
        let mut conn = pool.get()?;

        let res = block(move || items.load::<ItemDiesel>(&mut conn)).await??;

        let mut out: Vec<(ItemDiesel, Vec<SizeDiesel>, Vec<ItemsSizesDiesel>)> = vec![];
        for item in res {
            out.push((item, vec![], vec![]));
            // let sizes = block(move || SizeDiesel::belonging_to(&item).load(&mut conn))
        }

        Ok(out)
    }
}
