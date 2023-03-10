use ulid::Ulid;
use validator::Validate;

pub struct ID(String);

impl ID {
    pub fn new() -> Self {
        Self(Ulid::new().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Validate)]
pub struct Model {
    pub id: ID,
}

impl Model {
    pub fn new() -> Self {
        Self { id: ID::new() }
    }
}
