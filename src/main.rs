mod delivery_http;
mod delivery_grpc;
mod repo;
mod usecase;
mod config;
mod model;
mod infra;
mod errors;

use axum::{
    routing::get,
    Router,
};
use crate::delivery_http::register;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/register", get(register));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
