use crate::domain::user::User;
use async_trait::async_trait;
use common::error::UseCaseError;

#[async_trait]
pub trait RegisterUserUseCase: Send + Sync + 'static {
    async fn execute(
        &self,
        user_id: String,
        name: String,
        email: String,
    ) -> Result<User, UseCaseError>;
}
