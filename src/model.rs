use chrono::{DateTime, Local};

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
