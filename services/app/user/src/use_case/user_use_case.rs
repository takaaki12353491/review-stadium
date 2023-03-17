use crate::domain::user::User;
use async_trait::async_trait;
use common::error::UseCaseError;

#[async_trait]
pub trait UserUseCase: Send + Sync + 'static {
    async fn register(
        &self,
        user_id: String,
        name: String,
        email: String,
    ) -> Result<User, UseCaseError>;

    async fn get_by_user_id(&self, user_id: String) -> Result<Option<User>, UseCaseError>;
}
