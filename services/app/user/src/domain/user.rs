use common::{error::DomainError, model::Model, string::StringExt};
use validator::Validate;

#[derive(Debug, Validate)]
pub struct User {
    pub model: Model,
    #[validate(length(min = 1, max = 20))]
    pub user_id: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(email)]
    pub email: Option<String>,
}

impl User {
    pub fn new(user_id: String, name: String, email: String) -> Result<Self, DomainError> {
        let user = Self {
            model: Model::new(),
            user_id,
            name,
            email: email.to_option(),
        };
        user.validate()?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::error::DomainError;

    #[test]
    fn test_valid_user() {
        assert!(matches!(
            User::new(
                String::from("user_id"),
                String::from("name"),
                String::from("sample@example.com")
            ),
            Ok(_)
        ));
    }

    // user_idは必須で１〜２０文字
    #[test]
    fn test_user_id() {
        let no_user_id = User::new(
            "a".repeat(0),
            String::from("name"),
            String::from("sample@example.com"),
        );
        let min_user_id = User::new(
            "a".repeat(1),
            String::from("name"),
            String::from("sample@example.com"),
        );
        let max_user_id = User::new(
            "a".repeat(20),
            String::from("name"),
            String::from("sample@example.com"),
        );
        let over_user_id = User::new(
            "a".repeat(21),
            String::from("name"),
            String::from("sample@example.com"),
        );
        assert!(
            matches!(no_user_id, Err(DomainError::Validation(_)))
                && matches!(min_user_id, Ok(_))
                && matches!(max_user_id, Ok(_))
                && matches!(over_user_id, Err(DomainError::Validation(_)))
        );
    }

    // nameは必須で１〜２55文字
    #[test]
    fn test_name() {
        let no_name = User::new(
            String::from("user_id"),
            "a".repeat(0),
            String::from("sample@example.com"),
        );
        let min_name = User::new(
            String::from("user_id"),
            "a".repeat(1),
            String::from("sample@example.com"),
        );
        let max_name = User::new(
            String::from("user_id"),
            "a".repeat(255),
            String::from("sample@example.com"),
        );
        let over_name = User::new(
            String::from("user_id"),
            "a".repeat(256),
            String::from("sample@example.com"),
        );
        assert!(
            matches!(no_name, Err(DomainError::Validation(_)))
                && matches!(min_name, Ok(_))
                && matches!(max_name, Ok(_))
                && matches!(over_name, Err(DomainError::Validation(_)))
        );
    }

    // emailは必須ではない。RFC5322に従っている。
    #[test]
    fn test_email() {
        let no_email = User::new(String::from("user_id"), String::from("name"), "a".repeat(0));
        let invalid_email = User::new(
            String::from("user_id"),
            String::from("name"),
            String::from("example.com"),
        );
        let valid_email = User::new(
            String::from("user_id"),
            String::from("name"),
            String::from("sample@example.com"),
        );
        assert!(
            matches!(no_email, Ok(_))
                && matches!(invalid_email, Err(DomainError::Validation(_)))
                && matches!(valid_email, Ok(_))
        );
    }
}
