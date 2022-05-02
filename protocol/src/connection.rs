use std::net::{SocketAddr, TcpStream};

use crate::packet::packet_handler::PacketHandler;

pub struct Connection {
    address: SocketAddr,
    packet_handler: PacketHandler,
}

// Constructor
impl Connection {
    pub fn new(address: SocketAddr, stream: TcpStream) -> Connection {
        return Connection {
            address: address,
            packet_handler: PacketHandler::new(stream),
        };
    }
}

// TCP Reader
impl Connection {
    pub fn next(&mut self) {
        let packet = self.packet_handler.read_packet();
        match packet {
            None => { /* do nothing */ }
            Some(p) => {
                p.handle(&mut self.packet_handler);
            }
        }
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
