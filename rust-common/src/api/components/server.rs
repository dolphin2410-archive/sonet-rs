use std::future::Future;
use crate::api::util::packet_handler::PacketHandler;
use std::pin::Pin;
use std::io::Result;

pub trait Server<'a> {
    
    fn new(port: &i16) -> Self;

    fn shutdown(&self);

    fn start(&mut self) -> Pin<Box<dyn Future<Output = Result<()>> + '_>>;

    fn add_packet_handler(&mut self, handler: &'a Box<dyn PacketHandler>);

    fn remove_packet_handler(&mut self, handler: &'a Box<dyn PacketHandler>);
}