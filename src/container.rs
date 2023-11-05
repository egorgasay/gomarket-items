use std::sync::Arc;
use crate::domain::repositories::todo::Repository;
use crate::domain::services::service_context::ServiceContextService;
use crate::domain::services::order::CoreService;
use crate::infrastructure::databases::postgresql::db_pool;
use crate::infrastructure::repositories::mechanic::DieselRepository;
//use crate::infrastructure::services::service_context::ServiceContextServiceImpl;
use crate::services::mechanic::CoreServiceImpl;

pub struct Container {
    pub core_service: Arc<dyn CoreService>,
    // pub service_context_service: Arc<dyn ServiceContextService>
}

impl Container {
    pub fn new() -> Self {
        let repository: Arc<dyn Repository> = Arc::new(
            DieselRepository::new(Arc::new(db_pool()))
        );
        let core_service = Arc::new(
            CoreServiceImpl::new(repository)
        );
        // let service_context_service = Arc::new(
        //     ServiceContextServiceImpl::new(Arc::new(db_pool()))
        // );
        Container { core_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
