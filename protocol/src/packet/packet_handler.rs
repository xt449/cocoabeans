use std::net::TcpStream;

use crate::io::MinecraftStream;
use crate::packet::serverbound::*;
use crate::version_manager;
use crate::versions;
use crate::versions::ProtocolVersion;

pub enum State {
    HANDSHAKING = 0,
    STATUS = 1,
    LOGIN = 2,
    PLAY = 3,
}

pub trait IPacketHandler {
    // Handshaking
    fn handle_handshaking_handshake(&mut self, payload: &handshaking::HandshakePayload);
    // Status
    fn handle_status_request(&self, payload: &status::RequestPayload);
    fn handle_status_ping(&self, payload: &status::PingPayload);
    // Login
    fn handle_login_start(&self, payload: &login::StartPayload);
    fn handle_login_encryption_response(&self, payload: &login::EncryptionResponsePayload);
    fn handle_login_plugin_response(&self, payload: &login::PluginResponsePayload);
    // Play - TODO
}

pub struct PacketHandler {
    pub stream: MinecraftStream,
    pub state: State,
    pub protocol_version: &'static dyn ProtocolVersion,
    pub compression: bool,
    pub encryption: Option<u64>,
}

// Constructor
impl PacketHandler {
    pub fn new(stream: TcpStream) -> PacketHandler {
        return PacketHandler {
            stream: MinecraftStream::wrap(stream),
            state: State::HANDSHAKING,
            protocol_version: &versions::V758 {},
            compression: false,
            encryption: None,
        };
    }
}

// Packet read Pipeline start
impl PacketHandler {
    pub fn read_packet(&mut self) -> Option<ServerBoundPacket> {
        // TODO unused length data
        let length = self.stream.get_reader().read_varint();

        return match self.encryption {
            None => {
                PacketHandler::decode_packet(self.protocol_version, &self.state, &mut self.stream)
            }
            Some(_) => PacketHandler::decode_packet(
                self.protocol_version,
                &self.state,
                PacketHandler::decrypt_packet(&mut self.stream),
            ),
        };
    }
}

// Packet Decrypter
impl PacketHandler {
    fn decrypt_packet(stream: &mut MinecraftStream) -> &mut MinecraftStream {
        // TODO
        return stream;
    }
}

// Packet Decoder
impl PacketHandler {
    fn decode_packet(
        protocol_version: &dyn ProtocolVersion,
        state: &State,
        stream: &mut MinecraftStream,
    ) -> Option<ServerBoundPacket> {
        let id = stream.get_reader().read_varint();
        return match protocol_version.get_builder_from_id(state, id as u8) {
            None => None,
            Some(builder) => Some(builder(stream.get_reader())),
        };
    }
}

impl IPacketHandler for PacketHandler {
    // Handshaking

    fn handle_handshaking_handshake(&mut self, payload: &handshaking::HandshakePayload) {
        match version_manager::get_protocol_version(payload.protocol_version) {
            None => {}
            Some(version) => {
                self.protocol_version = version;
            }
        }
        // TODO - do something smart
        println!("{}", payload.address);
    }

    // Status

    fn handle_status_request(&self, payload: &status::RequestPayload) {
        todo!()
    }

    fn handle_status_ping(&self, payload: &status::PingPayload) {
        todo!()
    }

    // Login

    fn handle_login_start(&self, payload: &login::StartPayload) {
        todo!()
    }

    fn handle_login_encryption_response(&self, payload: &login::EncryptionResponsePayload) {
        todo!()
    }

    fn handle_login_plugin_response(&self, payload: &login::PluginResponsePayload) {
        todo!()
    }

    // Play - TODO
}
