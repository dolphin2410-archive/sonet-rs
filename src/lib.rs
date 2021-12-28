pub mod buffer;
pub mod serializer;
pub mod packet;
pub mod util;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use crate::buffer::read::SonetReadBuf;
use crate::packet::{Packet, PacketRegistry};
use crate::serializer::Codec;

packet! {
    @jvm("io.github.dolphin2410.MyPacket")
    MyPacket {
        s: String
    }
}

/// The SonetServer struct
pub struct SonetServer {
    pub socket_address: SocketAddr,
}

/// The Default Implementation
impl SonetServer {
    /// New SonetServer Future. Requires Asynchronous runtime
    pub async fn new(port: i32) -> Result<SonetServer, std::io::Error> {
        let socket_address = format!("127.0.0.1:{}", port).parse::<SocketAddr>().expect("Failed to bind port to address");
        Ok(SonetServer {
            socket_address,
        })
    }

    pub async fn handle(codec: Arc<Mutex<Codec>>, socket: TcpStream) {
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
                    eprintln!("Error: {}", e);
                    return;
                }
            };
        }

        let safe_codec = codec.lock().await;

        // Handle Read Data
        let mut buffer = SonetReadBuf::new(full_body);
        let boxed = safe_codec.deserialize(&mut buffer);
        let packet = cast_packet!(boxed as MyPacket);

        println!("1: {}", &packet.s);

        // Flush To Write
    }

    /// Starts the server. This requires the asynchronous runtime
    pub async fn start(&self, registry: PacketRegistry) -> Result<(), std::io::Error> {
        let mut registry = registry;
        register_packet!(registry, MyPacket);
        let codec = Arc::new(Mutex::new(Codec::new(registry)));
        let mut v: Vec<Box<dyn Packet>> = vec![];

        let socket = TcpListener::bind(self.socket_address).await?;

        loop {
            let (mut socket, _) = socket.accept().await?;

            // Spawn new async
            tokio::spawn(Self::handle(codec.clone(), socket));
            let mut serialized = vec![];
            for item in &v {
                let buf = codec.clone().lock().await.serialize(item);
                serialized.push(buf);
            }
        }
    }

    pub fn stop(&mut self) {}
}