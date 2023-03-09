use ulid::Ulid;

pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
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
