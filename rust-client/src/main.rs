extern crate rust_common;

use tokio::net::TcpStream;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use rust_common::test::*;
use rust_common::packets::Packet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    println!("Hello, world!");

    let my_packet = MyPacket {  };

    my_packet.send();

    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:9090".parse::<SocketAddr>()?).await?;

    let mut buf: [u8; 3] = [0; 3];

    for i in 1..4 {
        buf[i - 1] = i as u8;
    }

    stream.write_all(&buf).await?;

    Ok(())
}
