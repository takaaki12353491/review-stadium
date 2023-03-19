use super::user_use_case::UserUseCase;
use crate::domain::{user::User, user_repository::UserRepository};
use async_trait::async_trait;
use common::error::UseCaseError;

#[derive(Debug, Clone)]
pub struct UserInteractor<UR> {
    user_repository: UR,
}

impl<UR> UserInteractor<UR> {
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<UR> UserUseCase for UserInteractor<UR>
where
    UR: UserRepository,
{
    async fn register(
        &self,
        user_id: String,
        name: String,
        email: String,
    ) -> Result<User, UseCaseError> {
        let user = User::new(user_id, name, email)?;
        self.user_repository.create(&user).await?;
        Ok(user)
    }

    async fn get_by_user_id(&self, user_id: String) -> Result<Option<User>, UseCaseError> {
        let user = self.user_repository.find_by_user_id(&user_id).await?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user_repository::MockUserRepository;
    use anyhow::Error;
    use common::error::DomainError;
    use mockall::predicate::always;

    #[tokio::test]
    async fn test_register_success() {
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_create()
            .times(1)
            .with(always())
            .returning(|_| Ok(()));

        let user_usecase = UserInteractor::new(mock_user_repository);
        let res = user_usecase
            .register(
                String::from("user_id"),
                String::from("name"),
                String::from("sample@example.com"),
            )
            .await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_register_failed() {
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_create()
            .times(1)
            .with(always())
            .returning(|_| Err(DomainError::RepositoryError(Error::msg("Database Error"))));

        let user_usecase = UserInteractor::new(mock_user_repository);
        let res = user_usecase
            .register(
                String::from("user_id"),
                String::from("name"),
                String::from("sample@example.com"),
            )
            .await;

        assert!(res.is_err());
    }
}
