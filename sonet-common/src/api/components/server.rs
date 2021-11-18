use std::future::Future;
use crate::api::util::packet_handler::PacketHandler;
use std::pin::Pin;
use std::io::Result;
use crate::api::packets::Packet;
use crate::api::util::java_types::Int;

pub trait Server {
    
    fn new(port: Int) -> Self;

    fn shutdown(&self);

    fn start(&mut self) -> Pin<Box<dyn Future<Output = Result<()>> + '_>>;

    fn add_packet_handler<T>(&mut self, handler: &'static T) where T: Fn(Box<dyn Packet>);

    fn remove_packet_handler<T>(&mut self, handler: &'static T) where T: Fn(Box<dyn Packet>);
}