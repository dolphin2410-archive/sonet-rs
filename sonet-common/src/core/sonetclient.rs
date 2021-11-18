#![allow(dead_code)]

use std::future::Future;
use crate::api::components::client::Client;
use std::net::SocketAddr;
use std::pin::Pin;
use tokio::net::TcpStream;
use crate::api::util::optional::*;
use crate::api::packets::Packet;
use std::io::Result;
use tokio::io::AsyncWriteExt;
use crate::api::util::packet_serializer::PacketSerializer;
use crate::sonet_data;

pub struct SonetClient {
    stream: Optional<TcpStream>
}

impl Client for SonetClient {

    fn new() -> SonetClient {
        SonetClient { 
            stream: Optional::of(OptionalData::NULL)
        }
    }

    fn connect(&mut self, address: &'static SocketAddr) -> Pin<Box<dyn Future<Output = Result<()>> + '_>> {
        let future = async move {
            self.stream.set(OptionalData::DATA(TcpStream::connect(address).await?));

            Ok(())
        };

        Box::pin(future)
    }

    fn send_packet(&mut self, packet: Box<dyn Packet>) {
        self.stream.data.get_mutable().unwrap().write_all(PacketSerializer::serialize(packet));
    }

    fn abort(&mut self) {

    }
}