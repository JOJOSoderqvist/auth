use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;

const POSTGRES_URL: &str = "DATABASE_URL";
const REDIS_URL: &str = "REDIS_URL";
const APP_HOST: &str = "HOST";
const APP_PORT: &str = "PORT";
const GRPC_ADDR: &str = "GRPC";

#[derive(Clone)]
pub struct AppConfig {
    pub postgres_conn_string: String,
    pub redis_conn_string: String,
    pub host: String,
    pub port: String,
    pub grpc_addr: SocketAddr,
}

impl AppConfig {
    pub fn new() -> Self {
        dotenv().ok();

        let postgres_conn_string = env::var(POSTGRES_URL).expect("Postgres url is not set");
        let redis_conn_string = env::var(REDIS_URL).expect("Redis url is not set");
        let host = env::var(APP_HOST).expect("Host is not set");
        let port = env::var(APP_PORT).expect("Port is not set");
        let grpc_addr = env::var(GRPC_ADDR).expect("Grpc addr is not set");

        let grpc_addr = grpc_addr.parse::<SocketAddr>()
            .expect("failed to parse grpc server addr");


        Self {
            postgres_conn_string,
            redis_conn_string,
            host,
            port,
            grpc_addr
        }
    }
}
