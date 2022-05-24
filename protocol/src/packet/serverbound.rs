use crate::io::MinecraftReader;
use crate::packet::packet_handler::IPacketHandler;

pub type ServerBoundPacketBuilder = fn(reader: MinecraftReader) -> Option<ServerBoundPacket>;

pub type ServerBoundPacket = Box<dyn ServerBoundPayload>;

pub trait ServerBoundPayload {
    fn handle(&self, listener: &mut dyn IPacketHandler);
}

pub mod handshaking {
    use crate::packet::packet_handler::{IPacketHandler, State};
    use crate::packet::serverbound::{ServerBoundPacketBuilder, ServerBoundPayload};
    use extensions::OptionFrom;

    // payloads

    pub struct HandshakePayload {
        pub protocol_version: i32,
        pub address: String,
        pub port: u16,
        pub next_state: State,
    }

    impl ServerBoundPayload for HandshakePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_handshaking_handshake(self);
        }
    }

    impl HandshakePayload {
        pub const BUILDER: ServerBoundPacketBuilder = |mut reader| {
            Some(Box::new(Self {
                protocol_version: reader.read_varint(),
                address: reader.read_utf(),
                port: reader.read_unsigned_short(),
                next_state: State::option_from(reader.read_varint() as i8)?,
            }))
        };
    }
}

pub mod status {
    use crate::packet::packet_handler::IPacketHandler;
    use crate::packet::serverbound::{ServerBoundPacketBuilder, ServerBoundPayload};

    // payloads

    pub struct RequestPayload {
        // no fields
    }

    impl ServerBoundPayload for RequestPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_status_request(self);
        }
    }

    impl RequestPayload {
        pub const BUILDER: ServerBoundPacketBuilder = |_| Some(Box::new(Self {}));
    }

    pub struct PingPayload {
        pub payload: i64,
    }

    impl ServerBoundPayload for PingPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_status_ping(self);
        }
    }

    impl PingPayload {
        pub const BUILDER: ServerBoundPacketBuilder = |mut reader| {
            Some(Box::new(Self {
                payload: reader.read_long(),
            }))
        };
    }
}

pub mod login {
    use crate::packet::packet_handler::IPacketHandler;
    use crate::packet::serverbound::ServerBoundPayload;

    // payloads

    pub struct StartPayload {
        username: String,
    }

    impl ServerBoundPayload for StartPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_login_start(self);
        }
    }

    pub struct EncryptionResponsePayload {
        secret: Vec<u8>,
        verification_token: Vec<u8>,
    }

    impl ServerBoundPayload for EncryptionResponsePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_login_encryption_response(self);
        }
    }

    pub struct PluginResponsePayload {
        message_id: i32,
        successful: bool,
        data: Vec<u8>,
    }

    impl ServerBoundPayload for PluginResponsePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_login_plugin_response(self);
        }
    }
}

pub mod play {
    // payloads

    use crate::data::block_face::BlockFace;
    use crate::data::identifier::Identifier;
    use crate::data::item_stack::ItemStack;
    use crate::data::position::Position;
    use crate::data::{
        advancement_tab, chat_mode, client, command_block, entity, hand, inventory, recipe_book,
        resource_pack, structure_block,
    };
    use crate::packet::packet_handler::IPacketHandler;
    use crate::packet::serverbound::ServerBoundPayload;

    pub struct TeleportConfirmPayload {
        pub transaction_id: i32, // VARINT
    }

