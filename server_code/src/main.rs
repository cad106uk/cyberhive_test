use async_trait::async_trait;
use serde_json::Value;
use std::process::exit;
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
    // Add custom trait to the tokio::fs::File to append json to the open file
    // Takes a raw buffer for data, makes sure it is valid JSON, then appends
    async fn append_json_row(&mut self, input: &Vec<u8>) -> Result<(), String> {
        let json_data: Value = match serde_json::from_slice(&input) {
            Ok(json_data) => json_data,
            Err(e) => {
                let error = format!("Invalid json data; err = {:?}", e);
                println!("{}", error);
                return Err(String::from(error));
            }
        };

        // One record per line
        let mut j_string: String = json_data.to_string();
        j_string.push('\n');
        match self.write_all(&j_string.into_bytes()).await {
            Ok(_) => (),
            Err(e) => {
                let error = format!("Failed to write to file; err = {:?}", e);
                println!("{}", error);
                return Err(String::from(error));
            }
        };
        match self.sync_all().await {
            Ok(_) => (),
            Err(e) => {
                let error = format!("Failed to write to file; err = {:?}", e);
                println!("{}", error);
                return Err(String::from(format!(
                    "Failed to write to file; err = {:?}",
                    e
                )));
            }
        };
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup:
    // bind to local port (we are using ssh iptunnelling to communicate over the internet)
    // Open log file ready to append new rows
    // Open channels to syncronise the writing to the log file.
    let mut listener = TcpListener::bind("127.0.0.1:6969").await?;
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("log_records.json")
        .await?;
    let (producer, mut consumer) = mpsc::channel(1024);

    // start a separate co-routine to hanled appending to the logfile
    tokio::spawn(async move {
        println!("Starting consumer");
        while let Some(res) = consumer.recv().await {
            println!("received text");
            match file.append_json_row(&res).await {
                Ok(_) => (),
                Err(e) => {
                    println!("failed to write JSON to file; err = {:?}", e);
                    eprintln!("failed to write JSON to file; err = {:?}", e);
                    exit(1)
                }
            };
        }
    });

    loop {
        // For each connection start a new producer channel
        let (mut socket, _) = listener.accept().await?;
        let mut producer = producer.clone();

        // Each socket is handled in their own co-routine
        tokio::spawn(async move {
            println!("Starting to read socket");
            let mut input_vec: Vec<u8> = Vec::new();
            let mut buf = [0; 1024];

            // In a loop, read data from the socket in 1k chunks.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => break,
                    Ok(n) => n,
                    Err(e) => {
                        println!("failed to read from socket; err = {:?}", e);
                        eprintln!("failed to read from socket; err = {:?}", e);
                        exit(1)
                    }
                };

                // extends from slice copies the data
                input_vec.extend_from_slice(&buf[..n]);
            }

            // Send the read data to the channel to be processed later
            // The producer falling out of scope closes this channel letting the consumer
            // know when this message has ended
            match producer.send(input_vec).await {
                Ok(_) => (),
                Err(e) => {
                    println!("failed write received message to channel; err = {:?}", e);
                    eprintln!("failed write received message to channel; err = {:?}", e);
                    exit(1)
                }
            }
            println!("Put message on channel");
        });
    }
}
