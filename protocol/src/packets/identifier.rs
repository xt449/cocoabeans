pub mod clientbound {
    pub const STATUS_RESPONSE_PACKET: u8 = 0x00;
    pub const STATUS_PONG_PACKET: u8 = 0x01;

    pub const LOGIN_DISCONNECT_PACKET: u8 = 0x00;
    pub const LOGIN_ENCRYPTION_REQUEST_PACKET: u8 = 0x01;
    pub const LOGIN_SUCCESS_PACKET: u8 = 0x02;
    pub const LOGIN_SET_COMPRESSION_PACKET: u8 = 0x03;
    pub const LOGIN_PLUGIN_REQUEST_PACKET: u8 = 0x04;

    pub const PLAY_SPAWN_ENTITY_PACKET: u8 = 0x00;
    pub const PLAY_SPAWN_EXPERIENCE_ORB_PACKET: u8 = 0x01;
    pub const PLAY_SPAWN_LIVING_ENTITY_PACKET: u8 = 0x02;
    pub const PLAY_SPAWN_PAINTING_PACKET: u8 = 0x03;
    pub const PLAY_SPAWN_PLAYER_PACKET: u8 = 0x04;
    pub const PLAY_SCULK_VIBRATION_SIGNAL_PACKET: u8 = 0x05;
    pub const PLAY_ENTITY_ANIMATION_PACKET: u8 = 0x06;
    pub const PLAY_STATISTICS_PACKET: u8 = 0x07;
    pub const PLAY_ACKNOWLEDGE_PLAYER_DIGGING_PACKET: u8 = 0x08;
    pub const PLAY_BREAK_BLOCK_ANIMATION_PACKET: u8 = 0x09;
    pub const PLAY_BLOCK_ENTITY_DATA_PACKET: u8 = 0x0A;
    pub const PLAY_BLOCK_ACTION_PACKET: u8 = 0x0B;
    pub const PLAY_BLOCK_CHANGE_PACKET: u8 = 0x0C;
    pub const PLAY_BOSS_BAR_PACKET: u8 = 0x0D;
    pub const PLAY_SERVER_DIFFICULTY_PACKET: u8 = 0x0E;
    pub const PLAY_CHAT_MESSAGE_PACKET: u8 = 0x0F;
    pub const PLAY_CLEAR_TITLES_PACKET: u8 = 0x10;
    pub const PLAY_TAB_COMPLETE_PACKET: u8 = 0x11;
    pub const PLAY_DECLARE_COMMANDS_PACKET: u8 = 0x12;
    pub const PLAY_CLOSE_WINDOW_PACKET: u8 = 0x13;
    pub const PLAY_WINDOW_ITEMS_PACKET: u8 = 0x14;
    pub const PLAY_WINDOW_PROPERTY_PACKET: u8 = 0x15;
    pub const PLAY_SET_SLOT_PACKET: u8 = 0x16;
    pub const PLAY_SET_COOLDOWN_PACKET: u8 = 0x17;
    pub const PLAY_PLUGIN_MESSAGE_PACKET: u8 = 0x18;
    pub const PLAY_NAMED_SOUND_EFFECT_PACKET: u8 = 0x19;
    pub const PLAY_DISCONNECT_PACKET: u8 = 0x1A;
    pub const PLAY_ENTITY_STATUS_PACKET: u8 = 0x1B;
    pub const PLAY_EXPLOSION_PACKET: u8 = 0x1C;
    pub const PLAY_UNLOAD_CHUNK_PACKET: u8 = 0x1D;
    pub const PLAY_CHANGE_GAME_STATE_PACKET: u8 = 0x1E;
    pub const PLAY_OPEN_HORSE_WINDOW_PACKET: u8 = 0x1F;
    pub const PLAY_INITIALIZE_WORLD_BORDER_PACKET: u8 = 0x20;
    pub const PLAY_KEEP_ALIVE_PACKET: u8 = 0x21;
    pub const PLAY_CHUNK_DATA_PACKET: u8 = 0x22;
    pub const PLAY_EFFECT_PACKET: u8 = 0x23;
    pub const PLAY_PARTICLE_PACKET: u8 = 0x24;
    pub const PLAY_UPDATE_LIGHT_PACKET: u8 = 0x25;
    pub const PLAY_JOIN_GAME_PACKET: u8 = 0x26;
    pub const PLAY_MAP_DATA_PACKET: u8 = 0x27;
    pub const PLAY_TRADE_LIST_PACKET: u8 = 0x28;
    pub const PLAY_ENTITY_POSITION_AND_ROTATION_PACKET: u8 = 0x2A;
    pub const PLAY_ENTITY_ROTATION_PACKET: u8 = 0x2B;
    pub const PLAY_VEHICLE_MOVE_PACKET: u8 = 0x2C;
    pub const PLAY_OPEN_BOOK_PACKET: u8 = 0x2D;
    pub const PLAY_OPEN_WINDOW_PACKET: u8 = 0x2E;
    pub const PLAY_OPEN_SIGN_EDITOR_PACKET: u8 = 0x2F;
    pub const PLAY_PING_PACKET: u8 = 0x30;
    pub const PLAY_CRAFT_RECIPE_RESPONSE_PACKET: u8 = 0x31;
    pub const PLAY_PLAYER_ABILITIES_PACKET: u8 = 0x32;
    pub const PLAY_END_COMBAT_PACKET: u8 = 0x33;
    pub const PLAY_ENTER_COMBAT_PACKET: u8 = 0x34;
    pub const PLAY_DEATH_COMBAT_PACKET: u8 = 0x35;
    pub const PLAY_PLAYER_INFO_PACKET: u8 = 0x36;
    pub const PLAY_FACE_PACKET: u8 = 0x37;
    pub const PLAY_PLAYER_POSITION_AND_LOOK_PACKET: u8 = 0x38;
    pub const PLAY_UNLUCK_RECIPES_PACKET: u8 = 0x39;
    pub const PLAY_DESTROY_ENTITIES_PACKET: u8 = 0x3A;
    pub const PLAY_REMOVE_ENTITY_EFFECT_PACKET: u8 = 0x3B;
    pub const PLAY_RESOURCE_PACK_SEND_PACKET: u8 = 0x3C;
    pub const PLAY_RESPAWN_PACKET: u8 = 0x3D;
    pub const PLAY_ENTITY_HEAD_LOOK_PACKET: u8 = 0x3E;
    pub const PLAY_MULTI_BLOCK_CHANGE_PACKET: u8 = 0x3F;
    pub const PLAY_SELECT_ADVANCEMENT_TAB_PACKET: u8 = 0x40;
    pub const PLAY_ACTION_BAR_PACKET: u8 = 0x41;
    pub const PLAY_WORLD_BORDER_CENTER_PACKET: u8 = 0x42;
    pub const PLAY_WORLD_BORDER_LERP_SIZE_PACKET: u8 = 0x43;
    pub const PLAY_WORLD_BORDER_SIZE_PACKET: u8 = 0x44;
    pub const PLAY_WORLD_BORDER_WARNING_DELAY_PACKET: u8 = 0x45;
    pub const PLAY_WORLD_BORDER_WARNING_REACH_PACKET: u8 = 0x46;
    pub const PLAY_CAMERA_PACKET: u8 = 0x47;
    pub const PLAY_HELD_ITEM_CHANGE_PACKET: u8 = 0x48;
    pub const PLAY_UPDATE_VIEW_POSITION_PACKET: u8 = 0x49;
    pub const PLAY_UPDATE_VIEW_DISTANCE_PACKET: u8 = 0x4A;
    pub const PLAY_SPAWN_POSITION_PACKET: u8 = 0x4B;
    pub const PLAY_DISPLAY_SCOREBOARD_PACKET: u8 = 0x4C;
    pub const PLAY_ENTITY_METADATA_PACKET: u8 = 0x4D;
    pub const PLAY_ATTACH_ENTITY_PACKET: u8 = 0x4E;
    pub const PLAY_ENTITY_VELOCITY_PACKET: u8 = 0x4F;
    pub const PLAY_ENTITY_EQUIPMENT_PACKET: u8 = 0x50;
    pub const PLAY_SET_EXPERIENCE_PACKET: u8 = 0x51;
    pub const PLAY_UPDATE_HEALTH_PACKET: u8 = 0x52;
    pub const PLAY_SCOREBOARD_OBJECTIVE_PACKET: u8 = 0x53;
    pub const PLAY_SET_PASSENGERS_PACKET: u8 = 0x54;
    pub const PLAY_TEAMS_PACKET: u8 = 0x55;
    pub const PLAY_UPDATE_SCORE_PACKET: u8 = 0x56;
    pub const PLAY_UPDATE_SIMULATION_DISTANCE_PACKET: u8 = 0x57;
    pub const PLAY_SET_TITLE_SUBTITLE_PACKET: u8 = 0x58;
    pub const PLAY_TIME_UPDATE_PACKET: u8 = 0x59;
    pub const PLAY_SET_TITLE_TEXT_PACKET: u8 = 0x5A;
    pub const PLAY_SET_TITLE_TIMES_PACKET: u8 = 0x5B;
    pub const PLAY_ENTITY_SOUND_EFFECT_PACKET: u8 = 0x5C;
    pub const PLAY_SOUND_EFFECT_PACKET: u8 = 0x5D;
    pub const PLAY_STOP_SOUND_PACKET: u8 = 0x5E;
    pub const PLAY_PLAYER_LIST_HEADER_AND_FOOTER_PACKET: u8 = 0x5F;
    pub const PLAY_NBT_QUERY_RESPONSE_PACKET: u8 = 0x60;
    pub const PLAY_COLLECT_ITEM_PACKET: u8 = 0x61;
    pub const PLAY_ENTITY_TELEPORT_PACKET: u8 = 0x62;
    pub const PLAY_ADVANCEMENTS_PACKET: u8 = 0x63;
    pub const PLAY_ENTITY_PROPERTIES_PACKET: u8 = 0x64;
    pub const PLAY_ENTITY_EFFECT_PACKET: u8 = 0x65;
    pub const PLAY_DECLARE_RECIPES_PACKET: u8 = 0x66;
    pub const PLAY_TAGS_PACKET: u8 = 0x67;
}

