use rust_common::tokio::net::TcpListener;
use rust_common::tokio::io::AsyncReadExt;
use rust_common::tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Program");

    let listener = TcpListener::bind("127.0.0.1:9090").await?;

    println!("Server initialized");

    loop {
        println!("Waiting for data");

        let (mut socket, _) = listener.accept().await?;
        
        println!("Data Found!");

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {

                println!("Starting Read");

                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                println!("Finished Read");

                println!("Writing");

                for i in &buf[0..n] {
                    println!("Data: {}", i);
                }
            }
        });
    }
}
