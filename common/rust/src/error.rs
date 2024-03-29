use std::sync::Arc;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("{0}")]
    Validation(String),
    #[error(r#"{entity_type} was not found for entity_id "{entity_id}" and user_id "{user_id}"."#)]
    NotFound {
        entity_type: &'static str,
        entity_id: String,
        user_id: String,
    },
    #[error(transparent)]
    RepositoryError(anyhow::Error),
    #[error("{0}")]
    Unexpected(String),
}

impl From<ValidationErrors> for DomainError {
    fn from(err: ValidationErrors) -> Self {
        DomainError::Validation(err.to_string())
    }
}

impl From<sqlx::Error> for DomainError {
    fn from(error: sqlx::Error) -> Self {
        DomainError::RepositoryError(anyhow::Error::new(error))
    }
}

#[derive(Debug, Error)]
pub enum UseCaseError {
    #[error("{0}")]
    Validation(String),
    #[error(r#"{entity_type} was not found for entity_id "{entity_id}" and user_id "{user_id}"."#)]
    NotFound {
        entity_type: &'static str,
        entity_id: String,
        user_id: String,
    },
    #[error(transparent)]
    Other(anyhow::Error),
    #[error("{0}")]
    Unexpected(String),
}

impl From<DomainError> for UseCaseError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::Validation(message) => UseCaseError::Validation(message),
            DomainError::NotFound {
                entity_type,
                entity_id,
                user_id,
            } => UseCaseError::NotFound {
                entity_type,
                entity_id,
                user_id,
            },
            DomainError::RepositoryError(_) => UseCaseError::Other(anyhow::Error::new(err)),
            DomainError::Unexpected(message) => UseCaseError::Unexpected(message),
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum AdapterError {
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Validation(String),
    #[error(transparent)]
    OtherError(Arc<anyhow::Error>),
    #[error("{0}")]
    Unexpected(String),
}

impl From<UseCaseError> for AdapterError {
    fn from(err: UseCaseError) -> Self {
        match err {
            UseCaseError::NotFound { .. } => AdapterError::NotFound(err.to_string()),
            UseCaseError::Validation(_) => AdapterError::Validation(err.to_string()),
            UseCaseError::Other(_) => AdapterError::OtherError(Arc::new(anyhow::Error::new(err))),
            UseCaseError::Unexpected(message) => AdapterError::Unexpected(message),
        }
    }
}
