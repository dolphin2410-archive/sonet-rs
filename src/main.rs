use sonet_rs::{SonetServer, is_type, packet, cast_packet, register_packet};
use sonet_rs::packet::PacketRegistry;

packet! {
    @jvm("io.github.dolphin2410.MyPacket")
    MyPacket {
        s: String
    }
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut registry = PacketRegistry::new();
    register_packet!(registry, MyPacket);

    let mut server = SonetServer::new(9090).await?;
    server.add_handler(|packet, connection| {
        println!("Packet!");
        if is_type!(packet, MyPacket) {
            let my_packet = cast_packet!(packet as MyPacket);
            println!("MyPacket!");
            println!("s: {}", &my_packet.s);
            futures::executor::block_on(connection.send_packet(Box::new(my_packet)));
        }
    });

    server.start(registry).await?;

    Ok(())
}