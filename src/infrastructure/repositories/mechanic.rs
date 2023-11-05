use std::sync::Arc;
use actix_threadpool::run;
use actix_web::web::block;
use async_trait::async_trait;
use diesel::prelude::*;
use log::log;
use crate::domain::error::RepositoryError;

use crate::domain::repositories::repository::{RepositoryResult};
use crate::domain::repositories::todo::{Repository};
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::error::*;
use crate::infrastructure::models::mechanic::{MechanicDiesel};
use crate::infrastructure::schema::mechanics::dsl::mechanics;

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
    async fn new_mechanic(&self, mechanic: MechanicDiesel) -> RepositoryResult<()> {
        let pool = self.pool.clone();
        let mut conn = pool.get()?;

        let _ = block(move || {
            diesel::insert_into(mechanics)
                .values(mechanic)
                .execute(&mut conn)
        }).await??;

        Ok(())
    }
}