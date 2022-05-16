use crate::data::chat::ChatComponent;
use crate::data::item_stack::ItemStack;
use crate::data::position::Position;
use crate::data::direction::Direction;
use generated::blocks::BlockState;
use nbt::lib::Blob;
use crate::data::particle::Particle;
use crate::data::villager_data::VillagerData;
use crate::data::pose::Pose;

pub struct EntityMetadata {
    data: Vec<Box<Data>>
}

pub enum Data {
    Byte(i8),
    VarInt(i32),
    Float(f32),
    String(String),
    Chat(ChatComponent),
    OptionChat(Option<ChatComponent>),
    ItemStack(ItemStack),
    Boolean(bool),
    Rotation(f32, f32, f32),
    Position(Position),
    OptionPosition(Option<Position>),
    Direction(Direction),
    OptionUUID(Option<u128>),
    BlockState(Box<dyn BlockState>),
    NBT(Blob),
    Particle(Particle),
    VillagerData(VillagerData),
    OptionVarInt(i32),
    Pose(Pose),
}