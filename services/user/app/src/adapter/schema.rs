use crate::domain::user::User;
use async_graphql::SimpleObject;

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
