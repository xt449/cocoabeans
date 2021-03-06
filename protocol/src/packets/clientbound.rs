use std::io::Write;

use serde_json::Value;

use math::coordinate::BlockPosition;

use crate::packets::data::Destination;
use crate::packets::identifier;

pub trait Packet {
    fn write_to(&self, write: &mut dyn Write);

    fn get_id() -> u8;
}

// Handshaking

// Status

pub struct StatusResponsePacket {
    pub json_payload: Value,
}

impl Packet for StatusResponsePacket {
    fn write_to(&self, write: &mut dyn Write) {
        todo!()
    }

    fn get_id() -> u8 {
        identifier::clientbound::STATUS_RESPONSE_PACKET
    }
}

pub struct StatusPongPacket {
    pub payload: u64,
}

impl Packet for StatusPongPacket {
    fn write_to(&self, write: &mut dyn Write) {
        todo!()
    }

    fn get_id() -> u8 {
        identifier::clientbound::STATUS_PONG_PACKET
    }
}

// Login

pub struct LoginDisconnectPacket {
    pub json_chat: String,
}

impl Packet for LoginDisconnectPacket {
    fn write_to(&self, write: &mut dyn Write) {
        todo!()
    }

    fn get_id() -> u8 {
        identifier::clientbound::LOGIN_DISCONNECT_PACKET
    }
}

pub struct LoginEncryptionRequestPacket {
    pub server_id: String,
    pub key: Vec<u8>,
    pub verification_token: Vec<u8>,
}

impl Packet for LoginEncryptionRequestPacket {
    fn write_to(&self, write: &mut dyn Write) {
        todo!()
    }

    fn get_id() -> u8 {
        identifier::clientbound::LOGIN_ENCRYPTION_REQUEST_PACKET
    }
}

pub struct LoginSuccessPacket {
    pub uuid: u128,
    pub username: String,
}

impl Packet for LoginSuccessPacket {
    fn write_to(&self, write: &mut dyn Write) {
        todo!()
    }

    fn get_id() -> u8 {
        identifier::clientbound::LOGIN_SUCCESS_PACKET
    }
}

pub struct LoginSetCompressionPacket {
    pub compression_threshold: u32,
}

impl Packet for LoginSetCompressionPacket {
    fn write_to(&self, write: &mut dyn Write) {
        todo!()
    }

    fn get_id() -> u8 {
        identifier::clientbound::LOGIN_SET_COMPRESSION_PACKET
    }
}

pub struct LoginPluginRequestPacket {
    pub message_id: u32,
    pub identifier: String,
    pub data: Vec<u8>,
}

impl Packet for LoginPluginRequestPacket {
    fn write_to(&self, write: &mut dyn Write) {
        todo!()
    }

    fn get_id() -> u8 {
        identifier::clientbound::LOGIN_PLUGIN_REQUEST_PACKET
    }
}

// Play

pub struct PlaySpawnEntityPacket {
    pub entity_id: u32,
    pub object_uuid: u128,
    pub entity_type: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
    pub data: i32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

pub struct PlaySpawnExperienceOrbPacket {
    pub entity_id: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub count: u16,
}

pub struct PlaySpawnLivingEntityPacket {
    pub entity_id: u32,
    pub entity_uuid: u128,
    pub entity_type: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub head_yaw: f32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

pub struct PlaySpawnPaintingPacket {
    pub entity_id: u32,
    pub entity_uuid: u128,
    pub motive: u32,
    pub location: BlockPosition,
    pub direction: u8,
}

pub struct PlaySpawnPlayerPacket {
    pub entity_id: u32,
    pub plauer_uuid: u128,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

pub struct PlaySculkVibrationSignalPacket {
    pub source_location: BlockPosition,
    pub destination_identifier: String,
    pub destination: Destination,
    pub arrival_ticks: i32,
}

pub struct PlayPacket {
    pub entity_id: u32,
}
