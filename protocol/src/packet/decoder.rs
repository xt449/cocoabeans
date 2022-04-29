use std::net::TcpStream;
use super::super::stream_wrapper::MinecraftStream;
use super::packet_handler::IPacketHandler;
use super::serverbound::ServerBoundPayload;
use super::serverbound::ServerBoundPacket;

fn decode<P: ServerBoundPayload>(
    packet_handler: &Option<dyn IPacketHandler>,
    stream: &mut MinecraftStream,
) -> Option<ServerBoundPacket> {
    let length = stream.read_varint();
    let id = stream.read_varint();
    match packet_handler {
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

// fn decode<P: ServerBoundPayload>(
//     stream: &TcpStream,
//     client_connection: &ClientConnectionHandler,
// ) -> Option<ServerBoundPacket> {
//     let length = VarInt::read_from(stream);
//     let id = VarInt::read_from(stream);
//     match client_connection.packet_handler {
//         None => None,
//         Some(handler) => {
//             match client_connection
//                 .packet_handler
//                 .protocol_version
//                 .get_builder_from_id(handler.connection_state, id as u8)
//             {
//                 Ok(builder) => {
//                     return Some(builder(stream));
//                 }
//                 Err(_) => None,
//             }
//         }
//     }
// }

// impl Decoder for ClientConnectionHandler {
//     fn decode<P: ServerBoundPayload>(&self, stream: &TcpStream) -> Option<ServerBoundPacket<P>> {
//         let length = VarInt::read_from(stream);
//         let id = VarInt::read_from(stream);
//         match self.packet_handler {
//             None => None,
//             Some(handler) => {
//                 match self.packet_handler
//                     .protocol_version
//                     .get_builder_from_id(handler.connection_state, id as u8)
//                 {
//                     Ok(builder) => {
//                         return Some(ServerBoundPacket {
//                             payload: builder(stream),
//                         });
//                     }
//                     Err(_) => None,
//                 }
//             }
//         }
//         return decode(stream, self);
//     }
// }
