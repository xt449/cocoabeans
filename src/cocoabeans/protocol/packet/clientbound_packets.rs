use std::net::TcpStream;

use crate::cocoabeans::protocol::versions::ProtocolVersion;

pub trait ClientBoundPacket {
    fn write_to(&self, stream: &mut TcpStream, protocol_version: &dyn ProtocolVersion);
}

pub mod handshaking {
    // no client-bound handshaking packets
}

pub mod status {
    use std::io::Write;
    use std::net::TcpStream;

    use crate::cocoabeans::protocol::packet::clientbound_packets::ClientBoundPacket;
    use crate::cocoabeans::protocol::types;
    use crate::cocoabeans::protocol::versions::ProtocolVersion;

    pub struct ResponsePacket {
        pub protocol_version: types::VarInt,
        pub address: String,
        pub port: u16,
        pub next_state: types::VarInt,
    }

    impl ClientBoundPacket for ResponsePacket {
        #[allow(unused_must_use)]
        fn write_to(&self, stream: &mut TcpStream, protocol_version: &dyn ProtocolVersion) {
            stream.write(&[protocol_version.get_status_response_id()][..]);
            todo!()
        }
    }
}

pub mod login {
    // TODO
}

pub mod play {
    // TODO
}
