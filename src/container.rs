use crate::domain::constants::{POSTGRESQL_DB_URI, POSTGRESQL_POOL_SIZE, POSTGRESQL_POOL_SIZE_DEFAULT};
use crate::domain::repositories::items::Repository;
use crate::domain::services::order::CoreService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::items::DieselRepository;
use crate::services::items::CoreServiceImpl;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use log::warn;

pub struct Container {
    pub core_service: Arc<dyn CoreService>,
}

impl Container {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var(POSTGRESQL_DB_URI)
            .expect(&*format!("{value} must be set", value = POSTGRESQL_DB_URI));
        let pool_size =  env::var(POSTGRESQL_POOL_SIZE)
            .unwrap_or_else(|e | -> String {
                warn!("{value} doesn't exist! using default = {default}",
                    value = POSTGRESQL_POOL_SIZE, default = POSTGRESQL_POOL_SIZE_DEFAULT);
                "10".to_string()
            })
            .parse::<u32>()
            .expect("POSTGRESQL_POOL_SIZE must be a uint32 number");

        let repository: Arc<dyn Repository> =
            Arc::new(DieselRepository::new(Arc::new(db_pool(database_url, pool_size))));
        let core_service = Arc::new(CoreServiceImpl::new(repository));
        Container { core_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
