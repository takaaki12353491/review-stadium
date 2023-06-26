use crate::domain::{user::User, user_repository::UserRepository};
use common::error::UseCaseError;

#[derive(Debug, Clone)]
pub struct QueryUseCase<UR: UserRepository> {
    user_repository: UR,
}

impl<UR: UserRepository> QueryUseCase<UR> {
    pub fn new(user_repository: UR) -> Self {
        Self { user_repository }
    }
}

impl<UR: UserRepository> QueryUseCase<UR> {
    pub async fn get_by_id_name(&self, id_name: &str) -> Result<Option<User>, UseCaseError> {
        let user = self.user_repository.find_by_id_name(id_name).await?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user_repository::MockUserRepository;
    use anyhow::Error;
    use common::error::DomainError;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn test_get_by_id_name_success() {
        let id_name = String::from("id_name");
        let user = User::new(
            id_name.clone(),
            String::from("name"),
            String::from("sample@example.com"),
        )
        .unwrap();
        let saved_user = user.clone();

        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_find_by_id_name()
            .times(1)
            .with(eq(id_name.clone()))
            .returning(move |_| Ok(Some(saved_user.clone())));

        let user_usecase = QueryUseCase::new(mock_user_repository);
        let res = user_usecase.get_by_id_name(&id_name).await;

        assert_eq!(res.unwrap(), Some(user));
    }

    #[tokio::test]
    async fn test_get_by_id_name_not_found() {
        let id_name = String::from("id_name");

        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_find_by_id_name()
            .times(1)
            .with(eq(id_name.clone()))
            .returning(|_| Ok(None));

        let user_usecase = QueryUseCase::new(mock_user_repository);
        let res = user_usecase.get_by_id_name(&id_name).await;

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), None);
    }

    #[tokio::test]
    async fn test_get_by_id_name_failed() {
        let id_name = String::from("id_name");

        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_find_by_id_name()
            .times(1)
            .with(eq(id_name.clone()))
            .returning(|_| Err(DomainError::RepositoryError(Error::msg("Database Error"))));

        let user_usecase = QueryUseCase::new(mock_user_repository);
        let res = user_usecase.get_by_id_name(&id_name).await;

        assert!(res.is_err());
    }
}
