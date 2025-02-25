use std::{env::args, error::Error, io::{Read, Write}, net::TcpStream};

use chrono::Local;

fn main()->Result<(),Box<dyn Error>>{
    let args:Vec<String> = args().skip(1).collect();
    if args.len()<2 {
        eprintln!("Usage: <program> <host> <port>");
        return Err("Invalid arguments".into());
    }

    let host = &args[0];
    let port = &args[1];

    let address = format!("{}:{}", host, port);

    // Step 1: Get a socket and connect to the server
    let mut stream = TcpStream::connect(&address)?;
    println!("Connected to {}", address);

    // Step 2: Read data from the server
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
        eprintln!("No data received from the server");
        return Err("No data received from the server".into());
    }

    // Step 3: Print the received message
    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Message from server: {}", message);

    // Step 4: Send the current time to the server
    let current_time = Local::now().to_rfc3339();
    stream.write_all(current_time.as_bytes())?;
    return Ok(());
}