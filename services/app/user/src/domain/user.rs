use ulid::Ulid;

pub struct User {
    id: String,
    name: String,
    email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self {
            id: Ulid::new().to_string(),
            name,
            email,
        }
    }
}
