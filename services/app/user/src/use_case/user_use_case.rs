use super::user_dto::UserDTO;
use async_trait::async_trait;
use common::error::UseCaseError;

#[async_trait]
pub trait RegisterUserUseCase: Send + Sync + 'static {
    async fn execute(
        &self,
        user_id: String,
        name: String,
        email: String,
    ) -> Result<UserDTO, UseCaseError>;
}
