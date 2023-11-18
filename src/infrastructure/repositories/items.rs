use crate::domain::models::items::{GetItemsQuery, GetItemsSortBy};
use async_trait::async_trait;
use diesel::internal::table_macro::{BoxedSelectStatement, NoFromClause};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::sql_types::{Int8, Integer, VarChar};
use std::sync::Arc;
use testcontainers::Container;

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
                    if let Some(from) = price.from {
                        select = select.filter(items::price.ge(from));
                    }

                    if let Some(to) = price.to {
                        select = select.filter(items::price.le(to));
                    }
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

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::sync::{Mutex, RwLock};
    use super::*;
    use crate::infrastructure::databases::postgresql::db_pool;
    use diesel::connection::SimpleConnection;
    use std::thread;
    use actix_web::web::get;
    use diesel::r2d2::{ConnectionManager, PooledConnection};
    use lazy_static::lazy_static;
    use testcontainers::clients;
    use testcontainers::images::postgres;
    use testcontainers::images::postgres::Postgres;
    use crate::domain::models::items::PriceGetItemsQuery;

    fn migrate_tables(mut conn: Arc<Mutex<PooledConnection<ConnectionManager<PgConnection>>>>) {
        let mut conn = conn.lock().unwrap();

        conn.batch_execute("CREATE TABLE items (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    price DOUBLE PRECISION NOT NULL
);").unwrap();
        conn
            .batch_execute("CREATE TABLE sizes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);")
            .unwrap();
        conn.batch_execute("CREATE TABLE items_sizes (
    id BIGSERIAL PRIMARY KEY,
    item_id BIGSERIAL NOT NULL,
    size_id SERIAL NOT NULL,
    quantity INTEGER NOT NULL,
    FOREIGN KEY (item_id) REFERENCES items(id),
    FOREIGN KEY (size_id) REFERENCES sizes(id)
);").unwrap();
    }

    fn insert_test_data(connection: Arc<Mutex<PooledConnection<ConnectionManager<PgConnection>>>>) {
        let mut connection = connection.lock().unwrap();

        connection
            .batch_execute("INSERT INTO items (name, price, description) VALUES ('Item 1', 1000, '')")
            .unwrap();
        connection
            .batch_execute("INSERT INTO items (name, price, description) VALUES ('Item 2', 2000, '')")
            .unwrap();
        connection
            .batch_execute("INSERT INTO items (name, price, description) VALUES ('Item 3', 3000, '')")
            .unwrap();

        connection
            .batch_execute("INSERT INTO sizes (name) VALUES ('S')")
            .unwrap();
        connection
            .batch_execute("INSERT INTO sizes (name) VALUES ('M')")
            .unwrap();
        connection
            .batch_execute("INSERT INTO sizes (name) VALUES ('L')")
            .unwrap();

        connection
            .batch_execute("INSERT INTO items_sizes (item_id, size_id, quantity) VALUES (1, 1, 1)")
            .unwrap();
        connection
            .batch_execute("INSERT INTO items_sizes (item_id, size_id, quantity) VALUES (2, 2, 0)")
            .unwrap();
        connection
            .batch_execute("INSERT INTO items_sizes (item_id, size_id, quantity) VALUES (3, 3, 100)")
            .unwrap();
    }

    fn get_items_wanted() -> Vec<(ItemDiesel, Vec<SizeDiesel>, Vec<ItemsSizesDiesel>)> {
        vec![
            (
                ItemDiesel {
                    id: 1,
                    name: "Item 1".to_string(),
                    description: "".to_string(),
                    price: 1000.0,
                },
                vec![SizeDiesel {
                    id: 1,
                    name: "S".to_string(),
                }],
                vec![ItemsSizesDiesel {
                    id: 1,
                    item_id: 1,
                    size_id: 1,
                    quantity: 1,
                }],
            ),
            (
                ItemDiesel {
                    id: 2,
                    name: "Item 2".to_string(),
                    description: "".to_string(),
                    price: 2000.0,
                },
                vec![SizeDiesel {
                    id: 2,
                    name: "M".to_string(),
                }],
                vec![ItemsSizesDiesel {
                    id: 2,
                    item_id: 2,
                    size_id: 2,
                    quantity: 0,
                }],
            ),
            (
                ItemDiesel {
                    id: 3,
                    name: "Item 3".to_string(),
                    description: "".to_string(),
                    price: 3000.0,
                },
                vec![SizeDiesel {
                    id: 3,
                    name: "L".to_string(),
                }],
                vec![ItemsSizesDiesel {
                    id: 3,
                    item_id: 3,
                    size_id: 3,
                    quantity: 100,
                }],
            ),
        ]
    }

    #[tokio::test]
    async fn test_get_items_all() {
        let docker = clients::Cli::default();
        let image = postgres::Postgres::default();
        let container = docker.run(image);
        let conn_string = format!(
            "postgresql://postgres:postgres@127.0.0.1:{}/postgres",
            container.get_host_port_ipv4(5432),
        );

        let pool = db_pool(conn_string);
        let db = Arc::new(DieselRepository::new(Arc::new(pool.clone())));

        let connection = pool.get().unwrap();
        let protected_conn = Arc::new(Mutex::new(connection));

        migrate_tables(protected_conn.clone());
        insert_test_data(protected_conn);
        let want = get_items_wanted();

        let res = db.get_items(None, None, 0, 10).await.unwrap();
        assert_eq!(want, res);
    }

    #[tokio::test]
    async fn test_filter_by_price() {
        let docker = clients::Cli::default();
        let image = postgres::Postgres::default();
        let container = docker.run(image);
        let conn_string = format!(
            "postgresql://postgres:postgres@127.0.0.1:{}/postgres",
            container.get_host_port_ipv4(5432),
        );

        let pool = db_pool(conn_string);
        let db = Arc::new(DieselRepository::new(Arc::new(pool.clone())));

        let connection = pool.get().unwrap();
        let protected_conn = Arc::new(Mutex::new(connection));

        migrate_tables(protected_conn.clone());
        insert_test_data(protected_conn);
        let want = get_items_wanted()[2..].to_vec();

        let res = db.get_items(Some(GetItemsQuery{
            ids: None,
            price: Some(PriceGetItemsQuery{ from: Some(3000.0), to: None }),
            names: None,
        }), None, 0, 10).await.unwrap();
        assert_eq!(want, res);
    }
}
