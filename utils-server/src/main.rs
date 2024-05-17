use self::multiplex::MultiplexService;
use crate::grpc::{auth as auth_grpc_service, storage as storage_grpc_service};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use sea_orm::{ConnectOptions, Database};
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tonic::transport::Server;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use utils_auth::utils_auth_server::UtilsAuthServer;
use utils_storage::utils_data_back_up_server::UtilsDataBackUpServer;


mod grpc;
mod http;
mod multiplex;
mod pkg;
mod utils_auth;
mod utils_storage;
mod controllers;

const SERVER_PORT: u16 = 10785;
const DB_URL: &str = "postgres://drizzles:raindrops@localhost/utils-server-db";

//TODO: use shuttle runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the databse connection options/configuration
    let mut opt = ConnectOptions::new(DB_URL);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    // the database instance
    let db = Database::connect(opt).await?;

    let server_address = SocketAddr::from(([0, 0, 0, 0], SERVER_PORT));

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let auth_service = auth_grpc_service::AuthService::default();
    let storage_service = storage_grpc_service::StorageService::default();

    let grpc = Server::builder()
        .add_service(UtilsAuthServer::new(auth_service))
        .add_service(UtilsDataBackUpServer::new(storage_service))
        .into_service();

    let cors = CorsLayer::new()
        // .allow_credentials(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let state = pkg::AppState {
        db_client: db.clone(),
    };

    let http = crate::http::router::endpoints(state).layer(cors);

    tracing::debug!("Utils server listening on http://{0}", server_address);

    // run the migration programmatically during app startup
    // this would create the necessary tables
    let connection = db;
    Migrator::up(&connection, None).await?;

    // check for pending migrations
    let env = env::var("ENV").unwrap_or("production".to_string());
    if env == "development" {
        let migrations = Migrator::get_pending_migrations(&connection).await?;
        println!("{} pending migrations", migrations.len());
        println!("databse live at  {}", DB_URL);
    }

    // combine them into one service
    let service = MultiplexService::new(http, grpc);

    axum::Server::bind(&server_address)
        .serve(tower::make::Shared::new(service))
        .await?;

    Ok(())
}
