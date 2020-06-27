use serde_json::Value;
use tokio::io::File;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::sync::mpsc;

trait RecordJson {
    fn append_json_row(&mut self, input: Vec<u8>) -> Result<(), Box<dyn std::error::Error>>;
}

impl RecordJson for File {
    async fn append_json_row(&mut input: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let json_data: Value = match serde_json::from_slice(&input) {
            Ok(json_data) => json_data,
            Err(e) => {
                eprintln!("Invalid json data; err = {:?}", e);
                return Err(Box::new(e));
            }
        };

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:6969").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut input_vec: Vec<u8> = Vec::new();
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => break,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                input_vec.extend_from_slice(&buf[..n]);
            }

            let res = record_input(input_vec).await?;
        });
    }
}
