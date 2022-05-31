use crate::data::Destination;
use std::io::Write;

pub trait Packet {
    fn write_to(&self, write: &mut dyn Write);
}

// Handshaking

// Status

pub struct StatusResponsePacket {
    pub json_payload: String,
}

pub struct StatusPongPacket {
    pub payload: u64,
}

// Login

pub struct LoginDisconnectPacket {
    pub json_chat: String,
}

pub struct LoginEncryptionRequestPacket {
    pub server_id: String,
    pub key: Vec<u8>,
    pub verification_token: Vec<u8>,
}

pub struct LoginSuccessPacket {
    pub uuid: u128,
    pub username: String,
}

pub struct LoginSetCompressionPacket {
    pub compression_threshold: u32,
}

pub struct LoginPluginRequestPacket {
    pub message_id: u32,
    pub identifier: String,
    pub data: Vec<u8>,
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
    pub location: (i32, i16, i32),
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
    pub source_location: (i32, i16, i32),
    pub destination_identifier: String,
    pub destination: Destination,
    pub arrival_ticks: i32,
}

pub struct PlayPacket {
    pub entity_id: u32,
}
