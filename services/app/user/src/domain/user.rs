use common::{error::DomainError, model::Model};
use validator::Validate;

#[derive(Debug, Validate)]
pub struct User {
    pub model: Model,
    #[validate(length(min = 1, max = 20))]
    pub user_id: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(email)]
    pub email: String,
}

impl User {
    pub fn new(user_id: String, name: String, email: String) -> Result<Self, DomainError> {
        let user = Self {
            model: Model::new(),
            user_id,
            name,
            email,
        };
        user.validate()?;
        Ok(user)
    }
}
