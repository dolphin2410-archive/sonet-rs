use std::future::Future;
use std::net::SocketAddr;

use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use crate::buffer::Buffer;

pub mod buffer;
pub mod serializer;
pub mod packet;

pub struct SonetServer {
    pub socket: TcpListener
}

impl SonetServer {
    pub fn new(port: i32) -> impl Future<Output = Result<Self, std::io::Error>> + 'static {
        let socket_address = format!("127.0.0.1:{}", port).parse::<SocketAddr>().expect("Failed to bind port to address");

        async move {
            let socket = TcpListener::bind(socket_address).await?;

            Ok(SonetServer {
                socket
            })
        }
    }

    pub fn start(&mut self) -> impl Future<Output = Result<(), std::io::Error>> + '_ {
        async move {
            loop {
                let (mut socket, _) = self.socket.accept().await?;
                tokio::spawn(async move {
                    // Header Buffer
                    let mut header_buffer = [0; 4];

                    // Read Header
                    socket.read(&mut header_buffer).await.unwrap();

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
                        match socket.read_buf(&mut body_buffer).await {
                            Ok(_) => {
                                // Add all read data to the full body buffer
                                for i in body_buffer.clone() {
                                    full_body.push(i);
                                    read += 1;
                                }

                                // Clear Temp Buffer
                                body_buffer.clear();

                                // End if fully read
                                if read == body_size {
                                    break;
                                }
                            },
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                return;
                            }
                        };
                    }

                    // Handle Read Data
                    let mut buffer = Buffer::new(full_body);

                    println!("1: {}", buffer.read_int());
                    println!("2: {}", buffer.read_int());
                });
            }
        }
    }

    pub fn stop(&mut self) {
    }
}