use crate::cocoabeans::protocol;
use std::io;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub fn start() -> io::Result<()> {
    println!("Hello, world!");

    // Setup the TCP server socket.
    let address: SocketAddr = "0.0.0.0:25565".parse().unwrap();
    let server = TcpListener::bind(address)?;

    let protocol_version: &dyn protocol::versions::ProtocolVersion = &protocol::versions::V758 {};

    handle_connection(server.accept());

    return Ok(());
}

async fn handle_connection(result: io::Result<(TcpStream, SocketAddr)>) {
    // idk
}
