use std::any::Any;

use sonet_rs::{cast_packet, packet, packet_data};
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

    // Register Packet
    let mut registry = PacketRegistry::new();
    MyPacket::register(&mut registry);

    println!("Jvm Name: {}", MyPacket::jvm_name());

    // Call Packet Wrapper & Generate Packet Instance
    let packet_wrapper = &registry.keys["io.github.dolphin2410.packets.MyPacket"];
    let boxed_packet = packet_wrapper.create_instance(packet_data!("The Great String".to_string()));
    let my_packet= cast_packet!(boxed_packet -> MyPacket);

    let codec = Codec::new(registry);
    let mut serialized = codec.serialize(Box::new(my_packet));
    let deserialized = codec.deserialize(&mut serialized);
    let my_deserialized_packet= cast_packet!(deserialized -> MyPacket);
    println!("String: {}", &my_deserialized_packet.s);


    // let mut server = SonetServer::new(registry, 9090).await?;

    // server.start().await?;
    Ok(())
}