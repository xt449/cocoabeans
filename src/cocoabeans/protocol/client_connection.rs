use std::net::{SocketAddr, TcpStream};

use crate::cocoabeans::protocol::packet::packet_handler::PacketHandler;
use crate::cocoabeans::protocol::packet::serverbound::{ServerBoundPacket, ServerBoundPayload};
use crate::cocoabeans::protocol::stream_wrapper::MinecraftStream;

pub struct ClientConnectionHandler {
    socket_address: SocketAddr,
    stream: MinecraftStream,
    verification_token: [u8; 4],
    pub packet_handler: Option<PacketHandler>,
}

impl ClientConnectionHandler {
    pub fn start(connection: (TcpStream, SocketAddr)) -> ClientConnectionHandler {
        let mut cc = ClientConnectionHandler {
            socket_address: connection.1,
            stream: MinecraftStream::wrap(connection.0),
            verification_token: rand::random::<[u8; 4]>(),
            packet_handler: None,
        };
        cc.packet_handler = Some(PacketHandler::new(&cc));
        return cc;
    }

    fn decode<P: ServerBoundPayload>(
        &self,
        stream: &mut MinecraftStream,
    ) -> Option<ServerBoundPacket> {
        let length = stream.read_varint();
        let id = stream.read_varint();
        match &self.packet_handler {
            None => None,
            Some(handler) => {
                match handler
                    .protocol_version
                    .get_builder_from_id(&handler.connection_state, id as u8)
                {
                    Some(builder) => Some(builder(stream, length)),
                    None => None,
                }
            }
        }
    }
}
