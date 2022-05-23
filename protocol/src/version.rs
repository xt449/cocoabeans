use crate::packet::packet_handler::State;
use crate::packet::serverbound::ServerBoundPacketBuilder;

pub trait ProtocolVersion: PacketIdentifier + PacketBuilderManager {
    fn get_id(&self) -> i32;
}

// Outgoing (ClientBound)
pub trait PacketIdentifier {
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
}

// Incoming (ServerBound)
pub trait PacketBuilderManager {
    fn get_packet_builder_from_id(
        &self,
        connection_state: State,
        packet_id: u8,
    ) -> Option<ServerBoundPacketBuilder>;
}
