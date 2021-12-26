use std::any::Any;

use sonet_rs::packet;
use sonet_rs::packet::{Packet, Registry};

packet! {
    MyPacket {
        s: String
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut registry = Registry::new();

    MyPacket::register(&mut registry);
    let mut vec: Vec<Box<dyn Any>> = Vec::new();
    vec.push(Box::new("Oh This is Great!".to_string()));
    let packet = registry.map.get_mut("MyPacket").unwrap();
    let packet: Box<dyn Packet> = packet(vec);
    let packet = packet.as_any().downcast_ref::<MyPacket>().unwrap();

    println!("{}", &packet.s);

    // let mut server = SonetServer::new(9090).await?;

    // server.start().await?;
    Ok(())
}