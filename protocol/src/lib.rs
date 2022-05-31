use std::fs::read;
use crate::io::{MinecraftReader, MinecraftWriter};
use packets::data::io::{ReadVarIntExt, WriteVarIntExt};
use packets::wrapped::{clientbound, serverbound};
use packets::{Handler, State};
use serde_json::json;
use std::io::{Error, ErrorKind, Read, Take, Write};
use std::net::TcpStream;
use std::ops::Deref;
use packets::wrapped::serverbound::Packet;

pub mod connection;
pub mod io;

pub struct PacketHandler {
    stream: TcpStream,
    state: State,
    compression: bool,
    encryption: Option<u64>,
}

// Constructor
impl PacketHandler {
    pub fn new(stream: TcpStream) -> PacketHandler {
        stream.set_nonblocking(false).expect("Unable to make TcpStream blocking");
        return PacketHandler { stream: stream, state: State::HANDSHAKING, compression: false, encryption: None };
    }
}

// Packet Accessors
impl PacketHandler {
    pub fn write_packet<T: clientbound::Packet>(&mut self, packet: T) {
        let mut buffer = MinecraftWriter::new();
        packet.write_to(&mut buffer);
        let bytes = buffer.to_slice();
        println!("DEBUG Sending packet #{} with total length {}", bytes[0], bytes.len());

        //bytes.insert(0, bytes.len() as u8);
        println!("DEBUG [ {} ]", bytes.iter().map(|v| format!("{:02X}", v)).collect::<Vec<String>>().join(" "));

        // let mut buffer = MinecraftReader::from(buffer.to_slice());
        // println!(
        //     "DEBUG id: {}, string: {}",
        //     buffer.read_unsigned_byte()?,
        //     buffer.read_string()?
        // );

        self.stream.write_varint(bytes.len() as i32).unwrap();
        self.stream.write_all(bytes.deref()).unwrap();
    }

    pub fn next(&mut self) -> Result<(), Error> {
        let packet = self.read_next_packet()?;
        packet.handle(self);
        return Ok(());
    }

    fn read_next_packet(&mut self) -> Result<&dyn serverbound::Packet, Error> {
        let mut length: i32;
        loop {
            // do while
            match self.stream.read_varint() {
                Ok(v) => {
                    length = v;
                }
                Err(e) => {
                    return Err(e);
                }
            }
            if length != 0 {
                break;
            }
        }

        let taken = (&self.stream).take(length as u64);

        let packet_data = match self.encryption {
            None => MinecraftReader::from(taken),
            Some(cipher) => PacketHandler::decrypt_packet(cipher, MinecraftReader::from(taken)),
        };

        return self.decode_packet(packet_data);
    }

    fn decrypt_packet(cipher: u64, mut reader: MinecraftReader) -> MinecraftReader {
        // TODO
        return reader;
    }

    fn decode_packet(&self, mut reader: MinecraftReader) -> std::io::Result<&dyn serverbound::Packet> {
        println!("Decoding packet {} bytes long...", reader.remaining());
        let id = reader.read_varint().ok()?;
        println!(
            "Decoding packet id#{} in state {}",
            id,
            match self.state {
                State::HANDSHAKING => "HANDSHAKING",
                State::STATUS => "STATUS",
                State::LOGIN => "LOGIN",
                State::PLAY => "PLAY",
            }
        );

        // return match self.protocol_version.get_packet_builder_from_id(self.state.clone(), id as u8) {
        //     None => None,
        //     Some(builder) => builder(reader),
        // };

        todo!()
    }
}

// Packet Handler
impl Handler for PacketHandler {
    // Handshaking

    fn handle_handshaking(&mut self, payload: &serverbound::HandshakingPacket) {
        println!("Received HandshakePayload");
        // TODO: support multiple protocol versions
        if payload.protocol_version != 758 {
            return;
        }

        match payload.next_state {
            State::STATUS => {
                self.state = State::STATUS;
            }
            State::LOGIN => {
                self.state = State::LOGIN;
            }
            _ => {}
        }
    }

    // Status

    fn handle_status_request(&mut self, payload: &serverbound::StatusRequestPacket) {
        clientbound::StatusResponsePacket {
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
            })
            .to_string(),
        }.write_to(self);
    }

    fn handle_status_ping(&mut self, payload: &serverbound::StatusPingPacket) {
        clientbound::StatusPongPacket { payload: payload.payload }.write_to(self);
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
