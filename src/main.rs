#[tokio::main]
async fn main() -> anyhow::Result<()> {
    utils_cli::utils::Utils::run().await
}
