use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};

use protocol::connection::Connection;

fn handle_connection(connection_raw: (TcpStream, SocketAddr)) {
    let mut connection = Connection::new(connection_raw.1, connection_raw.0);
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
            Ok(connection) => {
                std::thread::spawn(move || handle_connection(connection));
            }
            Err(e) => {
                println!("Error accepting incoming connection: {}", e);
            }
        }
    }
}
