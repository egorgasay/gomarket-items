use crate::domain::models::items::{GetItemsQuery, GetItemsSortBy};
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;

use crate::domain::repositories::items::Repository;
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
        sort_by: Option<GetItemsSortBy>,
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
                        select = select.filter(
                            items::name.ilike(
                                partly
                                    .iter()
                                    .map(|el| format!("%{}%", el))
                                    .collect::<Vec<String>>()
                                    .join("OR ILIKE"),
                            ),
                        );
                    }
                }

                select = select.offset(offset).limit(limit)
            }

            if let Some(sort_by) = sort_by {
                let target = match (sort_by.field.as_str(), sort_by.desc) {
                    ("price", true) => select.order_by(items::price.desc()),
                    ("price", false) => select.order_by(items::price.asc()),
                    ("name", true) => select.order_by(items::name.desc()),
                    ("name", false) => select.order_by(items::name.asc()),
                    ("id", true) => select.order_by(items::id.desc()),
                    ("id", false) => select.order_by(items::id.asc()),
                    (&_, _) => select.order_by(items::price.asc()),
                };

                 select = target
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
