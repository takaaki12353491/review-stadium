use crate::domain::user::User;
use crate::use_case::user_use_case::UserUseCase;
use async_graphql::{Object, SimpleObject};
use common::error::AdapterError;

#[derive(SimpleObject, Clone)]
pub struct UserObject {
    id: String,
    user_id: String,
    name: String,
    email: String,
}

impl From<User> for UserObject {
    fn from(user: User) -> Self {
        Self {
            id: user.model.id.into_string(),
            user_id: user.user_id,
            name: user.name,
            email: user.email,
        }
    }
}

pub struct Query<UC> {
    uc: UC,
}

impl<UC> Query<UC> {
    pub fn new(uc: UC) -> Self {
        Self { uc }
    }
}

#[Object]
impl<UC> Query<UC>
where
    UC: UserUseCase,
{
    async fn get_by_user_id(&self, user_id: String) -> Result<Option<UserObject>, AdapterError> {
        let user = self.uc.get_by_user_id(user_id).await?;
        Ok(user.map(|user| user.into()))
    }
}

pub struct Mutation<UC> {
    uc: UC,
}

impl<UC> Mutation<UC> {
    pub fn new(uc: UC) -> Self {
        Self { uc }
    }
}

#[Object]
impl<UC> Mutation<UC>
where
    UC: UserUseCase,
{
    async fn register(
        &self,
        user_id: String,
        name: String,
        email: String,
    ) -> Result<UserObject, AdapterError> {
        let user = self.uc.register(user_id, name, email).await?;
        Ok(UserObject::from(user))
    }
}
