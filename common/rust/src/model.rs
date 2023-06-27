use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ID(Uuid);

impl ID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    // Uuidを文字列に変換します
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}

impl From<Uuid> for ID {
    fn from(value: Uuid) -> Self {
        ID(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Validate, Default)]
pub struct Model {
    pub id: ID,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Model {
    pub fn new() -> Self {
        let current_time = Utc::now();
        Self {
            id: ID::new(),
            created_at: current_time,
            updated_at: current_time,
        }
    }
}
