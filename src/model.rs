use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
struct User {
    id: uuid::Uuid,
    email: String,
    username: String,
    password_hash: String,
    salt: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}