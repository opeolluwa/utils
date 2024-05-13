use std::net::{Ipv4Addr, SocketAddrV4};

use tonic::transport::Server;

use utils_auth::utils_auth_server::UtilsAuthServer;

use utils_storage::utils_data_back_up_server::UtilsDataBackUpServer;


mod auth;
mod storage;
mod  utils_auth;
mod utils_storage;
const SERVER_PORT: u16 = 10785;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), SERVER_PORT);

    let auth_service = auth::AuthService::default();
    let storage_service = storage::StorageService::default();

    println!("Utils server listening on grpc://{}", server_address);

    Server::builder()
        .add_service(UtilsAuthServer::new(auth_service))
        .add_service(UtilsDataBackUpServer::new(storage_service))
        .serve(server_address.into())
        .await?;

    Ok(())
}
