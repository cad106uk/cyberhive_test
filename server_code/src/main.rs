use async_trait::async_trait;
use serde_json::Value;
use tokio::fs::{File, OpenOptions};
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::sync::mpsc;

#[async_trait]
trait RecordJson {
    async fn append_json_row(&mut self, input: &Vec<u8>) -> Result<(), String>;
}

#[async_trait]
impl RecordJson for File {
    async fn append_json_row(&mut self, input: &Vec<u8>) -> Result<(), String> {
        let json_data: Value = match serde_json::from_slice(&input) {
            Ok(json_data) => json_data,
            Err(e) => {
                return Err(String::from(format!("Invalid json data; err = {:?}", e)));
            }
        };

        match self.write_all(&json_data.to_string().into_bytes()).await {
            Ok(_) => (),
            Err(e) => {
                return Err(String::from(format!(
                    "Failed to write to file; err = {:?}",
                    e
                )));
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:6969").await?;
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("log_records.json")
        .await?;
    let (tx, mut rx) = mpsc::channel(1024);

    tokio::spawn(async move {
        while let Some(res) = rx.recv().await {
            match file.append_json_row(&res).await {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("failed to write JSON to file; err = {:?}", e);
                    return;
                }
            };
        }
    });

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut tx = tx.clone();

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

            match tx.send(input_vec).await {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("failed write received message to channel; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
