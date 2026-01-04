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

use crate::app::{AuthApp, init_router, serve};
use crate::config::AppConfig;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = AppConfig::new();

    let app = AuthApp::new(config.clone()).await;
    let router = init_router(Arc::new(app));

    serve(config.host, config.port, router).await;
}
