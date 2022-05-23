use std::net::{SocketAddr, TcpStream};

use crate::packet::packet_handler::PacketHandler;

pub struct Connection<'c> {
    address: SocketAddr,
    pub packet_handler: PacketHandler<'c>,
}

// Constructor
impl<'c> Connection<'c> {
    pub fn new(address: SocketAddr, stream: TcpStream) -> Connection<'c> {
        return Connection {
            address: address,
            packet_handler: PacketHandler::new(stream),
        };
    }
}

// Getters
impl<'c> Connection<'c> {
    pub fn get_address(&self) -> &SocketAddr {
        return &self.address;
    }

    pub fn get_packet_handler(&self) -> &PacketHandler {
        return &self.packet_handler;
    }
}
