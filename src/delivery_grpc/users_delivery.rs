use crate::delivery_grpc::users_delivery::auth::{GetUserRequest, GetUserResponse};
use crate::errors::DBError;
use async_trait::async_trait;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod auth {
    include!("../gen/auth.rs");
}

use auth::users_provider_server::UsersProvider;

#[async_trait]
pub trait IUserIDGetter: Send + Sync {
    async fn get_user(&self, session_id: Uuid) -> Result<Option<Uuid>, DBError>;
}

pub struct UsersDeliveryGRPC {
    user_id_getter: Arc<dyn IUserIDGetter>,
}

impl UsersDeliveryGRPC {
    pub fn new(user_id_getter: Arc<dyn IUserIDGetter>) -> Self {
        UsersDeliveryGRPC { user_id_getter }
    }
}

#[tonic::async_trait]
impl UsersProvider for UsersDeliveryGRPC {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let raw_session_id = request.into_inner().session_id;

        let session_id = if let Ok(id) = Uuid::parse_str(raw_session_id.as_str()) {
            id
        } else {
            return Err(Status::invalid_argument(
                "failed to parse session_id to uuid",
            ));
        };

        match self.user_id_getter.get_user(session_id).await {
            Ok(Some(user_id)) => {
                let message = GetUserResponse {
                    user_id: user_id.to_string(),
                };

                Ok(Response::new(message))
            }

            Ok(None) => Err(Status::not_found("user not found")),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
