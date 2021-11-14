use crate::api::packets::Packet;
use std::net::SocketAddr;
use async_trait::async_trait;

#[async_trait]
pub trait Client {
    fn new() -> Self;

    async fn connect(&mut self, address: &SocketAddr) -> Result<(), Box<dyn std::error::Error>>;

    fn send_packet(&self, packet: Box<dyn Packet>);

    fn abort(&self);
}