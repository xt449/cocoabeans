use crate::data::io::{ReadVarIntExt, WriteVarIntExt};
use serde_json::json;
use std::io::{Error, ErrorKind, Read, Take, Write};
use std::net::TcpStream;
use std::ops::Deref;

use crate::io::{MinecraftReader, MinecraftWriter};
use crate::packet::clientbound;
use crate::packet::clientbound::ClientBoundPacket;
use crate::packet::serverbound;
use crate::packet::serverbound::{handshaking, login, play, status};
use crate::version::ProtocolVersion;
use crate::version_manager;

use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Copy, Clone)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum State {
    HANDSHAKING = -1,
    PLAY = 0,
    STATUS = 1,
    LOGIN = 2,
}

impl State {
    pub fn next_state(self) -> State {
        return match self {
            State::STATUS => State::STATUS,
            State::LOGIN => State::LOGIN,
            _ => self,
        };
    }
}

pub trait IPacketHandler {
    // Handshaking
    fn handle_handshaking_handshake(&mut self, payload: &handshaking::HandshakePayload);
    // Status
    fn handle_status_request(&mut self, payload: &status::RequestPayload);
    fn handle_status_ping(&mut self, payload: &status::PingPayload);
    // Login
    fn handle_login_start(&mut self, payload: &login::StartPayload);
    fn handle_login_encryption_response(&mut self, payload: &login::EncryptionResponsePayload);
    fn handle_login_plugin_response(&mut self, payload: &login::PluginResponsePayload);
    // Play
    fn handle_play_teleport_confirm(&mut self, payload: &play::TeleportConfirmPayload);
    fn handle_play_query_block_nbt(&mut self, payload: &play::QueryBlockNBTPayload);
    fn handle_play_set_difficulty(&mut self, payload: &play::SetDifficultyPayload);
    fn handle_play_chat_message(&mut self, payload: &play::ChatMessagePayload);
    fn handle_play_client_status(&mut self, payload: &play::ClientStatusPayload);
    fn handle_play_client_settings(&mut self, payload: &play::ClientSettingsPayload);
    fn handle_play_tab_complete(&mut self, payload: &play::TabCompletePayload);
    fn handle_play_click_window_button(&mut self, payload: &play::ClickWindowButtonPayload);
    fn handle_play_click_window(&mut self, payload: &play::ClickWindowPayload);
    fn handle_play_close_window(&mut self, payload: &play::CloseWindowPayload);
    fn handle_play_plugin_message(&mut self, payload: &play::PluginMessagePayload);
    fn handle_play_edit_book(&mut self, payload: &play::EditBookPayload);
    fn handle_play_query_entity_nbt(&mut self, payload: &play::QueryEntityNBTPayload);
    fn handle_play_interact_entity(&mut self, payload: &play::InteractEntityPayload);
    fn handle_play_generate_structure(&mut self, payload: &play::GenerateStructurePayload);
    fn handle_play_keep_alive(&mut self, payload: &play::KeepAlivePayload);
    fn handle_play_lock_difficulty(&mut self, payload: &play::LockDifficultyPayload);
    fn handle_play_player_position(&mut self, payload: &play::PlayerPositionPayload);
    fn handle_play_player_position_and_rotation(
        &mut self,
        payload: &play::PlayerPositionAndRotationPayload,
    );
    fn handle_play_player_rotation(&mut self, payload: &play::PlayerRotationPayload);
    fn handle_play_player_movement(&mut self, payload: &play::PlayerMovementPayload);
    fn handle_play_vehicle_move(&mut self, payload: &play::VehicleMovePayload);
    fn handle_play_steer_boat(&mut self, payload: &play::SteerBoatPayload);
    fn handle_play_pick_item(&mut self, payload: &play::PickItemPayload);
    fn handle_play_craft_recipe_request(&mut self, payload: &play::CraftRecipeRequestPayload);
    fn handle_play_player_abilities(&mut self, payload: &play::PlayerAbilitiesPayload);
    fn handle_play_player_digging(&mut self, payload: &play::PlayerDiggingPayload);
    fn handle_play_entity_action(&mut self, payload: &play::EntityActionPayload);
    fn handle_play_steer_vehicle(&mut self, payload: &play::SteerVehiclePayload);
    fn handle_play_pong(&mut self, payload: &play::PongPayload);
    fn handle_play_set_recipe_book_state(&mut self, payload: &play::SetRecipeBookStatePayload);
    fn handle_play_set_displayed_recipe(&mut self, payload: &play::SetDisplayedRecipePayload);
    fn handle_play_name_item(&mut self, payload: &play::NameItemPayload);
    fn handle_play_resource_pack_status(&mut self, payload: &play::ResourcePackStatusPayload);
    fn handle_play_advancement_tab(&mut self, payload: &play::AdvancementTabPayload);
    fn handle_play_select_trade(&mut self, payload: &play::SelectTradePayload);
    fn handle_play_set_beacon_effect(&mut self, payload: &play::SetBeaconEffectPayload);
    fn handle_play_held_item_change(&mut self, payload: &play::HeldItemChangePayload);
    fn handle_play_update_command_block(&mut self, payload: &play::UpdateCommandBlockPayload);
    fn handle_play_update_command_block_minecart(
        &mut self,
        payload: &play::UpdateCommandBlockMinecartPayload,
    );
    fn handle_play_creative_inventory_action(
        &mut self,
        payload: &play::CreativeInventoryActionPayload,
    );
    fn handle_play_update_jigsaw_block(&mut self, payload: &play::UpdateJigsawBlockPayload);
    fn handle_play_update_structure_block(&mut self, payload: &play::UpdateStructureBlockPayload);
    fn handle_play_update_sign_block(&mut self, payload: &play::UpdateSignBlockPayload);
    fn handle_play_animation(&mut self, payload: &play::AnimationPayload);
    fn handle_play_spectate(&mut self, payload: &play::SpectatePayload);
    fn handle_play_player_block_placement(&mut self, payload: &play::PlayerBlockPlacementPayload);
    fn handle_play_use_item(&mut self, payload: &play::UseItemPayload);
}

