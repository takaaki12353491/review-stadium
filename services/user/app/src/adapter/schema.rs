use crate::domain::user::User;
use crate::use_case::user_use_case::UserUseCase;
use async_graphql::{Object, SimpleObject};
use common::error::AdapterError;

#[derive(SimpleObject, Clone)]
pub struct UserObject {
    id: String,
    id_name: String,
    name: String,
    email: String,
}

impl From<User> for UserObject {
    fn from(user: User) -> Self {
        Self {
            id: user.model.id.into_string(),
            id_name: user.id_name,
            name: user.name,
            email: user.email.unwrap_or_default(),
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
    async fn get_by_id_name(&self, id_name: String) -> Result<Option<UserObject>, AdapterError> {
        let user = self.uc.get_by_id_name(id_name).await?;
        Ok(user.map(|user| user.into()))
    }
}
