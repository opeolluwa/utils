use std::net::{Ipv4Addr, SocketAddrV4};

use tonic::transport::Server;

use crate::proto::utils_auth::utils_auth_server::UtilsAuthServer;

pub mod proto;
mod server;

const SERVER_PORT: u16 = 10785;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_address = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), SERVER_PORT);

    let server = server::AuthCliServer::default();

    println!("Utils server listening on grpc://{}", server_address);

    Server::builder()
        .add_service(UtilsAuthServer::new(server))
        .serve(server_address.into())
        .await?;

    Ok(())
}
