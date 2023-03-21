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

    impl From<User> for Result<User, DomainError> {
        fn from(user: User) -> Self {
            user.validate()?;
            Ok(user)
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

        fn from_name_length(len: usize) -> Result<Self, DomainError> {
            User {
                name: "a".repeat(len),
                ..User::default()
            }
            .into()
        }

        fn from_email(email: &str) -> Result<Self, DomainError> {
            User {
                email: email.to_string().to_option(),
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
        let is_under = matches!(
            User::from_user_id_length(0),
            Err(DomainError::Validation(_))
        );
        let is_min = matches!(User::from_user_id_length(1), Ok(_));
        let is_max = matches!(User::from_user_id_length(20), Ok(_));
        let is_over = matches!(
            User::from_user_id_length(21),
            Err(DomainError::Validation(_))
        );
        assert!(is_under && is_min && is_max && is_over);
    }

    // nameは必須で１〜２55文字
    #[test]
    fn test_name() {
        let is_under = matches!(User::from_name_length(0), Err(DomainError::Validation(_)));
        let is_min = matches!(User::from_name_length(1), Ok(_));
        let is_max = matches!(User::from_name_length(255), Ok(_));
        let is_over = matches!(User::from_name_length(256), Err(DomainError::Validation(_)));
        assert!(is_under && is_min && is_max && is_over);
    }

    // emailは必須ではない。RFC5322に従っている。
    #[test]
    fn test_email() {
        let is_nothing = matches!(User::from_email(""), Ok(_));
        let is_invalid = matches!(
            User::from_email("example.com"),
            Err(DomainError::Validation(_))
        );
        assert!(is_nothing && is_invalid);
    }
}
