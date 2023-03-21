use common::{error::DomainError, model::Model, string::StringExt};
use validator::Validate;

#[derive(Debug, Validate, Clone, PartialEq)]
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

    impl Default for User {
        fn default() -> Self {
            Self {
                model: Model::new(),
                user_id: "user_id".to_string(),
                name: "name".to_string(),
                email: Some("sample@example.com".to_string()),
            }
        }
    }

    impl Into<Result<Self, DomainError>> for User {
        fn into(self) -> Result<Self, DomainError> {
            self.validate()?;
            Ok(self)
        }
    }

    impl User {
        fn from_user_id_length(len: usize) -> Result<Self, DomainError> {
            User {
                user_id: "a".repeat(len),
                ..User::default()
            }
            .into()
        }
    }

    #[test]
    fn test_valid_user() {
        assert!(matches!(User::default().into(), Ok(_)));
    }

    // user_idは必須で１〜２０文字
    #[test]
    fn test_user_id() {
        let is_valid_0 = matches!(
            User::from_user_id_length(0),
            Err(DomainError::Validation(_))
        );
        let is_valid_1 = matches!(User::from_user_id_length(1), Ok(_));
        let is_valid_20 = matches!(User::from_user_id_length(20), Ok(_));
        let is_valid_21 = matches!(
            User::from_user_id_length(21),
            Err(DomainError::Validation(_))
        );
        assert!(is_valid_0 && is_valid_1 && is_valid_20 && is_valid_21);
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
            "a".to_string(),
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