pub struct PacketHandler<'a> {
    stream: TcpStream,
    state: State,
    protocol_version: &'a dyn ProtocolVersion,
    compression: bool,
    encryption: Option<u64>,
}

// Constructor
impl<'a> PacketHandler<'a> {
    pub fn new(stream: TcpStream) -> PacketHandler<'a> {
        stream
            .set_nonblocking(false)
            .expect("Unable to make TcpStream blocking");
        return PacketHandler {
            stream: stream,
            state: State::HANDSHAKING,
            protocol_version: &version_manager::V758 {},
            compression: false,
            encryption: None,
        };
    }
}

// Packet Accessors
impl<'a> PacketHandler<'a> {
    pub fn write_packet<T: ClientBoundPacket>(&mut self, packet: T) {
        let mut buffer = MinecraftWriter::new();
        packet.write_to(&mut buffer, self.protocol_version);
        let bytes = buffer.to_slice();
        println!(
            "DEBUG Sending packet #{} with total length {}",
            bytes[0],
            bytes.len()
        );

        //bytes.insert(0, bytes.len() as u8);
        println!(
            "DEBUG [ {} ]",
            bytes
                .iter()
                .map(|v| format!("{:02X}", v))
                .collect::<Vec<String>>()
                .join(" ")
        );

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

    fn read_next_packet(&mut self) -> Result<serverbound::ServerBoundPacket, Error> {
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

        return match self.decode_packet(packet_data) {
            None => Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to decrypt packet",
            )),
            Some(packet) => Ok(packet),
        };
    }

    fn decrypt_packet(cipher: u64, mut reader: MinecraftReader) -> MinecraftReader {
        // TODO
        return reader;
    }

    fn decode_packet(&self, mut reader: MinecraftReader) -> Option<serverbound::ServerBoundPacket> {
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
        return match self
            .protocol_version
            .get_packet_builder_from_id(self.state.clone(), id as u8)
        {
            None => None,
            Some(builder) => builder(reader),
        };
    }
}

// Packet Handler
impl<'a> IPacketHandler for PacketHandler<'a> {
    // Handshaking

