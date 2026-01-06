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

    let (http_app, grpc_router) = AuthApp::new(config.clone()).await;
    let router = init_router(Arc::new(http_app));

    serve(config.host, config.port, router, config.grpc_addr, grpc_router).await;
}
