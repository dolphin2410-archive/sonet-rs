use sonet_rs::SonetServer;
use sonet_rs::packet::PacketRegistry;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let registry = PacketRegistry::new();

    let server = SonetServer::new(9090).await?;
    server.start(registry).await?;

    Ok(())
}