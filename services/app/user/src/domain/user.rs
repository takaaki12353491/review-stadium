use common::model::Model;

pub struct User {
    pub model: Model,
    pub user_id: String,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(user_id: String, name: String, email: String) -> Self {
        Self {
            model: Model::new(),
            user_id: user_id,
            name,
            email,
        }
    }
}
