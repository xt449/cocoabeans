use std::io::{Error, ErrorKind};

pub mod clientbound;
pub mod data;
pub mod identifier;
pub mod io;
pub mod serverbound;

#[derive(Copy, Clone)]
pub enum State {
    HANDSHAKING = -1,
    PLAY = 0,
    STATUS = 1,
    LOGIN = 2,
}

impl TryFrom<u32> for State {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        return match value {
            v if v == Self::HANDSHAKING as u32 => Ok(Self::HANDSHAKING),
            v if v == Self::PLAY as u32 => Ok(Self::PLAY),
            v if v == Self::STATUS as u32 => Ok(Self::STATUS),
            v if v == Self::LOGIN as u32 => Ok(Self::LOGIN),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Unable to convert to State")),
        };
    }
}

pub trait Handler {
    // Handshaking
    fn handle_handshaking(&mut self, payload: &serverbound::HandshakingPacket);
    // Status
    fn handle_status_request(&mut self, payload: &serverbound::StatusRequestPacket);
    fn handle_status_ping(&mut self, payload: &serverbound::StatusPingPacket);
    // Login
    fn handle_login_start(&mut self, payload: &serverbound::LoginStartPacket);
    fn handle_login_encryption_response(&mut self, payload: &serverbound::LoginEncryptionResponsePacket);
    fn handle_login_plugin_response(&mut self, payload: &serverbound::LoginPluginResponsePacket);
    // Play
    fn handle_play_teleport_confirm(&mut self, payload: &serverbound::PlayTeleportConfirmPacket);
    fn handle_play_query_block_nbt(&mut self, payload: &serverbound::PlayQueryBlockNBTPacket);
    fn handle_play_set_difficulty(&mut self, payload: &serverbound::PlaySetDifficultyPacket);
    fn handle_play_chat_message(&mut self, payload: &serverbound::PlayChatMessagePacket);
    fn handle_play_client_status(&mut self, payload: &serverbound::PlayClientStatusPacket);
    fn handle_play_client_settings(&mut self, payload: &serverbound::PlayClientSettingsPacket);
    fn handle_play_tab_complete(&mut self, payload: &serverbound::PlayTabCompletePacket);
    fn handle_play_click_window_button(&mut self, payload: &serverbound::PlayClickWindowButtonPacket);
    fn handle_play_click_window(&mut self, payload: &serverbound::PlayClickWindowPacket);
    fn handle_play_close_window(&mut self, payload: &serverbound::PlayCloseWindowPacket);
    fn handle_play_plugin_message(&mut self, payload: &serverbound::PlayPluginMessagePacket);
    fn handle_play_edit_book(&mut self, payload: &serverbound::PlayEditBookPacket);
    fn handle_play_query_entity_nbt(&mut self, payload: &serverbound::PlayQueryEntityNBTPacket);
    fn handle_play_interact_entity(&mut self, payload: &serverbound::PlayInteractEntityPacket);
    fn handle_play_generate_structure(&mut self, payload: &serverbound::PlayGenerateStructurePacket);
    fn handle_play_keep_alive(&mut self, payload: &serverbound::PlayKeepAlivePacket);
    fn handle_play_lock_difficulty(&mut self, payload: &serverbound::PlayLockDifficultyPacket);
    fn handle_play_player_position(&mut self, payload: &serverbound::PlayPlayerPositionPacket);
    fn handle_play_player_position_and_rotation(&mut self, payload: &serverbound::PlayPlayerPositionAndRotationPacket);
    fn handle_play_player_rotation(&mut self, payload: &serverbound::PlayPlayerRotationPacket);
    fn handle_play_player_movement(&mut self, payload: &serverbound::PlayPlayerMovementPacket);
    fn handle_play_vehicle_move(&mut self, payload: &serverbound::PlayVehicleMovePacket);
    fn handle_play_steer_boat(&mut self, payload: &serverbound::PlaySteerBoatPacket);
    fn handle_play_pick_item(&mut self, payload: &serverbound::PlayPickItemPacket);
    fn handle_play_craft_recipe_request(&mut self, payload: &serverbound::PlayCraftRecipeRequestPacket);
    fn handle_play_player_abilities(&mut self, payload: &serverbound::PlayPlayerAbilitiesPacket);
    fn handle_play_player_digging(&mut self, payload: &serverbound::PlayPlayerDiggingPacket);
    fn handle_play_entity_action(&mut self, payload: &serverbound::PlayEntityActionPacket);
    fn handle_play_steer_vehicle(&mut self, payload: &serverbound::PlaySteerVehiclePacket);
    fn handle_play_pong(&mut self, payload: &serverbound::PlayPongPacket);
    fn handle_play_set_recipe_book_state(&mut self, payload: &serverbound::PlaySetRecipeBookStatePacket);
    fn handle_play_set_displayed_recipe(&mut self, payload: &serverbound::PlaySetDisplayedRecipePacket);
    fn handle_play_name_item(&mut self, payload: &serverbound::PlayNameItemPacket);
    fn handle_play_resource_pack_status(&mut self, payload: &serverbound::PlayResourcePackStatusPacket);
    fn handle_play_advancement_tab(&mut self, payload: &serverbound::PlayAdvancementTabPacket);
    fn handle_play_select_trade(&mut self, payload: &serverbound::PlaySelectTradePacket);
    fn handle_play_set_beacon_effect(&mut self, payload: &serverbound::PlaySetBeaconEffectPacket);
    fn handle_play_held_item_change(&mut self, payload: &serverbound::PlayHeldItemChangePacket);
    fn handle_play_update_command_block(&mut self, payload: &serverbound::PlayUpdateCommandBlockPacket);
    fn handle_play_update_command_block_minecart(&mut self, payload: &serverbound::PlayUpdateCommandBlockMinecartPacket);
    fn handle_play_creative_inventory_action(&mut self, payload: &serverbound::PlayCreativeInventoryActionPacket);
    fn handle_play_update_jigsaw_block(&mut self, payload: &serverbound::PlayUpdateJigsawBlockPacket);
    fn handle_play_update_structure_block(&mut self, payload: &serverbound::PlayUpdateStructureBlockPacket);
    fn handle_play_update_sign_block(&mut self, payload: &serverbound::PlayUpdateSignBlockPacket);
    fn handle_play_animation(&mut self, payload: &serverbound::PlayAnimationPacket);
    fn handle_play_spectate(&mut self, payload: &serverbound::PlaySpectatePacket);
    fn handle_play_player_block_placement(&mut self, payload: &serverbound::PlayPlayerBlockPlacementPacket);
    fn handle_play_use_item(&mut self, payload: &serverbound::PlayUseItemPacket);
}
