use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};

use crate::cocoabeans::protocol::client_connection::ClientConnectionHandler;

fn handle_connection(connection: (TcpStream, SocketAddr)) {
    ClientConnectionHandler::start(connection);
}

pub fn start() -> Result<(), Box<dyn Error>> {
    // Setup the TCP server socket.
    let address: SocketAddr = "0.0.0.0:25565".parse()?;
    let server = TcpListener::bind(address)?;

    println!("Server started!\nNow listening on '{}'", address);

    loop {
        match server.accept() {
            Ok(connection) => {
                std::thread::spawn(move || handle_connection(connection));
            }
            Err(e) => {
                println!("Error accepting incoming connection: {}", e);
            }
        }
    }
}
