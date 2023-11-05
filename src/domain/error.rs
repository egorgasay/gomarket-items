use diesel::r2d2;
use diesel::r2d2::Error;
use serde::Serialize;
use crate::infrastructure::error::AsyncPoolError;

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: u32,
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

#[derive(Debug)]
pub struct ApiError(CommonError);

impl From<CommonError> for ApiError {
    fn from(error: CommonError) -> ApiError {
        ApiError(error)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::BadRequest().json(&self.0)
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}

impl Into<CommonError> for RepositoryError {
    fn into(self) -> CommonError {
        CommonError {
            message: self.message,
            code: 1,
        }
    }
}

impl From<r2d2::PoolError> for RepositoryError {
    fn from(value: r2d2::PoolError) -> Self {
        RepositoryError {
            message: value.to_string(),
        }
    }
}

impl From<actix_web::error::BlockingError> for RepositoryError {
    fn from(error: actix_web::error::BlockingError) -> RepositoryError {
        RepositoryError {
            message: error.to_string(),
        }
    }
}

impl<T: std::fmt::Debug> From<AsyncPoolError<T>> for RepositoryError {
    fn from(error: AsyncPoolError<T>) -> RepositoryError {
        RepositoryError {
            message: error.to_string(),
        }
    }
}


impl From<diesel::result::Error> for RepositoryError {
    fn from(error: diesel::result::Error) -> RepositoryError {
        RepositoryError {
            message: error.to_string(),
        }
    }
}
