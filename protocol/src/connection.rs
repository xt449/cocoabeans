use std::net::{SocketAddr, TcpStream};

use crate::PacketHandler;

pub struct Connection {
    address: SocketAddr,
    pub packet_handler: PacketHandler,
}

// Constructor
impl Connection {
    pub fn new(address: SocketAddr, stream: TcpStream) -> Connection {
        return Connection { address: address, packet_handler: PacketHandler::new(stream) };
    }
}

// Getters
impl Connection {
    pub fn get_address(&self) -> &SocketAddr {
        return &self.address;
    }

    pub fn get_packet_handler(&self) -> &PacketHandler {
        return &self.packet_handler;
    }
}
