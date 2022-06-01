use std::io::{Error, ErrorKind};

use crate::wrapped::serverbound::*;

pub mod data;
mod identifier;
pub mod wrapped;

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
    fn handle_handshaking(&mut self, payload: &HandshakingPacket);
    // Status
    fn handle_status_request(&mut self, payload: &StatusRequestPacket);
    fn handle_status_ping(&mut self, payload: &StatusPingPacket);
    // Login
    fn handle_login_start(&mut self, payload: &LoginStartPacket);
    fn handle_login_encryption_response(&mut self, payload: &LoginEncryptionResponsePacket);
    fn handle_login_plugin_response(&mut self, payload: &LoginPluginResponsePacket);
    // Play
    fn handle_play_teleport_confirm(&mut self, payload: &PlayTeleportConfirmPacket);
    fn handle_play_query_block_nbt(&mut self, payload: &PlayQueryBlockNBTPacket);
    fn handle_play_set_difficulty(&mut self, payload: &PlaySetDifficultyPacket);
    fn handle_play_chat_message(&mut self, payload: &PlayChatMessagePacket);
    fn handle_play_client_status(&mut self, payload: &PlayClientStatusPacket);
    fn handle_play_client_settings(&mut self, payload: &PlayClientSettingsPacket);
    fn handle_play_tab_complete(&mut self, payload: &PlayTabCompletePacket);
    fn handle_play_click_window_button(&mut self, payload: &PlayClickWindowButtonPacket);
    fn handle_play_click_window(&mut self, payload: &PlayClickWindowPacket);
    fn handle_play_close_window(&mut self, payload: &PlayCloseWindowPacket);
    fn handle_play_plugin_message(&mut self, payload: &PlayPluginMessagePacket);
    fn handle_play_edit_book(&mut self, payload: &PlayEditBookPacket);
    fn handle_play_query_entity_nbt(&mut self, payload: &PlayQueryEntityNBTPacket);
    fn handle_play_interact_entity(&mut self, payload: &PlayInteractEntityPacket);
    fn handle_play_generate_structure(&mut self, payload: &PlayGenerateStructurePacket);
    fn handle_play_keep_alive(&mut self, payload: &PlayKeepAlivePacket);
    fn handle_play_lock_difficulty(&mut self, payload: &PlayLockDifficultyPacket);
    fn handle_play_player_position(&mut self, payload: &PlayPlayerPositionPacket);
    fn handle_play_player_position_and_rotation(&mut self, payload: &PlayPlayerPositionAndRotationPacket);
    fn handle_play_player_rotation(&mut self, payload: &PlayPlayerRotationPacket);
    fn handle_play_player_movement(&mut self, payload: &PlayPlayerMovementPacket);
    fn handle_play_vehicle_move(&mut self, payload: &PlayVehicleMovePacket);
    fn handle_play_steer_boat(&mut self, payload: &PlaySteerBoatPacket);
    fn handle_play_pick_item(&mut self, payload: &PlayPickItemPacket);
    fn handle_play_craft_recipe_request(&mut self, payload: &PlayCraftRecipeRequestPacket);
    fn handle_play_player_abilities(&mut self, payload: &PlayPlayerAbilitiesPacket);
    fn handle_play_player_digging(&mut self, payload: &PlayPlayerDiggingPacket);
    fn handle_play_entity_action(&mut self, payload: &PlayEntityActionPacket);
    fn handle_play_steer_vehicle(&mut self, payload: &PlaySteerVehiclePacket);
    fn handle_play_pong(&mut self, payload: &PlayPongPacket);
    fn handle_play_set_recipe_book_state(&mut self, payload: &PlaySetRecipeBookStatePacket);
    fn handle_play_set_displayed_recipe(&mut self, payload: &PlaySetDisplayedRecipePacket);
    fn handle_play_name_item(&mut self, payload: &PlayNameItemPacket);
    fn handle_play_resource_pack_status(&mut self, payload: &PlayResourcePackStatusPacket);
    fn handle_play_advancement_tab(&mut self, payload: &PlayAdvancementTabPacket);
    fn handle_play_select_trade(&mut self, payload: &PlaySelectTradePacket);
    fn handle_play_set_beacon_effect(&mut self, payload: &PlaySetBeaconEffectPacket);
    fn handle_play_held_item_change(&mut self, payload: &PlayHeldItemChangePacket);
    fn handle_play_update_command_block(&mut self, payload: &PlayUpdateCommandBlockPacket);
    fn handle_play_update_command_block_minecart(&mut self, payload: &PlayUpdateCommandBlockMinecartPacket);
    fn handle_play_creative_inventory_action(&mut self, payload: &PlayCreativeInventoryActionPacket);
    fn handle_play_update_jigsaw_block(&mut self, payload: &PlayUpdateJigsawBlockPacket);
    fn handle_play_update_structure_block(&mut self, payload: &PlayUpdateStructureBlockPacket);
    fn handle_play_update_sign_block(&mut self, payload: &PlayUpdateSignBlockPacket);
    fn handle_play_animation(&mut self, payload: &PlayAnimationPacket);
    fn handle_play_spectate(&mut self, payload: &PlaySpectatePacket);
    fn handle_play_player_block_placement(&mut self, payload: &PlayPlayerBlockPlacementPacket);
    fn handle_play_use_item(&mut self, payload: &PlayUseItemPacket);
}