    fn handle_handshaking_handshake(&mut self, payload: &handshaking::HandshakePayload) {
        println!("Received HandshakePayload");
        // TODO - support multiple protocol versions
        if payload.protocol_version != self.protocol_version.get_id() {
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

    fn handle_status_request(&mut self, payload: &status::RequestPayload) {
        self.write_packet(clientbound::status::ResponsePacket {
            json_payload: json!({
                "version": {
                    "name": "1.18.2 or not",
                    "protocol": self.protocol_version.get_id()
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
        });
    }

    fn handle_status_ping(&mut self, payload: &status::PingPayload) {
        self.write_packet(clientbound::status::PongPacket {
            payload: payload.payload,
        });
    }

    // Login

    fn handle_login_start(&mut self, payload: &login::StartPayload) {
        todo!()
    }

    fn handle_login_encryption_response(&mut self, payload: &login::EncryptionResponsePayload) {
        todo!()
    }

    fn handle_login_plugin_response(&mut self, payload: &login::PluginResponsePayload) {
        todo!()
    }

    // Play

    fn handle_play_teleport_confirm(&mut self, payload: &play::TeleportConfirmPayload) {
        todo!()
    }

    fn handle_play_query_block_nbt(&mut self, payload: &play::QueryBlockNBTPayload) {
        todo!()
    }

    fn handle_play_set_difficulty(&mut self, payload: &play::SetDifficultyPayload) {
        todo!()
    }

    fn handle_play_chat_message(&mut self, payload: &play::ChatMessagePayload) {
        todo!()
    }

    fn handle_play_client_status(&mut self, payload: &play::ClientStatusPayload) {
        todo!()
    }

    fn handle_play_client_settings(&mut self, payload: &play::ClientSettingsPayload) {
        todo!()
    }

    fn handle_play_tab_complete(&mut self, payload: &play::TabCompletePayload) {
        todo!()
    }

    fn handle_play_click_window_button(&mut self, payload: &play::ClickWindowButtonPayload) {
        todo!()
    }

    fn handle_play_click_window(&mut self, payload: &play::ClickWindowPayload) {
        todo!()
    }

    fn handle_play_close_window(&mut self, payload: &play::CloseWindowPayload) {
        todo!()
    }

    fn handle_play_plugin_message(&mut self, payload: &play::PluginMessagePayload) {
        todo!()
    }

    fn handle_play_edit_book(&mut self, payload: &play::EditBookPayload) {
        todo!()
    }

    fn handle_play_query_entity_nbt(&mut self, payload: &play::QueryEntityNBTPayload) {
        todo!()
    }

    fn handle_play_interact_entity(&mut self, payload: &play::InteractEntityPayload) {
        todo!()
    }

    fn handle_play_generate_structure(&mut self, payload: &play::GenerateStructurePayload) {
        todo!()
    }

    fn handle_play_keep_alive(&mut self, payload: &play::KeepAlivePayload) {
        todo!()
    }

    fn handle_play_lock_difficulty(&mut self, payload: &play::LockDifficultyPayload) {
        todo!()
    }

    fn handle_play_player_position(&mut self, payload: &play::PlayerPositionPayload) {
        todo!()
    }

    fn handle_play_player_position_and_rotation(
        &mut self,
        payload: &play::PlayerPositionAndRotationPayload,
    ) {
        todo!()
    }

    fn handle_play_player_rotation(&mut self, payload: &play::PlayerRotationPayload) {
        todo!()
    }

    fn handle_play_player_movement(&mut self, payload: &play::PlayerMovementPayload) {
        todo!()
    }

    fn handle_play_vehicle_move(&mut self, payload: &play::VehicleMovePayload) {
        todo!()
    }

    fn handle_play_steer_boat(&mut self, payload: &play::SteerBoatPayload) {
        todo!()
    }

    fn handle_play_pick_item(&mut self, payload: &play::PickItemPayload) {
        todo!()
    }

    fn handle_play_craft_recipe_request(&mut self, payload: &play::CraftRecipeRequestPayload) {
        todo!()
    }

    fn handle_play_player_abilities(&mut self, payload: &play::PlayerAbilitiesPayload) {
        todo!()
    }

    fn handle_play_player_digging(&mut self, payload: &play::PlayerDiggingPayload) {
        todo!()
    }

    fn handle_play_entity_action(&mut self, payload: &play::EntityActionPayload) {
        todo!()
    }

    fn handle_play_steer_vehicle(&mut self, payload: &play::SteerVehiclePayload) {
        todo!()
    }

    fn handle_play_pong(&mut self, payload: &play::PongPayload) {
        todo!()
    }

    fn handle_play_set_recipe_book_state(&mut self, payload: &play::SetRecipeBookStatePayload) {
        todo!()
    }

    fn handle_play_set_displayed_recipe(&mut self, payload: &play::SetDisplayedRecipePayload) {
        todo!()
    }

    fn handle_play_name_item(&mut self, payload: &play::NameItemPayload) {
        todo!()
    }

    fn handle_play_resource_pack_status(&mut self, payload: &play::ResourcePackStatusPayload) {
        todo!()
    }

    fn handle_play_advancement_tab(&mut self, payload: &play::AdvancementTabPayload) {
        todo!()
    }

    fn handle_play_select_trade(&mut self, payload: &play::SelectTradePayload) {
        todo!()
    }

    fn handle_play_set_beacon_effect(&mut self, payload: &play::SetBeaconEffectPayload) {
        todo!()
    }

    fn handle_play_held_item_change(&mut self, payload: &play::HeldItemChangePayload) {
        todo!()
    }

    fn handle_play_update_command_block(&mut self, payload: &play::UpdateCommandBlockPayload) {
        todo!()
    }

    fn handle_play_update_command_block_minecart(
        &mut self,
        payload: &play::UpdateCommandBlockMinecartPayload,
    ) {
        todo!()
    }

    fn handle_play_creative_inventory_action(
        &mut self,
        payload: &play::CreativeInventoryActionPayload,
    ) {
        todo!()
    }

    fn handle_play_update_jigsaw_block(&mut self, payload: &play::UpdateJigsawBlockPayload) {
        todo!()
    }

    fn handle_play_update_structure_block(&mut self, payload: &play::UpdateStructureBlockPayload) {
        todo!()
    }

    fn handle_play_update_sign_block(&mut self, payload: &play::UpdateSignBlockPayload) {
        todo!()
    }

    fn handle_play_animation(&mut self, payload: &play::AnimationPayload) {
        todo!()
    }

    fn handle_play_spectate(&mut self, payload: &play::SpectatePayload) {
        todo!()
    }

    fn handle_play_player_block_placement(&mut self, payload: &play::PlayerBlockPlacementPayload) {
        todo!()
    }

    fn handle_play_use_item(&mut self, payload: &play::UseItemPayload) {
        todo!()
    }
}
