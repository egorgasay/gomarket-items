use crate::domain::models::item::{GetItemsQuery};
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::domain::repositories::mechanic::Repository;
use crate::domain::repositories::repository::RepositoryResult;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::items::{ItemDiesel, ItemsSizesDiesel, SizeDiesel};
use crate::infrastructure::schema::items;
use crate::infrastructure::schema::items_sizes;
use crate::infrastructure::schema::sizes;

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
        query: Option<GetItemsQuery>,
        offset: i64,
        limit: i64,
    ) -> RepositoryResult<Vec<(ItemDiesel, Vec<SizeDiesel>, Vec<ItemsSizesDiesel>)>> {
        let pool = self.pool.clone();
        let mut conn = pool.get()?;

        let mut out: Vec<(ItemDiesel, Vec<SizeDiesel>, Vec<ItemsSizesDiesel>)> = vec![];

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let i_table = items::table;
            let mut select = i_table.into_boxed();

            if let Some(query) = query {
                if let Some(ids) = query.ids {
                    select = select.filter(items::id.eq_any(ids));
                }

                if let Some(price) = query.price {
                    select = select.filter(items::price.between(price.from, price.to));
                }

                if let Some(names) = query.names {
                    if let Some(full) = names.full {
                        select = select.filter(items::name.eq_any(full));
                    }

                    if let Some(partly) = names.partly {
                        for name in partly {
                            select = select.or_filter(items::name.ilike(format!("%{}%", name)));
                        }
                    }
                }

                select = select.offset(offset).limit(limit)
            }

            let res = select.load::<ItemDiesel>(conn)?;

            for item in res {
                let is = items_sizes::table
                    .filter(items_sizes::item_id.eq(item.id))
                    .load::<ItemsSizesDiesel>(conn)?;

                let si: Vec<SizeDiesel> = sizes::table
                    .filter(sizes::id.eq_any(is.iter().map(|x| x.size_id).collect::<Vec<i32>>()))
                    .load::<SizeDiesel>(conn)?;

                out.push((item, si, is));
            }

            Ok(())
        })?;

        Ok(out)
    }
}
