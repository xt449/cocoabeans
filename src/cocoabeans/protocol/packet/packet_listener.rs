use crate::cocoabeans::protocol::packet::serverbound_packets::*;

pub trait IPacketListener {
    fn handle_handshaking_handshake(&mut self, payload: &handshaking::HandshakePayload);
    fn handle_status_request(&self, payload: &status::RequestPayload);
    fn handle_status_ping(&self, payload: &status::PingPayload);
    fn handle_login_start(&self, payload: &login::StartPayload);
    fn handle_login_encryption_response(&self, payload: &login::EncryptionResponsePayload);
    fn handle_login_plugin_response(&self, payload: &login::PluginResponsePayload);
}

pub struct PacketListener {
    protocol_version: i32, // TODO - do something even smarter
}

impl IPacketListener for PacketListener {
    fn handle_handshaking_handshake(&mut self, payload: &handshaking::HandshakePayload) {
        self.protocol_version = payload.protocol_version;
        // TODO - do something smart
        println!("{}", payload.address);
    }

    fn handle_status_request(&self, payload: &status::RequestPayload) {
        todo!()
    }

    fn handle_status_ping(&self, payload: &status::PingPayload) {
        todo!()
    }

    fn handle_login_start(&self, payload: &login::StartPayload) {
        todo!()
    }

    fn handle_login_encryption_response(&self, payload: &login::EncryptionResponsePayload) {
        todo!()
    }

    fn handle_login_plugin_response(&self, payload: &login::PluginResponsePayload) {
        todo!()
    }
}
