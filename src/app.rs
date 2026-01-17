use crate::config::AppConfig;
use crate::delivery_grpc::users_delivery::UsersDeliveryGRPC;
use crate::delivery_grpc::users_delivery::auth::users_provider_server::UsersProviderServer;
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
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, COOKIE};
use axum::http::{HeaderValue, Method};
use axum::response::Response;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use std::net::SocketAddr;
use std::process;
use std::sync::Arc;
use tonic::transport::Server;
use tonic::transport::server::Router as grpc_router;
use tower_http::cors::CorsLayer;
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
    pub http_delivery: Arc<dyn IUsersDelivery>,
}

impl AuthApp {
    pub async fn new(config: AppConfig) -> (Self, grpc_router) {
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

        let sessions_for_grpc = session_repo.clone();

        let repo = Arc::new(UsersRepo::new(pool));

        let repo_for_usecase: Arc<dyn IUsersRepository> = repo.clone();
        let repo_for_delivery: Arc<dyn IUsersRepo> = repo.clone();

        let usecase = UserUsecase::new(repo_for_usecase);

        let delivery = Arc::new(UsersDelivery::new(
            repo_for_delivery,
            Arc::new(usecase),
            session_repo,
        ));

        let grpc_auth = UsersDeliveryGRPC::new(sessions_for_grpc);

        let grpc_router = Server::builder().add_service(UsersProviderServer::new(grpc_auth));

        (
            AuthApp {
                http_delivery: delivery,
            },
            grpc_router,
        )
    }
}

pub fn init_router(state: Arc<AuthApp>) -> Router {
    let origins = [
        "https://writehub.space".parse::<HeaderValue>().unwrap(),
        "https://www.writehub.space".parse::<HeaderValue>().unwrap(),
        "http://localhost:3000".parse::<HeaderValue>().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION, COOKIE])
        .allow_credentials(true);

    Router::new()
        .route("/api/v1/register", post(create_user))
        .route("/api/v1/users/{id}", get(get_user))
        .route("/api/v1/users/{id}", put(update_user))
        .route("/api/v1/users/{id}", delete(delete_user))
        .route("/api/v1/login", post(login))
        .route("/api/v1/logout", post(logout))
        .with_state(state)
        .layer(cors)
}

pub async fn serve(
    host: String,
    port: String,
    router: Router,
    grpc_addr: SocketAddr,
    grpc_router: grpc_router,
) {
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    let http_future = axum::serve(listener, router);
    let grpc_future = grpc_router.serve(grpc_addr);

    match tokio::join!(http_future, grpc_future) {
        (Ok(_), Ok(_)) => println!("Both servers stopped gracefully"),
        (Err(e), _) => eprintln!("gRPC server failed: {}", e),
        (_, Err(e)) => eprintln!("HTTP server failed: {}", e),
    }
}
