use std::future::Future;
use crate::api::components::server::Server;
use crate::api::util::packet_handler::PacketHandler;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::api::util::optional::*;
use std::ptr;
use std::pin::Pin;
use std::io::Result;
use crate::api::packets::Packet;
use crate::api::util::java_types::Int;

pub struct SonetServer  {
    pub address: SocketAddr,
    pub handlers: Vec<Box<dyn Fn(Box<dyn Packet>)>>,
    server: Optional<TcpListener>
}

impl Server for SonetServer {
    fn new(port: Int) -> Self {
        let address = format!("127.0.0.1:{port}", port = port).parse::<SocketAddr>().unwrap();
        SonetServer {
            address,
            handlers: Vec::new(),
            server: Optional::of(OptionalData::NULL)
        }
    }

    fn shutdown(&self) {
        let server: &TcpListener = self.server.data.get().unwrap();
        // TODO Finish
    }

    fn start(&mut self) -> Pin<Box<dyn Future<Output = Result<()>> + '_>> {
        let future = async move {
            let listener = TcpListener::bind(self.address).await.unwrap();
            self.server.set(OptionalData::DATA(listener));
            Ok(())
        };
        Box::pin(future)
    }

    fn add_packet_handler<T>(&mut self, handler: &'static T) where T: Fn(Box<dyn Packet>) {
        self.handlers.push(Box::new(handler));
    }

    fn remove_packet_handler<T>(&mut self, handler: &'static T) where T: Fn(Box<dyn Packet>) {
        for i in 0..self.handlers.len() {
            if ptr::eq(self.handlers.get(i).unwrap().as_ref(), handler) {
                self.handlers.remove(i);
                break
            }
        }
    }
}

