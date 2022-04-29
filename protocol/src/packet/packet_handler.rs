use crate::cocoabeans::protocol::client_connection::ClientConnectionHandler;
use crate::cocoabeans::protocol::packet::serverbound::*;
use crate::cocoabeans::protocol::{version_manager, versions, ConnectionState};

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
    pub connection_state: ConnectionState,
    pub protocol_version: &'static dyn versions::ProtocolVersion,
}

impl PacketHandler {
    pub fn new(client_connection: &ClientConnectionHandler) -> PacketHandler {
        return PacketHandler {
            connection_state: ConnectionState::HANDSHAKING,
            protocol_version: &versions::V758 {},
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
