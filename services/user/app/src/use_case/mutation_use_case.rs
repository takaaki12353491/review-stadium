use crate::domain::user::User;
use async_trait::async_trait;
use common::error::UseCaseError;

#[async_trait]
pub trait MutationUseCase: Send + Sync + 'static {
    async fn register(
        &self,
        id_name: String,
        name: String,
        email: String,
    ) -> Result<User, UseCaseError>;
}
