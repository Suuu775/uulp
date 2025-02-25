use std::{error::Error, io::Write, net::TcpListener};

use chrono::Local;

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:13000")?;
    loop {
        match listener.accept() {
            Ok((mut socket, _addr))=>{
                let _ = socket.write("The time here is...\n".as_bytes());
                let _ = socket.write(Local::now().to_rfc3339().as_bytes());
                
            },
            Err(e) =>{
                eprintln!("couldn't get client: {e:?}\n")
            }
        }
    }
}
