use crate::model::User;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use validator::Validate;

const USER_NOT_FOUND_MSG: &str = "user not found";

#[derive(Clone, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, message = "Username should not be empty"))]
    pub username: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Clone, Deserialize)]
pub struct LoginRequest {
    pub email: String,
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
