use std::any::Any;

use sonet_rs::packet;
use sonet_rs::packet::PacketRegistry;

packet! {
    MyPacket {
        s: String
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    // Register Packet
    let mut registry = PacketRegistry::new();
    MyPacket::register(&mut registry);

    // Setup Packet Data
    let mut packet_data: Vec<Box<dyn Any>> = Vec::new();
    packet_data.push(Box::new("Oh This is Great!".to_string()));

    // Call Packet Wrapper & Generate Packet Instance
    let packet_wrapper = &registry.keys["MyPacket"];
    let boxed_packet = packet_wrapper.create_instance(packet_data);
    let my_packet = boxed_packet.as_any().downcast_ref::<MyPacket>().unwrap();


    // Call Packet Data
    println!("{}", &my_packet.s);

    // Call Packet's fields
    let fields: Vec<&'static str> = packet_wrapper.get_fields();
    println!("{:?}", fields);

    // let mut server = SonetServer::new(9090).await?;

    // server.start().await?;
    Ok(())
}