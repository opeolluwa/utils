use tonic::transport::Server;

use utils_auth::utils_auth_server::UtilsAuthServer;

use utils_storage::utils_data_back_up_server::UtilsDataBackUpServer;

use self::multiplex::MultiplexService;
use crate::grpc::{auth as auth_grpc_service, storage as storage_grpc_service};

use std::net::SocketAddr;

mod grpc;
mod http;
mod multiplex;
mod utils_auth;
mod utils_storage;

const SERVER_PORT: u16 = 10785;

//TODO: use shuttle runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = SocketAddr::from(([0, 0, 0, 0], SERVER_PORT));

    let auth_service = auth_grpc_service::AuthService::default();
    let storage_service = storage_grpc_service::StorageService::default();

    println!(
        "Utils server listening on:\n\ngrpc://{0}\nhttp://{0}",
        server_address
    );

    let grpc = Server::builder()
        .add_service(UtilsAuthServer::new(auth_service))
        .add_service(UtilsDataBackUpServer::new(storage_service))
        .into_service();

    let http = crate::http::router::endpoints();

    tracing::debug!("listening on {}", server_address);
    // combine them into one service
    let service = MultiplexService::new(http, grpc);

    axum::Server::bind(&server_address)
        .serve(tower::make::Shared::new(service))
        .await?;

    Ok(())
}
