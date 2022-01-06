use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use crate::client::{Client, Connection};
use crate::packet::PacketRegistry;
use crate::serializer::Codec;

/// The SonetServer struct
pub struct SonetServer {
    pub socket_address: SocketAddr,
    pub client_handlers: Vec<Box<dyn Fn(&mut Client)>>
}

/// The Default Implementation
impl SonetServer {
    /// New SonetServer Future. Requires Asynchronous runtime
    pub async fn new(port: i32) -> Result<SonetServer, std::io::Error> {
        let socket_address = format!("127.0.0.1:{}", port).parse::<SocketAddr>().expect("Failed to bind port to address");
        Ok(SonetServer {
            socket_address,
            client_handlers: vec![]
        })
    }

    /// Starts the server. This requires the asynchronous runtime
    pub async fn start(&self, registry: PacketRegistry) -> Result<(), std::io::Error> {
        let codec = Arc::new(Mutex::new(Codec::new(registry)));

        let socket = TcpListener::bind(self.socket_address).await?;

        loop {
            let (socket, _) = socket.accept().await?;

             let mut client = Client {
                connection: Connection {
                    socket,
                    codec: codec.clone(),
                },
                packet_handlers: Arc::new(Mutex::new(vec![]))
            };

            for handler in self.client_handlers.iter() {
                handler(&mut client);
            }

            Box::leak(Box::new(client)).initialize().await?; // STOP BLOCKING! 
        }
    }

    pub fn add_client_handler<T>(&mut self, closure: T) where T: Fn(&mut Client) + 'static {
        self.client_handlers.push(Box::new(closure));
    }

    pub fn stop(&mut self) {}
}