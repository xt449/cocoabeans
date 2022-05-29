use crate::packet::packet_handler::State;
use crate::packet::serverbound;
use crate::packet::serverbound::ServerBoundPacketBuilder;
use crate::version::*;

/// Protocol Version for 758 (mc 1.18.2)
pub struct V758 {}

impl PacketIdentifier for V758 {
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
}

impl PacketBuilderManager for V758 {
    fn get_packet_builder_from_id(&self, connection_state: State, packet_id: u8) -> Option<ServerBoundPacketBuilder> {
        return match connection_state {
            State::HANDSHAKING => match packet_id {
                0 => Some(serverbound::handshaking::HandshakePayload::BUILDER),
                _ => None,
            },
            State::STATUS => match packet_id {
                0 => Some(serverbound::status::RequestPayload::BUILDER),
                1 => Some(serverbound::status::PingPayload::BUILDER),
                _ => None,
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

impl ProtocolVersion for V758 {
    fn get_id(&self) -> i32 {
        758
    }
}

// TODO - make rust happy
/*const MANAGER: std::collections::HashMap<i32, &'static dyn ProtocolVersion> = get_default_map();

const fn get_default_map() -> std::collections::HashMap<i32, &'static dyn ProtocolVersion> {
    let mut map: std::collections::HashMap<i32, &'static dyn ProtocolVersion> =
        std::collections::HashMap::new();
    map[&758] = &V758 {};
    return map;
}

pub fn register_protocol_version(version: i32, protocol: &'static dyn ProtocolVersion) {
    MANAGER[&version] = protocol;
}*/

const DEFAULT_VERSION: V758 = V758 {};

pub fn get_protocol_version(version: i32) -> Option<&'static dyn PacketIdentifier> {
    return Some(&DEFAULT_VERSION);
    /*return if MANAGER.contains_key(&version) {
        Some(MANAGER[&version])
    } else {
        None
    };*/
}
