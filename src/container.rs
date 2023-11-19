use std::env;
use crate::domain::repositories::items::Repository;
use crate::domain::services::order::CoreService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::items::DieselRepository;
use std::sync::Arc;
use dotenv::dotenv;
use crate::domain::constants::POSTGRESQL_DB_URI;
use crate::services::items::CoreServiceImpl;

pub struct Container {
    pub core_service: Arc<dyn CoreService>,
}

impl Container {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var(POSTGRESQL_DB_URI)
            .expect(&*format!("{value} must be set", value = POSTGRESQL_DB_URI));
        let repository: Arc<dyn Repository> = Arc::new(DieselRepository::new(Arc::new(db_pool(database_url))));
        let core_service = Arc::new(CoreServiceImpl::new(repository));
        Container { core_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
