use crate::model::User;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

const USER_NOT_FOUND_MSG: &str = "user not found";

#[derive(Clone, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Deserialize)]
pub struct UpdateUserRequest {
    pub new_username: String,
}

#[derive(Clone, Serialize)]
pub struct UserNotFoundResponse {
    pub error: String,
}

impl Default for UserNotFoundResponse {
    fn default() -> Self {
        UserNotFoundResponse {
            error: USER_NOT_FOUND_MSG.to_string(),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        UserResponse {
            id: value.id,
            email: value.email,
            username: value.username,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
