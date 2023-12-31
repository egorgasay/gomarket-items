use diesel;
use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;

pub type PostgresPool = Pool<diesel::pg::PgConnection>;
pub type DBConn = PostgresPool;

pub fn db_pool(database_url: String, pool_size: u32) -> DBConn {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(pool_size)
        .build(manager)
        .expect("Failed to register_order pool")
}
