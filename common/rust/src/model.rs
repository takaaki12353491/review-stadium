use ulid::Ulid;

pub struct ID {
    value: String,
}

impl ID {
    pub fn new() -> Self {
        let value = Ulid::new().to_string();
        Self { value }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn into_string(self) -> String {
        self.value
    }
}

pub struct Model {
    pub id: ID,
}

impl Model {
    pub fn new() -> Self {
        Self { id: ID::new() }
    }
}
