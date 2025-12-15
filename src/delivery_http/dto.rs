use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Serialize)]
pub struct RegisterResponse {
    pub user_id: uuid::Uuid,
    pub username: String,
}