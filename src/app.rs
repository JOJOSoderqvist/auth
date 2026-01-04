use crate::config::AppConfig;
use crate::delivery_http::dto::{LoginRequest, RegisterRequest, UpdateUserRequest};
use crate::delivery_http::users_delivery::{IUsersRepo, UsersDelivery};
use crate::errors::ApiError;
use crate::handlers::{create_user, delete_user, get_user, login, logout, update_user};
use crate::infra::postgres::PGPool;
use crate::infra::redis::RedisPool;
use crate::repo::sessions::SessionsRepo;
use crate::repo::users_repo::UsersRepo;
use crate::usecase::users_usecase::{IUsersRepository, UserUsecase};
use async_trait::async_trait;
use axum::extract::Path;
use axum::response::Response;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use std::process;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait IUsersDelivery: Send + Sync {
    async fn create_user(
        &self,
        jar: CookieJar,
        payload: Json<RegisterRequest>,
    ) -> Result<Response, ApiError>;
    async fn get_user(&self, payload: Path<Uuid>) -> Result<Response, ApiError>;
    async fn update_user(
        &self,
        id: Path<Uuid>,
        payload: Json<UpdateUserRequest>,
    ) -> Result<Response, ApiError>;
    async fn delete_user(&self, payload: Path<Uuid>) -> Result<Response, ApiError>;
    async fn login(
        &self,
        jar: CookieJar,
        payload: Json<LoginRequest>,
    ) -> Result<Response, ApiError>;
    async fn logout(&self, jar: CookieJar) -> Result<Response, ApiError>;
}

pub struct AuthApp {
    pub delivery: Arc<dyn IUsersDelivery>,
}

impl AuthApp {
    pub async fn new(config: AppConfig) -> Self {
        let pool = match PGPool::new(config.postgres_conn_string).await {
            Ok(pool) => pool,
            Err(e) => {
                eprintln!("error getting pg pool: {e}");
                process::exit(1)
            }
        };

        let redis_pool = match RedisPool::new(config.redis_conn_string) {
            Ok(pool) => pool,
            Err(e) => {
                eprintln!("error getting redis pool {e}");
                process::exit(1);
            }
        };

        let session_repo = Arc::new(SessionsRepo::new(redis_pool));

        let repo = Arc::new(UsersRepo::new(pool));

        let repo_for_usecase: Arc<dyn IUsersRepository> = repo.clone();
        let repo_for_delivery: Arc<dyn IUsersRepo> = repo.clone();

        let usecase = UserUsecase::new(repo_for_usecase);

        let delivery = Arc::new(UsersDelivery::new(
            repo_for_delivery,
            Arc::new(usecase),
            session_repo,
        ));

        AuthApp { delivery }
    }
}

pub fn init_router(state: Arc<AuthApp>) -> Router {
    Router::new()
        .route("/api/v1/register", post(create_user))
        .route("/api/v1/users/{id}", get(get_user))
        .route("/api/v1/users/{id}", put(update_user))
        .route("/api/v1/users/{id}", delete(delete_user))
        .route("/api/v1/login", post(login))
        .route("/api/v1/logout", post(logout))
        .with_state(state)
}

pub async fn serve(host: String, port: String, router: Router) {
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
