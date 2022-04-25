use std::net::TcpStream;

use crate::cocoabeans::protocol::packet::serverbound_packets::ServerBoundPayload;

// const MANAGER: std::collections::HashMap<i32, &'static dyn ProtocolVersion> = std::collections::HashMap::new();
//
// pub fn register_default_protocol() {
//     register_protocol_version(758, &V758 {})
// }
//
// pub fn register_protocol_version(version: i32, protocol: &'static dyn ProtocolVersion) {
//     MANAGER[&version] = protocol;
// }
//
// pub fn get_protocol_version(version: i32) -> &'static dyn ProtocolVersion {
//     return MANAGER[&version];
// }

pub type ServerBoundPacketBuilder = fn(stream: TcpStream) -> dyn ServerBoundPayload;

pub trait ProtocolVersion {
    // Handshaking - none

    // Status
    fn get_status_response_id(&self) -> u8;
    fn get_status_pong_id(&self) -> u8;

    // Login
    fn get_login_disconnect_id(&self) -> u8;
    fn get_login_encryption_request_id(&self) -> u8;
    fn get_login_success_id(&self) -> u8;
    fn get_login_set_compression_id(&self) -> u8;
    fn get_login_plugin_request_id(&self) -> u8;

    // Play - TODO

    // Incoming
    fn get_builder_from_id(&self, packet_id: u8) -> ServerBoundPacketBuilder;
}

pub struct V758 {}

impl ProtocolVersion for V758 {
    // Handshaking - none

    // Status
    fn get_status_response_id(&self) -> u8 {
        0x00
    }

    fn get_status_pong_id(&self) -> u8 {
        0x01
    }

    // Login
    fn get_login_disconnect_id(&self) -> u8 {
        0x00
    }

    fn get_login_encryption_request_id(&self) -> u8 {
        0x01
    }

    fn get_login_success_id(&self) -> u8 {
        0x02
    }

    fn get_login_set_compression_id(&self) -> u8 {
        0x03
    }

    fn get_login_plugin_request_id(&self) -> u8 {
        0x04
    }

    // Play

    // Incoming
    fn get_builder_from_id(&self, packet_id: u8) -> ServerBoundPacketBuilder {
        todo!()
    }
}
