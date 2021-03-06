use std::io::{Error, ErrorKind, Result};

use byteorder::{NetworkEndian, ReadBytesExt};
use registries::potion::PotionRegistry;

use math::coordinate::BlockPosition;

use crate::packets::data::io::{ReadBlockPositionExt, ReadBoolExt, ReadByteVecExt, ReadIdentifierExt, ReadItemStackExt, ReadLimitedStringExt, ReadVarIntExt};
use crate::packets::data::{Identifier, ItemStack};
use crate::packets::{Handler, State};

pub trait Packet {
    fn handle(&self, handler: &mut dyn Handler);
}

pub type PacketBuilder = fn(&[u8]) -> Result<Box<dyn Packet>>;

// Handshaking

pub struct HandshakingPacket {
    pub protocol_version: i32, // VARINT
    pub address: String,       // MAX 255
    pub port: u16,
    pub next_state: State, // VARINT
}

impl Packet for HandshakingPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_handshaking(self);
    }
}

impl HandshakingPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self {
            protocol_version: packet_bytes.read_varint()?,
            address: packet_bytes.read_limited_string(255)?,
            port: packet_bytes.read_u16::<NetworkEndian>()?,
            next_state: State::try_from(packet_bytes.read_varint()? as u32)?,
        }));
    };
}

// Status

pub struct StatusRequestPacket {
    // no fields
}

impl Packet for StatusRequestPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_status_request(self);
    }
}

impl StatusRequestPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self {}));
    };
}

pub struct StatusPingPacket {
    pub payload: u64,
}

impl Packet for StatusPingPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_status_ping(self);
    }
}

impl StatusPingPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { payload: packet_bytes.read_u64::<NetworkEndian>()? }));
    };
}

// Login

pub struct LoginStartPacket {
    username: String, // MAX 16
}

impl Packet for LoginStartPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_login_start(self);
    }
}

impl LoginStartPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { username: packet_bytes.read_limited_string(16)? }));
    };
}

pub struct LoginEncryptionResponsePacket {
    secret: Vec<u8>,
    verification_token: Vec<u8>,
}

impl Packet for LoginEncryptionResponsePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_login_encryption_response(self);
    }
}

impl LoginEncryptionResponsePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        let length = packet_bytes.read_varint()? as usize;
        let secret = packet_bytes.read_byte_vec(length)?;
        let length = packet_bytes.read_varint()? as usize;
        let verification_token = packet_bytes.read_byte_vec(length)?;
        return Ok(Box::new(Self { secret, verification_token }));
    };
}

pub struct LoginPluginResponsePacket {
    message_id: i32, // VARINT
    successful: bool,
    data: Option<Vec<u8>>,
}

impl Packet for LoginPluginResponsePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_login_plugin_response(self);
    }
}

impl LoginPluginResponsePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        let message_id = packet_bytes.read_varint()?;
        let successful = packet_bytes.read_bool()?;
        let data;
        if successful {
            data = Some(packet_bytes.read_byte_vec(packet_bytes.len())?);
        } else {
            data = None;
        }
        return Ok(Box::new(Self { message_id, successful, data }));
    };
}

// Play

pub struct PlayTeleportConfirmPacket {
    pub transaction_id: i32, // VARINT
}

impl Packet for PlayTeleportConfirmPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_teleport_confirm(self);
    }
}

impl PlayTeleportConfirmPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { transaction_id: packet_bytes.read_varint()? }));
    };
}

pub struct PlayQueryBlockNBTPacket {
    pub transaction_id: i32, // VARINT
    pub location: BlockPosition,
}

impl Packet for PlayQueryBlockNBTPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_query_block_nbt(self);
    }
}

impl PlayQueryBlockNBTPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { transaction_id: packet_bytes.read_varint()?, location: packet_bytes.read_block_position()? }));
    };
}

#[deprecated]
pub struct PlaySetDifficultyPacket {
    pub difficulty: u8,
}

impl Packet for PlaySetDifficultyPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_set_difficulty(self);
    }
}

impl PlaySetDifficultyPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { difficulty: packet_bytes.read_u8()? }));
    };
}

pub struct PlayChatMessagePacket {
    pub message: String, // MAX 256
}

impl Packet for PlayChatMessagePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_chat_message(self);
    }
}

impl PlayChatMessagePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { message: packet_bytes.read_limited_string(256)? }));
    };
}

pub struct PlayClientStatusPacket {
    pub action: i32, // VARINT
}

impl Packet for PlayClientStatusPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_client_status(self);
    }
}

