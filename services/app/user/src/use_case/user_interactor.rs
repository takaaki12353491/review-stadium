use async_trait::async_trait;
use common::error::UseCaseError;

use crate::{
    domain::{user::User, user_repository::UserRepository},
    use_case::{user_dto::UserDTO, user_use_case::RegisterUserUseCase},
};

pub struct RegisterUserInteractor<UR> {
    user_repository: UR,
}

impl<UR> RegisterUserInteractor<UR> {
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<UR> RegisterUserUseCase for RegisterUserInteractor<UR>
where
    UR: UserRepository,
{
    async fn execute(
        &self,
        user_id: String,
        name: String,
        email: String,
    ) -> Result<UserDTO, UseCaseError> {
        let user = User::new(user_id, name, email);
        self.user_repository.create(&user).await?;
        Ok(UserDTO::new(user.model.id.into_string()))
    }
}