    impl ServerBoundPayload for TeleportConfirmPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_teleport_confirm(self);
        }
    }

    pub struct QueryBlockNBTPayload {
        pub transaction_id: i32, // VARINT
        pub location: Position,
    }

    impl ServerBoundPayload for QueryBlockNBTPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_query_block_nbt(self);
        }
    }

    #[deprecated]
    pub struct SetDifficultyPayload {
        pub difficulty: i8,
    }

    impl ServerBoundPayload for SetDifficultyPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_set_difficulty(self);
        }
    }

    pub struct ChatMessagePayload {
        pub message: String,
    }

    impl ServerBoundPayload for ChatMessagePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_chat_message(self);
        }
    }

    pub struct ClientStatusPayload {
        pub action: client::Action, // VARINT
    }

    impl ServerBoundPayload for ClientStatusPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_client_status(self);
        }
    }

    pub struct ClientSettingsPayload {
        pub locale: String,
        pub view_distance: i8,
        pub chat_mode: chat_mode::ChatMode, // VARINT
        pub chat_colors: bool,
        pub skin_parts: u8,            // BITFLAG
        pub main_hand: hand::MainHand, // VARINT
        pub text_filtering: bool,      // always false
        pub server_listing: bool,
    }

    impl ServerBoundPayload for ClientSettingsPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_client_settings(self);
        }
    }

    pub struct TabCompletePayload {
        pub transaction_id: i32, // VARINT
        pub text: String,
    }

    impl ServerBoundPayload for TabCompletePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_tab_complete(self);
        }
    }

    pub struct ClickWindowButtonPayload {
        pub window_id: i8,
        pub button_id: i8,
    }

    impl ServerBoundPayload for ClickWindowButtonPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_click_window_button(self);
        }
    }

    pub struct ClickWindowPayload {
        pub window_id: u8, // actually unsigned
        pub state_id: i32, // VARINT
        pub slot: i16,
        pub button: i8,
        pub mode: inventory::ClickMode, // VARINT
        pub slots_count: i32,           // VARINT
        pub slots: Vec<(i16, ItemStack)>,
        pub carried_item: ItemStack, // must be empty for DropKey mode
    }

    impl ServerBoundPayload for ClickWindowPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_click_window(self);
        }
    }

    pub struct CloseWindowPayload {
        pub window_id: u8, // actually unsigned
    }

    impl ServerBoundPayload for CloseWindowPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_close_window(self);
        }
    }

    pub struct PluginMessagePayload {
        pub channel: Identifier,
        pub data: Vec<u8>, // length must be inferred from packet length
    }

    impl ServerBoundPayload for PluginMessagePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_plugin_message(self);
        }
    }

    pub struct EditBookPayload {
        pub slot: i32,            // VARINT
        pub count: i32,           // VARINT
        pub entries: Vec<String>, // length already given by previous field
        pub title: Option<String>,
    }

    impl ServerBoundPayload for EditBookPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_edit_book(self);
        }
    }

    pub struct QueryEntityNBTPayload {
        pub transaction_id: i32, // VARINT
        pub entity_id: i32,      // VARINT
    }

    impl ServerBoundPayload for QueryEntityNBTPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_query_entity_nbt(self);
        }
    }

    pub struct InteractEntityPayload {
        pub entity_id: i32, // VARINT
        pub interaction_type: entity::InteractionType,
        pub target_x: Option<f32>, // Optional unless interaction_type is InteractAt
        pub target_y: Option<f32>, // Optional unless interaction_type is InteractAt
        pub target_z: Option<f32>, // Optional unless interaction_type is InteractAt
        pub hand: hand::Hand,      // Optional unless interaction_type is Interact or InteractAt
        pub sneeking: bool,
    }

    impl ServerBoundPayload for InteractEntityPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_interact_entity(self);
        }
    }

    pub struct GenerateStructurePayload {
        pub location: Position,
        pub levels: i32, // VARINT
        pub keep_jigsaws: bool,
    }

    impl ServerBoundPayload for GenerateStructurePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_generate_structure(self);
        }
    }

    pub struct KeepAlivePayload {
        pub id: u64,
    }

    impl ServerBoundPayload for KeepAlivePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_keep_alive(self);
        }
    }

    #[deprecated]
    pub struct LockDifficultyPayload {
        pub locked: bool,
    }

    impl ServerBoundPayload for LockDifficultyPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_lock_difficulty(self);
        }
    }

    pub struct PlayerPositionPayload {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub on_ground: bool, // walking or swimming
    }

    impl ServerBoundPayload for PlayerPositionPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_player_position(self);
        }
    }

    pub struct PlayerPositionAndRotationPayload {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
        pub on_ground: bool, // walking or swimming
    }

    impl ServerBoundPayload for PlayerPositionAndRotationPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_player_position_and_rotation(self);
        }
    }

    pub struct PlayerRotationPayload {
        pub yaw: f32,
        pub pitch: f32,
        pub on_ground: bool, // walking or swimming
    }

    impl ServerBoundPayload for PlayerRotationPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_player_rotation(self);
        }
    }

    pub struct PlayerMovementPayload {
        pub on_ground: bool, // walking or swimming
    }

    impl ServerBoundPayload for PlayerMovementPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_player_movement(self);
        }
    }

    pub struct VehicleMovePayload {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
    }

    impl ServerBoundPayload for VehicleMovePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_vehicle_move(self);
        }
    }

    pub struct SteerBoatPayload {
        pub left_paddle: bool,
        pub right_paddle: bool,
    }

    impl ServerBoundPayload for SteerBoatPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_steer_boat(self);
        }
    }

    pub struct PickItemPayload {
        pub slot: i32, // VARINT
    }

    impl ServerBoundPayload for PickItemPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_pick_item(self);
        }
    }

    pub struct CraftRecipeRequestPayload {
        pub window_id: i8,
        pub recipe: Identifier,
        pub make_all: bool, // when shift clicking
    }

    impl ServerBoundPayload for CraftRecipeRequestPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_craft_recipe_request(self);
        }
    }

    pub struct PlayerAbilitiesPayload {
        pub flags: i8, // BITFLAG
    }

    impl ServerBoundPayload for PlayerAbilitiesPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_player_abilities(self);
        }
    }

    pub struct PlayerDiggingPayload {
        pub status: entity::Status, // VARINT
        pub location: Position,
        pub face: BlockFace, // BYTE
    }

    impl ServerBoundPayload for PlayerDiggingPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_player_digging(self);
        }
    }

    pub struct EntityActionPayload {
        pub entity_id: i32,         // VARINT
        pub action: entity::Action, // VARINT
        pub jump_boost: i32,        // VARINT (EFFECTIVELY optional unless action=HorseJumpStart)
    }

    impl ServerBoundPayload for EntityActionPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_entity_action(self);
        }
    }

    pub struct SteerVehiclePayload {
        pub sideways: f32,
        pub forward: f32,
        pub flags: i8, // BITFLAG
    }

    impl ServerBoundPayload for SteerVehiclePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_steer_vehicle(self);
        }
    }

    pub struct PongPayload {
        pub id: i32, // FULLINT
    }

    impl ServerBoundPayload for PongPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_pong(self);
        }
    }

    pub struct SetRecipeBookStatePayload {
        pub book_id: recipe_book::Type, // VARINT
        pub book_open: bool,
        pub filter_active: bool,
    }

    impl ServerBoundPayload for SetRecipeBookStatePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_set_recipe_book_state(self);
        }
    }

    pub struct SetDisplayedRecipePayload {
        pub recipe_id: Identifier,
    }

    impl ServerBoundPayload for SetDisplayedRecipePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_set_displayed_recipe(self);
        }
    }

    pub struct NameItemPayload {
        pub name: String,
    }

    impl ServerBoundPayload for NameItemPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_name_item(self);
        }
    }

    pub struct ResourcePackStatusPayload {
        pub result: resource_pack::Status, // VARINT
    }

    impl ServerBoundPayload for ResourcePackStatusPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_resource_pack_status(self);
        }
    }

    pub struct AdvancementTabPayload {
        pub action: advancement_tab::Action,
        pub tab_id: Option<Identifier>,
    }

    impl ServerBoundPayload for AdvancementTabPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_advancement_tab(self);
        }
    }

    pub struct SelectTradePayload {
        pub selected_slot: i32, // VARINT
    }

    impl ServerBoundPayload for SelectTradePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_select_trade(self);
        }
    }

    pub struct SetBeaconEffectPayload {
        pub primary_effect: registries::potion::PotionRegistry, // VARINT
        pub secondary_effect: registries::potion::PotionRegistry, // VARINT
    }

    impl ServerBoundPayload for SetBeaconEffectPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_set_beacon_effect(self);
        }
    }

    pub struct HeldItemChangePayload {
        pub slot: i16,
    }

    impl ServerBoundPayload for HeldItemChangePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_held_item_change(self);
        }
    }

    pub struct UpdateCommandBlockPayload {
        pub location: Position,
        pub command: String,
        pub mode: command_block::Mode, // VARINT
        pub flags: i8,                 // BITFLAG
    }

    impl ServerBoundPayload for UpdateCommandBlockPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_update_command_block(self);
        }
    }

    pub struct UpdateCommandBlockMinecartPayload {
        pub entity_id: i32, // VARINT
        pub command: String,
        pub track_output: bool,
    }

    impl ServerBoundPayload for UpdateCommandBlockMinecartPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_update_command_block_minecart(self);
        }
    }

    pub struct CreativeInventoryActionPayload {
        pub slot: i16,
        pub item: ItemStack,
    }

    impl ServerBoundPayload for CreativeInventoryActionPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_creative_inventory_action(self);
        }
    }

    pub struct UpdateJigsawBlockPayload {
        pub location: Position,
        pub name: Identifier,
        pub target: Identifier,
        pub pool: Identifier,
        pub final_state: String,
        pub joint_type: String, //rollable or aligned
    }

    impl ServerBoundPayload for UpdateJigsawBlockPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_update_jigsaw_block(self);
        }
    }

    pub struct UpdateStructureBlockPayload {
        pub location: Position,
        pub action: structure_block::Action, // VARINT
        pub mode: structure_block::Mode,     // VARINT
        pub name: String,
        pub offset_x: i8,
        pub offset_y: i8,
        pub offset_z: i8,
        pub size_x: i8,
        pub size_y: i8,
        pub size_z: i8,
        pub mirror: structure_block::Mirror,     // VARINT
        pub rotation: structure_block::Rotation, // VARINT
        pub metadata: String,
        pub integrety: f32,
        pub seed: i64, // VARLONG
        pub flags: i8, // BITFLAG
    }

    impl ServerBoundPayload for UpdateStructureBlockPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_update_structure_block(self);
        }
    }

    pub struct UpdateSignBlockPayload {
        pub location: Position,
        pub line_1: String,
        pub line_2: String,
        pub line_3: String,
        pub line_4: String,
    }

    impl ServerBoundPayload for UpdateSignBlockPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_update_sign_block(self);
        }
    }

    pub struct AnimationPayload {
        pub hand: hand::Hand,
    }

    impl ServerBoundPayload for AnimationPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_animation(self);
        }
    }

    pub struct SpectatePayload {
        pub target: u128,
    }

    impl ServerBoundPayload for SpectatePayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_spectate(self);
        }
    }

    pub struct PlayerBlockPlacementPayload {
        pub hand: hand::Hand,
        pub location: Position,
        pub face: BlockFace, // BYTE
        pub cusor_position: (f32, f32, f32),
        pub inside_block: bool,
    }

    impl ServerBoundPayload for PlayerBlockPlacementPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_player_block_placement(self);
        }
    }

    pub struct UseItemPayload {
        pub hand: hand::Hand,
    }

    impl ServerBoundPayload for UseItemPayload {
        fn handle(&self, listener: &mut dyn IPacketHandler) {
            listener.handle_play_use_item(self);
        }
    }

    // TODO
}