impl PlayClientStatusPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { action: packet_bytes.read_varint()? }));
    };
}

// TODO needs better wrapping
pub struct PlayClientSettingsPacket {
    pub locale: String, // MAX 16
    pub view_distance: u8,
    pub chat_mode: i32, // VARINT
    pub chat_colors: bool,
    pub skin_parts: u8,       // BITFLAG
    pub main_hand: i32,       // VARINT
    pub text_filtering: bool, // expect false
    pub server_listing: bool,
}

impl Packet for PlayClientSettingsPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_client_settings(self);
    }
}

impl PlayClientSettingsPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self {
            locale: packet_bytes.read_limited_string(16)?,
            view_distance: packet_bytes.read_u8()?,
            chat_mode: packet_bytes.read_varint()?,
            chat_colors: packet_bytes.read_bool()?,
            skin_parts: packet_bytes.read_u8()?,
            main_hand: packet_bytes.read_varint()?,
            text_filtering: packet_bytes.read_bool()?,
            server_listing: packet_bytes.read_bool()?,
        }));
    };
}

pub struct PlayTabCompletePacket {
    pub transaction_id: i32, // VARINT
    pub text: String,        // MAX 32500
}

impl Packet for PlayTabCompletePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_tab_complete(self);
    }
}

impl PlayTabCompletePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { transaction_id: packet_bytes.read_varint()?, text: packet_bytes.read_limited_string(32500)? }));
    };
}

pub struct PlayClickWindowButtonPacket {
    pub window_id: u8,
    pub button_id: u8,
}

impl Packet for PlayClickWindowButtonPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_click_window_button(self);
    }
}

impl PlayClickWindowButtonPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { window_id: packet_bytes.read_u8()?, button_id: packet_bytes.read_u8()? }));
    };
}

// TODO needs better wrapping
pub struct PlayClickWindowPacket {
    pub window_id: u8, // actually unsigned
    pub state_id: i32, // VARINT
    pub slot: i16,
    pub button: u8,
    pub mode: i32,        // VARINT
    pub slots_count: i32, // VARINT
    pub slots: Vec<(i16, ItemStack)>,
    pub carried_item: ItemStack, // must be empty for PlayDropKey mode
}

impl Packet for PlayClickWindowPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_click_window(self);
    }
}

impl PlayClickWindowPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        let window_id = packet_bytes.read_u8()?;
        let state_id = packet_bytes.read_varint()?;
        let slot = packet_bytes.read_i16::<NetworkEndian>()?;
        let button = packet_bytes.read_u8()?;
        let mode = packet_bytes.read_varint()?;
        let slots_count = packet_bytes.read_varint()?;
        let mut slots = Vec::with_capacity(slots_count as usize);

        for i in 0..(slots_count + 1) as usize {
            slots[i] = (packet_bytes.read_i16::<NetworkEndian>()?, packet_bytes.read_item_stack()?);
        }

        let carried_item = packet_bytes.read_item_stack()?;

        return Ok(Box::new(Self { window_id, state_id, slot, button, mode, slots_count, slots, carried_item }));
    };
}

pub struct PlayCloseWindowPacket {
    pub window_id: u8, // actually unsigned
}

impl Packet for PlayCloseWindowPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_close_window(self);
    }
}

impl PlayCloseWindowPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { window_id: packet_bytes.read_u8()? }));
    };
}

pub struct PlayPluginMessagePacket {
    pub channel: Identifier,
    pub data: Vec<u8>, // length must be inferred from packet length
}

impl Packet for PlayPluginMessagePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_plugin_message(self);
    }
}

impl PlayPluginMessagePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { channel: packet_bytes.read_identifier()?, data: packet_bytes.read_byte_vec(packet_bytes.len())? }));
    };
}

pub struct PlayEditBookPacket {
    pub slot: i32,             // VARINT
    pub count: i32,            // VARINT
    pub entries: Vec<String>,  // length already given by previous field, string MAX 8192
    pub title: Option<String>, // MAX 128
}

impl Packet for PlayEditBookPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_edit_book(self);
    }
}

impl PlayEditBookPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        let slot = packet_bytes.read_varint()?;
        let count = packet_bytes.read_varint()?;
        let entries_count = packet_bytes.read_varint()?;
        let mut entries = Vec::with_capacity(entries_count as usize);

        for i in 0..(entries_count + 1) as usize {
            entries[i] = packet_bytes.read_limited_string(8192)?;
        }

        let has_title = packet_bytes.read_bool()?;
        let title = if has_title { Some(packet_bytes.read_limited_string(128)?) } else { None };

        return Ok(Box::new(Self { slot, count, entries, title }));
    };
}

