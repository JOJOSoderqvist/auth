use crate::delivery_http::dto::UpdateUserRequest;
use chrono::{DateTime, Local};

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl From<UpdateUserRequest> for User {
    fn from(value: UpdateUserRequest) -> Self {
        User {
            id: Default::default(),
            email: "".to_string(),
            username: value.new_username,
            password_hash: "".to_string(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}
