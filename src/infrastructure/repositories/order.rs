use std::sync::Arc;
use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;

use crate::domain::models::order::{NewOrder, Good};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::todo::{TodoQueryParams, Repository};
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::models::orders::{NewOrderDiesel, GoodDiesel};

pub struct DieselRepository {
    pub pool: Arc<DBConn>
}

impl DieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        DieselRepository { pool: db }
    }
}

#[async_trait]
impl Repository for DieselRepository {

    async fn register_order(&self, new_order: &NewOrder) -> RepositoryResult<()> {
        use crate::infrastructure::schema::orders::dsl::orders;
        let new_order_diesel: NewOrderDiesel = NewOrderDiesel::from(new_order.clone());
        let mut conn = self.pool.get().unwrap();
        let result: NewOrderDiesel = run(move || diesel::insert_into(orders).values(new_order_diesel)
            .get_result(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(())
    }
    //
    // async fn list(&self, params: TodoQueryParams) -> RepositoryResult<ResultPaging<Todo>> {
    //     use crate::infrastructure::schema::todos::dsl::todos;
    //     let pool = self.pool.clone();
    //     let builder = todos.limit(params.limit()).offset(params.offset());
    //     let result = run(move || {
    //         let mut conn = pool.get().unwrap();
    //         builder.load::<GoodDiesel>(&mut conn)
    //     })
    //         .await
    //         .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
    //     Ok(ResultPaging {
    //         total: 0,
    //         items: result.into_iter().map(|v| v.into()).collect()
    //     })
    // }
    //
    // async fn get(&self, todo_id: i32) -> RepositoryResult<Todo> {
    //     use crate::infrastructure::schema::todos::dsl::{id, todos};
    //     let mut conn = self.pool.get().unwrap();
    //     run(move || todos.filter(id.eq(todo_id)).first::<GoodDiesel>(&mut conn))
    //         .await
    //         .map_err(|v| DieselRepositoryError::from(v).into_inner())
    //         .map(|v| -> Todo { v.into() })
    // }
    //
    // async fn delete(&self, todo_id: i32) -> RepositoryResult<()> {
    //     use crate::infrastructure::schema::todos::dsl::{id, todos};
    //     let mut conn = self.pool.get().unwrap();
    //     run(move || diesel::delete(todos).filter(id.eq(todo_id))
    //         .execute(&mut conn))
    //         .await
    //         .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
    //     Ok(())
    // }
}