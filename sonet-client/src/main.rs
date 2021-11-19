extern crate rust_common;

use tokio::net::TcpStream;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use rust_common::api::components::client::Client;
use rust_common::api::components::server::Server;
use rust_common::api::packets::Packet;
use rust_common::api::sonet::Sonet;
use rust_common::tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    // let mut stream: TcpStream = TcpStream::connect("127.0.0.1:9090".parse::<SocketAddr>()?).await?;
    //
    // let buf: [u8; 3] = [1, 2, 3];
    //
    // stream.write_all(&buf).await?;

    let mut server = Sonet::create_server(9090);
    server.add_packet_handler(&|packet| {
        let pc = packet.as_ref();
    });

    struct MyPacket;

    impl Packet for MyPacket {

    }

    let mut client = Sonet::create_client();
    client.connect("127.0.0.1:9090".parse::<SocketAddr>().unwrap());
    client.send_packet(Box::new(MyPacket));

    Ok(())
}
