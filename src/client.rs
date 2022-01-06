use std::{sync::Arc, net::SocketAddr};

use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}, sync::Mutex};

use crate::{serializer::Codec, buffer::read::SonetReadBuf, packet::{Packet, PacketRegistry}};

pub struct Client {
    pub connection: Connection,
    pub packet_handlers: Arc<Mutex<Vec<Box<dyn FnMut(&Box<dyn Packet + Send + Sync>, &mut Connection) + Send>>>>
}

pub struct Connection {
    pub socket: TcpStream,
    pub codec: Arc<Mutex<Codec>>
}

impl Client {
    pub async fn new(port: i32, registry: PacketRegistry) -> Client {
        Client {
            connection: Connection {
                socket: TcpStream::connect(format!("127.0.0.1:{}", port).parse::<SocketAddr>().unwrap()).await.unwrap(),
                codec: Arc::new(Mutex::new(Codec::new(registry)))
            },
            packet_handlers: Arc::new(Mutex::new(vec![]))
        }
    }

    pub async fn handle(connection: &mut Connection, packet_handlers: Arc<Mutex<Vec<Box<dyn FnMut(&Box<dyn Packet + Send + Sync>, &mut Connection) + Send>>>>) -> Result<(), std::io::Error> {
        loop {
            let packet = connection.retrieve().await?;

            for handler in packet_handlers.lock().await.iter_mut() {
                handler(&packet, connection);
            }
        }
    }

    pub async fn initialize(&'static mut self) -> Result<(), std::io::Error> {
        tokio::spawn(Self::handle(&mut self.connection, self.packet_handlers.clone()));

        Ok(())
    }

    pub fn add_handler<T>(&mut self, closure: T) where T: FnMut(&Box<dyn Packet + Send + Sync>, &mut Connection) + Send + 'static {
        futures::executor::block_on(async {
            self.packet_handlers.lock().await.push(Box::new(closure));
        })
    }
}

impl Connection {
    pub async fn retrieve(&mut self) -> Result<Box<dyn Packet + Send + Sync>, std::io::Error> {

        self.socket.readable().await?;

        // Header Buffer
        let mut header_buffer = [0; 4];

        // Read Header
        self.socket.read(&mut header_buffer).await.unwrap();

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
            match self.socket.read_buf(&mut body_buffer).await {
                Ok(_) => {
                    // Add all read data to the full body buffer
                    for byte in body_buffer.clone() {
                        full_body.push(byte);
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

        let safe_codec = self.codec.lock().await;

        let mut buffer = SonetReadBuf::new(full_body);

        Ok(safe_codec.deserialize(&mut buffer))
    }

    pub fn push_packet(&mut self, packet: Box<dyn Packet>) {
        futures::executor::block_on(async {
            self.socket.write_all(self.codec.lock().await
                .serialize(&packet).data
                .as_mut()).await.unwrap();
        });
    }
}