pub mod serverbound {
    use std::io::{Error, ErrorKind, Result};

    use crate::packets::serverbound::*;
    use crate::packets::State;

    pub fn build_packet(connection_state: State, packet_id: u32, packet_bytes: &[u8]) -> Result<Box<dyn Packet>> {
        return match connection_state {
            State::HANDSHAKING => HANDSHAKING.get(packet_id as usize).ok_or(Error::new(ErrorKind::InvalidData, "Unknown packet id"))?(packet_bytes),
            State::PLAY => PLAY.get(packet_id as usize).ok_or(Error::new(ErrorKind::InvalidData, "Unknown packet id"))?(packet_bytes),
            State::STATUS => STATUS.get(packet_id as usize).ok_or(Error::new(ErrorKind::InvalidData, "Unknown packet id"))?(packet_bytes),
            State::LOGIN => LOGIN.get(packet_id as usize).ok_or(Error::new(ErrorKind::InvalidData, "Unknown packet id"))?(packet_bytes),
        };
    }

    const HANDSHAKING: [PacketBuilder; 1] = [LoginPluginResponsePacket::BUILDER];
    const PLAY: [PacketBuilder; 2] = [StatusRequestPacket::BUILDER, StatusPingPacket::BUILDER];
    const STATUS: [PacketBuilder; 3] = [LoginStartPacket::BUILDER, LoginEncryptionResponsePacket::BUILDER, LoginPluginResponsePacket::BUILDER];
    const LOGIN: [PacketBuilder; 48] = [
        PlayTeleportConfirmPacket::BUILDER,
        PlayQueryBlockNBTPacket::BUILDER,
        PlaySetDifficultyPacket::BUILDER,
        PlayChatMessagePacket::BUILDER,
        PlayClientStatusPacket::BUILDER,
        PlayClientSettingsPacket::BUILDER,
        PlayTabCompletePacket::BUILDER,
        PlayClickWindowButtonPacket::BUILDER,
        PlayClickWindowPacket::BUILDER,
        PlayCloseWindowPacket::BUILDER,
        PlayPluginMessagePacket::BUILDER,
        PlayEditBookPacket::BUILDER,
        PlayQueryEntityNBTPacket::BUILDER,
        PlayInteractEntityPacket::BUILDER,
        PlayGenerateStructurePacket::BUILDER,
        PlayKeepAlivePacket::BUILDER,
        PlayLockDifficultyPacket::BUILDER,
        PlayPlayerPositionPacket::BUILDER,
        PlayPlayerPositionAndRotationPacket::BUILDER,
        PlayPlayerRotationPacket::BUILDER,
        PlayPlayerMovementPacket::BUILDER,
        PlayVehicleMovePacket::BUILDER,
        PlaySteerBoatPacket::BUILDER,
        PlayPickItemPacket::BUILDER,
        PlayCraftRecipeRequestPacket::BUILDER,
        PlayPlayerAbilitiesPacket::BUILDER,
        PlayPlayerDiggingPacket::BUILDER,
        PlayEntityActionPacket::BUILDER,
        PlaySteerVehiclePacket::BUILDER,
        PlayPongPacket::BUILDER,
        PlaySetRecipeBookStatePacket::BUILDER,
        PlaySetDisplayedRecipePacket::BUILDER,
        PlayNameItemPacket::BUILDER,
        PlayResourcePackStatusPacket::BUILDER,
        PlayAdvancementTabPacket::BUILDER,
        PlaySelectTradePacket::BUILDER,
        PlaySetBeaconEffectPacket::BUILDER,
        PlayHeldItemChangePacket::BUILDER,
        PlayUpdateCommandBlockPacket::BUILDER,
        PlayUpdateCommandBlockMinecartPacket::BUILDER,
        PlayCreativeInventoryActionPacket::BUILDER,
        PlayUpdateJigsawBlockPacket::BUILDER,
        PlayUpdateStructureBlockPacket::BUILDER,
        PlayUpdateSignBlockPacket::BUILDER,
        PlayAnimationPacket::BUILDER,
        PlaySpectatePacket::BUILDER,
        PlayPlayerBlockPlacementPacket::BUILDER,
        PlayUseItemPacket::BUILDER,
    ];
}
