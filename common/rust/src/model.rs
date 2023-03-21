use ulid::Ulid;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Validate, Default)]
pub struct ID {
    #[validate(length(min = 1))]
    value: String,
}

impl ID {
    pub fn new() -> Self {
        Self {
            value: Ulid::new().to_string(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn into_string(self) -> String {
        self.value
    }
}

impl From<String> for ID {
    fn from(value: String) -> Self {
        ID { value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Validate, Default)]
pub struct Model {
    pub id: ID,
}

impl Model {
    pub fn new() -> Self {
        Self { id: ID::new() }
    }
}
