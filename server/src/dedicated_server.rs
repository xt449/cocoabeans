use std::io::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};

use protocol::connection::Connection;

fn handle_connection(address: SocketAddr, stream: TcpStream) {
    println!("Starting client connection! ({address})");
    let mut connection = Connection::new(address, stream);
    loop {
        if let Err(_) = connection.packet_handler.next_packet() {
            // End of Stream
            break;
        }
    }
    println!("Closing client connection! ({address})");
}

pub fn start() -> Result<(), Error> {
    // Setup the TCP server socket.
    let address: SocketAddr = "0.0.0.0:25565".parse().unwrap();
    let server = TcpListener::bind(address)?;

    println!("Server started!\nNow listening on '{address}'");

    loop {
        match server.accept() {
            Ok((stream, address)) => {
                std::thread::spawn(move || handle_connection(address, stream));
            }
            Err(e) => {
                println!("Error accepting incoming connection: {e}");
            }
        }
    }
}
