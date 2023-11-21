use actix_threadpool::BlockingError;
use diesel::r2d2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CommonError {
    pub message: String,
    pub code: CommonErrorKind,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum CommonErrorKind {
    AlreadyExists,
    NotFound,
    Unknown,
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {:?}", self.message, self.code)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
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
        match &self.0.code {
            CommonErrorKind::AlreadyExists => actix_web::HttpResponse::Conflict().json(&self.0),
            CommonErrorKind::NotFound => actix_web::HttpResponse::Gone().json(&self.0),
            CommonErrorKind::Unknown => {
                actix_web::HttpResponse::InternalServerError().json(&self.0)
            }
        }
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
    pub code: RepositoryErrorKind,
}

#[derive(Debug)]
pub enum RepositoryErrorKind {
    UniqueViolation,
    NotFound,
    Unknown,
}

impl From<RepositoryError> for CommonError {
    fn from(error: RepositoryError) -> CommonError {
        CommonError {
            message: error.message,
            code: match error.code {
                RepositoryErrorKind::NotFound => CommonErrorKind::NotFound,
                RepositoryErrorKind::UniqueViolation => CommonErrorKind::AlreadyExists,
                RepositoryErrorKind::Unknown => CommonErrorKind::Unknown,
            },
        }
    }
}

pub type AsyncPoolError<T> = BlockingError<T>;
impl From<r2d2::PoolError> for RepositoryError {
    fn from(value: r2d2::PoolError) -> Self {
        RepositoryError {
            message: value.to_string(),
            code: RepositoryErrorKind::Unknown,
        }
    }
}

impl From<actix_web::error::BlockingError> for RepositoryError {
    fn from(error: actix_web::error::BlockingError) -> RepositoryError {
        RepositoryError {
            message: error.to_string(),
            code: RepositoryErrorKind::Unknown,
        }
    }
}

impl<T: std::fmt::Debug> From<AsyncPoolError<T>> for RepositoryError {
    fn from(error: AsyncPoolError<T>) -> RepositoryError {
        RepositoryError {
            message: error.to_string(),
            code: RepositoryErrorKind::Unknown,
        }
    }
}

impl From<diesel::result::Error> for RepositoryError {
    fn from(error: diesel::result::Error) -> RepositoryError {
        match error {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                info,
            ) => RepositoryError {
                message: info.message().to_string(),
                code: RepositoryErrorKind::UniqueViolation,
            },
            diesel::result::Error::NotFound => RepositoryError {
                message: error.to_string(),
                code: RepositoryErrorKind::NotFound,
            },
            _ => RepositoryError {
                message: error.to_string(),
                code: RepositoryErrorKind::Unknown,
            },
        }
    }
}
