mod app;
mod config;
mod delivery_grpc;
mod delivery_http;
mod errors;
mod handlers;
mod infra;
mod model;
mod repo;
mod usecase;

use crate::app::AuthApp;
use crate::delivery_http::users_delivery::{IUsersRepo, UsersDelivery};
use crate::handlers::{create_user, delete_user, get_user, update_user};
use crate::infra::postgres::PGPool;
use crate::infra::redis::RedisPool;
use crate::repo::sessions::SessionsRepo;
use crate::repo::users_repo::UsersRepo;
use crate::usecase::users_usecase::{IUsersCreatorRepo, UserUsecase};
use axum::routing::{delete, get, put};
use axum::{Router, routing::post};
use dotenvy::dotenv;
use std::sync::Arc;
use std::{env, process};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let conn_string = env::var("DATABASE_URL").expect("Failed to get db url");

    let pool = match PGPool::new(conn_string).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("error getting pg pool: {e}");
            process::exit(1)
        }
    };

    let conn_string = env::var("REDIS_URL").expect("Failed to get redis url");

    let redis_pool = match RedisPool::new(conn_string) {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("error getting redis pool {e}");
            process::exit(1);
        }
    };

    let session_repo = Arc::new(SessionsRepo::new(redis_pool));

    let repo = Arc::new(UsersRepo::new(pool));

    let repo_for_usecase: Arc<dyn IUsersCreatorRepo> = repo.clone();
    let repo_for_delivery: Arc<dyn IUsersRepo> = repo.clone();

    let usecase = UserUsecase::new(repo_for_usecase);

    let delivery = UsersDelivery::new(repo_for_delivery, Arc::new(usecase), session_repo);

    let app = Arc::new(AuthApp::new(Arc::new(delivery)).await);

    let app = Router::new()
        .route("/api/v1/register", post(create_user))
        .route("/api/v1/users/{id}", get(get_user))
        .route("/api/v1/users/{id}", put(update_user))
        .route("/api/v1/users/{id}", delete(delete_user))
        .with_state(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
