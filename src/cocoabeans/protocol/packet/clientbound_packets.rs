use std::net::TcpStream;

pub trait ClientBoundPacket {
    fn write_to(&self, stream: &TcpStream);
}

pub mod handshaking {
    // no client-bound handshaking packets
}

pub mod status {
    use crate::cocoabeans::protocol::packet::clientbound_packets::ClientBoundPacket;
    use crate::cocoabeans::protocol::types;
    use std::net::TcpStream;

    pub struct ResponsePacket {
        pub protocol_version: types::VarInt,
        pub address: String,
        pub port: u16,
        pub next_state: types::VarInt,
    }

    impl ClientBoundPacket for ResponsePacket {
        fn write_to(&self, stream: &TcpStream) {
            todo!()
        }
    }
}
