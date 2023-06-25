use crate::domain::user::User;
use async_trait::async_trait;
use common::error::UseCaseError;

#[async_trait]
pub trait UserUseCase: Send + Sync + 'static {
    async fn register(
        &self,
        id_name: String,
        name: String,
        email: String,
    ) -> Result<User, UseCaseError>;

    async fn get_by_id_name(&self, id_name: String) -> Result<Option<User>, UseCaseError>;
}
