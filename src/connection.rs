use std::sync::Arc;

use tokio::{net::TcpStream, sync::Mutex, io::AsyncWriteExt};

use crate::{packet::Packet, serializer::Codec};

pub struct Connection {
    pub stream: Arc<Mutex<TcpStream>>,
    pub codec: Arc<Mutex<Codec>>
}

impl Connection {
    pub fn new(stream: Arc<Mutex<TcpStream>>, codec: Arc<Mutex<Codec>>) -> Connection {
        Connection {
            stream,
            codec
        }
    }

    pub async fn send_packet(self, packet: Box<dyn Packet>) {
        self.stream.clone().lock().await
            .write_all(self.codec.lock().await
                .serialize(&packet).data
                .as_mut()).await.unwrap();
    }
}