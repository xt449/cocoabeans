use crate::packet::packet_handler::State;
use crate::packet::serverbound::ServerBoundPacketBuilder;

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
    fn get_packet_builder_from_id(
        &self,
        connection_state: &State,
        packet_id: u8,
    ) -> Option<ServerBoundPacketBuilder>;
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

    // Play - TODO

    // Incoming
    fn get_packet_builder_from_id(
        &self,
        connection_state: &State,
        packet_id: u8,
    ) -> Option<ServerBoundPacketBuilder> {
        return match connection_state {
            State::HANDSHAKING => match packet_id {
                _ => {
                    todo!();
                    None
                }
            },
            State::STATUS => match packet_id {
                _ => {
                    todo!();
                    None
                }
            },
            State::LOGIN => match packet_id {
                _ => {
                    todo!();
                    None
                }
            },
            State::PLAY => match packet_id {
                _ => {
                    todo!();
                    None
                }
            },
        };
    }
}