pub struct PlayQueryEntityNBTPacket {
    pub transaction_id: i32, // VARINT
    pub entity_id: i32,      // VARINT
}

impl Packet for PlayQueryEntityNBTPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_query_entity_nbt(self);
    }
}

impl PlayQueryEntityNBTPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { transaction_id: packet_bytes.read_varint()?, entity_id: packet_bytes.read_varint()? }));
    };
}

// TODO needs better wrapping
pub struct PlayInteractEntityPacket {
    pub entity_id: i32,        // VARINT
    pub interaction_type: i32, // VARINT
    pub target_x: Option<f32>, // None unless interaction_type is InteractAt
    pub target_y: Option<f32>, // None unless interaction_type is InteractAt
    pub target_z: Option<f32>, // None unless interaction_type is InteractAt
    pub hand: Option<i32>,     // None unless interaction_type is Interact or InteractAt
    pub sneeking: bool,
}

impl Packet for PlayInteractEntityPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_interact_entity(self);
    }
}

impl PlayInteractEntityPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        let entity_id = packet_bytes.read_varint()?;
        let interaction_type = packet_bytes.read_varint()?;
        let target_x;
        let target_y;
        let target_z;
        let hand;
        match interaction_type {
            0 => {
                target_x = None;
                target_y = None;
                target_z = None;
                hand = Some(packet_bytes.read_varint()?);
            }
            1 => {
                target_x = None;
                target_y = None;
                target_z = None;
                hand = None;
            }
            2 => {
                target_x = Some(packet_bytes.read_f32::<NetworkEndian>()?);
                target_y = Some(packet_bytes.read_f32::<NetworkEndian>()?);
                target_z = Some(packet_bytes.read_f32::<NetworkEndian>()?);
                hand = Some(packet_bytes.read_varint()?);
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "Unknown iteraction_type from primitive")),
        }

        let sneeking = packet_bytes.read_bool()?;

        return Ok(Box::new(Self { entity_id, interaction_type, target_x, target_y, target_z, hand, sneeking }));
    };
}

pub struct PlayGenerateStructurePacket {
    pub location: BlockPosition,
    pub levels: i32, // VARINT
    pub keep_jigsaws: bool,
}

impl Packet for PlayGenerateStructurePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_generate_structure(self);
    }
}

impl PlayGenerateStructurePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { location: packet_bytes.read_block_position()?, levels: packet_bytes.read_varint()?, keep_jigsaws: packet_bytes.read_bool()? }));
    };
}

pub struct PlayKeepAlivePacket {
    pub id: u64,
}

impl Packet for PlayKeepAlivePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_keep_alive(self);
    }
}

impl PlayKeepAlivePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { id: packet_bytes.read_u64::<NetworkEndian>()? }));
    };
}

#[deprecated]
pub struct PlayLockDifficultyPacket {
    pub locked: bool,
}

impl Packet for PlayLockDifficultyPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_lock_difficulty(self);
    }
}

impl PlayLockDifficultyPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { locked: packet_bytes.read_bool()? }));
    };
}

pub struct PlayPlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool, // walking or swimming
}

impl Packet for PlayPlayerPositionPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_player_position(self);
    }
}

impl PlayPlayerPositionPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { x: packet_bytes.read_f64::<NetworkEndian>()?, y: packet_bytes.read_f64::<NetworkEndian>()?, z: packet_bytes.read_f64::<NetworkEndian>()?, on_ground: packet_bytes.read_bool()? }));
    };
}

pub struct PlayPlayerPositionAndRotationPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool, // walking or swimming
}

impl Packet for PlayPlayerPositionAndRotationPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_player_position_and_rotation(self);
    }
}

impl PlayPlayerPositionAndRotationPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self {
            x: packet_bytes.read_f64::<NetworkEndian>()?,
            y: packet_bytes.read_f64::<NetworkEndian>()?,
            z: packet_bytes.read_f64::<NetworkEndian>()?,
            yaw: packet_bytes.read_f32::<NetworkEndian>()?,
            pitch: packet_bytes.read_f32::<NetworkEndian>()?,
            on_ground: packet_bytes.read_bool()?,
        }));
    };
}

pub struct PlayPlayerRotationPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool, // walking or swimming
}

impl Packet for PlayPlayerRotationPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_player_rotation(self);
    }
}

