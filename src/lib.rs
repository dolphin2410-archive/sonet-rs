pub mod buffer;
pub mod serializer;
pub mod packet;
pub mod util;

use std::net::SocketAddr;
use std::sync::Arc;
use packet::Packet;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use crate::buffer::read::SonetReadBuf;
use crate::packet::PacketRegistry;
use crate::serializer::Codec;

/// The SonetServer struct
pub struct SonetServer {
    pub socket_address: SocketAddr,
    pub handlers: Vec<Box<dyn Fn(&Box<dyn Packet>)>>
}

/// The Default Implementation
impl SonetServer {
    /// New SonetServer Future. Requires Asynchronous runtime
    pub async fn new(port: i32) -> Result<SonetServer, std::io::Error> {
        let socket_address = format!("127.0.0.1:{}", port).parse::<SocketAddr>().expect("Failed to bind port to address");
        Ok(SonetServer {
            socket_address,
            handlers: vec![]
        })
    }

    pub async fn write_packet(codec: Arc<Mutex<Codec>>, socket: &mut TcpStream, packet: Box<dyn Packet>) {
        socket.write_all(codec.lock().await.serialize(&packet).data.as_mut()).await.unwrap();
    }

    pub async fn handle(codec: Arc<Mutex<Codec>>, socket: TcpStream) -> Box<dyn Packet> {
        let mut mut_socket = socket;

        // Header Buffer
        let mut header_buffer = [0; 4];

        // Read Header
        mut_socket.read(&mut header_buffer).await.unwrap();

        // Body Size
        let body_size = i32::from_be_bytes(header_buffer);

        // The full body buffer
        let mut full_body = Vec::new();

        // Temporary Read Buffer
        let mut body_buffer = Vec::new();

        // Size Read
        let mut read = 0;

        loop {
            // Read Body
            match mut_socket.read_buf(&mut body_buffer).await {
                Ok(_) => {
                    // Add all read data to the full body buffer
                    for i in body_buffer.clone() {
                        full_body.push(i);
                        read += 1;
                    }

                    // Clear Temp Buffer
                    body_buffer.clear();

                    // End if fully read
                    if read >= body_size {
                        break;
                    }
                }
                Err(e) => {
                    panic!("Error: {}", e);
                }
            };
        }

        let safe_codec = codec.lock().await;

        // Handle Read Data
        let mut buffer = SonetReadBuf::new(full_body);
        safe_codec.deserialize(&mut buffer)

        // write_packet

        // Flush To Write
    }

    /// Starts the server. This requires the asynchronous runtime
    pub async fn start(&self, registry: PacketRegistry) -> Result<(), std::io::Error> {
        let codec = Arc::new(Mutex::new(Codec::new(registry)));
        // let mut v: Vec<Box<dyn Packet>> = vec![];

        let socket = TcpListener::bind(self.socket_address).await?;

        loop {
            let (socket, _) = socket.accept().await?;

            // Spawn new async
            let packet_obj = Self::handle(codec.clone(), socket).await;
            let boxed = Box::new(packet_obj);
            for closure in self.handlers.iter() {
                closure(&boxed)
            }

            // let mut serialized = vec![];
            // for item in &v {
            //     let buf = codec.clone().lock().await.serialize(item);
            //     serialized.push(buf);
            // }
        }
    }

    pub fn add_handler<T>(&mut self, closure: T) where T: Fn(&Box<dyn Packet>) + 'static {
        self.handlers.push(Box::new(closure));
    }

    pub fn stop(&mut self) {}
}