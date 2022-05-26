use blocks::BlockState;
use math::coordinate::Position;
use nbt::Value;
use std::collections::HashMap;
use std::ops::Deref;

use crate::data::chat::ChatComponent;
use crate::data::item_stack::ItemStack;
use crate::data::particle::Particle;
use crate::data::rotation::Rotation;
use crate::data::villager_data::VillagerData;
use crate::io::{MinecraftWritable, MinecraftWriter};

pub struct EntityMetadata {
    data: HashMap<u8, Box<Data>>,
}

const END_OF_ARRAY: u8 = 0xFF;

pub enum Data {
    Byte(i8),
    VarInt(i32),
    Float(f32),
    String(String),
    Chat(ChatComponent),
    OptionChat(Option<ChatComponent>),
    ItemStack(ItemStack),
    Boolean(bool),
    Rotation(Rotation),
    Position(Position),
    OptionPosition(Option<Position>),
    Direction(i32 /*VarInt*/),
    OptionUUID(Option<u128>),
    BlockState(Box<dyn BlockState>),
    NBT(Value),
    Particle(Particle),
    VillagerData(VillagerData),
    OptionVarInt(i32),
    Pose(i32 /*VarInt*/),
}

impl Data {
    pub fn get_id(&self) -> i32 {
        return match self {
            Data::Byte(_) => 0,
            Data::VarInt(_) => 1,
            Data::Float(_) => 2,
            Data::String(_) => 3,
            Data::Chat(_) => 4,
            Data::OptionChat(_) => 5,
            Data::ItemStack(_) => 6,
            Data::Boolean(_) => 7,
            Data::Rotation(_) => 8,
            Data::Position(_) => 9,
            Data::OptionPosition(_) => 10,
            Data::Direction(_) => 11,
            Data::OptionUUID(_) => 12,
            Data::BlockState(_) => 13,
            Data::NBT(_) => 14,
            Data::Particle(_) => 15,
            Data::VillagerData(_) => 16,
            Data::OptionVarInt(_) => 17,
            Data::Pose(_) => 18,
        };
    }
}

impl MinecraftWritable for EntityMetadata {
    fn serialize_to(&self, writer: &mut MinecraftWriter) {
        for entry in &self.data {
            writer.write_unsigned_byte(*entry.0);
            writer.write_varint(entry.1.get_id());
            match entry.1.deref() {
                Data::Byte(v) => writer.write_byte(*v),
                Data::VarInt(v) => writer.write_varint(*v),
                Data::Float(v) => writer.write_float(*v),
                Data::String(v) => writer.write_utf(v),
                Data::Chat(v) => writer.write_json(v),
                Data::OptionChat(v) => {
                    if let Some(v) = v {
                        writer.write_boolean(true);
                        writer.write_json(v);
                    } else {
                        writer.write_boolean(false);
                    }
                }
                Data::ItemStack(v) => writer.write(v),
                Data::Boolean(v) => writer.write_boolean(*v),
                Data::Rotation(v) => writer.write(v),
                Data::Position(v) => {}       /*writer.write(v)*/,// TODO
                Data::OptionPosition(v) => {} /*writer.write_option(v)*/,// TODO
                Data::Direction(v) => writer.write_varint(v.clone() as usize as i32),
                Data::OptionUUID(v) => {
                    if let Some(v) = v {
                        writer.write_boolean(true);
                        writer.write_uuid(*v);
                    } else {
                        writer.write_boolean(false);
                    }
                }
                Data::BlockState(v) => {
                    writer.write_varint(blocks::get_id_from_blockstate(v.deref()) as i32)
                }
                Data::NBT(v) => writer.write(v),
                Data::Particle(v) => writer.write(v),
                Data::VillagerData(v) => writer.write(v),
                Data::OptionVarInt(v) => writer.write_varint(*v),
                Data::Pose(v) => writer.write_varint(v.clone() as usize as i32),
            }
        }
        writer.write_unsigned_byte(END_OF_ARRAY);
        //self.to_writer(writer).unwrap();
    }
}
