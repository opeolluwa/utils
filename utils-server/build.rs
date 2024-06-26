use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth_proto_file = "proto/auth.proto";
    let backup_proto_file = "proto/storage.proto";

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional") // for older systems
        // .build_client(true) // don't compile the client code
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("auth.bin"))
        .out_dir("src")
        .compile(&[auth_proto_file, backup_proto_file], &["proto"])?;

    Ok(())
}
