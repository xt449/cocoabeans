use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};

use protocol::connection::Connection;

fn handle_connection(address: SocketAddr, stream: TcpStream) {
    let mut connection = Connection::new(address, stream);
    loop {
        connection.next();
    }
}

pub fn start() -> Result<(), Box<dyn Error>> {
    // Setup the TCP server socket.
    let address: SocketAddr = "0.0.0.0:25565".parse()?;
    let server = TcpListener::bind(address)?;

    println!("Server started!\nNow listening on '{}'", address);

    loop {
        match server.accept() {
            Ok((stream, address)) => {
                std::thread::spawn(move || handle_connection(address, stream));
            }
            Err(e) => {
                println!("Error accepting incoming connection: {}", e);
            }
        }
    }
}
