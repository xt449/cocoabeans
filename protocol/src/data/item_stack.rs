use nbt::lib::Value;
use num_traits::FromPrimitive;
use registries::item::ItemRegistry;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use crate::io::{MinecraftReadable, MinecraftReader, MinecraftWritable, MinecraftWriter};

pub struct ItemStack {
    pub count: u8,
    pub id: ItemRegistry,
    pub nbt: Option<Value>,
}

impl ItemStack {
    pub fn empty() -> Self {
        return ItemStack {
            count: 0,
            id: ItemRegistry::air,
            nbt: None,
        };
    }

    // pub fn new(item: ItemRegistry, size: u8) -> Self {
    //     return ItemStack {
    //         count: size,
    //         id: item,
    //         nbt: None,
    //     };
    // }
    //
    // pub fn new_single(item: ItemRegistry) -> Self {
    //     return ItemStack {
    //         count: 1,
    //         id: item,
    //         nbt: None,
    //     };
    // }
}

impl ItemStack {
    pub fn init_nbt(&mut self) {
        if let None = self.nbt {
            self.nbt = Some(Value::Compound(HashMap::new()));
        }
    }

    pub fn get_nbt(&self) -> &Option<Value> {
        return &self.nbt;
    }
}

impl MinecraftReadable<Self> for ItemStack {
    fn deserialize_from(reader: &mut MinecraftReader) -> std::io::Result<Self> {
        let no_empty = reader.read_boolean()?;
        return if no_empty {
            let id: ItemRegistry = FromPrimitive::from_i32(reader.read_varint()?).ok_or(
                Error::new(ErrorKind::InvalidInput, "Can not convert from primitive"),
            )?;
            let count = reader.read_unsigned_byte()?;
            let nbt = reader.read_option::<Value>()?;

            Ok(ItemStack { count, id, nbt })
        } else {
            Ok(ItemStack::empty())
        };
    }
}

impl MinecraftWritable for ItemStack {
    fn serialize_to(&self, writer: &mut MinecraftWriter) {
        writer.write_boolean(self.count > 0);
        if self.count > 0 {
            writer.write_varint(self.id as usize as i32);
            writer.write_unsigned_byte(self.count);
            writer.write_json(&self.nbt);
        }
    }
}
