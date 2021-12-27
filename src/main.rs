use std::any::Any;

use sonet_rs::{cast_packet, packet, packet_data, register_packet};
use sonet_rs::packet::PacketRegistry;
use sonet_rs::serializer::Codec;

packet! {
    @jvm("io.github.dolphin2410.packets.MyPacket")
    MyPacket {
        s: String
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let mut registry = PacketRegistry::new();
    register_packet!(registry, MyPacket);

    let packet_wrapper = registry.get("io.github.dolphin2410.packets.MyPacket");
    let my_packet = packet_wrapper.create_instance::<MyPacket>(packet_data!("The Great String".to_string()));

    println!("Jvm Name: {}", MyPacket::jvm_name());
    println!("String: {}", &my_packet.s);

    // let mut server = SonetServer::new(registry, 9090).await?;

    // server.start().await?;
    Ok(())
}