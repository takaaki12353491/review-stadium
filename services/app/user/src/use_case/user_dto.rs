use crate::domain::user::User;

pub struct UserDTO {
    pub id: String,
}

impl UserDTO {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO::new(user.id)
    }
}
