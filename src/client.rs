use std::sync::Arc;

use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}, sync::Mutex};

use crate::{serializer::Codec, buffer::read::SonetReadBuf, packet::Packet};

pub struct Client {
    pub connection: Connection,
    pub packet_handlers: Vec<Box<dyn FnMut(&Box<dyn Packet>, &mut Connection)>>
}

pub struct Connection {
    pub socket: TcpStream,
    pub codec: Arc<Mutex<Codec>>
}


impl Client {
    pub async fn initialize(&mut self) -> Result<(), std::io::Error> {
        loop {
            let packet = self.connection.retrieve().await?;

            for handler in self.packet_handlers.iter_mut() {
                handler(&packet, &mut self.connection);
            }
        }
    }

    pub fn add_handler<T>(&mut self, closure: T) where T: FnMut(&Box<dyn Packet>, &mut Connection) + 'static {
        self.packet_handlers.push(Box::new(closure));
    }
}

impl Connection {
    pub async fn retrieve(&mut self) -> Result<Box<dyn Packet>, std::io::Error> {

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