impl PlayPlayerRotationPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { yaw: packet_bytes.read_f32::<NetworkEndian>()?, pitch: packet_bytes.read_f32::<NetworkEndian>()?, on_ground: packet_bytes.read_bool()? }));
    };
}

pub struct PlayPlayerMovementPacket {
    pub on_ground: bool, // walking or swimming
}

impl Packet for PlayPlayerMovementPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_player_movement(self);
    }
}

impl PlayPlayerMovementPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { on_ground: packet_bytes.read_bool()? }));
    };
}

pub struct PlayVehicleMovePacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

impl Packet for PlayVehicleMovePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_vehicle_move(self);
    }
}

impl PlayVehicleMovePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self {
            x: packet_bytes.read_f64::<NetworkEndian>()?,
            y: packet_bytes.read_f64::<NetworkEndian>()?,
            z: packet_bytes.read_f64::<NetworkEndian>()?,
            yaw: packet_bytes.read_f32::<NetworkEndian>()?,
            pitch: packet_bytes.read_f32::<NetworkEndian>()?,
        }));
    };
}

pub struct PlaySteerBoatPacket {
    pub left_paddle: bool,
    pub right_paddle: bool,
}

impl Packet for PlaySteerBoatPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_steer_boat(self);
    }
}

impl PlaySteerBoatPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { left_paddle: packet_bytes.read_bool()?, right_paddle: packet_bytes.read_bool()? }));
    };
}

pub struct PlayPickItemPacket {
    pub slot: i32, // VARINT
}

impl Packet for PlayPickItemPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_pick_item(self);
    }
}

impl PlayPickItemPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { slot: packet_bytes.read_varint()? }));
    };
}

pub struct PlayCraftRecipeRequestPacket {
    pub window_id: u8,
    pub recipe: Identifier,
    pub make_all: bool, // when shift clicking
}

impl Packet for PlayCraftRecipeRequestPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_craft_recipe_request(self);
    }
}

impl PlayCraftRecipeRequestPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { window_id: packet_bytes.read_u8()?, recipe: packet_bytes.read_identifier()?, make_all: packet_bytes.read_bool()? }));
    };
}

pub struct PlayPlayerAbilitiesPacket {
    pub flags: u8, // BITFLAG
}

impl Packet for PlayPlayerAbilitiesPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_player_abilities(self);
    }
}

impl PlayPlayerAbilitiesPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { flags: packet_bytes.read_u8()? }));
    };
}

pub struct PlayPlayerDiggingPacket {
    pub status: i32, // VARINT
    pub location: BlockPosition,
    pub face: u8,
}

impl Packet for PlayPlayerDiggingPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_player_digging(self);
    }
}

impl PlayPlayerDiggingPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { status: packet_bytes.read_varint()?, location: packet_bytes.read_block_position()?, face: packet_bytes.read_u8()? }));
    };
}

pub struct PlayEntityActionPacket {
    pub entity_id: i32,  // VARINT
    pub action: i32,     // VARINT
    pub jump_boost: i32, // VARINT TODO: rename
}

impl Packet for PlayEntityActionPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_entity_action(self);
    }
}

impl PlayEntityActionPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { entity_id: packet_bytes.read_varint()?, action: packet_bytes.read_varint()?, jump_boost: packet_bytes.read_varint()? }));
    };
}

pub struct PlaySteerVehiclePacket {
    pub sideways: f32,
    pub forward: f32,
    pub flags: u8, // BITFLAG
}

impl Packet for PlaySteerVehiclePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_steer_vehicle(self);
    }
}

impl PlaySteerVehiclePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { sideways: packet_bytes.read_f32::<NetworkEndian>()?, forward: packet_bytes.read_f32::<NetworkEndian>()?, flags: packet_bytes.read_u8()? }));
    };
}

pub struct PlayPongPacket {
    pub id: i32, // FULLINT
}

impl Packet for PlayPongPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_pong(self);
    }
}

impl PlayPongPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { id: packet_bytes.read_i32::<NetworkEndian>()? }));
    };
}

pub struct PlaySetRecipeBookStatePacket {
    pub book_id: i32, // VARINT
    pub book_open: bool,
    pub filter_active: bool,
}

impl Packet for PlaySetRecipeBookStatePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_set_recipe_book_state(self);
    }
}

impl PlaySetRecipeBookStatePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { book_id: packet_bytes.read_varint()?, book_open: packet_bytes.read_bool()?, filter_active: packet_bytes.read_bool()? }));
    };
}

pub struct PlaySetDisplayedRecipePacket {
    pub recipe_id: Identifier,
}

impl Packet for PlaySetDisplayedRecipePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_set_displayed_recipe(self);
    }
}

impl PlaySetDisplayedRecipePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { recipe_id: packet_bytes.read_identifier()? }));
    };
}

pub struct PlayNameItemPacket {
    pub name: String, // MAX 32767
}

impl Packet for PlayNameItemPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_name_item(self);
    }
}

impl PlayNameItemPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { name: packet_bytes.read_limited_string(32767)? }));
    };
}

pub struct PlayResourcePackStatusPacket {
    pub result: i32, // VARINT
}

impl Packet for PlayResourcePackStatusPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_resource_pack_status(self);
    }
}

impl PlayResourcePackStatusPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { result: packet_bytes.read_varint()? }));
    };
}

pub struct PlayAdvancementTabPacket {
    pub action: i32, // VARINT
    pub tab_id: Option<Identifier>,
}

impl Packet for PlayAdvancementTabPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_advancement_tab(self);
    }
}

impl PlayAdvancementTabPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        let action = packet_bytes.read_varint()?;
        let tab_id;
        match action {
            0 => {
                tab_id = Some(packet_bytes.read_identifier()?);
            }
            1 => {
                tab_id = None;
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "Unknown action from primitive")),
        }

        return Ok(Box::new(Self { action, tab_id }));
    };
}

pub struct PlaySelectTradePacket {
    pub selected_slot: i32, // VARINT
}

impl Packet for PlaySelectTradePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_select_trade(self);
    }
}

impl PlaySelectTradePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { selected_slot: packet_bytes.read_varint()? }));
    };
}

pub struct PlaySetBeaconEffectPacket {
    pub primary_effect: PotionRegistry,   // VARINT
    pub secondary_effect: PotionRegistry, // VARINT
}

impl Packet for PlaySetBeaconEffectPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_set_beacon_effect(self);
    }
}

impl PlaySetBeaconEffectPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { primary_effect: PotionRegistry::try_from(packet_bytes.read_varint()? as u32)?, secondary_effect: PotionRegistry::try_from(packet_bytes.read_varint()? as u32)? }));
    };
}

pub struct PlayHeldItemChangePacket {
    pub slot: u16, // [0,8] TODO: really, a short for Playthis?
}

impl Packet for PlayHeldItemChangePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_held_item_change(self);
    }
}

impl PlayHeldItemChangePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { slot: packet_bytes.read_u16::<NetworkEndian>()? }));
    };
}

pub struct PlayUpdateCommandBlockPacket {
    pub location: BlockPosition,
    pub command: String, // MAX 32767
    pub mode: i32,       // VARINT
    pub flags: u8,       // BITFLAG
}

impl Packet for PlayUpdateCommandBlockPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_update_command_block(self);
    }
}

impl PlayUpdateCommandBlockPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { location: packet_bytes.read_block_position()?, command: packet_bytes.read_limited_string(32767)?, mode: packet_bytes.read_varint()?, flags: packet_bytes.read_u8()? }));
    };
}

pub struct PlayUpdateCommandBlockMinecartPacket {
    pub entity_id: i32,  // VARINT
    pub command: String, // MAX 32767
    pub track_output: bool,
}

impl Packet for PlayUpdateCommandBlockMinecartPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_update_command_block_minecart(self);
    }
}

impl PlayUpdateCommandBlockMinecartPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { entity_id: packet_bytes.read_varint()?, command: packet_bytes.read_limited_string(32767)?, track_output: packet_bytes.read_bool()? }));
    };
}

pub struct PlayCreativeInventoryActionPacket {
    pub slot: i16,
    pub item: ItemStack,
}

impl Packet for PlayCreativeInventoryActionPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_creative_inventory_action(self);
    }
}

impl PlayCreativeInventoryActionPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { slot: packet_bytes.read_i16::<NetworkEndian>()?, item: packet_bytes.read_item_stack()? }));
    };
}

pub struct PlayUpdateJigsawBlockPacket {
    pub location: BlockPosition,
    pub name: Identifier,
    pub target: Identifier,
    pub pool: Identifier,
    pub final_state: String, // MAX 32767
    // TODO: wrap this
    pub joint_type: String, // MAX 32767, "rollable" or "aligned"
}

impl Packet for PlayUpdateJigsawBlockPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_update_jigsaw_block(self);
    }
}

impl PlayUpdateJigsawBlockPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self {
            location: packet_bytes.read_block_position()?,
            name: packet_bytes.read_identifier()?,
            target: packet_bytes.read_identifier()?,
            pool: packet_bytes.read_identifier()?,
            final_state: packet_bytes.read_limited_string(32767)?,
            joint_type: packet_bytes.read_limited_string(32767)?,
        }));
    };
}

pub struct PlayUpdateStructureBlockPacket {
    pub location: BlockPosition,
    pub action: i32,  // VARINT
    pub mode: i32,    // VARINT
    pub name: String, // MAX 32767
    pub offset_x: i8,
    pub offset_y: i8,
    pub offset_z: i8,
    pub size_x: i8,
    pub size_y: i8,
    pub size_z: i8,
    pub mirror: i32,      // VARINT
    pub rotation: i32,    // VARINT
    pub metadata: String, // MAX 128
    pub integrety: f32,
    pub seed: i64, // VARLONG
    pub flags: u8, // BITFLAG
}

impl Packet for PlayUpdateStructureBlockPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_update_structure_block(self);
    }
}

impl PlayUpdateStructureBlockPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self {
            location: packet_bytes.read_block_position()?,
            action: packet_bytes.read_varint()?,
            mode: packet_bytes.read_varint()?,
            name: packet_bytes.read_limited_string(32767)?,
            offset_x: packet_bytes.read_i8()?,
            offset_y: packet_bytes.read_i8()?,
            offset_z: packet_bytes.read_i8()?,
            size_x: packet_bytes.read_i8()?,
            size_y: packet_bytes.read_i8()?,
            size_z: packet_bytes.read_i8()?,
            mirror: packet_bytes.read_varint()?,
            rotation: packet_bytes.read_varint()?,
            metadata: packet_bytes.read_limited_string(128)?,
            integrety: packet_bytes.read_f32::<NetworkEndian>()?,
            seed: packet_bytes.read_i64::<NetworkEndian>()?,
            flags: packet_bytes.read_u8()?,
        }));
    };
}

pub struct PlayUpdateSignBlockPacket {
    pub location: BlockPosition,
    pub line_1: String, // MAX 384
    pub line_2: String, // MAX 384
    pub line_3: String, // MAX 384
    pub line_4: String, // MAX 384
}

impl Packet for PlayUpdateSignBlockPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_update_sign_block(self);
    }
}

impl PlayUpdateSignBlockPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self {
            location: packet_bytes.read_block_position()?,
            line_1: packet_bytes.read_limited_string(384)?,
            line_2: packet_bytes.read_limited_string(384)?,
            line_3: packet_bytes.read_limited_string(384)?,
            line_4: packet_bytes.read_limited_string(384)?,
        }));
    };
}

pub struct PlayAnimationPacket {
    pub hand: i32, // VARINT
}

impl Packet for PlayAnimationPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_animation(self);
    }
}

impl PlayAnimationPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { hand: packet_bytes.read_varint()? }));
    };
}

pub struct PlaySpectatePacket {
    pub target: u128,
}

impl Packet for PlaySpectatePacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_spectate(self);
    }
}

impl PlaySpectatePacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { target: packet_bytes.read_u128::<NetworkEndian>()? }));
    };
}

pub struct PlayPlayerBlockPlacementPacket {
    pub hand: i32, // VARINT
    pub location: BlockPosition,
    pub face: u8, // BYTE
    pub cusor_position_x: f32,
    pub cusor_position_y: f32,
    pub cusor_position_z: f32,
    pub inside_block: bool,
}

impl Packet for PlayPlayerBlockPlacementPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_player_block_placement(self);
    }
}

impl PlayPlayerBlockPlacementPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self {
            hand: packet_bytes.read_varint()?,
            location: packet_bytes.read_block_position()?,
            face: packet_bytes.read_u8()?,
            cusor_position_x: packet_bytes.read_f32::<NetworkEndian>()?,
            cusor_position_y: packet_bytes.read_f32::<NetworkEndian>()?,
            cusor_position_z: packet_bytes.read_f32::<NetworkEndian>()?,
            inside_block: packet_bytes.read_bool()?,
        }));
    };
}

pub struct PlayUseItemPacket {
    pub hand: i32, // VARINT
}

impl Packet for PlayUseItemPacket {
    fn handle(&self, handler: &mut dyn Handler) {
        handler.handle_play_use_item(self);
    }
}

impl PlayUseItemPacket {
    pub const BUILDER: PacketBuilder = |mut packet_bytes| {
        return Ok(Box::new(Self { hand: packet_bytes.read_varint()? }));
    };
}
