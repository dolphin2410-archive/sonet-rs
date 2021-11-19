use std::future::Future;
use std::io::Result;
use crate::api::packets::Packet;
use std::net::SocketAddr;
use std::pin::Pin;

pub trait Client {
    fn new() -> Self;

    fn connect(&mut self, address: SocketAddr) -> Pin<Box<dyn Future<Output = Result<()>> + '_>>;

    fn send_packet(&mut self, packet: Box<dyn Packet>);

    fn abort(&mut self);
}