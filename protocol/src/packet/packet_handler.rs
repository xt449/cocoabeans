use std::io::Read;
use std::net::TcpStream;

use crate::io::MinecraftReader;
use crate::packet::serverbound::*;
use crate::versions;
use crate::versions::ProtocolVersion;
use crate::{read_helper, version_manager};

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
    pub stream: TcpStream,
    pub state: State,
    pub protocol_version: &'static dyn ProtocolVersion,
    pub compression: bool,
    pub encryption: Option<u64>,
}

// Constructor
impl PacketHandler {
    pub fn new(stream: TcpStream) -> PacketHandler {
        return PacketHandler {
            stream: stream,
            state: State::HANDSHAKING,
            protocol_version: &versions::V758 {},
            compression: false,
            encryption: None,
        };
    }
}

// Packet stuff
impl PacketHandler {
    pub fn read(&mut self) -> Option<ServerBoundPacket> {
        let length = read_helper::read_varint(&mut self.stream);

        let mut vec_backing = Vec::<u8>::with_capacity(length as usize);
        let slice: &mut [u8] = vec_backing.as_mut_slice();
        let read_result = self.stream.read(slice);
        match read_result {
            Ok(_) => { /*continue*/ }
            Err(_) => {
                return None;
            }
        }

        return match self.encryption {
            None => PacketHandler::decode_packet(
                &self.state,
                self.protocol_version,
                MinecraftReader::from(&slice),
            ),
            Some(cipher) => PacketHandler::decode_packet(
                &self.state,
                self.protocol_version,
                PacketHandler::decrypt_packet(cipher, &slice),
            ),
        };
    }

    fn decrypt_packet(cipher: u64, mut slice: &[u8]) -> MinecraftReader {
        // TODO
        return MinecraftReader::from(slice);
    }

    fn decode_packet(
        state: &State,
        protocol_version: &dyn ProtocolVersion,
        mut reader: MinecraftReader,
    ) -> Option<ServerBoundPacket> {
        let id = reader.read_varint();
        return match protocol_version.get_builder_from_id(state, id as u8) {
            None => None,
            Some(builder) => Some(builder(&reader)),
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
