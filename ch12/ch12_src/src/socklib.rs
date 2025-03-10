use std::{error::Error, net::{TcpListener, TcpStream}};

#[allow(dead_code)]
pub fn make_server_socket(port: i32) -> Result<TcpListener, Box<dyn Error>> {
    let bind_addr = format!("127.0.0.1:{}", port);
    Ok(TcpListener::bind(bind_addr)?)
}
#[allow(dead_code)]
pub fn connect_to_server(host: &str, port: i32) -> Result<TcpStream, Box<dyn Error>> {
    let connect_addr = format!("{}:{}", host, port);
    Ok(TcpStream::connect(connect_addr)?)
}