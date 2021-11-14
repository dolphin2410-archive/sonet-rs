#![allow(dead_code)]

use std::future::Future;
use crate::api::components::client::Client;
use std::net::SocketAddr;
use std::pin::Pin;
use tokio::net::TcpStream;
use crate::api::util::optional::*;
use crate::api::packets::Packet;
use std::io::Result;
use crate::sonet_data;

pub struct SonetClient {
    stream: Optional<TcpStream>
}

fn a() {
    sonet_data! {
        struct A {

        }
    }
}

impl Client for SonetClient {

    fn new() -> SonetClient {
        SonetClient { 
            stream: Optional::of(OptionalData::NULL)
        }
    }

    async fn connect(&mut self, address: &SocketAddr) -> Pin<Box<dyn Future<Output = Result<()>> + '_>> {
        self.stream.set(OptionalData::DATA(TcpStream::connect(address).await?));

        Ok(())
    }

    fn send_packet(&self, packet: Box<dyn Packet>) {
        
    }

    fn abort(&self) {

    }
}