use sonet_rs::packet;

packet! {
    @jvm("io.github.dolphin2410.MyPacket")
    MyPacket {
        s: String
    }
}
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    test().await.unwrap();

    Ok(())
}

async fn test() -> Result<(), std::io::Error> {
    use sonet_rs::packet::PacketRegistry;
    use sonet_rs::{cast_packet, is_type, register_packet, server::SonetServer};

    let mut registry = PacketRegistry::new();
    register_packet!(registry, MyPacket);

    let mut server = SonetServer::new(9090).await?;
    server.add_client_handler(|client| {
        client.add_handler(|packet, connection| {
            println!("Packet!");
            if is_type!(packet, MyPacket) {
                let mut my_packet = cast_packet!(packet as MyPacket);

                println!("From Kotlin: {}", &my_packet.s);

                // Modify Packet
                my_packet.s = "Hello, Rust!".to_string();
                connection.push_packet(Box::new(my_packet));
            }
        });
    });

    server.start(registry).await?;

    Ok(())
}
