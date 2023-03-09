use async_trait::async_trait;
use common::error::UseCaseError;

use crate::use_case::user_dto::UserDto;

#[async_trait]
pub trait RegisterUserUseCase: Send + Sync + 'static {
    async fn register_user(&self, user_id: &str) -> Result<UserDto, UseCaseError>;
}
