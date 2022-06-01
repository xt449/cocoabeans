use std::io::Result;
use std::net::{SocketAddr, TcpStream};

use serde_json::json;

use protocol::packets::io::PacketIO;
use protocol::packets::{clientbound, serverbound, Handler, State};

// Connection
pub struct Connection<'c> {
    address: SocketAddr,
    pub io: PacketIO<'c>,
}

// Connection Constructor
impl<'c> Connection<'c> {
    pub fn new(address: SocketAddr, stream: &'c TcpStream) -> Self {
        stream.set_nonblocking(false).expect("Unable to make TcpStream blocking");

        return Self { address: address, io: PacketIO::new(stream) };
    }
}

// Connection Methods
impl Connection<'_> {
    pub fn handle_next_packet(&mut self) -> Result<()> {
        return Ok(self.io.read_packet()?.handle(self));
    }
}

// Handler implementation
impl Handler for Connection<'_> {
    // Handshaking

    fn handle_handshaking(&mut self, payload: &serverbound::HandshakingPacket) {
        println!("Received HandshakePayload");
        // TODO: support multiple protocol versions
        if payload.protocol_version != 758 {
            return;
        }

        match payload.next_state {
            State::STATUS => self.io.state = State::STATUS,
            State::LOGIN => self.io.state = State::LOGIN,
            _ => {}
        }
    }

    // Status

    fn handle_status_request(&mut self, payload: &serverbound::StatusRequestPacket) {
        self.io
            .write_packet(clientbound::StatusResponsePacket {
                // TODO
                json_payload: json!({
                    "version": {
                        "name": "1.18.2 or not",
                        "protocol": 758
                    },
                    "players": {
                        "max": 1000000000,
                        "online": -1
                    },
                    "description": {
                        "text": "Hello world",
                        "color": "aqua"
                    }
                }),
            })
            .expect(/*TODO*/ "TODO: panic message");
    }

    fn handle_status_ping(&mut self, payload: &serverbound::StatusPingPacket) {
        self.io.write_packet(clientbound::StatusPongPacket { payload: payload.payload }).expect(/*TODO*/ "TODO: panic message");
    }

    // Login

    fn handle_login_start(&mut self, payload: &serverbound::LoginStartPacket) {
        todo!()
    }

    fn handle_login_encryption_response(&mut self, payload: &serverbound::LoginEncryptionResponsePacket) {
        todo!()
    }

    fn handle_login_plugin_response(&mut self, payload: &serverbound::LoginPluginResponsePacket) {
        todo!()
    }

    // Play

    fn handle_play_teleport_confirm(&mut self, payload: &serverbound::PlayTeleportConfirmPacket) {
        todo!()
    }

    fn handle_play_query_block_nbt(&mut self, payload: &serverbound::PlayQueryBlockNBTPacket) {
        todo!()
    }

    fn handle_play_set_difficulty(&mut self, payload: &serverbound::PlaySetDifficultyPacket) {
        todo!()
    }

    fn handle_play_chat_message(&mut self, payload: &serverbound::PlayChatMessagePacket) {
        todo!()
    }

    fn handle_play_client_status(&mut self, payload: &serverbound::PlayClientStatusPacket) {
        todo!()
    }

    fn handle_play_client_settings(&mut self, payload: &serverbound::PlayClientSettingsPacket) {
        todo!()
    }

    fn handle_play_tab_complete(&mut self, payload: &serverbound::PlayTabCompletePacket) {
        todo!()
    }

    fn handle_play_click_window_button(&mut self, payload: &serverbound::PlayClickWindowButtonPacket) {
        todo!()
    }

    fn handle_play_click_window(&mut self, payload: &serverbound::PlayClickWindowPacket) {
        todo!()
    }

    fn handle_play_close_window(&mut self, payload: &serverbound::PlayCloseWindowPacket) {
        todo!()
    }

    fn handle_play_plugin_message(&mut self, payload: &serverbound::PlayPluginMessagePacket) {
        todo!()
    }

    fn handle_play_edit_book(&mut self, payload: &serverbound::PlayEditBookPacket) {
        todo!()
    }

    fn handle_play_query_entity_nbt(&mut self, payload: &serverbound::PlayQueryEntityNBTPacket) {
        todo!()
    }

    fn handle_play_interact_entity(&mut self, payload: &serverbound::PlayInteractEntityPacket) {
        todo!()
    }

    fn handle_play_generate_structure(&mut self, payload: &serverbound::PlayGenerateStructurePacket) {
        todo!()
    }

    fn handle_play_keep_alive(&mut self, payload: &serverbound::PlayKeepAlivePacket) {
        todo!()
    }

    fn handle_play_lock_difficulty(&mut self, payload: &serverbound::PlayLockDifficultyPacket) {
        todo!()
    }

    fn handle_play_player_position(&mut self, payload: &serverbound::PlayPlayerPositionPacket) {
        todo!()
    }

    fn handle_play_player_position_and_rotation(&mut self, payload: &serverbound::PlayPlayerPositionAndRotationPacket) {
        todo!()
    }

    fn handle_play_player_rotation(&mut self, payload: &serverbound::PlayPlayerRotationPacket) {
        todo!()
    }

    fn handle_play_player_movement(&mut self, payload: &serverbound::PlayPlayerMovementPacket) {
        todo!()
    }

    fn handle_play_vehicle_move(&mut self, payload: &serverbound::PlayVehicleMovePacket) {
        todo!()
    }

    fn handle_play_steer_boat(&mut self, payload: &serverbound::PlaySteerBoatPacket) {
        todo!()
    }

    fn handle_play_pick_item(&mut self, payload: &serverbound::PlayPickItemPacket) {
        todo!()
    }

    fn handle_play_craft_recipe_request(&mut self, payload: &serverbound::PlayCraftRecipeRequestPacket) {
        todo!()
    }

    fn handle_play_player_abilities(&mut self, payload: &serverbound::PlayPlayerAbilitiesPacket) {
        todo!()
    }

    fn handle_play_player_digging(&mut self, payload: &serverbound::PlayPlayerDiggingPacket) {
        todo!()
    }

    fn handle_play_entity_action(&mut self, payload: &serverbound::PlayEntityActionPacket) {
        todo!()
    }

    fn handle_play_steer_vehicle(&mut self, payload: &serverbound::PlaySteerVehiclePacket) {
        todo!()
    }

    fn handle_play_pong(&mut self, payload: &serverbound::PlayPongPacket) {
        todo!()
    }

    fn handle_play_set_recipe_book_state(&mut self, payload: &serverbound::PlaySetRecipeBookStatePacket) {
        todo!()
    }

    fn handle_play_set_displayed_recipe(&mut self, payload: &serverbound::PlaySetDisplayedRecipePacket) {
        todo!()
    }

    fn handle_play_name_item(&mut self, payload: &serverbound::PlayNameItemPacket) {
        todo!()
    }

    fn handle_play_resource_pack_status(&mut self, payload: &serverbound::PlayResourcePackStatusPacket) {
        todo!()
    }

    fn handle_play_advancement_tab(&mut self, payload: &serverbound::PlayAdvancementTabPacket) {
        todo!()
    }

    fn handle_play_select_trade(&mut self, payload: &serverbound::PlaySelectTradePacket) {
        todo!()
    }

    fn handle_play_set_beacon_effect(&mut self, payload: &serverbound::PlaySetBeaconEffectPacket) {
        todo!()
    }

    fn handle_play_held_item_change(&mut self, payload: &serverbound::PlayHeldItemChangePacket) {
        todo!()
    }

    fn handle_play_update_command_block(&mut self, payload: &serverbound::PlayUpdateCommandBlockPacket) {
        todo!()
    }

    fn handle_play_update_command_block_minecart(&mut self, payload: &serverbound::PlayUpdateCommandBlockMinecartPacket) {
        todo!()
    }

    fn handle_play_creative_inventory_action(&mut self, payload: &serverbound::PlayCreativeInventoryActionPacket) {
        todo!()
    }

    fn handle_play_update_jigsaw_block(&mut self, payload: &serverbound::PlayUpdateJigsawBlockPacket) {
        todo!()
    }

    fn handle_play_update_structure_block(&mut self, payload: &serverbound::PlayUpdateStructureBlockPacket) {
        todo!()
    }

    fn handle_play_update_sign_block(&mut self, payload: &serverbound::PlayUpdateSignBlockPacket) {
        todo!()
    }

    fn handle_play_animation(&mut self, payload: &serverbound::PlayAnimationPacket) {
        todo!()
    }

    fn handle_play_spectate(&mut self, payload: &serverbound::PlaySpectatePacket) {
        todo!()
    }

    fn handle_play_player_block_placement(&mut self, payload: &serverbound::PlayPlayerBlockPlacementPacket) {
        todo!()
    }

    fn handle_play_use_item(&mut self, payload: &serverbound::PlayUseItemPacket) {
        todo!()
    }
}
