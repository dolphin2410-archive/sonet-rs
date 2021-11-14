use std::future::Future;
use crate::api::components::server::Server;
use crate::api::util::packet_handler::PacketHandler;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::api::util::optional::*;
use std::ptr;
use std::pin::Pin;
use std::io::Result;

pub struct SonetServer<'a> {
    pub address: SocketAddr,
    pub handlers: Vec<&'a Box<dyn PacketHandler>>,
    server: Optional<TcpListener>
}

impl <'a> Server<'a> for SonetServer<'a> {

    fn new(port: &i16) -> Self {
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

    fn add_packet_handler(&mut self, handler: &'a Box<dyn PacketHandler>) {
        self.handlers.push(&handler);
    }

    fn remove_packet_handler(&mut self, handler: &'a Box<dyn PacketHandler>) {
        let index = (&self.handlers).iter().position(|x| ptr::eq(&*x, &*&handler)).unwrap();
        self.handlers.remove(index);
    }
}

