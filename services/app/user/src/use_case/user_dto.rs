use crate::domain::user::User;

pub struct UserDto {
    pub id: String,
}

impl UserDto {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto::new(user.id)
    }
}
