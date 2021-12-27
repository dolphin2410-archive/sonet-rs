use std::any::Any;

use sonet_rs::packet;
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

    println!("{}", MyPacket::jvm_name());

    // Setup Packet Data
    let mut packet_data: Vec<Box<dyn Any>> = Vec::new();
    packet_data.push(Box::new("Oh This is Great!".to_string()));

    // Call Packet Wrapper & Generate Packet Instance
    let packet_wrapper = &registry.keys["io.github.dolphin2410.packets.MyPacket"];
    let boxed_packet = packet_wrapper.create_instance(packet_data);
    let my_packet= boxed_packet.as_any().downcast_ref::<MyPacket>().unwrap().to_owned();


    // Call Packet Data
    println!("{}", &my_packet.s);

    // Call Packet's fields
    let fields: Vec<&'static str> = packet_wrapper.get_fields();
    println!("{:?}", fields);

    let ha= Codec::new(registry);
    let mut serialized = ha.serialize(Box::new(my_packet));
    let deserialized = ha.deserialize(&mut serialized);
    let my_deserialized_packet= deserialized.as_any().downcast_ref::<MyPacket>().unwrap().to_owned();
    println!("FINAL: {}", &my_deserialized_packet.s);


    // let mut server = SonetServer::new(registry, 9090).await?;

    // server.start().await?;
    Ok(())
}