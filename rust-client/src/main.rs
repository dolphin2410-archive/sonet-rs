extern crate rust_common;

use tokio::net::TcpStream;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use rust_common::tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:9090".parse::<SocketAddr>()?).await?;

    let buf: [u8; 3] = [1, 2, 3];

    stream.write_all(&buf).await?;

    Ok(())
}
